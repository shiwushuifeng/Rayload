<script setup lang="ts">
/** @fileoverview Settings block for checking GitHub releases on Android. */
import { computed, onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { getVersion } from '@tauri-apps/api/app'
import { openUrl } from '@tauri-apps/plugin-opener'
import { NButton, NDivider, NFormItem, NIcon, NSwitch, NTag } from 'naive-ui'
import { CloudDownloadOutline, OpenOutline } from '@vicons/ionicons5'
import { usePreferenceStore } from '@/stores/preference'
import { useBuildInfoStore } from '@/stores/build'
import { useAppMessage } from '@/composables/useAppMessage'
import { checkGithubUpdate, type GithubUpdateInfo } from '@/api/githubUpdate'
import { getErrorMessage } from '@shared/utils/errorMessage'
import { friendlyErrorKind } from '@shared/utils/friendlyError'
import { resolveAppProxyUrl } from '@shared/utils/proxy'
import { PROXY_SCOPES } from '@shared/constants'
import { logger } from '@shared/logger'
import FriendlyErrorNotice from '@/components/common/FriendlyErrorNotice.vue'

const props = defineProps<{ stableOnly: boolean }>()
const emit = defineEmits<{ 'update:stableOnly': [value: boolean] }>()

const { t } = useI18n()
const preferenceStore = usePreferenceStore()
const buildInfo = useBuildInfoStore()
const message = useAppMessage()
const checking = ref(false)
const currentVersion = ref('')
const result = ref<GithubUpdateInfo | null>(null)
const errorRaw = ref('')

const errorSummary = computed(() =>
  friendlyErrorKind(errorRaw.value) === 'network'
    ? t('preferences.update-network-error')
    : t('preferences.update-check-failed'),
)

async function check(): Promise<void> {
  if (buildInfo.info.fdroid) {
    message.info(t('preferences.fdroid-update-hint'))
    return
  }
  if (checking.value) return
  checking.value = true
  result.value = null
  errorRaw.value = ''
  try {
    currentVersion.value = await getVersion()
    const proxy = resolveAppProxyUrl(preferenceStore.config.proxy, PROXY_SCOPES.UPDATE_APP)
    result.value = await checkGithubUpdate(currentVersion.value, props.stableOnly, proxy)
    await preferenceStore.updateAndSave({ lastCheckUpdateTime: Date.now() })
  } catch (error) {
    errorRaw.value = getErrorMessage(error)
    logger.warn('AndroidUpdateCheck', errorRaw.value)
  } finally {
    checking.value = false
  }
}

onMounted(() => void buildInfo.load())

async function openRelease(): Promise<void> {
  if (!result.value?.releaseUrl) return
  try {
    await openUrl(result.value.releaseUrl)
  } catch (error) {
    message.error(getErrorMessage(error))
  }
}
</script>

<template>
  <NDivider title-placement="left">{{ t('preferences.auto-update') }}</NDivider>
  <template v-if="buildInfo.info.fdroid">
    <NFormItem :label="t('preferences.check-update')">
      <span class="android-update__current">{{ t('preferences.fdroid-update-hint') }}</span>
    </NFormItem>
  </template>
  <template v-else>
    <NFormItem>
      <template #label>
        <span class="android-update__label">{{ t('preferences.update-stable-only') }}</span>
      </template>
      <NSwitch :value="stableOnly" @update:value="(value: boolean) => emit('update:stableOnly', value)" />
    </NFormItem>
    <NFormItem :label="t('preferences.check-update')">
      <div class="android-update__actions">
        <NButton :loading="checking" @click="check">
          <template #icon>
            <NIcon :size="14"><CloudDownloadOutline /></NIcon>
          </template>
          {{ checking ? t('preferences.checking-update') : t('preferences.check-update-now') }}
        </NButton>
        <span v-if="currentVersion" class="android-update__current">
          {{ t('preferences.current-version', { version: currentVersion }) }}
        </span>
      </div>
    </NFormItem>
    <NFormItem :show-label="false">
      <div class="android-update__result">
        <div v-if="result" class="android-update__card">
          <div class="android-update__card-heading">
            <strong v-if="result.updateAvailable">
              {{ t('preferences.update-available', { version: result.version }) }}
            </strong>
            <strong v-else>{{ t('preferences.update-up-to-date') }}</strong>
            <NTag v-if="result.prerelease" size="small" type="warning">Beta / Pre-release</NTag>
          </div>
          <p v-if="result.updateAvailable">{{ t('preferences.update-release-hint', { version: result.version }) }}</p>
          <p v-else>{{ t('preferences.update-up-to-date-hint', { version: currentVersion }) }}</p>
          <NButton v-if="result.updateAvailable" size="small" type="primary" @click="openRelease">
            <template #icon>
              <NIcon :size="14"><OpenOutline /></NIcon>
            </template>
            {{ t('preferences.update-open-release') }}
          </NButton>
        </div>
        <FriendlyErrorNotice
          v-if="errorRaw"
          :raw="errorRaw"
          source="github-update-check"
          :extra="{ stableOnly: String(stableOnly) }"
          :summary="errorSummary"
        />
      </div>
    </NFormItem>
  </template>
</template>

<style scoped>
.android-update__actions,
.android-update__card,
.android-update__result {
  display: grid;
  gap: 8px;
  justify-items: start;
}
.android-update__actions {
  grid-auto-flow: column;
  align-items: center;
}
.android-update__current {
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-sm);
}
.android-update__result,
.android-update__card {
  width: 100%;
}
.android-update__card {
  padding: 12px 14px;
  background: var(--m3-surface-container);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-lg);
}
.android-update__card p {
  margin: 0;
  color: var(--m3-on-surface-variant);
  line-height: 1.55;
}
.android-update__card-heading {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  align-items: center;
}
</style>
