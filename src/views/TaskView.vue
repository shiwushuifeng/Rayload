<script setup lang="ts">
/** @fileoverview Task list view, task actions, and batch selection controls. */
import { computed, h, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useTaskStore } from '@/stores/task'
import { useTaskSelectionStore } from '@/stores/taskSelection'
import { usePreferenceStore } from '@/stores/preference'
import { remoteTaskToAria2, useLanStore } from '@/stores/lan'

import { useTaskActions } from '@/composables/useTaskActions'

import { NButton, NCheckbox, NIcon, useDialog } from 'naive-ui'
import { CheckmarkDoneOutline, CloseOutline, PauseOutline, PlayOutline, TrashOutline } from '@vicons/ionicons5'
import { useAppMessage } from '@/composables/useAppMessage'
import { triggerHaptic } from '@/composables/useHaptics'
import { isEngineReady } from '@/api/aria2'
import { logger } from '@shared/logger'
import { getErrorMessage } from '@shared/utils/errorMessage'
import { TASK_STATUS } from '@shared/constants'
import type { Aria2Task, TaskDeletionOptions } from '@shared/types'
import TaskList from '@/components/task/TaskList.vue'
import TaskDetail from '@/components/task/TaskDetail.vue'
import TaskEmptyBrand from '@/components/task/TaskEmptyBrand.vue'
import HomeStatsBar from '@/components/task/HomeStatsBar.vue'

const props = withDefaults(defineProps<{ status?: string }>(), { status: 'all' })

const { t } = useI18n()
const taskStore = useTaskStore()
const preferenceStore = usePreferenceStore()
const lan = useLanStore()
const remoteTasks = computed(() => {
  const scope = props.status ?? 'all'
  return lan.remoteTasks
    .map(remoteTaskToAria2)
    .filter((task) =>
      scope === 'all'
        ? true
        : scope === 'progress'
          ? ['active', 'waiting', 'paused'].includes(task.status)
          : task.status === (scope === 'failed' ? 'error' : 'complete'),
    )
})
const showEmptyState = computed(() => taskStore.isCurrentListEmpty && remoteTasks.value.length === 0)
const dialog = useDialog()
const message = useAppMessage()

const selectedTasks = computed(() => {
  const selected = new Set(taskStore.checkedGids)
  return taskStore.taskList.filter((task) => selected.has(task.gid))
})
const allSelected = computed(
  () => taskStore.taskList.length > 0 && taskStore.taskList.every((task) => taskStore.checkedGids.includes(task.gid)),
)
const canPauseSelected = computed(() =>
  selectedTasks.value.some((task) => task.status === TASK_STATUS.ACTIVE || task.status === TASK_STATUS.WAITING),
)
const canResumeSelected = computed(() => selectedTasks.value.some((task) => task.status === TASK_STATUS.PAUSED))
const deleteFilesLabel = computed(() =>
  t(
    preferenceStore.config.fileDeletionMode === 'permanent'
      ? 'task.delete-local-files-permanent-label'
      : 'task.delete-local-files-trash-label',
  ),
)

function requireEngine(): boolean {
  if (isEngineReady()) return true
  message.warning(t('app.engine-not-ready'))
  return false
}

function toggleSelectAll() {
  triggerHaptic()
  if (allSelected.value) taskStore.clearChecked()
  else taskStore.checkAll(taskStore.taskList.map((task: Aria2Task) => task.gid))
}

async function batchPauseSelected() {
  if (!requireEngine() || !canPauseSelected.value) return
  triggerHaptic()
  try {
    const failed = await taskStore.pauseTasks(selectedTasks.value)
    if (failed > 0) message.warning(t('task.pause-all-task-fail'))
    else message.success(t('task.pause-all-task-success'))
    taskStore.clearChecked()
  } catch (error) {
    logger.warn('TaskView.batchPause', getErrorMessage(error))
    message.error(t('task.pause-all-task-fail'))
  }
}

async function batchResumeSelected() {
  if (!requireEngine() || !canResumeSelected.value) return
  triggerHaptic()
  try {
    const failed = await taskStore.resumeTasks(selectedTasks.value)
    if (failed > 0) message.warning(t('task.resume-all-task-fail'))
    else message.success(t('task.resume-all-task-success'))
    taskStore.clearChecked()
  } catch (error) {
    logger.warn('TaskView.batchResume', getErrorMessage(error))
    message.error(t('task.resume-all-task-fail'))
  }
}

