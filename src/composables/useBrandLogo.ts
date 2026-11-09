/** Theme-aware brand mark that keeps contrast on light and dark surfaces. */
import { computed } from 'vue'
import brandLogoForLight from '@/assets/logo.svg'
import brandLogoViolet from '@/assets/logo-violet.svg'
import { useTheme } from '@/composables/useTheme'
import { usePreferenceStore } from '@/stores/preference'

export function useBrandLogo() {
  const { isDark } = useTheme()
  const preferences = usePreferenceStore()
  const brandLogo = computed(() => {
    switch (preferences.config.brandLogoStyle) {
      case 'gold':
        return brandLogoForLight
      case 'violet':
        return brandLogoViolet
      default:
        return isDark.value ? brandLogoViolet : brandLogoForLight
    }
  })
  return { brandLogo }
}
