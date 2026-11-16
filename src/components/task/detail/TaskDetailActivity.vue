<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import LinearProgress from '@/components/common/LinearProgress.vue'
import AnimatedNumber from '@/components/common/AnimatedNumber.vue'
import { TASK_STATUS } from '@shared/constants'
import { bytesToSize, calcProgress, getTaskCompletedLength, timeFormat, timeRemaining } from '@shared/utils'
import type { Aria2Task } from '@shared/types'
import type { TaskTransferSummary } from '@/composables/useTaskDetailSummary'
import TaskGraphic from '../TaskGraphic.vue'

const props = defineProps<{
  task: Aria2Task | null
  transferSummary: TaskTransferSummary
}>()

const { t } = useI18n()
const formatSpeed = (value: number) => bytesToSize(Math.max(0, Math.round(value)))
const formatPercent = (value: number) => `${Math.round(value)}%`
const isActive = computed(() => props.task?.status === TASK_STATUS.ACTIVE)
const completedLengthValue = computed(() => (props.task ? getTaskCompletedLength(props.task) : 0))
const percent = computed(() => (props.task ? calcProgress(props.task.totalLength, completedLengthValue.value) : 0))
const remaining = computed(() => {
  if (!isActive.value || !props.task) return 0
  return timeRemaining(Number(props.task.totalLength), completedLengthValue.value, Number(props.task.downloadSpeed))
})
const remainingText = computed(() => {
  if (remaining.value <= 0) return ''
  return timeFormat(remaining.value, {
    prefix: t('task.remaining-prefix') || '',
    i18n: {
      gt1d: t('app.gt1d') || '>1d',
      hour: t('app.hour') || 'h',
      minute: t('app.minute') || 'm',
      second: t('app.second') || 's',
    },
  })
})
</script>

<template>
  <div v-if="task" class="activity-cards">
    <div v-if="task.bitfield" class="activity-card activity-card--graphic">
      <TaskGraphic :bitfield="task.bitfield" />
    </div>
    <div class="activity-card">
      <span>{{ t('task.task-progress-info') || 'Progress' }}</span>
      <div class="progress-row">
        <LinearProgress :percentage="percent" :height="10" />
        <span class="progress-pct"><AnimatedNumber :value="percent" :formatter="formatPercent" /></span>
      </div>
    </div>
    <div class="activity-card">
      <span>{{ t('task.task-file-size') || 'Size' }}</span>
      <strong>
        {{ bytesToSize(completedLengthValue, 2) }}
        <span v-if="Number(task.totalLength) > 0"> / {{ bytesToSize(task.totalLength, 2) }}</span>
        <span v-if="remainingText" class="remaining-text">{{ remainingText }}</span>
      </strong>
    </div>
    <div class="activity-card">
      <span>{{ t('task.task-download-speed') || 'DL Speed' }}</span>
      <strong><AnimatedNumber :value="Number(task.downloadSpeed) || 0" :formatter="formatSpeed" />/s</strong>
    </div>
    <div v-if="transferSummary.showUploadMetrics" class="activity-card">
      <span>{{ t('task.task-upload-speed') || 'UL Speed' }}</span>
      <strong><AnimatedNumber :value="Number(task.uploadSpeed) || 0" :formatter="formatSpeed" />/s</strong>
    </div>
    <div v-if="transferSummary.showUploadMetrics" class="activity-card">
      <span>{{ t('task.task-upload-length') || 'Uploaded' }}</span>
      <strong>{{ bytesToSize(task.uploadLength) }}</strong>
    </div>
    <div v-if="transferSummary.showUploadMetrics" class="activity-card">
      <span>{{ t('task.task-ratio') || 'Ratio' }}</span>
      <strong>{{ transferSummary.ratio }}</strong>
    </div>
    <div v-if="transferSummary.showSeeders" class="activity-card">
      <span>{{ t('task.task-num-seeders') || 'Seeders' }}</span>
      <strong>{{ task.numSeeders }}</strong>
    </div>
    <div class="activity-card">
      <span>{{ t('task.task-connections') || 'Connections' }}</span>
      <strong>{{ task.connections }}</strong>
    </div>
  </div>
</template>

<style scoped>
.activity-cards {
  display: grid;
  gap: 8px;
}
.activity-card {
  display: grid;
  grid-template-columns: minmax(88px, 0.36fr) minmax(0, 1fr);
  gap: 10px;
  align-items: center;
  padding: 10px 12px;
  background: var(--m3-surface-container-low);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-md);
}
.activity-card > span {
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-xs);
}
.activity-card strong {
  overflow-wrap: anywhere;
  font-weight: 500;
  font-variant-numeric: tabular-nums;
}
.activity-card--graphic {
  display: block;
}
.progress-row {
  display: flex;
  align-items: center;
  gap: 12px;
}
.progress-pct {
  white-space: nowrap;
  font-size: var(--font-size-xs);
  color: var(--m3-on-surface-variant);
  min-width: 45px;
  text-align: right;
}
.remaining-text {
  margin-left: 12px;
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-xs);
}
</style>
