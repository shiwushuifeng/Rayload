<script setup lang="ts">
/** @fileoverview Full-width rolling download speed trend chart. */
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { NCard, NTag } from 'naive-ui'
import { bytesToSize } from '@shared/utils/format'
import { SPEED_SAMPLE_INTERVAL_MS } from '@/composables/useSpeedTrend'

const props = defineProps<{
  samples: number[]
  title: string
  hint: string
  emptyText: string
}>()

const { t } = useI18n()

function niceSpeedCeiling(value: number): number {
  if (!Number.isFinite(value) || value <= 0) return 1
  const magnitude = 10 ** Math.floor(Math.log10(value))
  const normalized = value / magnitude
  const nice = normalized <= 1 ? 1 : normalized <= 2 ? 2 : normalized <= 5 ? 5 : 10
  return nice * magnitude
}

const chart = computed(() => {
  const width = 320
  const height = 190
  const left = 46
  const right = width - 12
  const top = 12
  const baseline = height - 24
  const window = props.samples.map((bytesPerSecond) => ({ bytesPerSecond }))
  const peak = window.reduce((max, sample) => Math.max(max, sample.bytesPerSecond), 0)
  const ceiling = niceSpeedCeiling(peak)
  const slots = Math.max(1, window.length - 1)
  const step = (right - left) / slots
  const points = window.map((sample, index) => {
    const x = left + index * step
    const ratio = Math.min(1, sample.bytesPerSecond / ceiling)
    return { x, y: baseline - (baseline - top) * ratio }
  })
  const startPoint = points[0]
  const lastPoint = points[points.length - 1]
  let line = ''
  if (startPoint && lastPoint && points.length > 1) {
    line = `M ${startPoint.x.toFixed(1)},${startPoint.y.toFixed(1)}`
    for (let index = 1; index < points.length - 1; index += 1) {
      const current = points[index]
      const next = points[index + 1]
      if (!current || !next) break
      const midX = (current.x + next.x) / 2
      const midY = (current.y + next.y) / 2
      line += ` Q ${current.x.toFixed(1)},${current.y.toFixed(1)} ${midX.toFixed(1)},${midY.toFixed(1)}`
    }
    line += ` L ${lastPoint.x.toFixed(1)},${lastPoint.y.toFixed(1)}`
  }
  const area =
    startPoint && lastPoint && line
      ? `M ${startPoint.x.toFixed(1)},${baseline} L ${line.slice(1)} L ${lastPoint.x.toFixed(1)},${baseline} Z`
      : ''
  const gridLines = [0, 0.5, 1].map((ratio) => ({
    y: baseline - (baseline - top) * ratio,
    label: bytesToSize(ceiling * ratio, 1),
  }))
  const average = window.length ? window.reduce((sum, sample) => sum + sample.bytesPerSecond, 0) / window.length : 0
  const averageY = baseline - (baseline - top) * Math.min(1, average / ceiling)
  const elapsedSeconds = Math.max(0, (window.length - 1) * (SPEED_SAMPLE_INTERVAL_MS / 1000))
  const windowLabel = elapsedSeconds >= 60 ? `−${Math.round(elapsedSeconds / 60)}m` : `−${Math.round(elapsedSeconds)}s`
  return {
    average,
    averageY,
    peak,
    width,
    height,
    left,
    right,
    baseline,
    line,
    area,
    gridLines,
    windowLabel,
    last: lastPoint ?? null,
  }
})

const ready = computed(() => props.samples.length > 1)
const speedLabel = (value: number): string => `${bytesToSize(value, 1)}/s`
</script>

