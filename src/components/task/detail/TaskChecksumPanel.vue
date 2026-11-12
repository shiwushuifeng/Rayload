<script setup lang="ts">
/** @fileoverview Background file checksum calculation with GitHub release comparison. */
import { computed, onMounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { NButton, NIcon, NInput, NSpin, NTag } from 'naive-ui'
import { CopyOutline, RefreshOutline, ShieldCheckmarkOutline } from '@vicons/ionicons5'
import { calculateFileDigests, fetchGitHubAssetDigest, type FileDigests, type ReleaseAssetDigest } from '@/api/checksum'
import { resolveExistingTaskFile } from '@/composables/useTaskFileTarget'
import { requestFileRecheck } from '@/composables/useTaskPaths'
import { writeAppClipboardText } from '@shared/utils'
import { getTaskUri } from '@shared/utils'
import { parseChecksum, type ChecksumAlgorithm } from '@shared/utils/checksum'
import { getErrorMessage } from '@shared/utils/errorMessage'
import { logger } from '@shared/logger'
import { useAppMessage } from '@/composables/useAppMessage'
import type { Aria2Task } from '@shared/types'

const props = defineProps<{ task: Aria2Task }>()
const { t } = useI18n()
const message = useAppMessage()

const targetPath = ref('')
const digests = ref<FileDigests | null>(null)
const computing = ref(false)
const computeError = ref('')
const expected = ref('')
const githubDigest = ref<ReleaseAssetDigest | null>(null)
const githubState = ref<'idle' | 'loading' | 'found' | 'none' | 'error'>('idle')
let calculationToken = 0
let githubToken = 0

const taskSource = computed(() => getTaskUri(props.task))
const fileName = computed(() => targetPath.value.split(/[\\/]/).pop() || targetPath.value)

const digestRows = computed(() => {
  const value = digests.value
  if (!value) return []
  return [
    { key: 'md5', label: 'MD5', value: value.md5 },
    { key: 'sha1', label: 'SHA-1', value: value.sha1 },
    { key: 'sha256', label: 'SHA-256', value: value.sha256 },
    { key: 'sha512', label: 'SHA-512', value: value.sha512 },
  ]
})

const parsedExpected = computed(() => (expected.value.trim() ? parseChecksum(expected.value) : null))
const expectedInvalid = computed(() => Boolean(expected.value.trim()) && !parsedExpected.value)

function digestForAlgorithm(algorithm: string): string {
  const value = digests.value
  if (!value) return ''
  switch (algorithm.toLowerCase()) {
    case 'md5':
      return value.md5
    case 'sha-1':
    case 'sha1':
      return value.sha1
    case 'sha-256':
    case 'sha256':
      return value.sha256
    case 'sha-512':
    case 'sha512':
      return value.sha512
    default:
      return ''
  }
}

const expectedMatches = computed(() => {
  const wanted = parsedExpected.value
  if (!wanted) return false
  return digestForAlgorithm(wanted.algorithm) === wanted.digest
})

const githubMatches = computed(() => {
  if (!githubDigest.value || !digests.value) return false
  const digest = digestForAlgorithm(githubDigest.value.algorithm)
  return Boolean(digest) && digest === githubDigest.value.digest
})

function canCompareAlgorithm(algorithm: ChecksumAlgorithm): boolean {
  return ['md5', 'sha-1', 'sha-256', 'sha-512'].includes(algorithm)
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

async function calculate() {
  const token = ++calculationToken
  computing.value = true
  computeError.value = ''
  digests.value = null
  try {
    if (!targetPath.value) {
      targetPath.value = (await resolveExistingTaskFile(props.task)) ?? ''
    }
    if (!targetPath.value) {
      computeError.value = t('task.file-not-exist')
      requestFileRecheck()
      return
    }
    const result = await calculateFileDigests(targetPath.value)
    if (token === calculationToken) digests.value = result
  } catch (error) {
    if (token === calculationToken) {
      computeError.value = getErrorMessage(error)
      logger.warn('TaskChecksum.calculate', getErrorMessage(error))
    }
  } finally {
    if (token === calculationToken) computing.value = false
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

function resetForTask() {
  targetPath.value = ''
  digests.value = null
  computeError.value = ''
  expected.value = ''
  githubDigest.value = null
  githubState.value = 'idle'
}

watch(
  () => props.task.gid,
  () => {
    resetForTask()
    void calculate()
    void loadGitHubDigest()
  },
)

onMounted(() => {
  void calculate()
  void loadGitHubDigest()
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
      <div>
        <div class="checksum-progress-title">{{ t('task.checksum-computing') }}</div>
        <div class="checksum-progress-detail">{{ fileName }}</div>
      </div>
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
          <code class="checksum-value technical-text-wrap">{{ githubDigest.algorithm }}:{{ githubDigest.digest }}</code>
          <NTag :type="githubMatches ? 'success' : 'error'" size="small" :bordered="false">
            {{ githubMatches ? t('task.checksum-match') : t('task.checksum-mismatch') }}
          </NTag>
        </div>
        <div v-else-if="githubState === 'error'" class="checksum-muted checksum-muted--error">
          {{ t('task.checksum-github-unavailable') }}
        </div>
        <div v-else class="checksum-muted">{{ t('task.checksum-github-none') }}</div>
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
            v-if="parsedExpected && canCompareAlgorithm(parsedExpected.algorithm)"
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
  padding: 18px 14px;
  border-radius: var(--radius-sm);
  background: color-mix(in srgb, var(--m3-primary) 8%, var(--m3-surface-container-low));
}
.checksum-progress-title {
  font-weight: 600;
}
.checksum-progress-detail {
  margin-top: 2px;
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-sm);
  overflow-wrap: anywhere;
}
.checksum-error {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 12px;
  color: var(--m3-error);
  border-radius: var(--radius-sm);
  background: color-mix(in srgb, var(--m3-error) 9%, var(--m3-surface-container-low));
}
.checksum-rows {
  display: flex;
  flex-direction: column;
  gap: 1px;
  overflow: hidden;
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-sm);
}
.checksum-row {
  display: grid;
  grid-template-columns: 72px minmax(0, 1fr) 44px;
  align-items: start;
  gap: 8px;
  padding: 10px 8px 10px 12px;
  background: var(--m3-surface-container-low);
}
.checksum-row + .checksum-row {
  border-top: 1px solid var(--m3-outline-variant);
}
.checksum-algorithm {
  padding-top: 2px;
  font-weight: 600;
}
.checksum-value {
  min-width: 0;
  color: var(--m3-on-surface);
  font-family: var(--font-family-mono);
  font-size: var(--font-size-xs);
  line-height: 1.55;
  overflow-wrap: anywhere;
  user-select: text;
}
.checksum-copy {
  min-width: 44px;
  min-height: 44px;
}
.checksum-section,
.checksum-expected {
  display: flex;
  flex-direction: column;
  gap: 8px;
}
.checksum-section-heading {
  display: flex;
  align-items: center;
  gap: 6px;
  font-weight: 600;
}
.checksum-section-heading :deep(.n-button) {
  min-width: 44px;
  min-height: 44px;
}
.github-result,
.checksum-expected-input {
  display: flex;
  align-items: center;
  gap: 8px;
  min-width: 0;
}
.github-result .checksum-value {
  flex: 1;
}
.checksum-muted--error {
  color: var(--m3-error);
}
</style>
