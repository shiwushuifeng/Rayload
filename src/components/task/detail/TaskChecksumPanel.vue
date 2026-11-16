<script setup lang="ts">
/** @fileoverview File checksum calculation, comparison, export and published-digest lookup. */
import { computed, onBeforeUnmount, onMounted, reactive, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { NButton, NIcon, NInput, NSpin, NTag } from 'naive-ui'
import {
  CloudDownloadOutline,
  CopyOutline,
  DocumentTextOutline,
  RefreshOutline,
  ShieldCheckmarkOutline,
  StopCircleOutline,
} from '@vicons/ionicons5'
import {
  calculateFileDigests,
  cancelFileDigests,
  exportChecksumFile,
  fetchGitHubAssetDigest,
  lookupKnownDigest,
  type FileDigests,
  type KnownDigestEntry,
  type ReleaseAssetDigest,
} from '@/api/checksum'
import { resolveExistingTaskFile } from '@/composables/useTaskFileTarget'
import { requestFileRecheck } from '@/composables/useTaskPaths'
import { isPublishedDigestSource } from '@/composables/useChecksumAutoVerify'
import { writeAppClipboardText } from '@shared/utils'
import { getTaskUri } from '@shared/utils'
import { getErrorMessage } from '@shared/utils/errorMessage'
import { logger } from '@shared/logger'
import { useAppMessage } from '@/composables/useAppMessage'
import { useLanStore } from '@/stores/lan'
import { usePreferenceStore } from '@/stores/preference'
import type { Aria2Task, ChecksumAlgorithm } from '@shared/types'

const props = defineProps<{ task: Aria2Task }>()
const { t } = useI18n()
const message = useAppMessage()
const lan = useLanStore()
const preferenceStore = usePreferenceStore()

const ALGORITHM_LABELS: Record<ChecksumAlgorithm, string> = {
  md5: 'MD5',
  sha1: 'SHA-1',
  sha256: 'SHA-256',
  sha512: 'SHA-512',
  blake3: 'BLAKE3',
  sm3: 'SM3',
  crc32: 'CRC32',
}

interface FileChecksumState {
  computing: boolean
  error: string
  result: FileDigests | null
}

const targetPath = ref('')
const digests = ref<FileDigests | null>(null)
const computing = ref(false)
const computeError = ref('')
const expected = ref('')
const githubDigest = ref<ReleaseAssetDigest | null>(null)
const githubState = ref<'idle' | 'loading' | 'found' | 'none' | 'error'>('idle')
const knownEntries = ref<KnownDigestEntry[]>([])
const onlineState = ref<'idle' | 'loading' | 'done' | 'error'>('idle')
const fileStates = reactive<Record<string, FileChecksumState>>({})
let calculationToken = 0
let githubToken = 0
let activeRequestId: string | null = null

const enabledAlgorithms = computed<ChecksumAlgorithm[]>(() => {
  const configured = preferenceStore.config.checksumAlgorithms
  return configured.length > 0 ? configured : (['md5', 'sha1', 'sha256', 'sha512'] as ChecksumAlgorithm[])
})
const taskSource = computed(() => getTaskUri(props.task))
const fileName = computed(() => targetPath.value.split(/[\\/]/).pop() || targetPath.value)
const torrentFiles = computed(() => (props.task.files ?? []).filter((file) => file.selected !== 'false'))
const showFileList = computed(() => !props.task.remote && torrentFiles.value.length > 1)

const digestRows = computed(() => {
  const value = digests.value
  if (!value) return []
  return enabledAlgorithms.value.flatMap((algorithm) =>
    value.digests[algorithm]
      ? [
          {
            key: algorithm,
            label: ALGORITHM_LABELS[algorithm],
            value: value.digests[algorithm],
            cached: value.cached.includes(algorithm),
          },
        ]
      : [],
  )
})

/** Pasted value with any `algorithm:` prefix removed, lowercased. */
const expectedValue = computed(() =>
  expected.value
    .trim()
    .toLowerCase()
    .replace(/^[a-z0-9-]+\s*[=:]\s*/, ''),
)
const expectedInvalid = computed(() => Boolean(expectedValue.value) && !/^[0-9a-f]+$/.test(expectedValue.value))
/**
 * Matches the pasted value against every computed digest. Length alone cannot
 * tell SHA-256, BLAKE3 and SM3 apart, so the comparison is algorithm-agnostic.
 */
const expectedMatches = computed(() => {
  const wanted = expectedValue.value
  if (!wanted || expectedInvalid.value || !digests.value) return false
  return Object.values(digests.value.digests).some((digest) => digest === wanted)
})
const matchedAlgorithm = computed<ChecksumAlgorithm | null>(() => {
  const wanted = expectedValue.value
  if (!wanted || !digests.value) return null
  const entry = Object.entries(digests.value.digests).find(([, digest]) => digest === wanted)
  return (entry?.[0] as ChecksumAlgorithm | undefined) ?? null
})

const githubMatches = computed(() => {
  if (!githubDigest.value || !digests.value) return false
  return digests.value.digests[githubDigest.value.algorithm] === githubDigest.value.digest
})
const catalogEnabled = computed(() => preferenceStore.config.checksumOnlineLookup)

function rowLabel(algorithm: string): string {
  return ALGORITHM_LABELS[algorithm as ChecksumAlgorithm] ?? algorithm.toUpperCase()
}

async function copyDigest(value: string, label: string) {
  try {
    await writeAppClipboardText(value)
    message.success(t('task.checksum-copied', { algorithm: label }))
  } catch (error) {
    logger.warn('TaskChecksum.copy', getErrorMessage(error))
    message.error(t('task.checksum-copy-failed'))
  }
}

/** Remote tasks ask the paired device; the local device never reads that file. */
async function loadRemoteDigests(): Promise<void> {
  const remote = props.task.remote
  if (!remote) return
  targetPath.value = props.task.files[0]?.path || props.task.dir
  const result = await lan.remoteChecksum(remote.deviceId, remote.gid)
  digests.value = {
    path: targetPath.value,
    size: result.size,
    mtimeSecs: 0,
    digests: {
      md5: result.md5,
      sha1: result.sha1,
      sha256: result.sha256,
      sha512: result.sha512,
    },
    cached: [],
    elapsedMs: 0,
  }
}

async function calculate() {
  const token = ++calculationToken
  computing.value = true
  computeError.value = ''
  digests.value = null
  knownEntries.value = []
  onlineState.value = 'idle'
  const requestId = `checksum-${props.task.gid}-${token}`
  activeRequestId = requestId
  try {
    if (props.task.remote) {
      await loadRemoteDigests()
      return
    }
    if (!targetPath.value) {
      targetPath.value = (await resolveExistingTaskFile(props.task)) ?? ''
    }
    if (!targetPath.value) {
      computeError.value = t('task.file-not-exist')
      requestFileRecheck()
      return
    }
    const result = await calculateFileDigests(targetPath.value, enabledAlgorithms.value, requestId)
    if (token === calculationToken) digests.value = result
    if (token === calculationToken && catalogEnabled.value && isPublishedDigestSource(taskSource.value)) {
      void loadOnlineDigests(result)
    }
  } catch (error) {
    if (token === calculationToken) {
      computeError.value = getErrorMessage(error)
      logger.warn('TaskChecksum.calculate', getErrorMessage(error))
    }
  } finally {
    if (token === calculationToken) {
      computing.value = false
      activeRequestId = null
    }
  }
}

async function cancelCalculation() {
  const requestId = activeRequestId
  if (!requestId) return
  try {
    await cancelFileDigests(requestId)
  } catch (error) {
    logger.debug('TaskChecksum.cancel', getErrorMessage(error))
  }
}

async function loadOnlineDigests(result: FileDigests): Promise<void> {
  onlineState.value = 'loading'
  try {
    const entries = await lookupKnownDigest(result.digests.sha1 ?? null, result.digests.sha256 ?? null)
    knownEntries.value = entries
    onlineState.value = 'done'
  } catch (error) {
    logger.debug('TaskChecksum.online', getErrorMessage(error))
    onlineState.value = 'error'
  }
}

async function loadGitHubDigest() {
  const token = ++githubToken
  if (!taskSource.value.toLowerCase().startsWith('https://github.com/')) {
    githubState.value = 'none'
    return
  }
  githubState.value = 'loading'
  githubDigest.value = null
  try {
    const result = await fetchGitHubAssetDigest(taskSource.value)
    if (token !== githubToken) return
    githubDigest.value = result
    githubState.value = result ? 'found' : 'none'
  } catch (error) {
    if (token !== githubToken) return
    logger.debug('TaskChecksum.github', getErrorMessage(error))
    githubState.value = 'error'
  }
}

/** Exports the digest that matched the expected value, otherwise the first one. */
async function exportDigest() {
  const value = digests.value
  const algorithm = matchedAlgorithm.value ?? enabledAlgorithms.value.find((item) => value?.digests[item])
  const digest = algorithm && value ? value.digests[algorithm] : ''
  if (!algorithm || !digest || !value?.path) return
  try {
    const written = await exportChecksumFile(value.path, algorithm, digest)
    message.success(t('task.checksum-export-done', { path: written }))
  } catch (error) {
    logger.warn('TaskChecksum.export', getErrorMessage(error))
    message.error(t('task.checksum-export-failed'))
  }
}

/** Per-file calculation for multi-file torrents. */
async function calculateFile(path: string) {
  const state = (fileStates[path] ??= { computing: false, error: '', result: null })
  if (state.computing) return
  state.computing = true
  state.error = ''
  const requestId = `checksum-file-${path}-${Date.now()}`
  try {
    state.result = await calculateFileDigests(path, enabledAlgorithms.value, requestId)
  } catch (error) {
    state.error = getErrorMessage(error)
    logger.warn('TaskChecksum.file', getErrorMessage(error))
  } finally {
    state.computing = false
  }
}

function fileStateRows(path: string) {
  const result = fileStates[path]?.result
  if (!result) return []
  return enabledAlgorithms.value.flatMap((algorithm) =>
    result.digests[algorithm]
      ? [{ key: algorithm, label: ALGORITHM_LABELS[algorithm], value: result.digests[algorithm] }]
      : [],
  )
}

function resetForTask() {
  targetPath.value = ''
  digests.value = null
  computeError.value = ''
  expected.value = ''
  githubDigest.value = null
  githubState.value = 'idle'
  knownEntries.value = []
  onlineState.value = 'idle'
  for (const key of Object.keys(fileStates)) delete fileStates[key]
}

function autoCalculateForLocalTask() {
  if (!props.task.remote) void calculate()
  void loadGitHubDigest()
}

watch(
  () => props.task.gid,
  () => {
    resetForTask()
    autoCalculateForLocalTask()
  },
)

watch(showFileList, (visible) => {
  if (!visible) return
  for (const key of Object.keys(fileStates)) delete fileStates[key]
})

onMounted(autoCalculateForLocalTask)
onBeforeUnmount(() => {
  calculationToken += 1
  if (activeRequestId) void cancelFileDigests(activeRequestId).catch(() => undefined)
})
</script>

<template>
  <div class="checksum-panel">
    <div class="checksum-file-card">
      <div class="checksum-file-copy">
        <span class="checksum-file-label">{{ t('task.checksum-file') }}</span>
        <span class="checksum-file-name technical-text-wrap">{{ fileName || t('task.checksum-no-file') }}</span>
      </div>
      <NButton size="small" quaternary :loading="computing" @click="calculate">
        <template #icon>
          <NIcon><RefreshOutline /></NIcon>
        </template>
        {{ t('task.checksum-recalculate') }}
      </NButton>
    </div>

    <div v-if="computing" class="checksum-progress" role="status" aria-live="polite">
      <NSpin size="small" />
      <div class="checksum-progress-copy">
        <div class="checksum-progress-title">{{ t('task.checksum-computing') }}</div>
        <div class="checksum-progress-detail">{{ fileName }}</div>
      </div>
      <NButton size="small" secondary @click="cancelCalculation">
        <template #icon>
          <NIcon><StopCircleOutline /></NIcon>
        </template>
        {{ t('task.checksum-cancel') }}
      </NButton>
    </div>
    <div v-else-if="computeError" class="checksum-error" role="alert">
      <span>{{ computeError }}</span>
      <NButton size="small" @click="calculate">{{ t('task.checksum-retry') }}</NButton>
    </div>

    <template v-else-if="digests">
      <div class="checksum-rows">
        <div v-for="row in digestRows" :key="row.key" class="checksum-row">
          <span class="checksum-algorithm">{{ row.label }}</span>
          <code class="checksum-value technical-text-wrap">{{ row.value }}</code>
          <NTag v-if="row.cached" size="tiny" :bordered="false" type="info">{{ t('task.checksum-cached') }}</NTag>
          <NButton
            class="checksum-copy"
            size="tiny"
            quaternary
            :aria-label="t('task.checksum-copy', { algorithm: row.label })"
            @click="copyDigest(row.value, row.label)"
          >
            <template #icon>
              <NIcon><CopyOutline /></NIcon>
            </template>
          </NButton>
        </div>
      </div>

      <div class="checksum-actions">
        <NButton size="small" secondary :disabled="digestRows.length === 0" @click="exportDigest">
          <template #icon>
            <NIcon><DocumentTextOutline /></NIcon>
          </template>
          {{ t('task.checksum-export') }}
        </NButton>
      </div>

      <div class="checksum-section">
        <div class="checksum-section-heading">
          <NIcon :size="16"><ShieldCheckmarkOutline /></NIcon>
          <span>{{ t('task.checksum-github-title') }}</span>
          <NButton
            size="tiny"
            quaternary
            :loading="githubState === 'loading'"
            :aria-label="t('task.checksum-github-refresh')"
            @click="loadGitHubDigest"
          >
            <template #icon>
              <NIcon><RefreshOutline /></NIcon>
            </template>
          </NButton>
        </div>
        <div v-if="githubState === 'loading'" class="checksum-muted">{{ t('task.checksum-github-loading') }}</div>
        <div v-else-if="githubState === 'found' && githubDigest" class="github-result">
          <code class="checksum-value technical-text-wrap">
            {{ rowLabel(githubDigest.algorithm) }}:{{ githubDigest.digest }}
          </code>
          <NTag :type="githubMatches ? 'success' : 'error'" size="small" :bordered="false">
            {{ githubMatches ? t('task.checksum-match') : t('task.checksum-mismatch') }}
          </NTag>
        </div>
        <div v-else-if="githubState === 'error'" class="checksum-muted checksum-muted--error">
          {{ t('task.checksum-github-unavailable') }}
        </div>
        <div v-else class="checksum-muted">{{ t('task.checksum-github-none') }}</div>
      </div>

      <div v-if="catalogEnabled" class="checksum-section">
        <div class="checksum-section-heading">
          <NIcon :size="16"><CloudDownloadOutline /></NIcon>
          <span>{{ t('task.checksum-online-title') }}</span>
        </div>
        <div v-if="onlineState === 'loading'" class="checksum-muted">{{ t('task.checksum-online-loading') }}</div>
        <div v-else-if="onlineState === 'error'" class="checksum-muted checksum-muted--error">
          {{ t('task.checksum-online-unavailable') }}
        </div>
        <div v-else-if="knownEntries.length === 0" class="checksum-muted">{{ t('task.checksum-online-none') }}</div>
        <div v-for="(entry, index) in knownEntries" :key="`${entry.source}-${index}`" class="github-result">
          <NTag size="small" :bordered="false" type="success">{{ entry.source }}</NTag>
          <span class="checksum-muted">
            {{ t('task.checksum-online-found', { name: entry.fileName || '—' }) }}
          </span>
        </div>
      </div>

      <div class="checksum-expected">
        <div class="checksum-section-heading">{{ t('task.checksum-expected') }}</div>
        <div class="checksum-expected-input">
          <NInput
            v-model:value="expected"
            clearable
            :placeholder="t('task.checksum-expected-placeholder')"
            :status="expectedInvalid ? 'error' : undefined"
          />
          <NTag
            v-if="expectedValue && !expectedInvalid"
            :type="expectedMatches ? 'success' : 'error'"
            size="small"
            :bordered="false"
          >
            {{ expectedMatches ? t('task.checksum-match') : t('task.checksum-mismatch') }}
          </NTag>
        </div>
        <div v-if="expectedInvalid" class="checksum-muted checksum-muted--error">
          {{ t('task.checksum-expected-invalid') }}
        </div>
      </div>

      <div v-if="showFileList" class="checksum-section">
        <div class="checksum-section-heading">{{ t('task.checksum-files-title') }}</div>
        <div v-for="file in torrentFiles" :key="file.path" class="checksum-file-row">
          <div class="checksum-file-row-head">
            <span class="checksum-file-row-name technical-text-wrap">
              {{ file.path.split(/[\\/]/).pop() || file.path }}
            </span>
            <NButton
              size="tiny"
              quaternary
              :loading="fileStates[file.path]?.computing"
              @click="calculateFile(file.path)"
            >
              {{ t('task.checksum-files-calculate') }}
            </NButton>
          </div>
          <div v-if="fileStates[file.path]?.error" class="checksum-muted checksum-muted--error">
            {{ fileStates[file.path]?.error }}
          </div>
          <div v-for="row in fileStateRows(file.path)" :key="row.key" class="checksum-row checksum-row--nested">
            <span class="checksum-algorithm">{{ row.label }}</span>
            <code class="checksum-value technical-text-wrap">{{ row.value }}</code>
            <NButton
              class="checksum-copy"
              size="tiny"
              quaternary
              :aria-label="t('task.checksum-copy', { algorithm: row.label })"
              @click="copyDigest(row.value, row.label)"
            >
              <template #icon>
                <NIcon><CopyOutline /></NIcon>
              </template>
            </NButton>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<style scoped>
.checksum-panel {
  display: flex;
  flex-direction: column;
  gap: 16px;
  min-width: 0;
}
.checksum-file-card {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  min-width: 0;
  padding: 12px;
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-sm);
  background: var(--m3-surface-container-low);
}
.checksum-file-card :deep(.n-button),
.checksum-error :deep(.n-button) {
  min-height: 44px;
}
.checksum-file-copy {
  display: flex;
  flex-direction: column;
  gap: 3px;
  min-width: 0;
}
.checksum-file-label,
.checksum-algorithm,
.checksum-muted {
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-sm);
}
.checksum-file-name {
  font-size: var(--font-size-base);
  overflow-wrap: anywhere;
}
.checksum-progress {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-wrap: wrap;
  padding: 18px 14px;
  border-radius: var(--radius-sm);
  background: color-mix(in srgb, var(--m3-primary) 8%, var(--m3-surface-container-low));
}
.checksum-progress-copy {
  min-width: 0;
  flex: 1 1 auto;
}
.checksum-progress-title {
  font-weight: 600;
}
.checksum-progress-detail {
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-xs);
  overflow-wrap: anywhere;
}
.checksum-error {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  min-width: 0;
  padding: 12px;
  color: var(--m3-error);
  border: 1px solid var(--m3-error);
  border-radius: var(--radius-sm);
}
.checksum-rows {
  display: flex;
  flex-direction: column;
  gap: 8px;
  min-width: 0;
}
.checksum-row {
  display: grid;
  grid-template-columns: minmax(64px, auto) minmax(0, 1fr) auto auto;
  gap: 8px;
  align-items: center;
  min-width: 0;
  padding: 8px 10px;
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-sm);
}
.checksum-row--nested {
  grid-template-columns: minmax(56px, auto) minmax(0, 1fr) auto;
  margin-top: 6px;
}
.checksum-value {
  min-width: 0;
  font-size: var(--font-size-xs);
  overflow-wrap: anywhere;
}
.checksum-actions {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}
.checksum-section {
  display: flex;
  flex-direction: column;
  gap: 8px;
  min-width: 0;
}
.checksum-section-heading {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: var(--font-size-sm);
  font-weight: 600;
  color: var(--m3-on-surface-variant);
}
.github-result {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
  min-width: 0;
}
.checksum-expected {
  display: flex;
  flex-direction: column;
  gap: 8px;
}
.checksum-expected-input {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
}
.checksum-muted--error {
  color: var(--m3-error);
}
.checksum-file-row {
  min-width: 0;
  padding: 8px 10px;
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-sm);
}
.checksum-file-row-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
  min-width: 0;
}
.checksum-file-row-name {
  min-width: 0;
  font-size: var(--font-size-sm);
  overflow-wrap: anywhere;
}
@media (max-width: 520px) {
  .checksum-row {
    grid-template-columns: minmax(56px, auto) minmax(0, 1fr) auto;
  }
}
</style>
