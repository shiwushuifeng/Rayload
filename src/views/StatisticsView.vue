<script setup lang="ts">
/** @fileoverview Data dashboard with live session totals and local history aggregation. */
import { computed, onActivated, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { RouterLink } from 'vue-router'
import { NCard, NIcon, NSpin, NTab, NTabs, NTag } from 'naive-ui'
import {
  ArchiveOutline,
  BarChartOutline,
  ChevronDownOutline,
  CloudDownloadOutline,
  CubeOutline,
  DocumentTextOutline,
  EllipsisHorizontalCircleOutline,
  FilmOutline,
  ImageOutline,
  LayersOutline,
  MusicalNotesOutline,
  TimeOutline,
  WifiOutline,
} from '@vicons/ionicons5'
import AnimatedNumber from '@/components/common/AnimatedNumber.vue'
import HistoryCategoryCard from '@/components/statistics/HistoryCategoryCard.vue'
import HistoryTrendCard from '@/components/statistics/HistoryTrendCard.vue'
import StatSparkline from '@/components/statistics/StatSparkline.vue'
import { useTaskStore } from '@/stores/task'
import { useHistoryStore } from '@/stores/history'
import { usePreferenceStore } from '@/stores/preference'
import { useLanStore } from '@/stores/lan'
import { deviceArchLabel } from '@/composables/usePlatform'
import { getGlobalStat } from '@/api/aria2'
import { bytesToSize } from '@shared/utils/format'
import { buildDefaultCategories, BUILTIN_CATEGORY_TEMPLATES } from '@shared/constants'
import { ensureStatisticsHistoryFile } from '@/composables/useStatisticsHistory'
import { SPEED_SAMPLE_INTERVAL_MS, SPEED_SAMPLE_LIMIT, useSpeedTrend } from '@/composables/useSpeedTrend'
import { logger } from '@shared/logger'
import type { Aria2GlobalStat, FileCategory, HistoryRecord } from '@shared/types'

const { t } = useI18n()
const taskStore = useTaskStore()
const historyStore = useHistoryStore()
const preferenceStore = usePreferenceStore()
const lan = useLanStore()
const speedTrend = useSpeedTrend()

const historyBytes = ref(0)
const historyTasks = ref(0)
const records = ref<HistoryRecord[]>([])
const globalStat = ref<Aria2GlobalStat | null>(null)
const loading = ref(true)
const showShortcuts = ref(false)
const activeTab = ref<'local' | 'all'>('local')

const allSpeedSamples = ref<number[]>([])

const sessionBytes = computed(() =>
  taskStore.taskList.reduce((total, task) => total + Math.max(0, Number(task.completedLength) || 0), 0),
)
const cumulativeBytes = computed(() => historyBytes.value + sessionBytes.value)
const liveSpeed = computed(() => globalStat.value?.downloadSpeed ?? 0)
const sessionActive = computed(() => globalStat.value?.numActive ?? 0)
const sessionWaiting = computed(() => globalStat.value?.numWaiting ?? 0)

const remoteHistory = computed(() => new Map(lan.historyStats.map((item) => [item.id, item])))

const allDevices = computed(() => [
  {
    id: 'local',
    name: t('statistics.this-device'),
    online: true,
    downloadSpeed: liveSpeed.value,
    activeTasks: sessionActive.value,
    arch: '',
    address: '',
    historyBytes: historyBytes.value,
    historyTasks: historyTasks.value,
  },
  ...lan.statuses.map((status) => {
    const device = lan.devices.find((item) => item.id === status.id)
    const history = remoteHistory.value.get(status.id)
    return {
      ...status,
      arch: device?.arch ?? '',
      address: device?.address ?? '',
      historyBytes: history?.totalBytes ?? 0,
      historyTasks: history?.totalTasks ?? 0,
    }
  }),
])
const onlineDeviceCount = computed(() => allDevices.value.filter((device) => device.online).length)
const allDeviceSpeed = computed(() =>
  allDevices.value.reduce((total, device) => total + (device.online ? device.downloadSpeed : 0), 0),
)
const allDeviceTasks = computed(() =>
  allDevices.value.reduce((total, device) => total + (device.online ? device.activeTasks : 0), 0),
)
const allHistoryBytes = computed(() => allDevices.value.reduce((total, device) => total + device.historyBytes, 0))
const allHistoryTasks = computed(() => allDevices.value.reduce((total, device) => total + device.historyTasks, 0))
const localSpeedSpark = computed(() => speedTrend.samples.value.map((sample) => sample.bytesPerSecond))
const peakDeviceSpeed = computed(() => Math.max(1, ...allDevices.value.map((device) => device.downloadSpeed)))

const categoryIcons: Record<string, typeof FilmOutline> = {
  'file-category-videos': FilmOutline,
  'file-category-music': MusicalNotesOutline,
  'file-category-images': ImageOutline,
  'file-category-documents': DocumentTextOutline,
  'file-category-archives': ArchiveOutline,
  'file-category-programs': CubeOutline,
}

const categoryRules = computed<FileCategory[]>(() =>
  preferenceStore.config.fileCategories.length ? preferenceStore.config.fileCategories : buildDefaultCategories(),
)

function recordBytes(record: HistoryRecord): number {
  return Math.max(0, Number(record.total_length) || 0)
}

function extensionOf(name: string): string {
  const dot = name.lastIndexOf('.')
  return dot >= 0 ? name.slice(dot + 1).toLowerCase() : ''
}

function localDayKey(date: Date): string {
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  return `${date.getFullYear()}-${month}-${day}`
}

function recentDayKeys(): string[] {
  const today = new Date()
  today.setHours(0, 0, 0, 0)
  return Array.from({ length: 7 }, (_, index) => {
    const date = new Date(today)
    date.setDate(date.getDate() - (6 - index))
    return localDayKey(date)
  })
}

/** Last seven calendar days, oldest first, in the user's local timezone. */
const trendValues = computed<number[]>(() => {
  const days: number[] = Array.from({ length: 7 }, () => 0)
  const today = new Date()
  today.setHours(0, 0, 0, 0)
  const buckets = Array.from({ length: 7 }, (_, index) => {
    const date = new Date(today)
    date.setDate(date.getDate() - (6 - index))
    return date
  })
  for (const record of records.value) {
    if (record.status !== 'complete') continue
    const stamp = record.completed_at || record.created_at
    if (!stamp) continue
    const completed = new Date(stamp)
    if (Number.isNaN(completed.getTime())) continue
    completed.setHours(0, 0, 0, 0)
    const index = buckets.findIndex((bucket) => bucket.getTime() === completed.getTime())
    if (index >= 0) days[index] += recordBytes(record)
  }
  return days
})

/** Adds paired-device daily bytes to the local seven-day series. */
const allTrendValues = computed<number[]>(() => {
  const values = [...trendValues.value]
  const keys = recentDayKeys()
  const remoteDays = new Map<string, number>()
  for (const history of lan.historyStats) {
    for (const day of history.days) {
      remoteDays.set(day.date, (remoteDays.get(day.date) ?? 0) + day.bytes)
    }
  }
  return values.map((value, index) => value + (remoteDays.get(keys[index]) ?? 0))
})

const SPEED_WINDOW_MINUTES = Math.round(((SPEED_SAMPLE_LIMIT - 1) * SPEED_SAMPLE_INTERVAL_MS) / 60000)

function niceSpeedCeiling(value: number): number {
  if (!Number.isFinite(value) || value <= 0) return 1
  const magnitude = 10 ** Math.floor(Math.log10(value))
  const normalized = value / magnitude
  const nice = normalized <= 1 ? 1 : normalized <= 2 ? 2 : normalized <= 5 ? 5 : 10
  return nice * magnitude
}

const speedChart = computed(() => {
  const width = 640
  const height = 176
  const left = 62
  const right = width - 12
  const top = 14
  const baseline = height - 24
  const window = speedTrend.samples.value
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
    width,
    height,
    left,
    right,
    baseline,
    points,
    line,
    area,
    window: window.length,
    peak,
    gridLines,
    windowLabel,
    last: lastPoint ?? null,
  }
})

