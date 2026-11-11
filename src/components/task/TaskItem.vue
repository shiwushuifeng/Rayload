<script setup lang="ts">
/** @fileoverview Individual task row in the task list with progress and controls. */
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { TASK_STATUS } from '@shared/constants'
import { NIcon } from 'naive-ui'
import LinearProgress from '@/components/common/LinearProgress.vue'
import MTooltip from '@/components/common/MTooltip.vue'
import {
  ArrowUpOutline,
  ArrowDownOutline,
  MagnetOutline,
  AlertCircleOutline,
  CloudUploadOutline,
  CheckmarkCircleOutline,
  TrashOutline,
  RadioOutline,
  TimeOutline,
  ListOutline,
  PauseOutline,
} from '@vicons/ionicons5'
import { useTaskCardModel } from '@/composables/useTaskCardModel'
import { useTaskFileIcon } from '@/composables/useTaskFileIcon'
import { usePlatform } from '@/composables/usePlatform'
import { canOpenTaskContent, dragTaskContent } from '@/composables/useTaskContentOpen'
import { useTaskFileMissing } from '@/composables/useTaskFileMissing'
import TaskItemActions from './TaskItemActions.vue'
import TaskTextTransition from './TaskTextTransition.vue'
import type { Aria2Task } from '@shared/types'

const props = withDefaults(defineProps<{ task: Aria2Task; actionPending?: boolean }>(), { actionPending: false })
const emit = defineEmits<{
  pause: [task: Aria2Task]
  resume: [task: Aria2Task]
  retry: [task: Aria2Task]
  redownload: [task: Aria2Task]
  'finish-sharing': [task: Aria2Task]
  'finish-media': [task: Aria2Task]
  delete: [task: Aria2Task]
  'delete-record': [task: Aria2Task]
  'copy-link': [task: Aria2Task]
  'show-info': [task: Aria2Task]
  folder: [task: Aria2Task]
  checksum: [task: Aria2Task]
  'open-file': [task: Aria2Task]
  'share-file': [task: Aria2Task]
  'select-files': [task: Aria2Task]
}>()

const { t } = useI18n()
const { isAndroid } = usePlatform()
const taskRef = computed(() => props.task)

const {
  taskFullName,
  isSharing,
  statusBadge,
  taskStatus,
  isActive,
  indeterminate,
  percent,
  completedSize,
  totalSize,
  hasSizeInfo,
  downloadSpeed,
  uploadSpeed,
  remaining,
  remainingText,
  transferSummary,
} = useTaskCardModel(taskRef)
const fileIcon = useTaskFileIcon(taskFullName)

const statusColorMap: Record<string, string> = {
  active: 'var(--m3-status-active)',
  waiting: 'var(--m3-status-waiting)',
  paused: 'var(--m3-status-paused)',
  error: 'var(--m3-status-error)',
  complete: 'var(--m3-status-success)',
  removed: 'var(--m3-status-paused)',
  sharing: 'var(--m3-status-success)',
}

const progressColor = computed(() => statusColorMap[taskStatus.value] || 'var(--m3-status-active)')
const displayErrorMessage = computed(() => props.task.errorMessage || props.task.bittorrent?.error?.message || '')
const displayStatus = computed(() => {
  if (fileMissing.value)
    return fileState.value === 'inaccessible' ? t('task.file-inaccessible') : t('task.file-missing')
  if (statusBadge.value) return statusBadge.value.label
  switch (taskStatus.value) {
    case 'waiting':
      return t('task.status-waiting')
    case 'paused':
      return t('task.status-paused')
    case 'error':
      return t('task.status-error')
    case 'complete':
      return t('task.status-complete')
    case 'sharing':
      return t('task.status-sharing')
    default:
      return t('task.status-active')
  }
})
const statusTextStyle = computed(() => {
  if (fileMissing.value) return { color: 'var(--m3-error)' }
  switch (statusBadge.value?.tone) {
    case 'success':
      return { color: 'var(--m3-status-success)' }
    case 'error':
      return { color: 'var(--m3-status-error)' }
    case 'waiting':
      return { color: 'var(--m3-status-waiting)' }
    case 'muted':
      return { color: 'var(--m3-status-paused)' }
  }
  switch (taskStatus.value) {
    case 'active':
      return { color: 'var(--m3-status-active)' }
    case 'paused':
      return { color: 'var(--m3-status-paused)' }
    case 'waiting':
      return { color: 'var(--m3-status-waiting)' }
    case 'complete':
    case 'sharing':
      return { color: 'var(--m3-status-success)' }
    case 'error':
      return { color: 'var(--m3-status-error)' }
    default:
      return { color: 'var(--m3-on-surface-variant)' }
  }
})

