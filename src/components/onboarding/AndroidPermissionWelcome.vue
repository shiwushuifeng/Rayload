<script setup lang="ts">
/** @fileoverview Lightweight Android permission onboarding and battery prompt. */
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { NButton, NIcon, NModal, NSelect } from 'naive-ui'
import {
  BatteryHalfOutline,
  CheckmarkCircleOutline,
  FolderOpenOutline,
  LanguageOutline,
  NotificationsOutline,
} from '@vicons/ionicons5'
import { useLocale } from '@/composables/useLocale'
import { usePlatform } from '@/composables/usePlatform'
import { usePreferenceStore } from '@/stores/preference'
import {
  readAndroidPermissionStatuses,
  requestAndroidBatteryOptimizationExemption,
  requestAndroidNotificationPermission,
  requestAndroidStoragePermission,
  type AndroidPermissionStatus,
} from '@/composables/useAndroidNative'
import { isSupportedLocale, LOCALE_CATALOG, type SupportedLocale } from '@shared/localeCatalog'
import { logger } from '@shared/logger'
import { resolveSystemLocale } from '@shared/utils/locale'
import { locale as getSystemLocale } from '@tauri-apps/plugin-os'

type PermissionKey = 'storage' | 'notification' | 'battery'

interface PermissionStep {
  key: PermissionKey
  title: string
  icon: typeof FolderOpenOutline
}

const STORAGE_KEY = 'rayload-onboarding-v1'
const { t, locale } = useI18n()
const { isAndroid } = usePlatform()
const { setLocale } = useLocale()
const preferenceStore = usePreferenceStore()
const visible = ref(false)
const firstRun = ref(true)
const languageStepActive = ref(false)
const dismissedThisSession = ref(false)
const currentIndex = ref(0)
const selectedLocale = ref(preferenceStore.locale || 'auto')
const statuses = ref<Record<PermissionKey, AndroidPermissionStatus>>({
  storage: 'unavailable',
  notification: 'unavailable',
  battery: 'unavailable',
})

const localeOptions = computed(() => [
  { label: t('preferences.follow-system'), value: 'auto' },
  ...LOCALE_CATALOG.map(({ code, label }) => ({ label, value: code })),
])

const allSteps = computed<PermissionStep[]>(() => [
  {
    key: 'storage',
    title: t('onboarding.storage-title'),
    icon: FolderOpenOutline,
  },
  {
    key: 'notification',
    title: t('onboarding.notification-title'),
    icon: NotificationsOutline,
  },
  {
    key: 'battery',
    title: t('onboarding.battery-title'),
    icon: BatteryHalfOutline,
  },
])

const steps = computed(() =>
  allSteps.value.filter((step) => {
    if (statuses.value[step.key] === 'unavailable') return false
    if (step.key === 'battery') return statuses.value.battery === 'denied'
    return firstRun.value && statuses.value[step.key] === 'denied'
  }),
)
const currentStep = computed(() => steps.value[currentIndex.value] ?? steps.value[0] ?? null)
const currentStatus = computed(() => (currentStep.value ? statuses.value[currentStep.value.key] : 'unavailable'))
const totalOnboardingSteps = computed(() => steps.value.length + (firstRun.value ? 1 : 0))
const progress = computed(() => {
  if (totalOnboardingSteps.value === 0) return 100
  const completedPermissions = languageStepActive.value ? 0 : currentIndex.value + 1
  const completedLanguage = firstRun.value && !languageStepActive.value ? 1 : 0
  return Math.round(((completedPermissions + completedLanguage) / totalOnboardingSteps.value) * 100)
})

function readOnboardingComplete(): boolean {
  try {
    return window.localStorage.getItem(STORAGE_KEY) === 'complete'
  } catch {
    return false
  }
}

function markOnboardingComplete(): void {
  try {
    window.localStorage.setItem(STORAGE_KEY, 'complete')
  } catch {
    // Storage can be unavailable in hardened WebViews; permission status still works.
  }
}

function refreshStatuses(): void {
  if (!isAndroid.value) return
  statuses.value = readAndroidPermissionStatuses()
  if (currentIndex.value >= steps.value.length) currentIndex.value = Math.max(0, steps.value.length - 1)
  visible.value = (languageStepActive.value || steps.value.length > 0) && !dismissedThisSession.value
}

function requestCurrentPermission(): void {
  switch (currentStep.value?.key) {
    case 'storage':
      requestAndroidStoragePermission()
      break
    case 'notification':
      requestAndroidNotificationPermission()
      break
    case 'battery':
      requestAndroidBatteryOptimizationExemption()
      break
  }
}

async function confirmLanguage(): Promise<void> {
  let localeToApply: SupportedLocale
  if (selectedLocale.value === 'auto') {
    try {
      localeToApply = resolveSystemLocale((await getSystemLocale()) || 'en-US')
    } catch (error) {
      logger.debug('AndroidPermissionWelcome.systemLocale', error)
      localeToApply = isSupportedLocale(locale.value) ? locale.value : 'en-US'
    }
  } else {
    localeToApply = isSupportedLocale(selectedLocale.value) ? selectedLocale.value : 'en-US'
  }
  try {
    await setLocale(localeToApply)
  } catch (error) {
    logger.error('AndroidPermissionWelcome.locale', error)
  }
  await preferenceStore.updateAndSave({ locale: selectedLocale.value })
  languageStepActive.value = false
  currentIndex.value = 0
  if (steps.value.length === 0) finish()
  else visible.value = true
}