const speedTrendReady = computed(() => speedTrend.samples.value.length > 1)

const totalLabel = (value: number): string => bytesToSize(value, 2)
const speedLabel = (value: number): string => `${bytesToSize(value, 1)}/s`
const countLabel = (value: number): string => String(Math.round(value))

let liveTimer: number | undefined
let historyTimer: number | undefined

watch(activeTab, (tab) => {
  if (tab === 'all') lan.startStatusPolling()
  else lan.stopStatusPolling()
})

function startLiveRefresh(): void {
  stopLiveRefresh()
  liveTimer = window.setInterval(() => void loadGlobalStat(), SPEED_SAMPLE_INTERVAL_MS)
  historyTimer = window.setInterval(() => void loadHistory(), 30000)
}

function stopLiveRefresh(): void {
  if (liveTimer !== undefined) window.clearInterval(liveTimer)
  if (historyTimer !== undefined) window.clearInterval(historyTimer)
  liveTimer = undefined
  historyTimer = undefined
}

function refreshNow(): void {
  void Promise.all([loadHistory(), loadGlobalStat()])
}

function trendDayLabel(index: number): string {
  const date = new Date()
  date.setDate(date.getDate() + index - 6)
  return `${date.getMonth() + 1}/${date.getDate()}`
}

interface ExtensionAggregate {
  bytes: number
  tasks: number
}