const statusBadgeIcon = computed(() => {
  if (!statusBadge.value) {
    switch (taskStatus.value) {
      case 'active':
        return RadioOutline
      case 'paused':
        return PauseOutline
      case 'waiting':
        return TimeOutline
      case 'complete':
        return CheckmarkCircleOutline
      case 'error':
        return AlertCircleOutline
      default:
        return CloudUploadOutline
    }
  }
  switch (statusBadge.value?.key) {
    case TASK_STATUS.COMPLETE:
      return CheckmarkCircleOutline
    case TASK_STATUS.ERROR:
    case 'bt-error':
      return AlertCircleOutline
    case TASK_STATUS.REMOVED:
      return TrashOutline
    case TASK_STATUS.WAITING:
      return TimeOutline
    case 'awaiting-selection':
    case 'bt-file-selection':
      return ListOutline
    case 'probing':
    case 'recording':
    case 'downloading':
    case 'finalizing':
    case 'bt-metadata-fetching':
      return RadioOutline
    default:
      return CloudUploadOutline
  }
})

const { fileMissing, fileState } = useTaskFileMissing(taskRef)
// The rail orders the list; the rest of a finished card drags its file out.
const contentDraggable = computed(
  () =>
    !isAndroid.value &&
    !fileMissing.value &&
    !props.actionPending &&
    (props.task.status === TASK_STATUS.COMPLETE || isSharing.value),
)
</script>

<template>
  <div
    class="task-item"
    :class="{
      'is-sharing': isSharing,
    }"
    @dblclick="!fileMissing && !actionPending && canOpenTaskContent(task, $event) && emit('open-file', task)"
  >
    <div class="task-body" :draggable="contentDraggable" @dragstart="dragTaskContent(task, $event)">
      <div class="task-primary">
        <span class="file-type-icon" :class="`file-type-icon--${fileIcon.tone}`" aria-hidden="true">
          <NIcon :size="18"><component :is="fileIcon.icon" /></NIcon>
        </span>
        <div class="task-heading">
          <MTooltip placement="bottom-start">
            <template #trigger>
              <div class="task-name">
                <TaskTextTransition :value="taskFullName">
                  <span class="technical-text-wrap">{{ taskFullName }}</span>
                </TaskTextTransition>
              </div>
            </template>
            {{ taskFullName }}
          </MTooltip>
          <div class="task-progress">
            <LinearProgress v-if="!indeterminate" :percentage="percent" :color="progressColor" :height="6" />
            <div class="task-progress-info">
              <span class="progress-left" :class="{ 'info-hidden': !hasSizeInfo }">
                {{ indeterminate ? '' : `${percent}% · ` }}{{ completedSize }} / {{ totalSize }}
              </span>
              <span class="progress-right" :class="{ 'is-idle': !isActive }">
                <span v-if="remaining > 0" class="speed-text">{{ remainingText }}</span>
                <span v-if="transferSummary.showUploadMetrics" class="speed-text">
                  <NIcon :size="10"><ArrowUpOutline /></NIcon>
                  <span>{{ uploadSpeed }}/s</span>
                </span>
                <span class="speed-text">
                  <NIcon :size="10"><ArrowDownOutline /></NIcon>
                  <span>{{ downloadSpeed }}/s</span>
                </span>
                <span v-if="transferSummary.showSeeders" class="speed-text">
                  <NIcon :size="10"><MagnetOutline /></NIcon>
                  <span>{{ task.numSeeders }}</span>
                </span>
              </span>
            </div>
          </div>
        </div>
        <span class="task-status-text" :style="statusTextStyle">
          <NIcon :size="13"><component :is="statusBadgeIcon" /></NIcon>
          <span>{{ displayStatus }}</span>
        </span>
      </div>
      <div class="task-actions-row">
        <TaskItemActions
          :task="task"
          :file-missing="fileMissing"
          :pending="actionPending"
          density="compact"
          @pause="emit('pause', task)"
          @resume="emit('resume', task)"
          @retry="emit('retry', task)"
          @redownload="emit('redownload', task)"
          @finish-sharing="emit('finish-sharing', task)"
          @finish-media="emit('finish-media', task)"
          @delete="emit('delete', task)"
          @delete-record="emit('delete-record', task)"
          @copy-link="emit('copy-link', task)"
          @show-info="emit('show-info', task)"
          @folder="emit('folder', task)"
          @checksum="emit('checksum', task)"
          @share-file="emit('share-file', task)"
          @select-files="emit('select-files', task)"
        />
      </div>
      <div v-if="displayErrorMessage" class="error-message technical-text-wrap">
        {{ displayErrorMessage }}
      </div>
    </div>
  </div>
</template>