async function advance(): Promise<void> {
  if (languageStepActive.value) {
    await confirmLanguage()
    return
  }
  if (currentStatus.value !== 'granted') {
    requestCurrentPermission()
    return
  }
  if (currentIndex.value + 1 < steps.value.length) {
    currentIndex.value += 1
    return
  }
  finish()
}

function finish(): void {
  firstRun.value = false
  dismissedThisSession.value = true
  markOnboardingComplete()
  visible.value = false
}

function dismissForNow(): void {
  finish()
}

function onVisibilityOrFocus(): void {
  if (visible.value || isAndroid.value) refreshStatuses()
}

onMounted(() => {
  firstRun.value = !readOnboardingComplete()
  languageStepActive.value = firstRun.value
  window.setTimeout(() => {
    refreshStatuses()
    window.addEventListener('focus', onVisibilityOrFocus)
    document.addEventListener('visibilitychange', onVisibilityOrFocus)
    window.addEventListener('rayload-permission-state-changed', onVisibilityOrFocus)
  }, 350)
})

onBeforeUnmount(() => {
  window.removeEventListener('focus', onVisibilityOrFocus)
  document.removeEventListener('visibilitychange', onVisibilityOrFocus)
  window.removeEventListener('rayload-permission-state-changed', onVisibilityOrFocus)
})
</script>

<template>
  <NModal :show="visible" :mask-closable="false" :close-on-esc="false" :block-scroll="true" transform-origin="center">
    <section class="permission-welcome" role="dialog" aria-modal="true" :aria-label="t('onboarding.title')">
      <div class="permission-welcome__accent" />
      <div class="permission-welcome__heading">
        <h2>{{ languageStepActive ? t('preferences.language') : t('onboarding.title') }}</h2>
        <p>{{ languageStepActive ? t('preferences.select-language') : t('onboarding.description') }}</p>
      </div>

      <div v-if="languageStepActive" class="permission-step permission-step--language">
        <div class="permission-step__icon">
          <NIcon :size="26"><LanguageOutline /></NIcon>
        </div>
        <div class="permission-step__copy">
          <NSelect
            v-model:value="selectedLocale"
            :options="localeOptions"
            :aria-label="t('preferences.select-language')"
            filterable
            size="large"
          />
        </div>
      </div>

      <div v-else-if="currentStep" class="permission-step">
        <div class="permission-step__icon">
          <NIcon :size="26"><component :is="currentStep.icon" /></NIcon>
        </div>
        <div class="permission-step__copy">
          <h3>{{ currentStep.title }}</h3>
        </div>
        <NIcon v-if="currentStatus === 'granted'" :size="24" class="permission-step__check">
          <CheckmarkCircleOutline />
        </NIcon>
      </div>

      <div class="permission-progress" aria-hidden="true">
        <span :style="{ width: `${progress}%` }" />
      </div>

      <div class="permission-welcome__actions">
        <NButton v-if="!languageStepActive" quaternary @click="dismissForNow">
          {{ t('onboarding.later') }}
        </NButton>
        <NButton type="primary" @click="advance">
          {{
            languageStepActive
              ? t('onboarding.next')
              : currentStatus === 'granted'
                ? currentIndex + 1 < steps.length
                  ? t('onboarding.next')
                  : t('onboarding.finish')
                : t('onboarding.allow')
          }}
        </NButton>
      </div>
    </section>
  </NModal>
</template>

<style scoped>
.permission-welcome {
  position: relative;
  width: min(440px, calc(100vw - 32px));
  padding: 28px;
  overflow: hidden;
  color: var(--m3-on-surface);
  background: var(--m3-surface-container-high);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-xl);
  box-shadow: 0 18px 60px var(--m3-shadow);
}
.permission-welcome__accent {
  position: absolute;
  inset: 0 0 auto;
  height: 4px;
  background: linear-gradient(90deg, var(--m3-primary), var(--m3-tertiary));
}
.permission-welcome__heading h2 {
  margin: 6px 0 8px;
  font-size: 24px;
  font-weight: 600;
}
.permission-welcome__heading p,
.permission-step p {
  margin: 0;
  color: var(--m3-on-surface-variant);
  line-height: 1.55;
}
.permission-step {
  display: grid;
  grid-template-columns: 48px minmax(0, 1fr) auto;
  gap: 14px;
  align-items: center;
  margin: 24px 0 18px;
  padding: 18px;
  background: var(--m3-surface-container);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-lg);
}
.permission-step__icon {
  display: grid;
  width: 48px;
  height: 48px;
  place-items: center;
  color: var(--m3-on-primary-container);
  background: var(--m3-primary-container);
  border-radius: 16px;
}
.permission-step__copy {
  min-width: 0;
}
.permission-step--language .permission-step__copy {
  width: 100%;
}
.permission-step h3 {
  margin: 0 0 5px;
  font-size: 16px;
}
.permission-step__check {
  color: var(--m3-success);
}
.permission-progress {
  height: 4px;
  overflow: hidden;
  background: var(--m3-surface-container-highest);
  border-radius: var(--radius-full);
}
.permission-progress span {
  display: block;
  height: 100%;
  background: var(--m3-primary);
  border-radius: inherit;
  transition: width 240ms cubic-bezier(0.2, 0, 0, 1);
}
.permission-welcome__actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  margin-top: 20px;
}
</style>
