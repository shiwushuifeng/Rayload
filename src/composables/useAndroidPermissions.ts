/** @fileoverview Shared Android permission status and guided authorization flow. */
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import {
  getRayloadNativeBridge,
  readAndroidPermissionStatuses,
  requestAndroidBatteryOptimizationExemption,
  requestAndroidClipboardPermission,
  requestAndroidInstallPermission,
  requestAndroidLocalNetworkPermission,
  requestAndroidNotificationPermission,
  requestAndroidStoragePermission,
  type AndroidPermissionStatus,
} from '@/composables/useAndroidNative'

export type AndroidPermissionKey = 'storage' | 'notification' | 'local-network' | 'battery' | 'install' | 'clipboard'

export const ANDROID_PERMISSION_KEYS: readonly AndroidPermissionKey[] = [
  'storage',
  'notification',
  'local-network',
  'battery',
  'install',
  'clipboard',
]

export function useAndroidPermissions() {
  const statuses = ref<Record<AndroidPermissionKey, AndroidPermissionStatus>>(readAndroidPermissionStatuses())
  const authorizing = ref(false)
  let queue: AndroidPermissionKey[] = []

  const supported = computed(() => getRayloadNativeBridge() !== null)
  const missing = computed(() => ANDROID_PERMISSION_KEYS.filter((key) => statuses.value[key] === 'denied'))

  function refresh(): void {
    statuses.value = readAndroidPermissionStatuses()
  }

  function request(key: AndroidPermissionKey): void {
    if (statuses.value[key] === 'granted') return
    switch (key) {
      case 'storage':
        requestAndroidStoragePermission()
        break
      case 'notification':
        requestAndroidNotificationPermission()
        break
      case 'local-network':
        requestAndroidLocalNetworkPermission()
        break
      case 'battery':
        requestAndroidBatteryOptimizationExemption()
        break
      case 'install':
        requestAndroidInstallPermission()
        break
      case 'clipboard':
        requestAndroidClipboardPermission()
        break
    }
  }

  function requestNext(): void {
    queue = queue.filter((key) => statuses.value[key] === 'denied')
    const next = queue[0]
    if (!next) {
      queue = []
      authorizing.value = false
      return
    }
    request(next)
  }

  /** Requests the first missing permission; grants advance to the next one. */
  function requestAll(): void {
    refresh()
    queue = [...missing.value]
    if (queue.length === 0) {
      authorizing.value = false
      return
    }
    authorizing.value = true
    requestNext()
  }

  function onStateChanged(): void {
    refresh()
    if (!authorizing.value) return
    const current = queue[0]
    if (current && statuses.value[current] === 'granted') {
      queue.shift()
      requestNext()
      return
    }
    if (current && statuses.value[current] === 'denied') authorizing.value = false
  }

  onMounted(() => {
    window.addEventListener('focus', onStateChanged)
    window.addEventListener('rayload-permission-state-changed', onStateChanged)
  })

  onBeforeUnmount(() => {
    window.removeEventListener('focus', onStateChanged)
    window.removeEventListener('rayload-permission-state-changed', onStateChanged)
  })

  return { statuses, supported, missing, authorizing, refresh, request, requestAll }
}
