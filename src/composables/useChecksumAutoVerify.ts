/**
 * @fileoverview Post-download checksum verification.
 *
 * Only GitHub sources are queried by default: that is where release assets carry
 * a published digest and where CIRCL most often knows the file. A download
 * without a published digest stays silent instead of warning the user.
 */
import { calculateFileDigests, fetchGitHubAssetDigest, lookupKnownDigest } from '@/api/checksum'
import { resolveExistingTaskFile } from '@/composables/useTaskFileTarget'
import { getTaskUri } from '@shared/utils'
import { logger } from '@shared/logger'
import type { Aria2Task, AppConfig, ChecksumRule } from '@shared/types'

/** Hosts whose downloads are looked up automatically. */
const LOOKUP_HOSTS = ['github.com', 'www.github.com', 'objects.githubusercontent.com']

/** True when the source URL belongs to a host whose digests are published. */
export function isPublishedDigestSource(url: string): boolean {
  const match = /^https:\/\/([^/]+)/i.exec(url.trim())
  if (!match) return false
  return LOOKUP_HOSTS.includes(match[1].toLowerCase())
}

/** Converts a wildcard pattern (`*`, `?`) into an anchored, case-insensitive pattern. */
export function wildcardToRegExp(pattern: string): RegExp {
  const escaped = pattern
    .replace(/[.+^${}()|[\]\\]/g, '\\$&')
    .replace(/\*/g, '.*')
    .replace(/\?/g, '.')
  return new RegExp(`^${escaped}$`, 'i')
}

export function checksumRuleMatches(rule: ChecksumRule, url: string): boolean {
  if (!rule.enabled || !rule.pattern) return false
  if (rule.regex) {
    try {
      return new RegExp(rule.pattern, 'i').test(url)
    } catch {
      // An invalid expression is ignored rather than blocking verification.
      return false
    }
  }
  return wildcardToRegExp(rule.pattern).test(url)
}

/** Deny rules skip verification; an allow rule overrides a broader deny. */
export function isVerificationSkipped(rules: readonly ChecksumRule[], url: string): boolean {
  const matching = rules.filter((rule) => checksumRuleMatches(rule, url))
  if (matching.some((rule) => rule.mode === 'allow')) return false
  return matching.some((rule) => rule.mode === 'deny')
}

/** The category filter selects by file extension; an empty selection means all. */
export function categoryIncluded(config: AppConfig, path: string): boolean {
  const selected = config.checksumAutoVerifyCategories
  if (selected.length === 0) return true
  const extension = path.split('.').pop()?.toLowerCase() ?? ''
  if (!extension) return false
  return config.fileCategories.some(
    (category) =>
      selected.includes(category.label) &&
      category.extensions.some((item) => item.toLowerCase().replace(/^\./, '') === extension),
  )
}

export interface VerifyDeps {
  config: () => AppConfig
  t: (key: string, params?: Record<string, unknown>) => string
  messageSuccess: (content: string) => void
  messageWarning: (content: string) => void
}

type Comparison = 'match' | 'mismatch' | 'unknown'

/** Compares computed digests against every published digest that is known. */
async function compareWithPublished(
  source: string,
  paths: { url: string; digests: Record<string, string> },
): Promise<Comparison> {
  let compared = false
  if (isPublishedDigestSource(paths.url)) {
    const release = await fetchGitHubAssetDigest(paths.url).catch(() => null)
    if (release) {
      const computed = paths.digests[release.algorithm]
      if (computed) {
        compared = true
        if (computed.toLowerCase() !== release.digest.toLowerCase()) return 'mismatch'
      }
    }
  }
  const known = await lookupKnownDigest(paths.digests.sha1 ?? null, paths.digests.sha256 ?? null).catch(() => [])
  for (const entry of known) {
    for (const [algorithm, digest] of Object.entries(paths.digests)) {
      const published = algorithm === 'sha1' ? entry.sha1 : algorithm === 'sha256' ? entry.sha256 : null
      if (!published) continue
      compared = true
      if (digest.toLowerCase() !== published.toLowerCase()) return 'mismatch'
    }
  }
  logger.debug('ChecksumAutoVerify.compare', `source=${source} compared=${compared}`)
  return compared ? 'match' : 'unknown'
}

/**
 * Verifies a finished download when the feature is enabled, the file matches the
 * category filter and no deny rule suppresses it.
 */
export async function verifyCompletedDownload(task: Aria2Task, deps: VerifyDeps): Promise<void> {
  const config = deps.config()
  if (!config.checksumAutoVerify || task.remote) return

  const url = getTaskUri(task)
  if (!url) return
  if (isVerificationSkipped(config.checksumRules, url)) {
    logger.debug('ChecksumAutoVerify.skip', `gid=${task.gid} reason=rule`)
    return
  }

  const path = await resolveExistingTaskFile(task).catch(() => null)
  if (!path) return
  if (!categoryIncluded(config, path)) {
    logger.debug('ChecksumAutoVerify.skip', `gid=${task.gid} reason=category`)
    return
  }

  const name = path.split(/[\\/]/).pop() || path
  const algorithms = config.checksumAlgorithms
  const digests = await calculateFileDigests(path, algorithms.length > 0 ? algorithms : ['sha256']).catch(
    (error: unknown) => {
      logger.warn('ChecksumAutoVerify.digest', error)
      return null
    },
  )
  if (!digests) return

  const comparison = await compareWithPublished(url, { url, digests: digests.digests })
  if (comparison === 'mismatch') {
    deps.messageWarning(deps.t('task.checksum-auto-verify-mismatch', { taskName: name }))
    return
  }
  if (comparison === 'match') {
    deps.messageSuccess(deps.t('task.checksum-auto-verify-match', { taskName: name }))
  }
}