function extensionStatsFromRecords(source: HistoryRecord[]): Map<string, ExtensionAggregate> {
  const stats = new Map<string, ExtensionAggregate>()
  for (const record of source) {
    if (record.status !== 'complete') continue
    const extension = extensionOf(record.name)
    const aggregate = stats.get(extension) ?? { bytes: 0, tasks: 0 }
    aggregate.bytes += recordBytes(record)
    aggregate.tasks += 1
    stats.set(extension, aggregate)
  }
  return stats
}

function mergeExtensionStats(
  left: Map<string, ExtensionAggregate>,
  right: Map<string, ExtensionAggregate>,
): Map<string, ExtensionAggregate> {
  const merged = new Map([...left].map(([extension, aggregate]) => [extension, { ...aggregate }]))
  for (const [extension, aggregate] of right) {
    const current = merged.get(extension) ?? { bytes: 0, tasks: 0 }
    current.bytes += aggregate.bytes
    current.tasks += aggregate.tasks
    merged.set(extension, current)
  }
  return merged
}

const localExtensionStats = computed(() => extensionStatsFromRecords(records.value))
const allExtensionStats = computed(() => {
  const remote = new Map<string, ExtensionAggregate>()
  for (const history of lan.historyStats) {
    for (const extension of history.extensions) {
      const aggregate = remote.get(extension.extension) ?? { bytes: 0, tasks: 0 }
      aggregate.bytes += extension.bytes
      aggregate.tasks += extension.tasks
      remote.set(extension.extension, aggregate)
    }
  }
  return mergeExtensionStats(localExtensionStats.value, remote)
})

function buildCategoryRows(stats: Map<string, ExtensionAggregate>) {
  const rules = categoryRules.value
  const totals = new Map<string, number>()
  const counts = new Map<string, number>()
  for (const [extension, aggregate] of stats) {
    const rule = rules.find((category) => category.extensions.includes(extension))
    if (!rule) continue
    totals.set(rule.label, (totals.get(rule.label) ?? 0) + aggregate.bytes)
    counts.set(rule.label, (counts.get(rule.label) ?? 0) + aggregate.tasks)
  }
  const knownBytes = [...totals.values()].reduce((sum, value) => sum + value, 0)
  const knownCount = [...counts.values()].reduce((sum, value) => sum + value, 0)
  const totalBytes = [...stats.values()].reduce((sum, value) => sum + value.bytes, 0)
  const totalCount = [...stats.values()].reduce((sum, value) => sum + value.tasks, 0)
  const otherValue = Math.max(0, totalBytes - knownBytes)
  const otherCount = Math.max(0, totalCount - knownCount)
  const max = Math.max(...totals.values(), otherValue, 1)
  const rows: Array<{
    key: string
    label: string
    icon: typeof FilmOutline
    value: number
    count: number
    share: number
  }> = BUILTIN_CATEGORY_TEMPLATES.map((template) => {
    const value = totals.get(template.label) ?? 0
    return {
      key: template.label,
      label: t(`preferences.${template.label}`),
      icon: categoryIcons[template.label] ?? CubeOutline,
      value,
      count: counts.get(template.label) ?? 0,
      share: value > 0 ? Math.max(4, Math.round((value / max) * 100)) : 0,
    }
  })
  rows.push({
    key: 'other',
    label: t('statistics.category-other'),
    icon: EllipsisHorizontalCircleOutline,
    value: otherValue,
    count: otherCount,
    share: otherValue > 0 ? Math.max(4, Math.round((otherValue / max) * 100)) : 0,
  })
  return rows.sort((a, b) => b.value - a.value)
}

const categories = computed(() => buildCategoryRows(localExtensionStats.value))
const allCategories = computed(() => buildCategoryRows(allExtensionStats.value))

onMounted(async () => {
  await Promise.all([loadHistory(), loadGlobalStat(), ensureStatisticsHistoryFile()])
  loading.value = false
  startLiveRefresh()
})

