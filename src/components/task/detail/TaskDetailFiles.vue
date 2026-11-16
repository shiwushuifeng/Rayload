<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { NButton, NDropdown, NIcon, NSelect } from 'naive-ui'
import { EllipsisHorizontal } from '@vicons/ionicons5'
import { bytesToSize } from '@shared/utils'
import type { Aria2File, BtFilePriority } from '@shared/types'
import { buildFileDetailRows, type FileDetailRow } from '@/composables/useTaskDetailRows'
import { changeOption } from '@/api/aria2'
import { useAppMessage } from '@/composables/useAppMessage'
import { logger } from '@shared/logger'
import { getErrorMessage } from '@shared/utils/errorMessage'

const props = defineProps<{
  files: Aria2File[]
  tooltip: string
  onCopy: (value: string, label: string) => void
  gid?: string
  editable?: boolean
  terminal?: boolean
}>()

const { t } = useI18n()
const message = useAppMessage()
const priorityOverrides = ref<Record<number, BtFilePriority>>({})
const pendingPriority = ref<number | null>(null)
const rows = computed(() => buildFileDetailRows(props.files))
const priorityOptions = computed(() =>
  (['off', 'normal', 'high', 'top'] as const).map((value) => ({
    value,
    label: t(`task.file-priority-${value}`),
  })),
)

watch(
  () => props.gid,
  () => {
    priorityOverrides.value = {}
  },
)

async function updatePriority(row: FileDetailRow, priority: BtFilePriority) {
  if (!props.gid || !props.editable || pendingPriority.value !== null) return
  pendingPriority.value = row.idx
  try {
    await changeOption({ gid: props.gid, options: { 'bt-file-priority': `${row.idx}=${priority}` } })
    priorityOverrides.value = { ...priorityOverrides.value, [row.idx]: priority }
    message.success(t('task.options-applied'))
  } catch (error) {
    logger.warn('TaskDetail.files.priority', getErrorMessage(error))
    message.error(t('task.options-apply-failed'))
  } finally {
    pendingPriority.value = null
  }
}

function filePath(row: FileDetailRow): string {
  return props.files.find((file) => Number(file.index) === row.idx)?.path ?? ''
}

function fileMenu(row: FileDetailRow) {
  return [
    { label: t('task.copy-file-name'), key: 'name' },
    { label: t('task.copy-file-path'), key: 'path', disabled: !filePath(row) },
  ]
}

function onFileMenu(row: FileDetailRow, key: string) {
  if (key === 'name') props.onCopy(row.name, t('task.copy-file-name'))
  if (key === 'path') props.onCopy(filePath(row), t('task.copy-file-path'))
}
</script>

<template>
  <!-- shortcut: renders every file card; move to virtual scrolling if huge torrents lag. -->
  <div class="file-cards">
    <div v-for="row in rows" :key="row.idx" class="file-card">
      <div class="file-card-main">
        <button type="button" class="file-card-name" @click="onFileMenu(row, 'name')">{{ row.name }}</button>
        <div class="file-card-meta">{{ bytesToSize(String(row.length)) }} · {{ row.percent }}%</div>
      </div>
      <NSelect
        v-if="!terminal"
        class="file-card-priority"
        :value="priorityOverrides[row.idx] ?? row.priority"
        :options="priorityOptions"
        size="small"
        :disabled="!editable || pendingPriority !== null"
        :loading="pendingPriority === row.idx"
        @update:value="(value: BtFilePriority) => void updatePriority(row, value)"
      />
      <NDropdown
        trigger="click"
        :options="fileMenu(row)"
        @select="(key: string | number) => onFileMenu(row, String(key))"
      >
        <NButton quaternary circle size="small" class="file-card-menu" :aria-label="t('task.file-actions')">
          <template #icon>
            <NIcon><EllipsisHorizontal /></NIcon>
          </template>
        </NButton>
      </NDropdown>
    </div>
  </div>
</template>

<style scoped>
.file-cards {
  display: flex;
  flex-direction: column;
  gap: 8px;
}
.file-card {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 10px;
  background: var(--task-item-bg);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-sm);
}
.file-card-main {
  flex: 1 1 auto;
  min-width: 0;
}
.file-card-name {
  display: block;
  width: 100%;
  padding: 0;
  overflow: hidden;
  color: var(--m3-on-surface);
  font: inherit;
  font-size: var(--font-size-sm);
  text-align: left;
  text-overflow: ellipsis;
  white-space: nowrap;
  background: none;
  border: 0;
  cursor: pointer;
}
.file-card-meta {
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-xs);
  font-variant-numeric: tabular-nums;
}
.file-card-priority {
  flex: 0 0 88px;
  width: 88px;
}
.file-card-menu {
  flex: 0 0 auto;
}
</style>
