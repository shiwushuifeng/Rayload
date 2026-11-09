/** @fileoverview Lightweight touch feedback exposed by the Android WebView host. */

import { getRayloadNativeBridge } from '@/composables/useAndroidNative'

/**
 * Triggers the platform's short key-tap haptic. The native host respects the
 * user's system haptics setting and exposes no method outside Android.
 */
export function triggerHaptic(): void {
  try {
    getRayloadNativeBridge()?.lightTap()
  } catch {
    // Haptics are optional feedback and must never interrupt the action.
  }
}
