<script setup lang="ts">
/** @fileoverview Compact trend line shown inside a statistics card. */
import { computed } from 'vue'

const props = defineProps<{
  values: number[]
  label: string
}>()

const geometry = computed(() => {
  const width = 120
  const height = 32
  const pad = 2
  const values = props.values.length > 1 ? props.values : [0, 0]
  const max = Math.max(...values, 1)
  const step = (width - pad * 2) / (values.length - 1)
  const points = values.map((value, index) => {
    const x = pad + index * step
    const y = height - pad - ((height - pad * 2) * value) / max
    return `${x.toFixed(1)},${y.toFixed(1)}`
  })
  return {
    width,
    height,
    line: points.join(' '),
    area: `${pad},${height - pad} ${points.join(' ')} ${width - pad},${height - pad}`,
  }
})
</script>

<template>
  <svg
    class="stat-sparkline"
    :viewBox="`0 0 ${geometry.width} ${geometry.height}`"
    preserveAspectRatio="none"
    role="img"
    :aria-label="label"
  >
    <polygon class="stat-sparkline__area" :points="geometry.area" />
    <polyline class="stat-sparkline__line" :points="geometry.line" />
  </svg>
</template>

<style scoped>
.stat-sparkline {
  display: block;
  width: 100%;
  height: 34px;
  color: var(--m3-primary);
  overflow: visible;
}

.stat-sparkline__area {
  fill: currentColor;
  opacity: 0.16;
  stroke: none;
}

.stat-sparkline__line {
  fill: none;
  stroke: currentColor;
  stroke-width: 1.6;
  stroke-linecap: round;
  stroke-linejoin: round;
  vector-effect: non-scaling-stroke;
}
</style>
