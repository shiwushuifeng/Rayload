<script setup lang="ts">
/** @fileoverview Reusable seven-day download history chart. */
import { computed } from 'vue'
import { NCard, NTag } from 'naive-ui'
import { bytesToSize } from '@shared/utils/format'

const props = defineProps<{
  values: number[]
  title: string
  hint: string
  emptyText: string
  dayLabel: (index: number) => string
  tagText?: string
}>()

const hasData = computed(() => props.values.some((value) => value > 0))

const geometry = computed(() => {
  const width = 320
  const height = 136
  const left = 46
  const right = width - 12
  const top = 12
  const baseline = height - 24
  const step = (right - left) / 6
  const max = Math.max(...props.values, 1)
  const points = props.values.map((value, index) => {
    const x = left + index * step
    const y = baseline - ((baseline - top) * value) / max
    return { x, y, value }
  })
  const line = points.map((point) => `${point.x.toFixed(1)},${point.y.toFixed(1)}`).join(' ')
  const area = `${left.toFixed(1)},${baseline} ${line} ${right.toFixed(1)},${baseline}`
  const gridLines = [0, 0.33, 0.66, 1].map((ratio) => ({
    y: baseline - (baseline - top) * ratio,
    label: bytesToSize(max * ratio, 1),
  }))
  return { width, height, baseline, points, line, area, gridLines }
})
</script>

<template>
  <NCard class="history-trend-card" :bordered="false">
    <div class="history-section-heading">
      <div>
        <h2>{{ title }}</h2>
        <p>{{ hint }}</p>
      </div>
      <NTag v-if="tagText" size="small" :bordered="false" type="success">{{ tagText }}</NTag>
    </div>
    <svg
      class="history-trend-card__svg"
      :viewBox="`0 0 ${geometry.width} ${geometry.height}`"
      role="img"
      :aria-label="title"
    >
      <defs>
        <linearGradient id="rayload-trend-fill" x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%" stop-color="currentColor" stop-opacity="0.28" />
          <stop offset="100%" stop-color="currentColor" stop-opacity="0" />
        </linearGradient>
      </defs>
      <g class="history-trend-card__grid">
        <line
          v-for="line in geometry.gridLines"
          :key="line.y"
          x1="14"
          :y1="line.y"
          :x2="geometry.width - 12"
          :y2="line.y"
        />
      </g>
      <polygon :points="geometry.area" fill="url(#rayload-trend-fill)" />
      <polyline
        :points="geometry.line"
        fill="none"
        stroke="currentColor"
        stroke-width="2.5"
        stroke-linecap="round"
        stroke-linejoin="round"
      />
      <g class="history-trend-card__dots">
        <circle
          v-for="(point, index) in geometry.points"
          :key="index"
          :cx="point.x"
          :cy="point.y"
          :r="point.value > 0 ? 3 : 2"
          :class="{ 'is-empty': point.value === 0 }"
        />
      </g>
      <g class="history-trend-card__labels">
        <text
          v-for="index in 7"
          :key="index"
          :x="index === 7 ? geometry.width - 12 : 14 + (index - 1) * ((geometry.width - 26) / 6)"
          y="132"
          :text-anchor="index === 1 ? 'start' : index === 7 ? 'end' : 'middle'"
        >
          {{ dayLabel(index - 1) }}
        </text>
      </g>
    </svg>
    <p v-if="!hasData" class="history-trend-card__empty">{{ emptyText }}</p>
  </NCard>
</template>

<style scoped>
.history-trend-card {
  overflow: hidden;
  background: var(--m3-surface-container);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-lg);
}
.history-section-heading {
  display: flex;
  gap: 12px;
  align-items: flex-start;
  justify-content: space-between;
  margin-bottom: 10px;
}
.history-section-heading h2 {
  margin: 0 0 3px;
  font-size: 17px;
}
.history-section-heading p {
  margin: 0;
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-sm);
}
.history-trend-card__svg {
  display: block;
  width: 100%;
  height: auto;
  color: var(--m3-primary);
}
.history-trend-card__grid line {
  stroke: var(--m3-outline-variant);
  stroke-width: 1;
  stroke-dasharray: 3 5;
}
.history-trend-card__dots circle {
  fill: var(--m3-primary);
  stroke: var(--m3-surface-container);
  stroke-width: 1.5;
}
.history-trend-card__dots circle.is-empty {
  fill: var(--m3-outline);
}
.history-trend-card__labels text {
  fill: var(--m3-on-surface-variant);
  font-size: 11px;
  text-anchor: middle;
}
.history-trend-card__empty {
  margin: 6px 0 0;
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-xs);
  text-align: center;
}
</style>
