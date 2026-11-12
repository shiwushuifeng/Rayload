import { describe, expect, it } from 'vitest'
import { buildErrorLog, friendlyErrorKind } from '../friendlyError'

describe('friendly error classification', () => {
  it('maps common download failures to readable categories', () => {
    expect(friendlyErrorKind('HTTP 404 Not Found')).toBe('not-found')
    expect(friendlyErrorKind('HTTP 403 Forbidden')).toBe('forbidden')
    expect(friendlyErrorKind('connect timeout after 30s')).toBe('timeout')
    expect(friendlyErrorKind('No space left on device (ENOSPC)')).toBe('space')
    expect(friendlyErrorKind('network is unreachable')).toBe('network')
    expect(friendlyErrorKind('SSL certificate verify failed')).toBe('tls')
    expect(friendlyErrorKind('HTTP 503 Service Unavailable')).toBe('server')
    expect(friendlyErrorKind('permission denied while creating file')).toBe('disk')
  })

  it('keeps unmatched errors in the generic category', () => {
    expect(friendlyErrorKind('weird engine answer')).toBe('unknown')
    expect(friendlyErrorKind('')).toBe('unknown')
  })

  it('builds a pasteable log with context but not secrets', () => {
    const log = buildErrorLog('HTTP 404', { gid: 'g1', url: 'https://example.com/a.zip', source: 'test' })
    expect(log).toContain('gid: g1')
    expect(log).toContain('url: https://example.com/a.zip')
    expect(log).toContain('error: HTTP 404')
  })
})
