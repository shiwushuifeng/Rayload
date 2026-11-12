<script setup lang="ts">
/** @fileoverview Compact download overview pinned above the home task list. */
import { computed, onActivated, onBeforeUnmount, onDeactivated, onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import { NIcon } from 'naive-ui'
import { BarChartOutline, ChevronForwardOutline } from '@vicons/ionicons5'
import { useTaskStore } from '@/stores/task'
import { TASK_STATUS } from '@shared/constants'
import { useHistoryStore } from '@/stores/history'
import { bytesToSize } from '@shared/utils/format'
import { logger } from '@shared/logger'

const { t } = useI18n()
const router = useRouter()
const taskStore = useTaskStore()
const historyStore = useHistoryStore()

const historyBytes = ref(0)
const averageSpeed = ref(0)

const sessionBytes = computed(() =>
  taskStore.taskList.reduce((total, task) => total + Math.max(0, Number(task.completedLength) || 0), 0),
)

const averageSpeedLabel = computed(() => `${bytesToSize(averageSpeed.value, 1)}/s`)

const tiles = computed(() => [
  { key: 'total', label: t('statistics.total-downloaded'), value: bytesToSize(historyBytes.value, 2) },
  { key: 'average', label: t('statistics.average-speed'), value: averageSpeedLabel.value },
  { key: 'session', label: t('statistics.session-downloaded'), value: bytesToSize(sessionBytes.value, 2) },
])

async function loadHistory(): Promise<void> {
  try {
    const records = await historyStore.getRecords()
    historyBytes.value = records
      .filter((record) => record.status === 'complete')
      .reduce((total, record) => total + Math.max(0, Number(record.total_length) || 0), 0)
  } catch (error) {
    logger.debug('HomeStatsBar.history', error)
  }
}

function openStatistics(): void {
  void router.push('/statistics')
}

let averageTimer: number | undefined
let lastTickAt = 0
let lastBytes = 0
let activeMs = 0
let activeBytes = 0

function resetAverageSample(): void {
  lastTickAt = Date.now()
  lastBytes = sessionBytes.value
  activeMs = 0
  activeBytes = 0
}

/** Average speed over the time downloads were actually running in this session. */
function sampleAverageSpeed(): void {
  const now = Date.now()
  const elapsed = lastTickAt ? now - lastTickAt : 0
  lastTickAt = now
  const bytes = sessionBytes.value
  const delta = Math.max(0, bytes - lastBytes)
  lastBytes = bytes
  const active = taskStore.taskList.some(
    (task) => task.status === TASK_STATUS.ACTIVE || task.status === TASK_STATUS.WAITING,
  )
  if (!active || elapsed <= 0) return
  activeMs += elapsed
  activeBytes += delta
  averageSpeed.value = activeBytes / (activeMs / 1000)
}

function startAverageTimer(): void {
  stopAverageTimer()
  resetAverageSample()
  averageTimer = window.setInterval(sampleAverageSpeed, 2000)
}

function stopAverageTimer(): void {
  if (averageTimer !== undefined) window.clearInterval(averageTimer)
  averageTimer = undefined
}

onMounted(() => {
  void loadHistory()
  startAverageTimer()
})

onActivated(() => {
  void loadHistory()
  startAverageTimer()
})

onDeactivated(stopAverageTimer)
onBeforeUnmount(stopAverageTimer)
</script>

<template>
  <section class="home-stats" :aria-label="t('navigation.statistics')">
    <div class="home-stats__tiles">
      <div v-for="tile in tiles" :key="tile.key" class="home-stats__tile">
        <span class="home-stats__label">{{ tile.label }}</span>
        <strong class="home-stats__value">{{ tile.value }}</strong>
      </div>
    </div>
    <button type="button" class="home-stats__more" :aria-label="t('navigation.statistics')" @click="openStatistics">
      <NIcon :size="18"><BarChartOutline /></NIcon>
      <NIcon :size="16"><ChevronForwardOutline /></NIcon>
    </button>
  </section>
</template>

<style scoped>
.home-stats {
  display: flex;
  gap: 8px;
  align-items: stretch;
}
.home-stats__tiles {
  display: grid;
  flex: 1;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 8px;
  min-width: 0;
}
.home-stats__tile {
  display: grid;
  gap: 2px;
  min-width: 0;
  padding: 8px 10px;
  background: var(--m3-surface-container);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-md, 12px);
}
.home-stats__label {
  overflow: hidden;
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-xs);
  text-overflow: ellipsis;
  white-space: nowrap;
}
.home-stats__value {
  overflow: hidden;
  color: var(--m3-on-surface);
  font-size: var(--font-size-sm);
  font-variant-numeric: tabular-nums;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.home-stats__more {
  display: grid;
  min-width: 44px;
  min-height: 44px;
  place-items: center;
  color: var(--m3-on-primary-container);
  cursor: pointer;
  background: var(--m3-primary-container);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-md, 12px);
}
</style>
