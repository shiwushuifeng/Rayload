<script setup lang="ts">
/** Smoothly interpolates numeric telemetry such as speed and progress. */
import { onBeforeUnmount, ref, watch } from 'vue'

const props = withDefaults(
  defineProps<{
    value: number
    formatter: (value: number) => string
    duration?: number
  }>(),
  { duration: 520 },
)

const displayValue = ref(Number.isFinite(props.value) ? props.value : 0)
let frameId = 0

function prefersReducedMotion(): boolean {
  if (typeof window === 'undefined') return false
  return (
    document.documentElement.classList.contains('reduce-motion') ||
    window.matchMedia?.('(prefers-reduced-motion: reduce)').matches === true
  )
}

function stopAnimation() {
  if (frameId) {
    cancelAnimationFrame(frameId)
    frameId = 0
  }
}

function animateTo(next: number) {
  stopAnimation()
  if (!Number.isFinite(next)) {
    displayValue.value = 0
    return
  }
  if (prefersReducedMotion()) {
    displayValue.value = next
    return
  }

  const from = displayValue.value
  if (Math.abs(next - from) < 0.5) {
    displayValue.value = next
    return
  }

  const start = performance.now()
  const duration = Math.max(120, props.duration)
  const step = (now: number) => {
    const progress = Math.min(1, (now - start) / duration)
    const eased = 1 - (1 - progress) ** 3
    displayValue.value = from + (next - from) * eased
    if (progress < 1) frameId = requestAnimationFrame(step)
    else {
      displayValue.value = next
      frameId = 0
    }
  }
  frameId = requestAnimationFrame(step)
}

watch(
  () => props.value,
  (next) => animateTo(next),
)

onBeforeUnmount(stopAnimation)
</script>

<template>
  <span class="animated-number">{{ props.formatter(displayValue) }}</span>
</template>

<style scoped>
.animated-number {
  display: inline-block;
  font-variant-numeric: tabular-nums;
  white-space: nowrap;
}
</style>
