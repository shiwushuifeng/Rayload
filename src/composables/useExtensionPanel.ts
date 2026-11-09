/** Shared visibility state for the extension setup panel opened from Settings. */
import { ref } from 'vue'

const extensionPanelVisible = ref(false)

export function useExtensionPanel() {
  function openExtensionPanel(): void {
    extensionPanelVisible.value = true
  }

  function closeExtensionPanel(): void {
    extensionPanelVisible.value = false
  }

  return {
    extensionPanelVisible,
    openExtensionPanel,
    closeExtensionPanel,
  }
}
