/** Localized release highlights shown after an update and at the end of setup. */
export interface WhatsNewEntry {
  version: string
  highlights: string[]
}

export const WHATS_NEW_ENTRIES: WhatsNewEntry[] = [
  {
    version: '1.0.4',
    highlights: [
      'whats-new.1-0-4-lan',
      'whats-new.1-0-4-history',
      'whats-new.1-0-4-permission',
      'whats-new.1-0-4-settings',
      'whats-new.1-0-4-wizard',
    ],
  },
]

export const WHATS_NEW_STORAGE_KEY = 'rayload-whats-new-seen'

export function whatsNewFor(version: string): WhatsNewEntry | null {
  return WHATS_NEW_ENTRIES.find((entry) => entry.version === version) ?? null
}

export function readWhatsNewSeen(): string {
  try {
    return window.localStorage.getItem(WHATS_NEW_STORAGE_KEY) ?? ''
  } catch {
    return ''
  }
}

export function markWhatsNewSeen(version: string): void {
  try {
    window.localStorage.setItem(WHATS_NEW_STORAGE_KEY, version)
  } catch {
    // Hardened WebViews can reject storage; the dialog simply shows again.
  }
}