function batchDeleteSelected() {
  if (!requireEngine() || selectedTasks.value.length === 0) return
  triggerHaptic()
  const gids = selectedTasks.value.map((task) => task.gid)
  const count = gids.length
  const deleteFiles = ref(false)
  const d = dialog.error({
    title: t('task.delete-selected'),
    content: () =>
      h('div', {}, [
        h('p', { style: 'margin: 0 0 12px;' }, t('task.batch-delete-task-confirm', { count })),
        h(
          NCheckbox,
          {
            checked: deleteFiles.value,
            'onUpdate:checked': (value: boolean) => {
              deleteFiles.value = value
            },
          },
          { default: () => deleteFilesLabel.value },
        ),
      ]),
    positiveText: t('app.yes'),
    negativeText: t('app.no'),
    onPositiveClick: async () => {
      d.loading = true
      d.negativeButtonProps = { disabled: true }
      d.closable = false
      d.maskClosable = false
      try {
        const options: TaskDeletionOptions = deleteFiles.value
          ? { deleteMode: preferenceStore.config.fileDeletionMode }
          : {}
        const result = await taskStore.batchRemoveTask(gids, options)
        if (result.failed.length > 0) {
          const key = result.succeeded.length > 0 ? 'task.batch-delete-task-partial' : 'task.batch-delete-task-fail'
          message[result.succeeded.length > 0 ? 'warning' : 'error'](
            t(key, { removed: result.succeeded.length, failed: result.failed.length }),
          )
        } else {
          message.success(t('task.batch-delete-task-success'))
        }
        taskStore.clearChecked()
      } catch (error) {
        logger.warn('TaskView.batchDelete', getErrorMessage(error))
        message.error(t('task.batch-delete-task-fail'))
      } finally {
        d.destroy()
      }
    },
  })
}

const {
  handlePauseTask,
  handleResumeTask,
  handleRetryTask,
  handleRedownloadTask,
  handleFinishSharing,
  handleFinishMedia,
  handleDeleteTask,
  handleDeleteRecord,
  handleCopyLink,
  handleShowInfo,
  handleShowInFolder,
  handleVerifyChecksum,
  handleOpenFile,
  handleShareFile,
  handleSelectFiles,
} = useTaskActions({
  taskStore,
  preferenceConfig: () => preferenceStore.config,
  t,
  dialog,
  message,
  requestMagnetSelection: (gid) => useTaskSelectionStore().request({ kind: 'bt', gid }),
})

watch(
  () => props.status,
  (status) => {
    void taskStore.changeCurrentList(status)
  },
  { immediate: true },
)

onMounted(() => lan.startTaskPolling())
onBeforeUnmount(() => lan.stopTaskPolling())

async function remoteAction(task: Aria2Task, action: 'pause' | 'unpause' | 'remove'): Promise<void> {
  if (!task.remote) return
  try {
    await lan.remoteAction(task.remote.deviceId, task.remote.gid, action)
    if (action === 'remove') message.success(t('task.remote-removed'))
  } catch (error) {
    logger.warn('TaskView.remoteAction', getErrorMessage(error))
    message.error(getErrorMessage(error))
  }
}

function pauseTask(task: Aria2Task): void {
  if (task.remote) void remoteAction(task, 'pause')
  else handlePauseTask(task)
}

function resumeTask(task: Aria2Task): void {
  if (task.remote) void remoteAction(task, 'unpause')
  else handleResumeTask(task)
}

function deleteTask(task: Aria2Task): void {
  if (task.remote) void remoteAction(task, 'remove')
  else handleDeleteTask(task)
}
</script>