<style scoped>
.task-item {
  position: relative;
  display: grid;
  grid-template-columns: minmax(0, 1fr);
  min-height: 78px;
  background-color: var(--task-item-bg);
  border: 1px solid var(--m3-outline-variant);
  /* Reserve 3px left border at base color so sharing only animates color */
  border-left: 3px solid var(--m3-outline-variant);
  border-radius: var(--radius-sm);
  overflow: hidden;
  transition: border-color 0.2s cubic-bezier(0.2, 0, 0, 1);
}
/* Gradient overlay — always present, hidden by default */
.task-item::before {
  content: '';
  position: absolute;
  inset: 0;
  border-radius: inherit;
  background: linear-gradient(90deg, color-mix(in srgb, var(--m3-success) 6%, transparent) 0%, transparent 40%);
  opacity: 0;
  pointer-events: none;
  transition: opacity var(--task-motion-state) var(--task-motion-ease);
}
/* ── Seeding state (static) ────────────────────────────────────────── */
.task-item.is-sharing {
  border-left-color: var(--m3-success);
}
.task-item.is-sharing::before {
  opacity: 1;
}
.task-body {
  display: flex;
  flex-direction: column;
  gap: 8px;
  min-width: 0;
  padding: 12px;
}
/*
 * Information row: icon, filename/progress, status badge.
 * The action toolbar lives on its own row below so the metadata line and
 * buttons can never share width or collide.
 */
.task-primary {
  display: grid;
  grid-template-columns: 36px minmax(0, 1fr) auto;
  column-gap: 10px;
  align-items: start;
  min-width: 0;
}
.file-type-icon {
  display: grid;
  width: 36px;
  height: 36px;
  place-items: center;
  align-self: center;
  border-radius: 10px;
  color: var(--m3-primary);
  background: color-mix(in srgb, var(--m3-primary) 12%, var(--m3-surface-container));
}
.file-type-icon--video {
  color: var(--m3-tertiary);
  background: color-mix(in srgb, var(--m3-tertiary) 12%, var(--m3-surface-container));
}
.file-type-icon--audio {
  color: var(--m3-success);
  background: color-mix(in srgb, var(--m3-success) 12%, var(--m3-surface-container));
}
.file-type-icon--image {
  color: var(--m3-secondary);
  background: color-mix(in srgb, var(--m3-secondary) 12%, var(--m3-surface-container));
}
.file-type-icon--archive,
.file-type-icon--disk {
  color: var(--m3-status-waiting);
  background: color-mix(in srgb, var(--m3-status-waiting) 14%, var(--m3-surface-container));
}
.task-heading {
  display: flex;
  flex-direction: column;
  gap: 5px;
  min-width: 0;
}
.task-primary :deep(.n-tooltip-trigger) {
  display: block;
  min-width: 0;
  max-width: 100%;
}
.task-name {
  display: block;
  min-width: 0;
  color: var(--m3-on-surface-variant);
  overflow: hidden;
}
.task-name > .task-text-transition {
  display: block;
  min-width: 0;
}
.task-name :deep(.task-text-transition-content) {
  display: block;
  font-size: var(--font-size-base);
  line-height: 22px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.task-name :deep(.technical-text-wrap) {
  display: block;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.task-status-text {
  display: inline-flex;
  align-items: center;
  flex-shrink: 0;
  gap: 4px;
  max-width: min(10em, 36vw);
  padding: 3px 8px;
  overflow: hidden;
  font-size: var(--font-size-sm);
  line-height: 18px;
  white-space: nowrap;
  text-overflow: ellipsis;
  border-radius: var(--radius-full);
  background: color-mix(in srgb, currentColor 9%, transparent);
}
.task-status-text > span {
  overflow: hidden;
  text-overflow: ellipsis;
}
.task-progress {
  display: flex;
  flex-direction: column;
  min-width: 0;
}
.task-progress-info {
  display: flex;
  flex-wrap: wrap;
  align-content: flex-start;
  align-items: center;
  justify-content: space-between;
  gap: 2px 10px;
  min-width: 0;
  /* Reserve two metadata lines so pausing hides the speed metrics without
   * changing the card height. */
  min-height: 30px;
  margin-top: 4px;
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-xs);
  line-height: 14px;
  white-space: nowrap;
  font-variant-numeric: tabular-nums;
}
.progress-left {
  flex: 1 1 100%;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  transition: opacity var(--task-motion-state) var(--task-motion-ease);
}
.progress-right {
  display: inline-flex;
  flex: 1 1 100%;
  flex-wrap: wrap;
  justify-content: flex-start;
  align-items: center;
  gap: 2px 8px;
  white-space: nowrap;
  transition: opacity var(--task-motion-state) var(--task-motion-ease);
}
.progress-right.is-idle {
  opacity: 0.55;
}
.speed-text {
  display: inline-flex;
  align-items: center;
  gap: 2px;
  font-size: var(--font-size-xs);
  line-height: 14px;
  white-space: nowrap;
  transition: opacity 0.25s cubic-bezier(0.2, 0, 0, 1);
}
.info-hidden {
  display: none;
}
.task-actions-row {
  display: flex;
  justify-content: flex-end;
  min-width: 0;
  padding-top: 8px;
  border-top: 1px solid color-mix(in srgb, var(--m3-outline-variant) 60%, transparent);
}
.error-message {
  flex-basis: 100%;
  font-size: var(--font-size-2xs);
  color: var(--m3-error);
  opacity: 0.85;
  line-height: 1.4;
}
</style>