<template>
  <NCard class="speed-trend-card" :bordered="false">
    <div class="speed-trend-heading">
      <div>
        <h2>{{ title }}</h2>
        <p>{{ hint }}</p>
      </div>
      <NTag size="small" :bordered="false" type="info">
        {{ t('statistics.speed-trend-peak', { speed: speedLabel(chart.peak) }) }}
      </NTag>
    </div>
    <div class="speed-trend-chart">
      <svg
        v-if="ready"
        class="speed-trend-svg"
        :viewBox="`0 0 ${chart.width} ${chart.height}`"
        role="img"
        :aria-label="title"
      >
        <defs>
          <linearGradient id="rayload-speed-fill" x1="0" y1="0" x2="0" y2="1">
            <stop offset="0%" stop-color="currentColor" stop-opacity="0.24" />
            <stop offset="100%" stop-color="currentColor" stop-opacity="0" />
          </linearGradient>
        </defs>
        <g class="speed-trend-grid">
          <line
            v-for="line in chart.gridLines"
            :key="`speed-grid-${line.y}`"
            :x1="chart.left"
            :y1="line.y"
            :x2="chart.right"
            :y2="line.y"
          />
        </g>
        <g class="speed-trend-axis">
          <text v-for="line in chart.gridLines" :key="`speed-label-${line.y}`" x="6" :y="line.y + 3">
            {{ line.label }}
          </text>
          <text :x="chart.left" :y="chart.height - 6">{{ chart.windowLabel }}</text>
          <text :x="chart.right" :y="chart.height - 6" text-anchor="end">{{ t('statistics.speed-trend-now') }}</text>
        </g>
        <line
          v-if="chart.average > 0"
          class="speed-trend-avg"
          :x1="chart.left"
          :x2="chart.right"
          :y1="chart.averageY"
          :y2="chart.averageY"
        />
        <text
          v-if="chart.average > 0"
          class="speed-trend-avg-label"
          :x="chart.left + 2"
          :y="Math.max(12, chart.averageY - 5)"
          text-anchor="start"
        >
          {{ t('statistics.speed-trend-average', { speed: speedLabel(chart.average) }) }}
        </text>
        <path :d="chart.area" fill="url(#rayload-speed-fill)" />
        <path
          :d="chart.line"
          fill="none"
          stroke="currentColor"
          stroke-width="2.5"
          stroke-linecap="round"
          stroke-linejoin="round"
        />
        <circle v-if="chart.last" class="is-live" :cx="chart.last.x" :cy="chart.last.y" r="3.6" />
      </svg>
      <p v-else class="speed-trend-empty">{{ emptyText }}</p>
    </div>
  </NCard>
</template>

<style scoped>
.speed-trend-card {
  overflow: hidden;
  background: var(--m3-surface-container);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-lg);
}
.speed-trend-heading {
  display: flex;
  gap: 12px;
  align-items: flex-start;
  justify-content: space-between;
  margin-bottom: 10px;
}
.speed-trend-heading h2 {
  margin: 0 0 3px;
  font-size: 17px;
}
.speed-trend-heading p {
  margin: 0;
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-sm);
}
.speed-trend-chart {
  position: relative;
  aspect-ratio: 320 / 190;
  width: 100%;
}
.speed-trend-svg {
  position: absolute;
  inset: 0;
  display: block;
  width: 100%;
  height: 100%;
  color: var(--m3-tertiary);
}
.speed-trend-grid line {
  stroke: var(--m3-outline-variant);
  stroke-width: 1;
  stroke-dasharray: 3 5;
}
.speed-trend-axis text {
  fill: var(--m3-on-surface-variant);
  font-size: 11px;
  font-variant-numeric: tabular-nums;
}
.speed-trend-avg {
  stroke: var(--m3-secondary);
  stroke-width: 1.6;
  stroke-dasharray: 5 4;
  opacity: 0.9;
}
.speed-trend-avg-label {
  fill: var(--m3-secondary);
  font-size: 11px;
  font-weight: 600;
}
.speed-trend-svg circle.is-live {
  fill: var(--m3-tertiary);
  stroke: var(--m3-surface-container);
  stroke-width: 1.6;
}
.speed-trend-empty {
  position: absolute;
  inset: 0;
  display: grid;
  place-items: center;
  margin: 0;
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-xs);
  text-align: center;
}
</style>
