//! Cached file digests keyed by path, size and modification time.
//!
//! Re-reading a multi-gigabyte file to show its checksum is wasteful, so every
//! successful computation is stored. A lookup is only a hit when the size and
//! the modification time still match, which is how a rewritten file invalidates
//! itself without any explicit bookkeeping.
use super::Database;
use crate::error::AppError;

/// One algorithm result restored from the cache.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CachedDigest {
    pub algorithm: String,
    pub digest: String,
}

impl Database {
    /// Returns the cached digests of one file version, ordered by algorithm.
    pub async fn read_cached_digests(
        &self,
        path: &str,
        size: u64,
        mtime_secs: i64,
    ) -> Result<Vec<CachedDigest>, AppError> {
        let conn = self.connection().await?;
        let mut statement = conn.prepare(
            "SELECT algorithm, digest FROM file_hash_cache \
             WHERE path = ?1 AND size = ?2 AND mtime_secs = ?3 ORDER BY algorithm",
        )?;
        let rows = statement.query_map((path, size as i64, mtime_secs), |row| {
            Ok(CachedDigest {
                algorithm: row.get(0)?,
                digest: row.get(1)?,
            })
        })?;
        let mut digests = Vec::new();
        for row in rows {
            digests.push(row?);
        }
        Ok(digests)
    }

    /// Stores the digests of one file version and drops stale versions of it.
    pub async fn write_cached_digests(
        &self,
        path: &str,
        size: u64,
        mtime_secs: i64,
        digests: &[(String, String)],
    ) -> Result<(), AppError> {
        if digests.is_empty() {
            return Ok(());
        }
        let conn = self.connection().await?;
        conn.execute(
            "DELETE FROM file_hash_cache WHERE path = ?1 AND (size <> ?2 OR mtime_secs <> ?3)",
            (path, size as i64, mtime_secs),
        )?;
        let mut statement = conn.prepare(
            "INSERT INTO file_hash_cache (path, size, mtime_secs, algorithm, digest) \
             VALUES (?1, ?2, ?3, ?4, ?5) \
             ON CONFLICT(path, size, mtime_secs, algorithm) DO UPDATE SET \
               digest = excluded.digest, computed_at = CURRENT_TIMESTAMP",
        )?;
        for (algorithm, digest) in digests {
            statement.execute((path, size as i64, mtime_secs, algorithm, digest))?;
        }
        Ok(())
    }

    /// Empties the cache and returns how many rows were removed.
    pub async fn clear_cached_digests(&self) -> Result<usize, AppError> {
        let conn = self.connection().await?;
        Ok(conn.execute("DELETE FROM file_hash_cache", [])?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn caches_digests_per_file_version() {
        let db = Database::open_in_memory().unwrap();
        db.write_cached_digests(
            "/downloads/a.bin",
            10,
            1_700_000_000,
            &[("sha256".into(), "aa".into())],
        )
        .await
        .unwrap();

        let hit = db
            .read_cached_digests("/downloads/a.bin", 10, 1_700_000_000)
            .await
            .unwrap();
        assert_eq!(
            hit,
            vec![CachedDigest {
                algorithm: "sha256".into(),
                digest: "aa".into(),
            }]
        );

        // A rewritten file has a new size or mtime and must miss.
        let miss = db
            .read_cached_digests("/downloads/a.bin", 11, 1_700_000_000)
            .await
            .unwrap();
        assert!(miss.is_empty());
    }

    #[tokio::test]
    async fn keeps_one_row_per_algorithm_and_drops_stale_versions() {
        let db = Database::open_in_memory().unwrap();
        db.write_cached_digests("/f", 1, 1, &[("md5".into(), "11".into())])
            .await
            .unwrap();
        db.write_cached_digests(
            "/f",
            1,
            1,
            &[("md5".into(), "22".into()), ("sha1".into(), "33".into())],
        )
        .await
        .unwrap();

        let rows = db.read_cached_digests("/f", 1, 1).await.unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].algorithm, "md5");
        assert_eq!(rows[0].digest, "22");

        db.write_cached_digests("/f", 2, 5, &[("md5".into(), "44".into())])
            .await
            .unwrap();
        assert!(db.read_cached_digests("/f", 1, 1).await.unwrap().is_empty());
        assert_eq!(db.clear_cached_digests().await.unwrap(), 1);
    }
}
