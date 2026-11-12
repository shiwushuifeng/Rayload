/** @fileoverview Android in-app GitHub release check. */
import { invoke } from '@tauri-apps/api/core'

export interface GithubUpdateInfo {
  version: string
  name: string
  body: string | null
  releaseUrl: string
  prerelease: boolean
  publishedAt: string | null
  currentVersion: string
  updateAvailable: boolean
}

export function checkGithubUpdate(
  currentVersion: string,
  stableOnly: boolean,
  proxy: string | null,
): Promise<GithubUpdateInfo> {
  return invoke<GithubUpdateInfo>('check_github_update', {
    currentVersion,
    stableOnly,
    proxy,
  })
}
