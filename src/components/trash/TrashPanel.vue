<script setup lang="ts">
/** @fileoverview Recoverable file trash for Rayload's Android download folders. */
import { computed, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { invoke } from '@tauri-apps/api/core'
import { NButton, NEmpty, NIcon, NModal, NSpin, useDialog } from 'naive-ui'
import {
  ArrowUndoOutline,
  CloseOutline,
  DocumentOutline,
  FolderOutline,
  RefreshOutline,
  TrashOutline,
} from '@vicons/ionicons5'
import { usePreferenceStore } from '@/stores/preference'
import { useAppMessage } from '@/composables/useAppMessage'
import { bytesToSize } from '@shared/utils/format'
import { logger } from '@shared/logger'
import { resolveUserVisibleDownloadDir } from '@shared/utils/userVisibleDirectory'
import type { TrashItem } from '@shared/types'

const props = defineProps<{ show: boolean }>()
const emit = defineEmits<{ close: [] }>()

const { t, locale } = useI18n()
const preferenceStore = usePreferenceStore()
const message = useAppMessage()
const dialog = useDialog()
const items = ref<TrashItem[]>([])
const loading = ref(false)
const failed = ref(false)
const busyPath = ref('')

const dateFormatter = computed(
  () =>
    new Intl.DateTimeFormat(locale.value, {
      dateStyle: 'medium',
      timeStyle: 'short',
    }),
)

function formatDate(value: number | null): string {
  if (!value) return ''
  const date = new Date(value)
  return Number.isNaN(date.getTime()) ? '' : dateFormatter.value.format(date)
}

async function loadItems(): Promise<void> {
  if (loading.value) return
  loading.value = true
  failed.value = false
  try {
    const resolved = await resolveUserVisibleDownloadDir({ configuredDir: preferenceStore.config.dir })
    items.value = await invoke<TrashItem[]>('list_trash_items', { roots: [resolved.path] })
  } catch (error) {
    failed.value = true
    logger.warn('TrashPanel.load', error)
  } finally {
    loading.value = false
  }
}

async function restoreItem(item: TrashItem): Promise<void> {
  if (busyPath.value) return
  busyPath.value = item.path
  try {
    await invoke<string>('restore_trash_item', { path: item.path })
    message.success(t('trash.restore-success', { name: item.name }))
    await loadItems()
  } catch (error) {
    logger.warn('TrashPanel.restore', error)
    message.error(t('trash.restore-failed'))
  } finally {
    busyPath.value = ''
  }
}

function confirmDelete(item: TrashItem): void {
  const dialogInstance = dialog.warning({
    title: t('trash.delete-confirm-title'),
    content: t('trash.delete-confirm', { name: item.name }),
    positiveText: t('trash.delete-permanently'),
    negativeText: t('app.cancel'),
    onPositiveClick: async () => {
      dialogInstance.loading = true
      busyPath.value = item.path
      try {
        await invoke<boolean>('delete_trash_item', { path: item.path })
        message.success(t('trash.delete-success'))
        await loadItems()
      } catch (error) {
        logger.warn('TrashPanel.delete', error)
        message.error(t('trash.delete-failed'))
        return false
      } finally {
        busyPath.value = ''
        dialogInstance.loading = false
      }
    },
  })
}

watch(
  () => props.show,
  (visible) => {
    if (visible) void loadItems()
  },
  { immediate: true },
)
</script>

<template>
  <NModal
    :show="show"
    transform-origin="center"
    :mask-closable="true"
    @update:show="(visible: boolean) => !visible && emit('close')"
  >
    <section class="trash-panel" role="dialog" aria-modal="true" :aria-label="t('trash.title')">
      <header class="trash-header">
        <div class="trash-heading">
          <span class="trash-heading__icon" aria-hidden="true">
            <NIcon :size="24"><TrashOutline /></NIcon>
          </span>
          <div>
            <h2>{{ t('trash.title') }}</h2>
            <p>{{ t('trash.subtitle') }}</p>
          </div>
        </div>
        <div class="trash-header__actions">
          <NButton quaternary circle :aria-label="t('trash.refresh')" :loading="loading" @click="loadItems">
            <template #icon
              ><NIcon><RefreshOutline /></NIcon
            ></template>
          </NButton>
          <NButton quaternary circle :aria-label="t('app.close')" @click="emit('close')">
            <template #icon
              ><NIcon><CloseOutline /></NIcon
            ></template>
          </NButton>
        </div>
      </header>

      <div class="trash-content">
        <div v-if="loading && items.length === 0" class="trash-state">
          <NSpin size="large" />
          <span>{{ t('trash.loading') }}</span>
        </div>
        <div v-else-if="failed" class="trash-state">
          <NEmpty :description="t('trash.load-failed')">
            <template #extra>
              <NButton @click="loadItems">{{ t('trash.retry') }}</NButton>
            </template>
          </NEmpty>
        </div>
        <NEmpty v-else-if="items.length === 0" class="trash-empty" :description="t('trash.empty')" />
        <template v-else>
          <p class="trash-count">{{ t('trash.item-count', { count: items.length }) }}</p>
          <ul class="trash-list">
            <li v-for="item in items" :key="item.path" class="trash-item">
              <span class="trash-item__icon" aria-hidden="true">
                <NIcon :size="24">
                  <FolderOutline v-if="item.isDir" />
                  <DocumentOutline v-else />
                </NIcon>
              </span>
              <div class="trash-item__details">
                <strong :title="item.name">{{ item.name }}</strong>
                <span class="trash-item__path" :title="item.originalPath">{{ item.originalPath }}</span>
                <span class="trash-item__meta">
                  <span>{{ item.isDir ? t('trash.folder') : t('trash.file') }}</span>
                  <span>{{ bytesToSize(item.size) }}</span>
                  <span v-if="formatDate(item.trashedAt)">{{ formatDate(item.trashedAt) }}</span>
                </span>
              </div>
              <div class="trash-item__actions">
                <NButton
                  secondary
                  :loading="busyPath === item.path"
                  :disabled="Boolean(busyPath) && busyPath !== item.path"
                  @click="restoreItem(item)"
                >
                  <template #icon
                    ><NIcon><ArrowUndoOutline /></NIcon
                  ></template>
                  {{ t('trash.restore') }}
                </NButton>
                <NButton
                  quaternary
                  type="error"
                  :disabled="Boolean(busyPath) && busyPath !== item.path"
                  @click="confirmDelete(item)"
                >
                  <template #icon
                    ><NIcon><TrashOutline /></NIcon
                  ></template>
                  {{ t('trash.delete-permanently') }}
                </NButton>
              </div>
            </li>
          </ul>
        </template>
      </div>
    </section>
  </NModal>
</template>

<style scoped>
.trash-panel {
  display: flex;
  width: min(820px, calc(100vw - 28px));
  max-height: min(760px, calc(100dvh - 40px));
  flex-direction: column;
  overflow: hidden;
  color: var(--m3-on-surface);
  background: var(--m3-surface-container-high);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-xl);
  box-shadow: 0 24px 80px var(--m3-shadow);
}
.trash-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 16px;
  padding: 22px 22px 16px;
  border-bottom: 1px solid var(--m3-outline-variant);
}
.trash-heading {
  display: flex;
  min-width: 0;
  align-items: center;
  gap: 14px;
}
.trash-heading__icon {
  display: grid;
  width: 46px;
  height: 46px;
  flex: 0 0 46px;
  place-items: center;
  color: var(--m3-on-primary-container);
  background: var(--m3-primary-container);
  border-radius: 16px;
}
.trash-heading h2 {
  margin: 0;
  font-size: var(--font-size-2xl);
  font-weight: 600;
}
.trash-heading p {
  margin: 5px 0 0;
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-sm);
  line-height: 1.5;
}
.trash-header__actions,
.trash-item__actions {
  display: flex;
  align-items: center;
  gap: 8px;
}
.trash-content {
  min-height: 240px;
  overflow-y: auto;
  overscroll-behavior: contain;
  padding: 16px 22px 22px;
}
.trash-state,
.trash-empty {
  padding: 54px 16px;
}
.trash-state {
  display: grid;
  place-items: center;
  gap: 14px;
  color: var(--m3-on-surface-variant);
}
.trash-count {
  margin: 0 0 10px;
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-sm);
}
.trash-list {
  display: grid;
  gap: 10px;
  margin: 0;
  padding: 0;
  list-style: none;
}
.trash-item {
  display: grid;
  grid-template-columns: 42px minmax(0, 1fr) auto;
  align-items: center;
  gap: 14px;
  min-width: 0;
  padding: 14px;
  background: var(--m3-surface-container);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-lg);
}
.trash-item__icon {
  display: grid;
  width: 42px;
  height: 42px;
  place-items: center;
  color: var(--m3-primary);
  background: color-mix(in srgb, var(--m3-primary) 12%, transparent);
  border-radius: 14px;
}
.trash-item__details {
  display: grid;
  min-width: 0;
  gap: 4px;
}
.trash-item__details strong,
.trash-item__path {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.trash-item__details strong {
  font-size: var(--font-size-base);
  font-weight: 600;
}
.trash-item__path,
.trash-item__meta {
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-xs);
}
.trash-item__meta {
  display: flex;
  flex-wrap: wrap;
  gap: 4px 12px;
}
.trash-item__actions :deep(.n-button) {
  min-height: 44px;
}
@media (max-width: 680px) {
  .trash-panel {
    width: calc(100vw - 20px);
    max-height: calc(100dvh - 24px);
  }
  .trash-header,
  .trash-content {
    padding-inline: 16px;
  }
  .trash-item {
    grid-template-columns: 42px minmax(0, 1fr);
  }
  .trash-item__actions {
    grid-column: 1 / -1;
    justify-content: flex-end;
  }
}
@media (max-width: 420px) {
  .trash-heading p {
    display: none;
  }
  .trash-item__actions {
    align-items: stretch;
    flex-direction: column;
  }
  .trash-item__actions :deep(.n-button) {
    width: 100%;
  }
}
</style>
