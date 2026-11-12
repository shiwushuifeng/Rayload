/** @fileoverview Classifies technical download errors for friendly UI copy. */

export type FriendlyErrorKind =
  | 'not-found'
  | 'forbidden'
  | 'timeout'
  | 'space'
  | 'network'
  | 'tls'
  | 'server'
  | 'disk'
  | 'unknown'

const HTTP_NOT_FOUND = /\b404\b|not\s+found/i
const HTTP_FORBIDDEN = /\b40[13]\b|forbidden|unauthorized/i
const TIMEOUT = /timeout|timed\s*out|etimedout|deadline\s+exceeded/i
const NO_SPACE = /no\s+space|enospc|disk\s+full|not\s+enough\s+space|空间不足/i
const TLS = /ssl|tls|certificate|cert\s+verify|handshake/i
const NETWORK =
  /network|connect(?:ion)?\s+(?:refused|reset|failed)|unreachable|dns|name\s+resolution|econnrefused|econnreset|enetunreach|failed\s+to\s+fetch|fetch\s+failed|load\s+failed|networkerror/i
const SERVER = /\b5\d{2}\b|server\s+unavailable|bad\s+gateway|service\s+unavailable/i
const DISK = /read-only|permission\s+denied|eacces|erofs|file\s+create|io\s+error/i

/** Maps an engine or network error to a stable UI category. */
export function friendlyErrorKind(raw: string): FriendlyErrorKind {
  const value = raw.trim()
  if (!value) return 'unknown'
  if (HTTP_NOT_FOUND.test(value)) return 'not-found'
  if (HTTP_FORBIDDEN.test(value)) return 'forbidden'
  if (TIMEOUT.test(value)) return 'timeout'
  if (NO_SPACE.test(value)) return 'space'
  if (TLS.test(value)) return 'tls'
  if (NETWORK.test(value)) return 'network'
  if (SERVER.test(value)) return 'server'
  if (DISK.test(value)) return 'disk'
  return 'unknown'
}

export interface ErrorLogContext {
  gid?: string
  url?: string
  source?: string
  extra?: Record<string, string | number | undefined>
}

/** Builds a pasteable diagnostic log without hiding the original error. */
export function buildErrorLog(raw: string, context: ErrorLogContext = {}): string {
  const lines = ['Rayload error log', `time: ${new Date().toISOString()}`, `error: ${raw}`]
  if (context.source) lines.push(`source: ${context.source}`)
  if (context.gid) lines.push(`gid: ${context.gid}`)
  if (context.url) lines.push(`url: ${context.url}`)
  for (const [key, value] of Object.entries(context.extra ?? {})) {
    if (value !== undefined && value !== '') lines.push(`${key}: ${value}`)
  }
  return lines.join('\n')
}
