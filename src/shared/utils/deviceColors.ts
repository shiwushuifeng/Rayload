/**
 * Persisted accent colors for paired LAN devices.
 *
 * A device without an explicitly chosen color gets a stable color derived from
 * its identity, so the settings list, the color picker, the task rail and the
 * cards all show exactly the same flat value. Flat hex is deliberate: mixing the
 * color with the theme surface made the swatch in the picker differ from the
 * icon and the rail, which read as "the color is not applied".
 */
import { reactive } from 'vue'

const STORAGE_KEY = 'rayload.lan-device-colors.v1'
export const deviceColors = reactive<Record<string, string>>({})
let hydrated = false

/** Accent used when nothing identifies the device: the theme's own accent. */
export const DEFAULT_DEVICE_ACCENT = 'var(--m3-primary)'

/** Saturation and lightness of an identity-derived color. */
const DERIVED_SATURATION = 62
const DERIVED_LIGHTNESS = 52

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

/** Raw stored color of one key, without any composition. */
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

/** HSL to `#rrggbb`, so every consumer and the picker share one notation. */
function hslToHex(hue: number, saturation: number, lightness: number): string {
  const h = ((hue % 360) + 360) % 360
  const s = saturation / 100
  const l = lightness / 100
  const c = (1 - Math.abs(2 * l - 1)) * s
  const x = c * (1 - Math.abs(((h / 60) % 2) - 1))
  const m = l - c / 2
  const [r, g, b] =
    h < 60
      ? [c, x, 0]
      : h < 120
        ? [x, c, 0]
        : h < 180
          ? [0, c, x]
          : h < 240
            ? [0, x, c]
            : h < 300
              ? [x, 0, c]
              : [c, 0, x]
  const toHex = (value: number) =>
    Math.round((value + m) * 255)
      .toString(16)
      .padStart(2, '0')
  return `#${toHex(r)}${toHex(g)}${toHex(b)}`
}

/** Identity-derived flat color of a device. */
export function identityColor(identity: string): string {
  return hslToHex(identityHue(identity), DERIVED_SATURATION, DERIVED_LIGHTNESS)
}

function storedColor(keys: Array<string | null | undefined>): string | undefined {
  hydrate()
  for (const key of keys) {
    if (!key) continue
    const stored = deviceColors[key]
    if (stored) return stored
  }
  return undefined
}

/**
 * Flat color of a device: the explicit choice when there is one, otherwise the
 * identity-derived color. Used by the rail, the icon, the detail header and the
 * color picker, so all of them agree.
 */
export function resolveDeviceColor(...keys: Array<string | null | undefined>): string | null {
  const stored = storedColor(keys)
  if (stored) return stored
  for (const key of keys) {
    if (key) return identityColor(key)
  }
  return null
}

/** Accent of a device, falling back to the theme accent with no identity. */
export function resolveDeviceAccent(...keys: Array<string | null | undefined>): string {
  return resolveDeviceColor(...keys) ?? DEFAULT_DEVICE_ACCENT
}
