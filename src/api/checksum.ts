/** @fileoverview Local file digests and GitHub release-asset digest lookup. */
import { invoke } from '@tauri-apps/api/core'

export interface FileDigests {
  md5: string
  sha1: string
  sha256: string
  sha512: string
  size: number
}

export interface ReleaseAssetDigest {
  algorithm: string
  digest: string
}

/** Calculates MD5, SHA-1, SHA-256 and SHA-512 in one backend pass. */
export function calculateFileDigests(path: string): Promise<FileDigests> {
  return invoke<FileDigests>('file_digests', { path })
}

/** Reads the digest GitHub publishes for a release asset, when available. */
export function fetchGitHubAssetDigest(url: string): Promise<ReleaseAssetDigest | null> {
  return invoke<ReleaseAssetDigest | null>('github_asset_digest', { url })
}
