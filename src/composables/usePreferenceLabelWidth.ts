/**
 * @fileoverview Responsive label column for the settings forms.
 *
 * Every preference form used a hardcoded `label-width="260px"`. On a tablet in
 * portrait the pane is narrower than that plus a usable control, so the control
 * was pushed past the right edge. The width now shrinks with the pane instead of
 * forcing an overflow.
 */
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'

/** Width below which the label column has to shrink. */
const NARROW_BREAKPOINT = 900

export function usePreferenceLabelWidth() {
  const viewport = ref(0)

  function measure() {
    viewport.value = typeof window === 'undefined' ? 0 : window.innerWidth
  }

  onMounted(() => {
    measure()
    window.addEventListener('resize', measure, { passive: true })
  })

  onBeforeUnmount(() => {
    window.removeEventListener('resize', measure)
  })

  const preferenceLabelWidth = computed(() => {
    const width = viewport.value
    // Unknown viewport (SSR or the first paint): keep the desktop value.
    if (!width) return '260px'
    if (width < 600) return '96px'
    if (width < NARROW_BREAKPOINT) return '150px'
    if (width < 1200) return '200px'
    return '260px'
  })

  return { preferenceLabelWidth }
}