onActivated(() => {
  refreshNow()
  startLiveRefresh()
})

onBeforeUnmount(() => {
  stopLiveRefresh()
  lan.stopStatusPolling()
})

async function loadHistory(): Promise<void> {
  try {
    const result = await historyStore.getRecords()
    records.value = result
    let bytes = 0
    let completed = 0
    for (const record of result) {
      if (record.status !== 'complete') continue
      bytes += recordBytes(record)
      completed += 1
    }
    historyBytes.value = bytes
    historyTasks.value = completed
  } catch (error) {
    logger.warn('StatisticsView.history', error)
  }
}

async function loadGlobalStat(): Promise<void> {
  try {
    const stat = await getGlobalStat()
    globalStat.value = {
      downloadSpeed: Number(stat.downloadSpeed) || 0,
      uploadSpeed: Number(stat.uploadSpeed) || 0,
      numActive: Number(stat.numActive) || 0,
      numWaiting: Number(stat.numWaiting) || 0,
      numStopped: Number(stat.numStopped) || 0,
      numStoppedTotal: Number(stat.numStoppedTotal) || 0,
    }
    speedTrend.recordSpeed(globalStat.value.downloadSpeed)
    const next = [...allSpeedSamples.value, allDeviceSpeed.value]
    allSpeedSamples.value = next.slice(-SPEED_SAMPLE_LIMIT)
  } catch (error) {
    logger.debug('StatisticsView.globalStat', error)
  }
}
</script>

