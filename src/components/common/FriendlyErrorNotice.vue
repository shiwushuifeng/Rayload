<script setup lang="ts">
/** @fileoverview Friendly error summary with a copyable technical log. */
import { computed, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { NAlert, NButton } from 'naive-ui'
import { writeAppClipboardText } from '@shared/utils'
import { buildErrorLog, friendlyErrorKind, type ErrorLogContext } from '@shared/utils/friendlyError'
import { useAppMessage } from '@/composables/useAppMessage'
import { logger } from '@shared/logger'

const props = defineProps<{
  raw: string
  gid?: string
  url?: string
  source?: string
  extra?: Record<string, string | number | undefined>
  /** Optional caller-supplied friendly summary that overrides classification. */
  summary?: string
}>()

const { t } = useI18n()
const message = useAppMessage()
const showDetails = ref(false)

const kind = computed(() => friendlyErrorKind(props.raw))
const summary = computed(() => props.summary || t(`task.error-friendly-${kind.value}`))
const context = computed<ErrorLogContext>(() => ({
  gid: props.gid,
  url: props.url,
  source: props.source,
  extra: props.extra,
}))

async function copyLog(): Promise<void> {
  try {
    await writeAppClipboardText(buildErrorLog(props.raw, context.value))
    message.success(t('task.error-log-copied'))
  } catch (error) {
    logger.debug('FriendlyErrorNotice.copy', error)
    message.error(t('task.error-log-copy-failed'))
  }
}
</script>

<template>
  <NAlert class="friendly-error" type="error" :show-icon="true">
    <div class="friendly-error__body">
      <strong class="friendly-error__summary">{{ summary }}</strong>
      <div class="friendly-error__actions">
        <NButton size="small" @click="copyLog">{{ t('task.error-copy-log') }}</NButton>
        <NButton text size="small" @click="showDetails = !showDetails">
          {{ showDetails ? t('task.error-hide-details') : t('task.error-show-details') }}
        </NButton>
      </div>
      <pre v-if="showDetails" class="friendly-error__raw">{{ raw }}</pre>
    </div>
  </NAlert>
</template>

<style scoped>
.friendly-error__body {
  display: grid;
  gap: 10px;
}
.friendly-error__summary {
  line-height: 1.55;
}
.friendly-error__actions {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  align-items: center;
}
.friendly-error__raw {
  max-height: 180px;
  margin: 0;
  padding: 10px;
  overflow: auto;
  color: var(--m3-on-surface-variant);
  background: var(--m3-surface-container);
  border-radius: var(--radius-md);
  font-family: var(--font-family-mono);
  font-size: var(--font-size-xs);
  white-space: pre-wrap;
  overflow-wrap: anywhere;
}
</style>
