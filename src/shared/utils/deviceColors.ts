/** Persisted accent colors for paired LAN devices. */
import { reactive } from 'vue'

const STORAGE_KEY = 'rayload.lan-device-colors.v1'
export const deviceColors = reactive<Record<string, string>>({})
/**
 * Accent palette used when a device has no custom color. Settings, task list
 * and cards must all resolve through {@link resolveDeviceAccent}; using a
 * different default in one place is what made the chosen color look desynced.
 */
const ACCENT_PALETTE = [
  'var(--m3-primary)',
  'var(--m3-tertiary)',
  'var(--m3-secondary)',
  'var(--m3-success)',
  'var(--m3-status-waiting)',
  'var(--m3-error)',
] as const
let hydrated = false

function hydrate(): void {
  if (hydrated || typeof window === 'undefined') return
  hydrated = true
  try {
    const saved = JSON.parse(window.localStorage.getItem(STORAGE_KEY) ?? 'null') as unknown
    if (!saved || typeof saved !== 'object' || Array.isArray(saved)) return
    for (const [id, color] of Object.entries(saved)) {
      if (typeof color === 'string') deviceColors[id] = color
    }
  } catch {
    // Storage can be unavailable in hardened WebViews; defaults still work.
  }
}

export function getDeviceColor(id: string): string | undefined {
  hydrate()
  return deviceColors[id]
}

export function useDeviceColors(): Record<string, string> {
  hydrate()
  return deviceColors
}

export function setDeviceColor(id: string, color: string): void {
  hydrate()
  deviceColors[id] = color
  try {
    window.localStorage.setItem(STORAGE_KEY, JSON.stringify(deviceColors))
  } catch {
    // Keep the color for this session when storage is unavailable.
  }
}

/**
 * Resolves the accent for a device identity: the stored custom color when there
 * is one, otherwise a stable color derived from the identity so the same device
 * always gets the same accent across every view.
 */
export function resolveDeviceAccent(identity: string): string {
  const custom = getDeviceColor(identity)
  if (custom) return custom
  let hash = 0
  for (const char of identity) hash = (hash * 31 + char.charCodeAt(0)) >>> 0
  return ACCENT_PALETTE[hash % ACCENT_PALETTE.length]
}
