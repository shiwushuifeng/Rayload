/** Typed access to the Android WebView bridge used by Rayload's native shell. */

export type AndroidPermissionStatus = 'granted' | 'denied' | 'unavailable'

interface RayloadNativeBridge {
  lightTap(): void
  chooseDirectory(): void
  shareFile(path: string): boolean
  openFile(path: string): boolean
  setAppIcon(style: 'purple' | 'gold' | 'blue' | 'teal'): boolean
  storagePermissionStatus(): string
  requestStoragePermission(): void
  notificationPermissionStatus(): string
  requestNotificationPermission(): void
  batteryOptimizationStatus(): string
  requestBatteryOptimizationExemption(): void
  setEngineCredentials(port: number, secret: string): void
  setEnginePaused(paused: boolean): void
  setKeepScreenOn(enabled: boolean): void
}

declare global {
  interface Window {
    RayloadNative?: RayloadNativeBridge
  }
}

interface DirectorySelectionDetail {
  path?: string
  message?: string
}

function normalizeStatus(value: string | undefined): AndroidPermissionStatus {
  return value === 'granted' || value === 'denied' ? value : 'unavailable'
}

export function getRayloadNativeBridge(): RayloadNativeBridge | null {
  return typeof window === 'undefined' ? null : (window.RayloadNative ?? null)
}

export function readAndroidPermissionStatuses(): Record<
  'storage' | 'notification' | 'battery',
  AndroidPermissionStatus
> {
  const bridge = getRayloadNativeBridge()
  if (!bridge) {
    return { storage: 'unavailable', notification: 'unavailable', battery: 'unavailable' }
  }
  return {
    storage: normalizeStatus(bridge.storagePermissionStatus()),
    notification: normalizeStatus(bridge.notificationPermissionStatus()),
    battery: normalizeStatus(bridge.batteryOptimizationStatus()),
  }
}

export function requestAndroidStoragePermission(): void {
  getRayloadNativeBridge()?.requestStoragePermission()
}

export function requestAndroidNotificationPermission(): void {
  getRayloadNativeBridge()?.requestNotificationPermission()
}

export function requestAndroidBatteryOptimizationExemption(): void {
  getRayloadNativeBridge()?.requestBatteryOptimizationExemption()
}

/** Shares the local aria2 RPC endpoint with the Android foreground service. */
export function syncAndroidEngineCredentials(port: number, secret: string): void {
  getRayloadNativeBridge()?.setEngineCredentials(port, secret)
}

/** Mirrors the global pause state into the foreground notification actions. */
export function setAndroidEnginePaused(paused: boolean): void {
  getRayloadNativeBridge()?.setEnginePaused(paused)
}

/** Keeps the Android window awake while the full-screen download dashboard is open. */
export function setAndroidKeepScreenOn(enabled: boolean): void {
  getRayloadNativeBridge()?.setKeepScreenOn(enabled)
}

export function shareAndroidFile(path: string): boolean {
  return getRayloadNativeBridge()?.shareFile(path) ?? false
}

/** Launches an Android ACTION_VIEW intent for a local file through FileProvider. */
export function openAndroidFile(path: string): boolean {
  return getRayloadNativeBridge()?.openFile(path) ?? false
}

export function setAndroidAppIcon(style: 'purple' | 'gold' | 'blue' | 'teal'): boolean {
  return getRayloadNativeBridge()?.setAppIcon(style) ?? false
}

/** Opens Android's DocumentsUI tree picker and resolves the selected filesystem path. */
export function chooseAndroidDirectory(): Promise<string | null> {
  const bridge = getRayloadNativeBridge()
  if (!bridge) return Promise.resolve(null)

  return new Promise((resolve) => {
    let settled = false
    const cleanup = () => {
      window.removeEventListener('rayload-directory-selected', onSelected)
      window.removeEventListener('rayload-directory-selection-cancelled', onCancelled)
      window.removeEventListener('rayload-directory-selection-failed', onFailed)
    }
    const settle = (value: string | null) => {
      if (settled) return
      settled = true
      cleanup()
      resolve(value)
    }
    const onSelected = (event: Event) => {
      const detail = (event as CustomEvent<DirectorySelectionDetail>).detail
      settle(detail?.path?.trim() || null)
    }
    const onCancelled = () => settle(null)
    const onFailed = () => settle(null)

    window.addEventListener('rayload-directory-selected', onSelected)
    window.addEventListener('rayload-directory-selection-cancelled', onCancelled)
    window.addEventListener('rayload-directory-selection-failed', onFailed)
    bridge.chooseDirectory()
  })
}