<template>
  <div class="statistics-view">
    <NTabs v-model:value="activeTab" class="statistics-tabs" type="segment" animated :tabs-padding="8">
      <NTab name="local">{{ t('statistics.tab-this-device') }}</NTab>
      <NTab name="all">{{ t('statistics.tab-all-devices') }}</NTab>
    </NTabs>

    <template v-if="activeTab === 'local'">
      <NSpin :show="loading">
        <section class="statistics-cards">
          <NCard class="stat-card" :bordered="false">
            <div class="stat-card__head">
              <span class="stat-card__icon"
                ><NIcon :size="18"><CloudDownloadOutline /></NIcon
              ></span>
              <span class="stat-card__label">{{ t('statistics.live-speed') }}</span>
            </div>
            <strong class="stat-card__value"><AnimatedNumber :value="liveSpeed" :formatter="speedLabel" /></strong>
            <StatSparkline :values="localSpeedSpark" :label="t('statistics.live-speed')" />
            <span class="stat-card__hint">{{ t('statistics.live-speed-hint') }}</span>
          </NCard>
          <NCard class="stat-card" :bordered="false">
            <div class="stat-card__head">
              <span class="stat-card__icon"
                ><NIcon :size="18"><CloudDownloadOutline /></NIcon
              ></span>
              <span class="stat-card__label">{{ t('statistics.total-cumulative') }}</span>
            </div>
            <strong class="stat-card__value"
              ><AnimatedNumber :value="cumulativeBytes" :formatter="totalLabel"
            /></strong>
            <StatSparkline :values="trendValues" :label="t('statistics.total-cumulative')" />
            <span class="stat-card__hint">{{ t('statistics.total-cumulative-hint') }}</span>
          </NCard>
          <NCard class="stat-card" :bordered="false">
            <div class="stat-card__head">
              <span class="stat-card__icon"
                ><NIcon :size="18"><LayersOutline /></NIcon
              ></span>
              <span class="stat-card__label">{{ t('statistics.total-tasks') }}</span>
            </div>
            <strong class="stat-card__value"><AnimatedNumber :value="historyTasks" :formatter="countLabel" /></strong>
            <span class="stat-card__hint">{{ t('statistics.total-tasks-hint') }}</span>
          </NCard>
          <NCard class="stat-card stat-card--accent" :bordered="false">
            <div class="stat-card__head">
              <span class="stat-card__icon"
                ><NIcon :size="18"><BarChartOutline /></NIcon
              ></span>
              <span class="stat-card__label">{{ t('statistics.session-downloaded') }}</span>
            </div>
            <strong class="stat-card__value"><AnimatedNumber :value="sessionBytes" :formatter="totalLabel" /></strong>
            <span class="stat-card__hint">
              {{ t('statistics.session-hint', { active: sessionActive, waiting: sessionWaiting }) }}
            </span>
          </NCard>
        </section>
      </NSpin>

      <HistoryTrendCard
        :values="trendValues"
        :title="t('statistics.trend-title')"
        :hint="t('statistics.trend-hint')"
        :empty-text="t('statistics.trend-empty')"
        :day-label="trendDayLabel"
        :tag-text="t('statistics.placeholder-tag')"
      />

      <NCard class="statistics-chart statistics-speed-trend" :bordered="false">
        <div class="statistics-section-heading">
          <div>
            <h2>{{ t('statistics.speed-trend-title') }}</h2>
            <p>{{ t('statistics.speed-trend-hint', { minutes: SPEED_WINDOW_MINUTES }) }}</p>
          </div>
          <NTag size="small" :bordered="false" type="info">
            {{ t('statistics.speed-trend-peak', { speed: speedLabel(speedChart.peak) }) }}
          </NTag>
        </div>
        <svg
          v-if="speedTrendReady"
          class="statistics-chart__svg statistics-speed-trend__svg"
          :viewBox="`0 0 ${speedChart.width} ${speedChart.height}`"
          role="img"
          :aria-label="t('statistics.speed-trend-title')"
        >
          <defs>
            <linearGradient id="rayload-speed-fill" x1="0" y1="0" x2="0" y2="1">
              <stop offset="0%" stop-color="currentColor" stop-opacity="0.24" />
              <stop offset="100%" stop-color="currentColor" stop-opacity="0" />
            </linearGradient>
          </defs>
          <g class="statistics-chart__grid">
            <line
              v-for="line in speedChart.gridLines"
              :key="`speed-grid-${line.y}`"
              :x1="speedChart.left"
              :y1="line.y"
              :x2="speedChart.right"
              :y2="line.y"
            />
          </g>
          <g class="statistics-speed-trend__axis">
            <text v-for="line in speedChart.gridLines" :key="`speed-label-${line.y}`" x="6" :y="line.y + 3">
              {{ line.label }}
            </text>
            <text :x="speedChart.left" :y="speedChart.height - 6">{{ speedChart.windowLabel }}</text>
            <text :x="speedChart.right" :y="speedChart.height - 6" text-anchor="end">
              {{ t('statistics.speed-trend-now') }}
            </text>
          </g>
          <line
            v-if="speedChart.average > 0"
            class="statistics-speed-trend__avg"
            :x1="speedChart.left"
            :x2="speedChart.right"
            :y1="speedChart.averageY"
            :y2="speedChart.averageY"
          />
          <text
            v-if="speedChart.average > 0"
            class="statistics-speed-trend__avg-label"
            :x="speedChart.left + 2"
            :y="Math.max(12, speedChart.averageY - 5)"
            text-anchor="start"
          >
            {{ t('statistics.speed-trend-average', { speed: speedLabel(speedChart.average) }) }}
          </text>
          <path :d="speedChart.area" fill="url(#rayload-speed-fill)" />
          <path
            :d="speedChart.line"
            fill="none"
            stroke="currentColor"
            stroke-width="2.5"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
          <circle v-if="speedChart.last" class="is-live" :cx="speedChart.last.x" :cy="speedChart.last.y" r="3.6" />
        </svg>
        <p v-else class="statistics-chart__empty">{{ t('statistics.speed-trend-empty') }}</p>
      </NCard>

      <HistoryCategoryCard
        :rows="categories"
        :title="t('statistics.categories-title')"
        :hint="t('statistics.categories-hint')"
      />

      <section class="statistics-shortcuts">
        <button
          type="button"
          class="statistics-shortcuts__toggle"
          :aria-expanded="showShortcuts"
          @click="showShortcuts = !showShortcuts"
        >
          <span>{{ t('statistics.shortcuts-title') }}</span>
          <NIcon :size="18" :class="{ 'is-open': showShortcuts }"><ChevronDownOutline /></NIcon>
        </button>
        <Transition name="shortcuts">
          <div v-if="showShortcuts" class="statistics-links">
            <RouterLink :to="{ name: 'lan-devices' }" class="statistics-link">
              <NIcon :size="20"><WifiOutline /></NIcon>
              <span>{{ t('statistics.open-lan') }}</span>
            </RouterLink>
            <RouterLink :to="{ name: 'task', params: { status: 'completed' } }" class="statistics-link">
              <NIcon :size="20"><TimeOutline /></NIcon>
              <span>{{ t('statistics.open-history') }}</span>
            </RouterLink>
          </div>
        </Transition>
      </section>
    </template>

    <template v-else>
      <section class="statistics-cards">
        <NCard class="stat-card" :bordered="false">
          <div class="stat-card__head">
            <span class="stat-card__icon"
              ><NIcon :size="18"><WifiOutline /></NIcon
            ></span>
            <span class="stat-card__label">{{ t('statistics.all-devices-online') }}</span>
          </div>
          <strong class="stat-card__value">{{ onlineDeviceCount }} / {{ allDevices.length }}</strong>
          <span class="stat-card__hint">{{ t('statistics.all-devices-hint') }}</span>
        </NCard>
        <NCard class="stat-card" :bordered="false">
          <div class="stat-card__head">
            <span class="stat-card__icon"
              ><NIcon :size="18"><CloudDownloadOutline /></NIcon
            ></span>
            <span class="stat-card__label">{{ t('statistics.all-devices-speed') }}</span>
          </div>
          <strong class="stat-card__value"><AnimatedNumber :value="allDeviceSpeed" :formatter="speedLabel" /></strong>
          <StatSparkline :values="allSpeedSamples" :label="t('statistics.all-devices-speed')" />
          <span class="stat-card__hint">{{ t('statistics.live-speed-hint') }}</span>
        </NCard>
        <NCard class="stat-card" :bordered="false">
          <div class="stat-card__head">
            <span class="stat-card__icon"
              ><NIcon :size="18"><LayersOutline /></NIcon
            ></span>
            <span class="stat-card__label">{{ t('statistics.all-devices-active') }}</span>
          </div>
          <strong class="stat-card__value"><AnimatedNumber :value="allDeviceTasks" :formatter="countLabel" /></strong>
          <span class="stat-card__hint">{{ t('statistics.all-devices-hint') }}</span>
        </NCard>
        <NCard class="stat-card" :bordered="false">
          <div class="stat-card__head">
            <span class="stat-card__icon"
              ><NIcon :size="18"><ArchiveOutline /></NIcon
            ></span>
            <span class="stat-card__label">{{ t('statistics.total-cumulative') }}</span>
          </div>
          <strong class="stat-card__value"><AnimatedNumber :value="allHistoryBytes" :formatter="totalLabel" /></strong>
          <StatSparkline :values="allTrendValues" :label="t('statistics.total-cumulative')" />
          <span class="stat-card__hint">{{ t('statistics.all-history-hint') }}</span>
        </NCard>
        <NCard class="stat-card" :bordered="false">
          <div class="stat-card__head">
            <span class="stat-card__icon"
              ><NIcon :size="18"><LayersOutline /></NIcon
            ></span>
            <span class="stat-card__label">{{ t('statistics.total-tasks') }}</span>
          </div>
          <strong class="stat-card__value"><AnimatedNumber :value="allHistoryTasks" :formatter="countLabel" /></strong>
          <span class="stat-card__hint">{{ t('statistics.all-history-hint') }}</span>
        </NCard>
      </section>

      <HistoryTrendCard
        :values="allTrendValues"
        :title="t('statistics.trend-title')"
        :hint="t('statistics.all-trend-hint')"
        :empty-text="t('statistics.trend-empty')"
        :day-label="trendDayLabel"
      />

      <HistoryCategoryCard
        :rows="allCategories"
        :title="t('statistics.categories-title')"
        :hint="t('statistics.all-categories-hint')"
      />

      <NCard class="statistics-all-devices" :bordered="false">
        <div class="statistics-section-heading">
          <div>
            <h2>{{ t('statistics.all-devices-title') }}</h2>
            <p>{{ t('statistics.all-devices-hint') }}</p>
          </div>
        </div>
        <div class="statistics-device-grid">
          <article
            v-for="device in allDevices"
            :key="device.id"
            class="statistics-device"
            :class="{ offline: !device.online }"
          >
            <div class="statistics-device__head">
              <span class="statistics-device-dot" :class="{ offline: !device.online }" aria-hidden="true" />
              <strong>{{ device.name }}</strong>
              <NTag size="small" :bordered="false" :type="device.online ? 'success' : 'default'">
                {{ device.online ? t('lan-devices.online') : t('lan-devices.offline') }}
              </NTag>
            </div>
            <div class="statistics-device__metrics">
              <div>
                <span>{{ t('statistics.live-speed') }}</span>
                <strong>{{ speedLabel(device.downloadSpeed) }}</strong>
              </div>
              <div>
                <span>{{ t('statistics.all-devices-active') }}</span>
                <strong>{{ device.activeTasks }}</strong>
              </div>
              <div>
                <span>{{ t('statistics.total-cumulative') }}</span>
                <strong>{{ totalLabel(device.historyBytes) }}</strong>
              </div>
              <div>
                <span>{{ t('statistics.total-tasks') }}</span>
                <strong>{{ device.historyTasks }}</strong>
              </div>
            </div>
            <div class="statistics-device__meter" aria-hidden="true">
              <i :style="{ width: `${Math.min(100, (device.downloadSpeed / peakDeviceSpeed) * 100)}%` }" />
            </div>
            <code v-if="device.arch || device.address"
              >{{ deviceArchLabel(device.arch) || '—' }} · {{ device.address || '—' }}</code
            >
          </article>
        </div>
      </NCard>
    </template>
  </div>
