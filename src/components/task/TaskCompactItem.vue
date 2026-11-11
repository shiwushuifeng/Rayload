<script setup lang="ts">
/** @fileoverview Single-line compact task row with percentage and actions. */
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { TASK_STATUS } from '@shared/constants'
import { NIcon } from 'naive-ui'
import MTooltip from '@/components/common/MTooltip.vue'
import { AlertCircleOutline, RadioOutline, TimeOutline } from '@vicons/ionicons5'
import { useTaskCardModel } from '@/composables/useTaskCardModel'
import { useTaskFileIcon } from '@/composables/useTaskFileIcon'
import { usePlatform } from '@/composables/usePlatform'
import { canOpenTaskContent, dragTaskContent } from '@/composables/useTaskContentOpen'
import { useTaskFileMissing } from '@/composables/useTaskFileMissing'
import TaskItemActions from './TaskItemActions.vue'
import TaskTextTransition from './TaskTextTransition.vue'
import type { Component } from 'vue'
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
  'select-files': [task: Aria2Task]
}>()

const { t } = useI18n()
const { isAndroid } = usePlatform()
const taskRef = computed(() => props.task)
const { taskFullName, isSharing, statusBadge, isActive, indeterminate, percent } = useTaskCardModel(taskRef)
const fileIcon = useTaskFileIcon(taskFullName)
const { fileMissing, fileState } = useTaskFileMissing(taskRef)
// The rail orders the list; the rest of a finished card drags its file out.
const contentDraggable = computed(
  () =>
    !isAndroid.value &&
    !fileMissing.value &&
    !props.actionPending &&
    (props.task.status === TASK_STATUS.COMPLETE || isSharing.value),
)

const compactStatus = computed<{ label: string; tone: string; icon: Component } | null>(() => {
  if (fileMissing.value) {
    return {
      label: fileState.value === 'inaccessible' ? t('task.file-inaccessible') : t('task.file-missing'),
      tone: 'error',
      icon: AlertCircleOutline,
    }
  }
  if (!statusBadge.value) return null

  switch (statusBadge.value.key) {
    case TASK_STATUS.ERROR:
      return { label: statusBadge.value.label, tone: 'error', icon: AlertCircleOutline }
    case TASK_STATUS.WAITING:
      return { label: statusBadge.value.label, tone: statusBadge.value.tone, icon: TimeOutline }
    case 'bt-metadata-fetching':
      return { label: statusBadge.value.label, tone: statusBadge.value.tone, icon: RadioOutline }
    default:
      return { label: statusBadge.value.label, tone: statusBadge.value.tone, icon: RadioOutline }
  }
})

const compactStatusColor = computed(() => {
  if (!compactStatus.value && isActive.value) return 'var(--m3-status-active)'
  switch (compactStatus.value?.tone) {
    case 'error':
      return 'var(--m3-error)'
    case 'success':
      return 'var(--m3-status-success)'
    case 'waiting':
      return 'var(--m3-status-waiting)'
    default:
      return 'var(--m3-on-surface-variant)'
  }
})
</script>

<template>
  <div
    class="task-compact-item"
    :class="{
      'is-sharing': isSharing,
    }"
    @dblclick="!fileMissing && !actionPending && canOpenTaskContent(task, $event) && emit('open-file', task)"
  >
    <div class="compact-body" :draggable="contentDraggable" @dragstart="dragTaskContent(task, $event)">
      <span class="compact-file-icon" :class="`compact-file-icon--${fileIcon.tone}`" aria-hidden="true">
        <NIcon :size="15"><component :is="fileIcon.icon" /></NIcon>
      </span>
      <MTooltip placement="bottom-start">
        <template #trigger>
          <div class="compact-name">
            <TaskTextTransition :value="taskFullName">{{ taskFullName }}</TaskTextTransition>
          </div>
        </template>
        {{ taskFullName }}
      </MTooltip>
      <span
        class="compact-percent"
        :style="{ color: compactStatusColor }"
        :title="compactStatus?.label ?? t('task.status-active')"
      >
        {{ indeterminate ? '—' : `${percent}%` }}
      </span>
      <span class="compact-status-text">{{ compactStatus?.label ?? t('task.status-active') }}</span>
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
        @select-files="emit('select-files', task)"
      />
    </div>
  </div>
</template>

<style scoped>
.task-compact-item {
  position: relative;
  display: grid;
  grid-template-columns: minmax(0, 1fr);
  min-height: 44px;
  background-color: var(--task-item-bg);
  border: 1px solid var(--m3-outline-variant);
  border-left: 3px solid var(--m3-outline-variant);
  border-radius: var(--radius-sm);
  overflow: hidden;
  transition: border-color 0.2s cubic-bezier(0.2, 0, 0, 1);
}
.task-compact-item::before {
  content: '';
  position: absolute;
  inset: 0;
  border-radius: inherit;
  background: linear-gradient(90deg, color-mix(in srgb, var(--m3-success) 6%, transparent) 0%, transparent 40%);
  opacity: 0;
  pointer-events: none;
  transition: opacity var(--task-motion-state) var(--task-motion-ease);
}
.task-compact-item.is-sharing {
  border-left-color: var(--m3-success);
}
.task-compact-item.is-sharing::before {
  opacity: 1;
}
.compact-body {
  display: flex;
  align-items: center;
  gap: 8px;
  min-width: 0;
  min-height: 44px;
  padding: 6px 8px;
}
.compact-file-icon {
  display: grid;
  flex: 0 0 24px;
  width: 24px;
  height: 24px;
  place-items: center;
  color: var(--m3-primary);
  background: color-mix(in srgb, var(--m3-primary) 12%, var(--m3-surface-container));
  border-radius: 8px;
}
.compact-file-icon--video {
  color: var(--m3-tertiary);
  background: color-mix(in srgb, var(--m3-tertiary) 12%, var(--m3-surface-container));
}
.compact-file-icon--audio {
  color: var(--m3-success);
  background: color-mix(in srgb, var(--m3-success) 12%, var(--m3-surface-container));
}
.compact-file-icon--image {
  color: var(--m3-secondary);
  background: color-mix(in srgb, var(--m3-secondary) 12%, var(--m3-surface-container));
}
.compact-body :deep(.n-tooltip-trigger) {
  flex: 1 1 auto;
  min-width: 0;
  max-width: 100%;
}
.compact-body :deep(.task-item-actions) {
  flex: 0 0 auto;
}
.compact-name {
  display: block;
  min-width: 0;
  max-width: 100%;
  overflow: hidden;
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-base);
  line-height: 20px;
  white-space: nowrap;
  text-overflow: ellipsis;
}
.compact-name > .task-text-transition {
  display: block;
  min-width: 0;
}
.compact-name :deep(.task-text-transition-content) {
  display: block;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.compact-percent {
  flex: 0 0 auto;
  min-width: 4ch;
  font-size: var(--font-size-xs);
  line-height: 14px;
  font-weight: 600;
  text-align: end;
  font-variant-numeric: tabular-nums;
}
.compact-status-text {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border: 0;
}
</style>
