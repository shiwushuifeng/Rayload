import { afterEach, describe, expect, it, vi } from 'vitest'
import { triggerHaptic } from '../useHaptics'

describe('triggerHaptic', () => {
  afterEach(() => {
    delete window.RayloadNative
  })

  it('forwards touch feedback to the Android host when available', () => {
    const lightTap = vi.fn()
    window.RayloadNative = {
      lightTap,
      chooseDirectory: vi.fn(),
      shareFile: vi.fn(() => false),
      openFile: vi.fn(() => false),
      setAppIcon: vi.fn(() => true),
      storagePermissionStatus: vi.fn(() => 'denied'),
      requestStoragePermission: vi.fn(),
      notificationPermissionStatus: vi.fn(() => 'denied'),
      requestNotificationPermission: vi.fn(),
      batteryOptimizationStatus: vi.fn(() => 'denied'),
      requestBatteryOptimizationExemption: vi.fn(),
      installPermissionStatus: vi.fn(() => 'denied'),
      requestInstallPermission: vi.fn(),
      clipboardPermissionStatus: vi.fn(() => 'denied'),
      requestClipboardPermission: vi.fn(),
      localIpAddress: vi.fn(() => '192.168.1.10'),
      setEngineCredentials: vi.fn(),
      setEnginePaused: vi.fn(),
      setEngineNotificationQuiet: vi.fn(),
      setKeepScreenOn: vi.fn(),
      restartApp: vi.fn(),
    }

    triggerHaptic()

    expect(lightTap).toHaveBeenCalledOnce()
  })

  it('treats haptics as optional when no native host exists', () => {
    expect(() => triggerHaptic()).not.toThrow()
  })
})