</template>

<style scoped>
.statistics-view {
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
  gap: 14px;
  height: 100%;
  padding: var(--content-gutter);
  overflow-y: auto;
  overscroll-behavior: contain;
  -webkit-overflow-scrolling: touch;
}
.statistics-view > * {
  flex: 0 0 auto;
}
.statistics-tabs {
  max-width: 420px;
}
.statistics-all-devices {
  overflow: hidden;
  background: var(--m3-surface-container);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-lg);
}
.statistics-device-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
  gap: 10px;
}
.statistics-device {
  display: grid;
  gap: 10px;
  padding: 12px;
  background: var(--m3-surface-container-low);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-md, 12px);
}
.statistics-device.offline {
  opacity: 0.62;
}
.statistics-device__head {
  display: flex;
  gap: 8px;
  align-items: center;
  min-width: 0;
}
.statistics-device__head strong {
  overflow: hidden;
  flex: 1;
  font-size: var(--font-size-sm);
  text-overflow: ellipsis;
  white-space: nowrap;
}
.statistics-device__metrics {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 10px;
}
.statistics-device__metrics div {
  display: grid;
  gap: 2px;
}
.statistics-device__metrics span {
  color: var(--m3-on-surface-variant);
  font-size: 11px;
}
.statistics-device__metrics strong {
  font-size: 16px;
  font-variant-numeric: tabular-nums;
}
.statistics-device__meter {
  overflow: hidden;
  height: 4px;
  background: var(--m3-surface-container-high);
  border-radius: 999px;
}
.statistics-device__meter i {
  display: block;
  height: 100%;
  background: color-mix(in srgb, var(--m3-primary) 66%, var(--m3-outline));
  border-radius: inherit;
  transition: width 0.25s ease;
}
.statistics-device code {
  overflow: hidden;
  color: var(--m3-on-surface-variant);
  font-size: 11px;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.statistics-device-dot {
  flex: none;
  width: 8px;
  height: 8px;
  background: var(--m3-status-success);
  border-radius: 50%;
}
.statistics-device-dot.offline {
  background: var(--m3-outline);
}
.statistics-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 10px;
}
.stat-card {
  position: relative;
  overflow: hidden;
  background: var(--m3-surface-container);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-lg);
  transition:
    transform 0.18s ease,
    box-shadow 0.18s ease;
}
.stat-card::before {
  position: absolute;
  inset: 0 0 auto;
  height: 3px;
  content: '';
  background: linear-gradient(90deg, var(--m3-primary), var(--m3-tertiary));
  opacity: 0.55;
}
.stat-card:active {
  transform: scale(0.99);
}
.stat-card :deep(.n-card-content) {
  display: grid;
  gap: 6px;
}
.stat-card__head {
  display: flex;
  gap: 8px;
  align-items: center;
}
.stat-card__icon {
  display: grid;
  width: 30px;
  height: 30px;
  place-items: center;
  color: var(--m3-on-primary-container);
  background: var(--m3-primary-container);
  border-radius: 9px;
}
.stat-card__label {
  overflow: hidden;
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-sm);
  text-overflow: ellipsis;
  white-space: nowrap;
}
.stat-card__value {
  font-size: clamp(21px, 3.6vw, 28px);
  font-variant-numeric: tabular-nums;
  letter-spacing: -0.02em;
}
.stat-card__hint {
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-xs);
  line-height: 1.45;
}
.stat-card--accent {
  background: linear-gradient(
    160deg,
    color-mix(in srgb, var(--m3-primary) 14%, var(--m3-surface-container)),
    var(--m3-surface-container)
  );
}
.statistics-chart,
.statistics-categories {
  overflow: hidden;
  background: var(--m3-surface-container);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-lg);
}
.statistics-section-heading {
  display: flex;
  gap: 12px;
  align-items: flex-start;
  justify-content: space-between;
  margin-bottom: 10px;
}
.statistics-section-heading h2 {
  margin: 0 0 3px;
  font-size: 17px;
}
.statistics-section-heading p {
  margin: 0;
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-sm);
}
.statistics-chart__svg {
  display: block;
  width: 100%;
  height: auto;
  color: var(--m3-primary);
}
.statistics-chart__grid line {
  stroke: var(--m3-outline-variant);
  stroke-width: 1;
  stroke-dasharray: 3 5;
}
.statistics-chart__dots circle {
  fill: var(--m3-primary);
  stroke: var(--m3-surface-container);
  stroke-width: 1.5;
}
.statistics-chart__dots circle.is-empty {
  fill: var(--m3-outline);
}
.statistics-chart__labels text {
  fill: var(--m3-on-surface-variant);
  font-size: 11px;
  text-anchor: middle;
}
.statistics-speed-trend__svg {
  color: var(--m3-tertiary);
}
.statistics-speed-trend__axis text {
  fill: var(--m3-on-surface-variant);
  font-size: 13px;
  font-variant-numeric: tabular-nums;
}
.statistics-speed-trend__avg {
  stroke: var(--m3-secondary);
  stroke-width: 1.6;
  stroke-dasharray: 5 4;
  opacity: 0.9;
}
.statistics-speed-trend__avg-label {
  fill: var(--m3-secondary);
  font-size: 12px;
  font-weight: 600;
}
.statistics-speed-trend__svg circle.is-live {
  fill: var(--m3-tertiary);
  stroke: var(--m3-surface-container);
  stroke-width: 1.6;
}
.statistics-chart__empty {
  margin: 6px 0 0;
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-xs);
  text-align: center;
}
.statistics-category-grid {
  display: grid;
  gap: 8px;
}
.statistics-category {
  display: grid;
  grid-template-columns: 38px minmax(0, 1fr) 96px;
  gap: 10px;
  align-items: center;
  padding: 8px 10px;
  background: var(--m3-surface-container-low);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-md, 12px);
}
.statistics-category__icon {
  display: grid;
  width: 38px;
  height: 38px;
  place-items: center;
  color: var(--m3-on-primary-container);
  background: var(--m3-primary-container);
  border-radius: 11px;
}
.statistics-category__copy {
  display: grid;
  gap: 2px;
  min-width: 0;
}
.statistics-category__copy strong {
  overflow: hidden;
  font-size: var(--font-size-sm);
  text-overflow: ellipsis;
  white-space: nowrap;
}
.statistics-category__copy span {
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-xs);
  font-variant-numeric: tabular-nums;
}
.statistics-category__bar {
  display: block;
  height: 6px;
  overflow: hidden;
  background: var(--m3-surface-container-highest);
  border-radius: var(--radius-full);
}
.statistics-category__bar span {
  display: block;
  height: 100%;
  background: linear-gradient(90deg, var(--m3-primary), var(--m3-tertiary));
  border-radius: inherit;
  transition: width 0.4s cubic-bezier(0.2, 0, 0, 1);
}
.statistics-shortcuts {
  display: grid;
  gap: 8px;
}
.statistics-shortcuts__toggle {
  display: flex;
  align-items: center;
  justify-content: space-between;
  min-height: 44px;
  padding: 0 12px;
  color: var(--m3-on-surface-variant);
  cursor: pointer;
  background: var(--m3-surface-container);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-lg);
  font-size: var(--font-size-sm);
}
.statistics-shortcuts__toggle .n-icon {
  transition: transform 0.2s ease;
}
.statistics-shortcuts__toggle .n-icon.is-open {
  transform: rotate(180deg);
}
.statistics-links {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(160px, 1fr));
  gap: 8px;
}
.statistics-link {
  display: flex;
  gap: 8px;
  align-items: center;
  min-height: 48px;
  padding: 0 12px;
  color: var(--m3-on-surface);
  text-decoration: none;
  background: var(--m3-surface-container);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-lg);
  font-size: var(--font-size-sm);
}
.shortcuts-enter-active,
.shortcuts-leave-active {
  transition:
    opacity 0.18s ease,
    transform 0.18s ease;
}
.shortcuts-enter-from,
.shortcuts-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}
@media (max-width: 520px) {
  .statistics-category {
    grid-template-columns: 38px minmax(0, 1fr) 72px;
  }
}
</style>
