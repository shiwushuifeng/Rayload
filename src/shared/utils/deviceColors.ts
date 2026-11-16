/**
 * Persisted accent colors for paired LAN devices.
 *
 * A device without an explicitly chosen color gets a stable color derived from
 * its identity, so the settings list, the task rail and the cards always agree
 * and each device looks different. Cards also get a contrasting hue for their
 * background gradient, which is what gives the rail and the card body their
 * deliberate clash instead of a flat wash of one color.
 *
 * Stored and generated colors alike are blended with the current theme surface,
 * so switching between light and dark keeps the hue without looking off.
 */
import { reactive } from 'vue'

const STORAGE_KEY = 'rayload.lan-device-colors.v1'
export const deviceColors = reactive<Record<string, string>>({})
let hydrated = false

/** Accent used when nothing identifies the device: the theme's own accent. */
export const DEFAULT_DEVICE_ACCENT = 'var(--m3-primary)'
/** Hue difference between the rail and the card gradient. */
const CONTRAST_HUE_SHIFT = 152

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
 * device name, because remote tasks only carry one of the two.
 */
export function setDeviceColor(id: string, color: string, alias?: string): void {
  hydrate()
  deviceColors[id] = color
  const aliasKey = alias?.trim()
  if (aliasKey && aliasKey !== id) deviceColors[aliasKey] = color
  persist()
}

/** Clears an explicit color so the device falls back to its derived hue. */
export function clearDeviceColor(id: string, alias?: string): void {
  hydrate()
  delete deviceColors[id]
  const aliasKey = alias?.trim()
  if (aliasKey && aliasKey !== id) delete deviceColors[aliasKey]
  persist()
}

/** Stable hue (0–359) derived from a device identity. */
export function identityHue(identity: string): number {
  let hash = 2166136261
  for (let index = 0; index < identity.length; index += 1) {
    hash ^= identity.charCodeAt(index)
    hash = Math.imul(hash, 16777619)
  }
  return (hash >>> 0) % 360
}

function hsl(hue: number, saturation: number, lightness: number): string {
  return `hsl(${Math.round((hue + 360) % 360)} ${saturation}% ${lightness}%)`
}

/** Blends a color with the theme surface so light and dark keep the hue. */
export function themedDeviceColor(base: string): string {
  return `color-mix(in srgb, ${base} 86%, var(--m3-surface-container))`
}

/** Hue of a stored `#rrggbb` color, or null when it cannot be parsed. */
function hexHue(color: string): number | null {
  const match = /^#([0-9a-f]{6})$/i.exec(color.trim())
  if (!match) return null
  const value = Number.parseInt(match[1], 16)
  const red = ((value >> 16) & 255) / 255
  const green = ((value >> 8) & 255) / 255
  const blue = (value & 255) / 255
  const max = Math.max(red, green, blue)
  const min = Math.min(red, green, blue)
  const delta = max - min
  if (delta === 0) return 0
  let hue: number
  if (max === red) hue = ((green - blue) / delta) % 6
  else if (max === green) hue = (blue - red) / delta + 2
  else hue = (red - green) / delta + 4
  return Math.round(hue * 60)
}

function resolveHue(keys: Array<string | null | undefined>): number | null {
  hydrate()
  for (const key of keys) {
    if (!key) continue
    const stored = deviceColors[key]
    if (stored) return hexHue(stored) ?? identityHue(key)
  }
  for (const key of keys) {
    if (key) return identityHue(key)
  }
  return null
}

/** Accent for the rail, the device icon and the detail header. */
export function resolveDeviceAccent(...keys: Array<string | null | undefined>): string {
  const hue = resolveHue(keys)
  if (hue === null) return DEFAULT_DEVICE_ACCENT
  hydrate()
  const stored = keys.map((key) => (key ? deviceColors[key] : undefined)).find(Boolean)
  return themedDeviceColor(stored ?? hsl(hue, 62, 52))
}

/** Contrasting accent used by the card background gradient. */
export function resolveDeviceContrastAccent(...keys: Array<string | null | undefined>): string {
  const hue = resolveHue(keys)
  if (hue === null) return 'var(--m3-tertiary)'
  return themedDeviceColor(hsl(hue + CONTRAST_HUE_SHIFT, 58, 56))
}
