/** Checksum preferences own the algorithm set, auto verification and rules. */
import type { AppConfig, ChecksumAlgorithm, ChecksumRule } from '@shared/types'

export interface ChecksumForm {
  [key: string]: unknown
  checksumAlgorithms: ChecksumAlgorithm[]
  checksumAutoVerify: boolean
  checksumAutoVerifyCategories: string[]
  checksumOnlineLookup: boolean
  checksumRules: ChecksumRule[]
}

export function buildChecksumForm(config: AppConfig): ChecksumForm {
  return {
    checksumAlgorithms: [...config.checksumAlgorithms],
    checksumAutoVerify: config.checksumAutoVerify,
    checksumAutoVerifyCategories: [...config.checksumAutoVerifyCategories],
    checksumOnlineLookup: config.checksumOnlineLookup,
    checksumRules: config.checksumRules.map((rule) => ({ ...rule })),
  }
}

/** Checksum settings never map to engine options. */
export function buildChecksumSystemConfig(): Record<string, string> {
  return {}
}

/** A new rule row with a stable id for the list key. */
export function createChecksumRule(index: number): ChecksumRule {
  return {
    id: `checksum-rule-${Date.now()}-${index}`,
    enabled: true,
    pattern: '',
    regex: false,
    mode: 'deny',
  }
}
