import { describe, expect, it } from 'vitest'
import {
  categoryIncluded,
  checksumRuleMatches,
  isPublishedDigestSource,
  isVerificationSkipped,
  wildcardToRegExp,
} from '../useChecksumAutoVerify'
import { createDefaultAppConfig } from '@shared/utils/configHydration'
import type { ChecksumRule } from '@shared/types'

function rule(overrides: Partial<ChecksumRule>): ChecksumRule {
  return { id: 'r', enabled: true, pattern: '*', regex: false, mode: 'deny', ...overrides }
}

describe('checksum auto verify helpers', () => {
  it('translates wildcards into anchored patterns', () => {
    expect(wildcardToRegExp('*.zip').test('tool-1.2.zip')).toBe(true)
    expect(wildcardToRegExp('*.zip').test('tool-1.2.7z')).toBe(false)
    expect(wildcardToRegExp('https://example.com/*').test('https://example.com/a/b')).toBe(true)
    // Special characters must stay literal instead of acting as regex syntax.
    expect(wildcardToRegExp('a+b.zip').test('a+b.zip')).toBe(true)
    expect(wildcardToRegExp('a+b.zip').test('aab.zip')).toBe(false)
  })

  it('reads regex rules and ignores invalid expressions', () => {
    expect(
      checksumRuleMatches(rule({ pattern: '/releases/download/', regex: true }), 'https://x/releases/download/a'),
    ).toBe(true)
    expect(checksumRuleMatches(rule({ pattern: '(', regex: true }), 'https://x')).toBe(false)
    expect(checksumRuleMatches(rule({ enabled: false }), 'https://x')).toBe(false)
  })

  it('lets allow rules override a broader deny', () => {
    const rules = [
      rule({ id: 'deny', pattern: '*', mode: 'deny' }),
      rule({ id: 'allow', pattern: '*trusted*', mode: 'allow' }),
    ]
    expect(isVerificationSkipped(rules, 'https://cdn.example.com/trusted/file.zip')).toBe(false)
    expect(isVerificationSkipped(rules, 'https://cdn.example.com/file.zip')).toBe(true)
    expect(isVerificationSkipped([], 'https://cdn.example.com/file.zip')).toBe(false)
  })

  it('only treats published-digest hosts as lookup sources', () => {
    expect(isPublishedDigestSource('https://github.com/o/r/releases/download/v1/a.apk')).toBe(true)
    expect(isPublishedDigestSource('https://objects.githubusercontent.com/x')).toBe(true)
    expect(isPublishedDigestSource('https://example.com/a.apk')).toBe(false)
    expect(isPublishedDigestSource('ftp://github.com/a')).toBe(false)
  })

  it('filters by the configured categories unless every category is selected', () => {
    const config = createDefaultAppConfig()
    expect(categoryIncluded(config, '/downloads/setup.apk')).toBe(true)

    const filtered = { ...config, checksumAutoVerifyCategories: ['video'] }
    const video = { label: 'video', extensions: ['mp4'], directory: '', directoryMode: 'relative' as const }
    const archives = { label: 'archives', extensions: ['zip'], directory: '', directoryMode: 'relative' as const }
    const withCategories = { ...filtered, fileCategories: [video, archives] }
    expect(categoryIncluded(withCategories, '/downloads/movie.MP4')).toBe(true)
    expect(categoryIncluded(withCategories, '/downloads/archive.zip')).toBe(false)
    expect(categoryIncluded(withCategories, '/downloads/no-extension')).toBe(false)
  })
})
