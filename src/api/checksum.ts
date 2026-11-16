/** @fileoverview File digests, checksum export and published-digest lookups. */
import { invoke } from '@tauri-apps/api/core'
import type { ChecksumAlgorithm } from '@shared/types'

/** Digests computed for one file, plus which ones came from the cache. */
export interface FileDigests {
  path: string
  size: number
  mtimeSecs: number
  /** Algorithm name to lowercase hex digest. */
  digests: Record<string, string>
  /** Algorithms answered by the cache without reading the file again. */
  cached: string[]
  elapsedMs: number
}

export interface ReleaseAssetDigest {
  algorithm: string
  digest: string
}

/** One public database that knows a hash. */
export interface KnownDigestEntry {
  source: string
  fileName: string | null
  sha1: string | null
  sha256: string | null
}

/**
 * Calculates the requested algorithms in one backend pass. The backend reuses
 * cached digests for an unchanged file, so calling this repeatedly is cheap.
 */
export function calculateFileDigests(
  path: string,
  algorithms: ChecksumAlgorithm[],
  requestId?: string,
): Promise<FileDigests> {
  return invoke<FileDigests>('file_digests', { path, algorithms, requestId })
}

/** Stops a running calculation; the promise rejects with the cancel message. */
export function cancelFileDigests(requestId: string): Promise<void> {
  return invoke<void>('cancel_file_digests', { requestId })
}

/** Writes a `digest  filename` checksum file next to the download. */
export function exportChecksumFile(path: string, algorithm: string, digest: string): Promise<string> {
  return invoke<string>('export_checksum_file', { path, algorithm, digest })
}

/** Reads the digest GitHub publishes for a release asset, when available. */
export function fetchGitHubAssetDigest(url: string): Promise<ReleaseAssetDigest | null> {
  return invoke<ReleaseAssetDigest | null>('github_asset_digest', { url })
}

/** Asks CIRCL Hash Lookup about a file by SHA-1 and/or SHA-256. */
export function lookupKnownDigest(sha1: string | null, sha256: string | null): Promise<KnownDigestEntry[]> {
  return invoke<KnownDigestEntry[]>('online_checksum_lookup', { sha1, sha256 })
}

/** Empties the SQLite digest cache and returns how many rows were removed. */
export function clearChecksumCache(): Promise<number> {
  return invoke<number>('clear_checksum_cache')
}