<template>
  <div class="task-view">
    <Transition name="selection-bar">
      <div v-if="taskStore.selectionMode" class="task-selection-header">
        <NButton
          quaternary
          circle
          class="selection-close"
          :aria-label="t('task.clear-selection')"
          @click="taskStore.clearChecked()"
        >
          <template #icon>
            <NIcon><CloseOutline /></NIcon>
          </template>
        </NButton>
        <span class="selection-spacer" aria-hidden="true" />
        <NButton size="small" secondary @click="toggleSelectAll">
          <template #icon>
            <NIcon><CheckmarkDoneOutline /></NIcon>
          </template>
          {{ allSelected ? t('task.clear-selection') : t('app.menu-select-all') }}
        </NButton>
      </div>
    </Transition>
    <div class="panel-body">
      <TaskEmptyBrand :show="showEmptyState" :show-logo="preferenceStore.config.showLogoWhenEmpty" />
      <div class="panel-content" :class="{ 'panel-content--selecting': taskStore.selectionMode }">
        <HomeStatsBar v-if="!showEmptyState && !taskStore.selectionMode" class="task-home-stats" />
        <TaskList
          :remote-tasks="remoteTasks"
          @pause="pauseTask"
          @resume="resumeTask"
          @retry="handleRetryTask"
          @redownload="handleRedownloadTask"
          @finish-sharing="handleFinishSharing"
          @finish-media="handleFinishMedia"
          @delete="deleteTask"
          @delete-record="handleDeleteRecord"
          @copy-link="handleCopyLink"
          @show-info="handleShowInfo"
          @folder="handleShowInFolder"
          @checksum="handleVerifyChecksum"
          @open-file="handleOpenFile"
          @share-file="handleShareFile"
          @select-files="handleSelectFiles"
        />
      </div>
    </div>
    <Transition name="selection-bar">
      <div v-if="taskStore.selectionMode" class="task-batch-bar">
        <NButton class="batch-button" :disabled="!canPauseSelected" @click="batchPauseSelected">
          <template #icon>
            <NIcon><PauseOutline /></NIcon>
          </template>
          {{ t('task.pause-selected') }}
        </NButton>
        <NButton class="batch-button" :disabled="!canResumeSelected" @click="batchResumeSelected">
          <template #icon>
            <NIcon><PlayOutline /></NIcon>
          </template>
          {{ t('task.resume-selected') }}
        </NButton>
        <NButton class="batch-button" type="error" secondary @click="batchDeleteSelected">
          <template #icon>
            <NIcon><TrashOutline /></NIcon>
          </template>
          {{ t('task.delete-selected') }}
        </NButton>
      </div>
    </Transition>
    <TaskDetail
      :show="taskStore.taskDetailVisible"
      :task="taskStore.currentTaskItem"
      :files="taskStore.currentTaskFiles"
      @close="taskStore.hideTaskDetail()"
    />
  </div>
</template>

<style scoped>
.task-view {
  position: relative;
  height: 100%;
  display: flex;
  flex-direction: column;
}
.task-selection-header {
  display: flex;
  align-items: center;
  gap: 10px;
  min-height: 52px;
  padding: 4px var(--content-gutter);
  color: var(--m3-on-primary-container);
  background: color-mix(in srgb, var(--m3-primary-container) 46%, var(--m3-surface-container));
  border-bottom: 1px solid color-mix(in srgb, var(--m3-primary) 28%, var(--m3-outline-variant));
  transition:
    background-color 240ms var(--task-motion-ease),
    border-color 240ms var(--task-motion-ease),
    color 240ms var(--task-motion-ease);
}
.selection-close {
  flex: 0 0 auto;
  width: 44px;
  height: 44px;
}
.selection-spacer {
  flex: 1;
  min-width: 0;
}
.panel-body {
  position: relative;
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
}
.panel-content {
  padding: 0;
  flex: 1;
  min-height: 0;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
}
.task-home-stats {
  margin: 10px var(--content-gutter, 16px) 4px;
}
.panel-content--selecting {
  padding-bottom: calc(76px + env(safe-area-inset-bottom, 0px));
}
.task-batch-bar {
  position: absolute;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 5;
  display: flex;
  gap: 8px;
  padding: 10px var(--content-gutter) calc(10px + env(safe-area-inset-bottom, 0px));
  background: color-mix(in srgb, var(--m3-surface-container) 96%, transparent);
  border-top: 1px solid var(--m3-outline-variant);
  backdrop-filter: blur(16px);
}
.batch-button {
  flex: 1;
  min-height: 44px;
}
.selection-bar-enter-active,
.selection-bar-leave-active {
  transition:
    opacity 160ms ease,
    transform 200ms cubic-bezier(0.2, 0, 0, 1);
}
.selection-bar-enter-from,
.selection-bar-leave-to {
  opacity: 0;
  transform: translateY(8px);
}
</style>
