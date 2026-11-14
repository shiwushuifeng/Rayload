/** Adds a lightweight Material state-layer ripple to non-Naive controls. */
import { onBeforeUnmount, onMounted } from 'vue'

const HOST_SELECTOR = 'button:not(.n-button), a[href], [role="button"], .sidebar-item, .task-swipe-row, .ripple-surface'
const IGNORE_SELECTOR =
  '.n-button, .n-input, .n-input-number, .n-base-selection, .n-checkbox, .n-radio, .n-switch, .n-slider, [data-no-ripple]'

function resolveHost(target: EventTarget | null): HTMLElement | null {
  if (!(target instanceof Element)) return null
  if (target.closest(IGNORE_SELECTOR)) return null
  const host = target.closest<HTMLElement>(HOST_SELECTOR)
  if (!host || host.matches(':disabled') || host.getAttribute('aria-disabled') === 'true') return null
  return host
}

function spawnRipple(host: HTMLElement, event: PointerEvent) {
  if (document.documentElement.classList.contains('reduce-motion')) return
  const hostStyle = getComputedStyle(host)
  // Inline hosts position an absolute child against their first line box,
  // which puts the circle at the top instead of the tap point.
  if (hostStyle.display === 'inline') return

  host.classList.add('ripple-host')
  // A static host would anchor the wave to the page instead of itself.
  if (getComputedStyle(host).position === 'static') host.style.position = 'relative'

  const rect = host.getBoundingClientRect()
  // Scale from screen space (getBoundingClientRect) back to the host's own
  // pixels so transformed controls keep the circle under the pointer.
  const scaleX = host.offsetWidth > 0 ? rect.width / host.offsetWidth : 1
  const scaleY = host.offsetHeight > 0 ? rect.height / host.offsetHeight : 1
  if (rect.width === 0 || rect.height === 0) return

  const wave = document.createElement('span')
  const diameter = Math.hypot(host.offsetWidth, host.offsetHeight) * 1.15
  wave.className = 'material-ripple-wave'
  wave.style.width = `${diameter}px`
  wave.style.height = `${diameter}px`
  wave.style.left = `${(event.clientX - rect.left) / scaleX - diameter / 2}px`
  wave.style.top = `${(event.clientY - rect.top) / scaleY - diameter / 2}px`
  host.appendChild(wave)
  wave.addEventListener('animationend', () => wave.remove(), { once: true })
}

export function useMaterialRipple(): void {
  let active = false
  const onPointerDown = (event: PointerEvent) => {
    if (event.button !== 0) return
    const host = resolveHost(event.target)
    if (host) spawnRipple(host, event)
  }

  onMounted(() => {
    document.addEventListener('pointerdown', onPointerDown, { passive: true })
    active = true
  })

  onBeforeUnmount(() => {
    if (!active) return
    document.removeEventListener('pointerdown', onPointerDown)
    active = false
  })
}
