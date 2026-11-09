/** Theme-aware brand mark that keeps contrast on light and dark surfaces. */
import { computed } from 'vue'
import brandLogoForLight from '@/assets/logo.svg'
import brandLogoForDark from '@/assets/logo-for-dark.svg'
import { useTheme } from '@/composables/useTheme'

export function useBrandLogo() {
  const { isDark } = useTheme()
  const brandLogo = computed(() => (isDark.value ? brandLogoForDark : brandLogoForLight))
  return { brandLogo }
}
