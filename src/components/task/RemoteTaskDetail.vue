<script setup lang="ts">
/** @fileoverview Detail drawer for a task running on a paired LAN device. */
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { NButton, NDescriptions, NDescriptionsItem, NDrawer, NDrawerContent, NIcon, NTag } from 'naive-ui'
import { writeAppClipboardText } from '@shared/utils'
import { bytesToSize } from '@shared/utils/format'
import { getErrorMessage } from '@shared/utils/errorMessage'
import { logger } from '@shared/logger'
import { useAppMessage } from '@/composables/useAppMessage'
import { usePlatform } from '@/composables/usePlatform'
import { buildTaskTransferSummary } from '@/composables/useTaskDetailSummary'
import TaskDetailActivity from './detail/TaskDetailActivity.vue'
import TaskDetailFiles from './detail/TaskDetailFiles.vue'
import TaskChecksumPanel from './detail/TaskChecksumPanel.vue'
import SendToDeviceIcon from './SendToDeviceIcon.vue'
import type { Aria2Task } from '@shared/types'

const props = defineProps<{ show: boolean; task: Aria2Task | null }>()
const emit = defineEmits<{ close: []; 'send-to-local': [task: Aria2Task] }>()

const { t } = useI18n()
const message = useAppMessage()
const { isAndroid } = usePlatform()
const drawerWidth = computed(() => (isAndroid.value ? '92vw' : '61.8%'))
const transferSummary = computed(() => buildTaskTransferSummary(props.task))
const offline = computed(() => Boolean(props.task?.remote && !props.task.remote.online))
const complete = computed(
  () => props.task?.status === 'complete' || props.task?.status === 'removed' || Boolean(props.task?.remote?.seeder),
)
const terminal = computed(() => ['complete', 'error', 'removed'].includes(props.task?.status ?? ''))
const statusLabel = computed(() => {
  if (!props.task) return ''
  if (offline.value) return t('task.remote-offline')
  switch (props.task.status) {
    case 'complete':
      return t('task.status-complete')
    case 'error':
      return t('task.status-error')
    case 'paused':
      return t('task.status-paused')
    case 'waiting':
      return t('task.status-waiting')
    case 'removed':
      return t('task.status-removed')
    default:
      return t('task.status-active')
  }
})
const statusTagType = computed(() => {
  if (offline.value || props.task?.status === 'error') return 'error' as const
  if (complete.value) return 'success' as const
  return props.task?.status === 'paused' ? ('default' as const) : ('warning' as const)
})

async function copyValue(value: string, label: string) {
  if (!value) return
  try {
    await writeAppClipboardText(value)
    message.success(t('preferences.copied-to-clipboard', { label }))
  } catch (error) {
    logger.warn('RemoteTaskDetail.copy', getErrorMessage(error))
    message.error(getErrorMessage(error))
  }
}
</script>

<template>
  <NDrawer
    :show="show"
    :width="drawerWidth"
    placement="right"
    @update:show="(value: boolean) => !value && emit('close')"
  >
    <NDrawerContent :title="t('task.remote-details-title')" closable @close="emit('close')">
      <template v-if="task">
        <NDescriptions :column="1" label-placement="left" bordered size="small">
          <NDescriptionsItem :label="t('task.remote-device')">
            {{ task.remote?.deviceName }}
          </NDescriptionsItem>
          <NDescriptionsItem :label="t('task.task-status')">
            <NTag :type="statusTagType" size="small">{{ statusLabel }}</NTag>
          </NDescriptionsItem>
          <NDescriptionsItem :label="t('task.task-name')">{{ task.files[0]?.path || '—' }}</NDescriptionsItem>
          <NDescriptionsItem :label="t('task.remote-file-size')">
            {{ bytesToSize(task.completedLength, 2) }} / {{ bytesToSize(task.totalLength, 2) }}
          </NDescriptionsItem>
          <NDescriptionsItem :label="t('task.task-download-speed')">
            {{ bytesToSize(task.downloadSpeed, 1) }}/s
          </NDescriptionsItem>
          <NDescriptionsItem :label="t('task.remote-file-path')">
            <span class="technical-text-wrap">{{ task.dir || '—' }}</span>
          </NDescriptionsItem>
          <NDescriptionsItem :label="t('task.task-gid')">{{ task.remote?.gid }}</NDescriptionsItem>
        </NDescriptions>

        <div class="section-divider">{{ t('task.task-tab-activity') }}</div>
        <TaskDetailActivity :task="task" :transfer-summary="transferSummary" />

        <div class="section-divider">{{ t('task.task-tab-files') }}</div>
        <TaskDetailFiles
          :files="task.files"
          :editable="false"
          :terminal="terminal"
          :tooltip="t('about.click-to-copy')"
          :on-copy="copyValue"
        />

        <div class="section-divider">{{ t('task.task-tab-checksum') }}</div>
        <TaskChecksumPanel :key="task.gid" :task="task" />
      </template>

      <template #footer>
        <NButton
          type="primary"
          block
          :disabled="!complete || offline || !task"
          @click="task && emit('send-to-local', task)"
        >
          <template #icon>
            <NIcon><SendToDeviceIcon /></NIcon>
          </template>
          {{ t('task.remote-send-local') }}
        </NButton>
      </template>
    </NDrawerContent>
  </NDrawer>
</template>

<style scoped>
.section-divider {
  margin: 20px 0 12px;
  color: var(--m3-primary);
  font-size: var(--font-size-sm);
  font-weight: 600;
  letter-spacing: 0.5px;
}
:deep(.technical-text-wrap) {
  overflow-wrap: anywhere;
}
</style>
