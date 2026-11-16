/**
 * Persisted accent colors for paired LAN devices.
 *
 * One resolver owns the accent used by the settings list, the task rail and the
 * cards. Stored colors are composed with the current theme surface so the hue
 * the user picked survives a light/dark switch instead of looking like a
 * different color, and a device without a stored color falls back to the theme
 * accent rather than to a hash palette (which used to make the same device look
 * green in one view and pink in another).
 */
import { reactive } from 'vue'

const STORAGE_KEY = 'rayload.lan-device-colors.v1'
export const deviceColors = reactive<Record<string, string>>({})
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

function persist(): void {
  try {
    window.localStorage.setItem(STORAGE_KEY, JSON.stringify(deviceColors))
  } catch {
    // Keep the color for this session when storage is unavailable.
  }
}

/** Raw stored color of one key, without any theme composition. */
export function getDeviceColor(id: string): string | undefined {
  hydrate()
  return deviceColors[id]
}

export function useDeviceColors(): Record<string, string> {
  hydrate()
  return deviceColors
}

/**
 * Stores a device color under its id and, when given, an alias such as the
 * device name. Remote tasks only carry the id or the name, so writing both keeps
 * the list and the cards in agreement.
 */
export function setDeviceColor(id: string, color: string, alias?: string): void {
  hydrate()
  deviceColors[id] = color
  const aliasKey = alias?.trim()
  if (aliasKey && aliasKey !== id) deviceColors[aliasKey] = color
  persist()
}

/**
 * Blends a stored color with the current theme surface so it follows light and
 * dark themes while keeping the chosen hue.
 */
export function themedDeviceColor(base: string): string {
  return `color-mix(in srgb, ${base} 86%, var(--m3-surface-container))`
}

/** Accent used when a device has no custom color: the theme's own accent. */
export const DEFAULT_DEVICE_ACCENT = 'var(--m3-primary)'

/**
 * Resolves the accent from the first key that has a stored color. Callers pass
 * every identity a device can appear under (id, name, fingerprint) so the same
 * device always resolves to the same accent.
 */
export function resolveDeviceAccent(...keys: Array<string | null | undefined>): string {
  hydrate()
  for (const key of keys) {
    if (!key) continue
    const stored = deviceColors[key]
    if (stored) return themedDeviceColor(stored)
  }
  return DEFAULT_DEVICE_ACCENT
}
