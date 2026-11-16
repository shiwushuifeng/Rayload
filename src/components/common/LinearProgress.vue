<script setup lang="ts">
/** @fileoverview Determinate progress bar that moves only on the compositor.
 *
 * The fill is scaled, never resized: a width or max-width change re-lays out
 * and repaints the list on every frame of its transition, and an endless
 * sweep keeps doing so for every active download (#799).
 */
import { computed } from 'vue'

const props = withDefaults(
  defineProps<{
    /** 0–100 */
    percentage: number
    color?: string
    height?: number
    /** Adds the moving sheen that marks a download still in flight. */
    active?: boolean
  }>(),
  { color: 'var(--m3-primary)', height: 6, active: false },
)

const ratio = computed(() => Math.min(Math.max(props.percentage, 0), 100) / 100)
</script>

<template>
  <div
    class="linear-progress"
    :class="{ 'linear-progress--active': active }"
    role="progressbar"
    aria-valuemin="0"
    aria-valuemax="100"
    :aria-valuenow="Math.round(ratio * 100)"
    :style="{ height: `${height}px`, '--progress-color': color }"
  >
    <div class="linear-progress__fill" :style="{ transform: `scaleX(${ratio})` }">
      <span class="linear-progress__sheen" aria-hidden="true" />
    </div>
  </div>
</template>

<style scoped>
.linear-progress {
  /* Track: a quiet inset rail so the colored fill reads as the only raised part. */
  position: relative;
  overflow: hidden;
  border-radius: 999px;
  background: color-mix(in srgb, var(--m3-outline-variant) 70%, transparent);
  box-shadow: inset 0 1px 1px color-mix(in srgb, var(--m3-on-surface) 8%, transparent);
  contain: strict;
  width: 100%;
  --progress-color: var(--m3-primary);
}
.linear-progress__fill {
  position: absolute;
  inset: 0;
  overflow: hidden;
  border-radius: inherit;
  transform-origin: 0 50%;
  /* Gradient keeps the hue legible while giving the bar depth. */
  background: linear-gradient(
    90deg,
    color-mix(in srgb, var(--progress-color) 78%, var(--m3-surface-container)) 0%,
    var(--progress-color) 62%,
    color-mix(in srgb, var(--progress-color) 82%, var(--m3-on-surface)) 100%
  );
  transition:
    transform var(--task-motion-progress) var(--task-motion-ease),
    background-color var(--task-motion-state) var(--task-motion-ease);
}
/* Inner highlight along the top edge. */
.linear-progress__fill::after {
  content: '';
  position: absolute;
  inset: 1px 1px auto;
  height: 42%;
  border-radius: 999px;
  background: linear-gradient(180deg, color-mix(in srgb, var(--m3-on-surface) 14%, transparent), transparent);
  pointer-events: none;
}
.linear-progress__sheen {
  position: absolute;
  inset: 0;
  background: linear-gradient(
    100deg,
    transparent 12%,
    color-mix(in srgb, var(--m3-on-surface) 16%, transparent) 42%,
    transparent 68%
  );
  opacity: 0;
  pointer-events: none;
}
.linear-progress--active .linear-progress__sheen {
  opacity: 1;
  animation: linear-progress-sheen 1.9s linear infinite;
}
@keyframes linear-progress-sheen {
  from {
    transform: translate3d(-100%, 0, 0);
  }
  to {
    transform: translate3d(100%, 0, 0);
  }
}
@media (prefers-reduced-motion: reduce) {
  .linear-progress--active .linear-progress__sheen {
    animation: none;
    opacity: 0;
  }
  .linear-progress__fill {
    transition: none;
  }
}
</style>
