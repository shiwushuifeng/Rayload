/** Persisted visibility for destinations in the compact bottom navigation. */
import { computed, reactive } from 'vue'

export const NAVIGATION_ITEM_KEYS = [
  'all',
  'progress',
  'failed',
  'completed',
  'keep-awake',
  'statistics',
  'trash',
  'about',
] as const

export type NavigationItemKey = (typeof NAVIGATION_ITEM_KEYS)[number]
export type NavigationVisibility = Record<NavigationItemKey, boolean>

// v2 hides the secondary task scopes by default on a fresh configuration.
const STORAGE_KEY = 'rayload.bottom-navigation.visibility.v2'

function createDefaults(): NavigationVisibility {
  return {
    all: true,
    progress: false,
    failed: false,
    completed: false,
    'keep-awake': true,
    statistics: false,
    trash: true,
    about: true,
  }
}

const visibility = reactive<NavigationVisibility>(createDefaults())
let hydrated = false

function persist(): void {
  if (typeof window === 'undefined') return
  try {
    window.localStorage.setItem(STORAGE_KEY, JSON.stringify(visibility))
  } catch {
    // Keep the in-memory configuration when storage is unavailable.
  }
}

function hydrate(): void {
  if (hydrated) return
  hydrated = true
  if (typeof window === 'undefined') return

  try {
    const saved = JSON.parse(window.localStorage.getItem(STORAGE_KEY) ?? 'null') as unknown
    if (!saved || typeof saved !== 'object' || Array.isArray(saved)) return
    for (const key of NAVIGATION_ITEM_KEYS) {
      const value = (saved as Record<string, unknown>)[key]
      if (typeof value === 'boolean') visibility[key] = value
    }
  } catch {
    // Invalid or unavailable storage falls back to all destinations visible.
  }
}

export function useNavigationVisibility() {
  hydrate()

  const visibleItems = computed(() => NAVIGATION_ITEM_KEYS.filter((key) => visibility[key]))

  function isItemVisible(key: NavigationItemKey): boolean {
    hydrate()
    return visibility[key]
  }

  function setItemVisible(key: NavigationItemKey, visible: boolean): void {
    hydrate()
    visibility[key] = visible
    persist()
  }

  function resetItems(): void {
    hydrate()
    Object.assign(visibility, createDefaults())
    persist()
  }

  return {
    visibility,
    visibleItems,
    isItemVisible,
    setItemVisible,
    resetItems,
  }
}
