<script setup lang="ts">
/** @fileoverview Local plugin list, permission review and import. */
import { computed, onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { invoke } from '@tauri-apps/api/core'
import { open as openDialog } from '@tauri-apps/plugin-dialog'
import { readTextFile } from '@tauri-apps/plugin-fs'
import { NButton, NCard, NCheckbox, NIcon, NModal, NTag } from 'naive-ui'
import {
  AddCircleOutline,
  CloudUploadOutline,
  ExtensionPuzzleOutline,
  PlayOutline,
  TrashOutline,
  WarningOutline,
} from '@vicons/ionicons5'
import { useAppMessage } from '@/composables/useAppMessage'
import { useTaskStore } from '@/stores/task'
import { getErrorMessage } from '@shared/utils/errorMessage'
import { bytesToSize } from '@shared/utils'
import { logger } from '@shared/logger'

type PluginKind = 'rule' | 'script'
type PermissionKey = 'network' | 'cookies' | 'modify_headers' | 'read_clipboard'

interface PluginEntry {
  file: string
  name: string
  version: string
  description: string
  kind: PluginKind
  runtime: string
  permissions: PermissionKey[]
  grantedPermissions: PermissionKey[]
  size: number
}

interface PendingImport {
  filename: string
  content: string
  name: string
  kind: PluginKind
  permissions: PermissionKey[]
  domains: string[]
  script: boolean
}

const PERMISSIONS: readonly PermissionKey[] = ['network', 'cookies', 'modify_headers', 'read_clipboard']

const { t } = useI18n()
const message = useAppMessage()
const taskStore = useTaskStore()
const plugins = ref<PluginEntry[]>([])
const loading = ref(false)
const importing = ref(false)
const reviewShow = ref(false)
const riskAcknowledged = ref(false)
const pending = ref<PendingImport | null>(null)
const testing = ref('')
const testResult = ref<{ file: string; text: string } | null>(null)

const highRisk = computed(() => {
  const item = pending.value
  if (!item) return false
  return item.script || item.permissions.includes('cookies') || item.permissions.includes('read_clipboard')
})
const canConfirm = computed(() => pending.value !== null && (!highRisk.value || riskAcknowledged.value))

function severity(permission: PermissionKey): 'success' | 'warning' | 'error' {
  if (permission === 'cookies') return 'error'
  if (permission === 'network') return 'success'
  return 'warning'
}

function safeName(value: string): string {
  return value
    .trim()
    .replace(/[^\p{L}\p{N}._-]+/gu, '-')
    .replace(/^-+|-+$/g, '')
}

function basename(path: string): string {
  return path.split(/[\\/]/).pop() ?? path
}

function parsePermissions(value: unknown): PermissionKey[] {
  if (!Array.isArray(value)) return []
  return PERMISSIONS.filter((permission) => value.includes(permission))
}

async function load(): Promise<void> {
  loading.value = true
  try {
    plugins.value = await invoke<PluginEntry[]>('plugin_list')
  } catch (error) {
    logger.debug('Plugins.load', error)
    message.error(getErrorMessage(error))
  } finally {
    loading.value = false
  }
}

async function requestImport(): Promise<void> {
  if (importing.value) return
  try {
    const selected = await openDialog({
      title: t('plugins.import'),
      multiple: false,
      filters: [
        { name: 'JSON', extensions: ['json'] },
        { name: 'Shell', extensions: ['sh'] },
      ],
    })
    if (typeof selected !== 'string') return
    const content = await readTextFile(selected)
    reviewImport(content, basename(selected))
  } catch (error) {
    logger.warn('Plugins.read', error)
    message.error(t('plugins.import-failed'))
  }
}

/** Parses a manifest or script into the shared permission-review dialog. */
function reviewImport(content: string, file: string): void {
  try {
    if (file.toLowerCase().endsWith('.sh')) {
      const stem = safeName(file.replace(/\.sh$/iu, '')) || 'plugin'
      pending.value = {
        filename: `${stem}.sh`,
        content,
        name: stem,
        kind: 'script',
        permissions: [],
        domains: [],
        script: true,
      }
    } else {
      const manifest = JSON.parse(content) as Record<string, unknown>
      const kind: PluginKind = manifest.type === 'script' || manifest.runtime === 'script' ? 'script' : 'rule'
      const stem = safeName(String(manifest.name ?? 'plugin')) || 'plugin'
      pending.value = {
        filename: `${stem}.json`,
        content,
        name: String(manifest.name ?? stem),
        kind,
        permissions: parsePermissions(manifest.permissions),
        domains: Array.isArray(manifest.allowedDomains)
          ? manifest.allowedDomains.filter((domain): domain is string => typeof domain === 'string')
          : [],
        script: false,
      }
    }
    riskAcknowledged.value = false
    reviewShow.value = true
  } catch (error) {
    logger.warn('Plugins.review', error)
    message.error(t('plugins.import-failed'))
  }
}

async function confirmImport(): Promise<void> {
  const item = pending.value
  if (importing.value || !item || !canConfirm.value) return
  importing.value = true
  try {
    if (item.script) {
      await invoke('plugin_save_script', { filename: item.filename, content: item.content })
    } else {
      await invoke('plugin_save', {
        filename: item.filename,
        content: item.content,
        grantedPermissions: item.permissions,
      })
    }
    message.success(t('plugins.imported', { name: item.name }))
    reviewShow.value = false
    pending.value = null
    await load()
  } catch (error) {
    logger.warn('Plugins.import', error)
    message.error(t('plugins.import-failed'))
  } finally {
    importing.value = false
  }
}

function closeReview(): void {
  if (importing.value) return
  reviewShow.value = false
  pending.value = null
}

async function remove(plugin: PluginEntry): Promise<void> {
  try {
    await invoke('plugin_remove', { file: plugin.file })
    await load()
  } catch (error) {
    message.error(getErrorMessage(error))
  }
}

async function installSample(): Promise<void> {
  try {
    const content = await invoke<string>('plugin_sample_manifest')
    reviewImport(content, 'rayload-download-test.json')
  } catch (error) {
    message.error(getErrorMessage(error))
  }
}

async function testPlugin(plugin: PluginEntry): Promise<void> {
  if (testing.value) return
  testing.value = plugin.file
  testResult.value = null
  try {
    const result = await invoke<Record<string, unknown>>('plugin_test', { file: plugin.file })
    testResult.value = { file: plugin.file, text: JSON.stringify(result) }
    const downloadUrl = typeof result.downloadUrl === 'string' ? result.downloadUrl : ''
    if (downloadUrl) {
      const filename = typeof result.filename === 'string' ? result.filename : ''
      await taskStore.addUri({ uris: [downloadUrl], outs: filename ? [filename] : [], options: {} })
      message.success(t('plugins.test-download-started', { name: plugin.name }))
    } else {
      message.success(t('plugins.test-passed', { name: plugin.name }))
    }
  } catch (error) {
    message.error(t('plugins.test-failed', { error: getErrorMessage(error) }))
  } finally {
    testing.value = ''
  }
}

onMounted(load)
</script>

<template>
  <div class="plugins-view">
    <header class="plugins-header">
      <h2>{{ t('plugins.title') }}</h2>
      <p>{{ t('plugins.description') }}</p>
    </header>

    <div class="plugins-toolbar">
      <NButton type="primary" :loading="importing" @click="requestImport">
        <template #icon
          ><NIcon><CloudUploadOutline /></NIcon
        ></template>
        {{ t('plugins.import') }}
      </NButton>
      <NButton secondary @click="installSample">
        <template #icon
          ><NIcon><AddCircleOutline /></NIcon
        ></template>
        {{ t('plugins.sample') }}
      </NButton>
      <span>{{ t('plugins.count', { count: plugins.length }) }}</span>
    </div>

    <p v-if="!loading && plugins.length === 0" class="plugins-empty">
      <NIcon :size="28"><ExtensionPuzzleOutline /></NIcon>
      {{ t('plugins.empty') }}
    </p>

    <NCard v-for="plugin in plugins" :key="plugin.file" class="plugin-card" :bordered="false">
      <span class="plugin-icon"
        ><NIcon :size="20"><ExtensionPuzzleOutline /></NIcon
      ></span>
      <div class="plugin-copy">
        <strong>{{ plugin.name }}</strong>
        <span>{{ plugin.description || t('plugins.no-description') }}</span>
        <code>{{ plugin.file }} · {{ bytesToSize(plugin.size) }}</code>
      </div>
      <NTag size="small" :bordered="false" :type="plugin.kind === 'script' ? 'warning' : 'info'">
        {{ t(`plugins.type-${plugin.kind}`) }}
      </NTag>
      <NTag v-if="plugin.version" size="small" :bordered="false">{{ plugin.version }}</NTag>
      <NButton
        size="small"
        secondary
        :loading="testing === plugin.file"
        :aria-label="t('plugins.test')"
        @click="testPlugin(plugin)"
      >
        <template #icon
          ><NIcon><PlayOutline /></NIcon
        ></template>
        {{ t('plugins.test') }}
      </NButton>
      <NButton size="small" quaternary :aria-label="t('plugins.remove')" @click="remove(plugin)">
        <template #icon
          ><NIcon><TrashOutline /></NIcon
        ></template>
      </NButton>
      <code v-if="testResult?.file === plugin.file" class="plugin-test-result">{{ testResult.text }}</code>
    </NCard>

    <NModal
      :show="reviewShow"
      :mask-closable="false"
      :z-index="1600"
      @update:show="(value: boolean) => (value ? undefined : closeReview())"
    >
      <section
        v-if="pending"
        class="plugin-review"
        :class="{ 'plugin-review--script': pending.kind === 'script' }"
        role="alertdialog"
        aria-modal="true"
        :aria-label="t('plugins.review-title')"
      >
        <header class="plugin-review__head">
          <NIcon :size="26" class="plugin-review__icon"><WarningOutline /></NIcon>
          <div>
            <h3>{{ t('plugins.review-title') }}</h3>
            <span>{{ pending.name }}</span>
          </div>
          <NTag size="small" :bordered="false" :type="pending.kind === 'script' ? 'error' : 'info'">
            {{ t(`plugins.type-${pending.kind}`) }}
          </NTag>
        </header>

        <p class="plugin-review__warning">{{ t(`plugins.warning-${pending.kind}`) }}</p>

        <div v-if="pending.domains.length" class="plugin-review__domains">
          <strong>{{ t('plugins.allowed-domains') }}</strong>
          <code>{{ pending.domains.join(' · ') }}</code>
        </div>

        <div v-if="pending.permissions.length" class="plugin-permissions">
          <article
            v-for="permission in pending.permissions"
            :key="permission"
            class="plugin-permission"
            :class="`plugin-permission--${severity(permission)}`"
          >
            <strong>{{ t(`plugins.permission-${permission}-title`) }}</strong>
            <span>{{ t(`plugins.permission-${permission}-body`, { domains: pending.domains.join(', ') }) }}</span>
          </article>
        </div>
        <p v-else class="plugin-review__no-permissions">{{ t('plugins.no-permissions') }}</p>

        <label v-if="highRisk" class="plugin-review__ack">
          <NCheckbox v-model:checked="riskAcknowledged" />
          <span>{{ t('plugins.warning-confirm') }}</span>
        </label>

        <div class="plugin-review__actions">
          <NButton secondary :disabled="importing" @click="closeReview">{{ t('app.cancel') }}</NButton>
          <NButton type="error" :disabled="!canConfirm" :loading="importing" @click="confirmImport">
            {{ t('plugins.review-confirm') }}
          </NButton>
        </div>
      </section>
    </NModal>
  </div>
</template>

<style scoped>
.plugins-view {
  box-sizing: border-box;
  display: grid;
  align-content: start;
  gap: 14px;
  width: 100%;
  height: 100%;
  max-width: 860px;
  padding: 18px var(--content-gutter) 40px;
  overflow-y: auto;
  overscroll-behavior: contain;
  -webkit-overflow-scrolling: touch;
}
.plugins-header h2 {
  margin: 0 0 4px;
  font-size: 20px;
}
.plugins-header p,
.plugins-toolbar span,
.plugins-empty {
  margin: 0;
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-sm);
}
.plugins-toolbar {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
  align-items: center;
  justify-content: space-between;
}
.plugins-empty {
  display: grid;
  gap: 8px;
  place-items: center;
  padding: 40px 16px;
  border: 1px dashed var(--m3-outline-variant);
  border-radius: var(--radius-lg);
}
.plugin-card {
  background: var(--m3-surface-container);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-lg);
}
.plugin-card :deep(.n-card__content) {
  display: grid;
  grid-template-columns: 40px minmax(0, 1fr) auto auto auto auto;
  gap: 10px;
  align-items: center;
}
@media (max-width: 560px) {
  .plugin-card :deep(.n-card__content) {
    grid-template-columns: 40px minmax(0, 1fr) auto;
  }
  .plugin-card :deep(.n-button) {
    grid-column: span 1;
  }
}
.plugin-test-result {
  grid-column: 1 / -1;
  padding: 8px 10px;
  overflow-wrap: anywhere;
  color: var(--m3-on-surface-variant);
  background: var(--m3-surface-container-low);
  border-radius: var(--radius-sm);
  font-size: 12px;
}
.plugin-icon {
  display: grid;
  width: 40px;
  height: 40px;
  place-items: center;
  color: var(--m3-on-surface-variant);
  background: var(--m3-surface-container-high);
  border-radius: 12px;
}
.plugin-copy {
  display: grid;
  gap: 2px;
  min-width: 0;
}
.plugin-copy span,
.plugin-copy code {
  overflow: hidden;
  color: var(--m3-on-surface-variant);
  font-size: 12px;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.plugin-review {
  display: grid;
  gap: 14px;
  width: min(560px, calc(100vw - 28px));
  max-height: calc(100dvh - 40px);
  padding: 20px;
  overflow-y: auto;
  color: var(--m3-on-surface);
  background: var(--m3-surface-container-high);
  border: 2px solid var(--m3-warning);
  border-radius: var(--radius-xl);
  box-shadow: 0 18px 60px var(--m3-shadow);
}
.plugin-review--script {
  border-color: var(--m3-error);
}
.plugin-review__head {
  display: flex;
  gap: 10px;
  align-items: center;
}
.plugin-review__head > div {
  display: grid;
  flex: 1;
  gap: 2px;
  min-width: 0;
}
.plugin-review__head h3 {
  margin: 0;
  font-size: var(--font-size-lg);
}
.plugin-review__head span {
  overflow: hidden;
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-sm);
  text-overflow: ellipsis;
  white-space: nowrap;
}
.plugin-review__icon {
  color: var(--m3-warning);
}
.plugin-review--script .plugin-review__icon {
  color: var(--m3-error);
}
.plugin-review__warning {
  margin: 0;
  padding: 10px 12px;
  color: var(--m3-on-warning-container);
  background: var(--m3-warning-container);
  border-radius: var(--radius-md);
  font-size: var(--font-size-sm);
  line-height: 1.6;
}
.plugin-review--script .plugin-review__warning {
  color: var(--m3-on-error-container);
  background: var(--m3-error-container);
}
.plugin-review__domains {
  display: grid;
  gap: 4px;
  font-size: var(--font-size-sm);
}
.plugin-review__domains code {
  overflow-wrap: anywhere;
  color: var(--m3-on-surface-variant);
}
.plugin-permissions {
  display: grid;
  gap: 8px;
}
.plugin-permission {
  display: grid;
  gap: 3px;
  padding: 10px 12px;
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-md);
  font-size: var(--font-size-sm);
}
.plugin-permission span {
  color: var(--m3-on-surface-variant);
  line-height: 1.55;
}
.plugin-permission--success {
  border-color: var(--m3-success);
  background: color-mix(in srgb, var(--m3-success-container) 35%, transparent);
}
.plugin-permission--warning {
  border-color: var(--m3-warning);
  background: color-mix(in srgb, var(--m3-warning-container) 35%, transparent);
}
.plugin-permission--error {
  border-color: var(--m3-error);
  background: color-mix(in srgb, var(--m3-error-container) 55%, transparent);
}
.plugin-permission--error strong {
  color: var(--m3-error);
}
.plugin-review__no-permissions {
  margin: 0;
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-sm);
}
.plugin-review__ack {
  display: flex;
  gap: 10px;
  align-items: center;
  padding: 10px 12px;
  border: 1px solid var(--m3-error);
  border-radius: var(--radius-md);
  color: var(--m3-error);
  font-size: var(--font-size-sm);
  font-weight: 600;
}
.plugin-review__actions {
  display: flex;
  gap: 10px;
  justify-content: flex-end;
}
</style>
