/** Persisted accent colors for paired LAN devices. */
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
