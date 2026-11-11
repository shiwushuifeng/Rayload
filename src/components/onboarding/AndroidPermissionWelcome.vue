<script setup lang="ts">
/** @fileoverview Lightweight Android permission onboarding and battery prompt. */
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { NButton, NIcon, NModal } from 'naive-ui'
import {
  BatteryHalfOutline,
  CheckmarkCircleOutline,
  FolderOpenOutline,
  LanguageOutline,
  NotificationsOutline,
} from '@vicons/ionicons5'
import { useLocale } from '@/composables/useLocale'
import { usePlatform } from '@/composables/usePlatform'
import { useTheme } from '@/composables/useTheme'
import { usePreferenceStore } from '@/stores/preference'
import { COLOR_SCHEMES } from '@shared/constants'
import {
  readAndroidPermissionStatuses,
  requestAndroidBatteryOptimizationExemption,
  requestAndroidNotificationPermission,
  requestAndroidStoragePermission,
  setAndroidAppIcon,
  type AndroidPermissionStatus,
} from '@/composables/useAndroidNative'
import { isSupportedLocale, LOCALE_CATALOG, type SupportedLocale } from '@shared/localeCatalog'
import { logger } from '@shared/logger'
import { resolveSystemLocale } from '@shared/utils/locale'
import { locale as getSystemLocale } from '@tauri-apps/plugin-os'
import type { AppConfig } from '@shared/types'
import brandLogoGold from '@/assets/logo.svg'
import brandLogoViolet from '@/assets/logo-violet.svg'
import brandLogoBlue from '@/assets/logo-blue.svg'
import brandLogoTeal from '@/assets/logo-teal.svg'

type PermissionKey = 'storage' | 'notification' | 'battery'

interface PermissionStep {
  key: PermissionKey
  title: string
  icon: typeof FolderOpenOutline
}

const LEGACY_STORAGE_KEY = 'rayload-onboarding-v1'
const GUIDE_STORAGE_KEY = 'has_seen_permission_guide'
const { t, locale } = useI18n()
const { isAndroid } = usePlatform()
const { isDark } = useTheme()
const { setLocale } = useLocale()
const preferenceStore = usePreferenceStore()
const visible = ref(false)
const firstRun = ref(true)
const languageStepActive = ref(false)
const appearanceStepActive = ref(false)
const dismissedThisSession = ref(false)
const currentIndex = ref(0)
const selectedLocale = ref(preferenceStore.locale || 'auto')
const selectedTheme = ref<AppConfig['theme']>(preferenceStore.config.theme)
const selectedColorScheme = ref(preferenceStore.config.colorScheme)
const selectedBrandLogoStyle = ref<AppConfig['brandLogoStyle']>(preferenceStore.config.brandLogoStyle)
const selectedAndroidAppIcon = ref<AppConfig['androidAppIcon']>(preferenceStore.config.androidAppIcon)
const statuses = ref<Record<PermissionKey, AndroidPermissionStatus>>({
  storage: 'unavailable',
  notification: 'unavailable',
  battery: 'unavailable',
})

const localeOptions = computed(() => [
  { label: t('preferences.follow-system'), value: 'auto' },
  ...LOCALE_CATALOG.map(({ code, label }) => ({ label, value: code })),
])

const themeOptions = computed(
  () =>
    [
      { label: t('preferences.theme-auto'), value: 'auto' },
      { label: t('preferences.theme-light'), value: 'light' },
      { label: t('preferences.theme-dark'), value: 'dark' },
    ] satisfies Array<{ label: string; value: AppConfig['theme'] }>,
)

const brandLogoOptions = computed(
  () =>
    [
      { label: t('preferences.brand-logo-auto'), value: 'auto' },
      { label: t('preferences.brand-logo-gold'), value: 'gold' },
      { label: t('preferences.brand-logo-violet'), value: 'violet' },
      { label: t('preferences.brand-logo-blue'), value: 'blue' },
      { label: t('preferences.brand-logo-teal'), value: 'teal' },
    ] satisfies Array<{ label: string; value: AppConfig['brandLogoStyle'] }>,
)

const androidAppIconOptions = computed(
  () =>
    [
      { label: t('preferences.android-app-icon-gold'), value: 'gold' },
      { label: t('preferences.android-app-icon-purple'), value: 'purple' },
      { label: t('preferences.android-app-icon-blue'), value: 'blue' },
      { label: t('preferences.android-app-icon-teal'), value: 'teal' },
    ] satisfies Array<{ label: string; value: AppConfig['androidAppIcon'] }>,
)

const brandLogoPreview = computed(() => {
  switch (selectedBrandLogoStyle.value) {
    case 'gold':
      return brandLogoGold
    case 'violet':
      return brandLogoViolet
    case 'blue':
      return brandLogoBlue
    case 'teal':
      return brandLogoTeal
    default:
      return selectedTheme.value === 'dark' || (selectedTheme.value === 'auto' && isDark.value)
        ? brandLogoViolet
        : brandLogoGold
  }
})

const androidAppIconPreview = computed(() => {
  switch (selectedAndroidAppIcon.value) {
    case 'purple':
      return brandLogoViolet
    case 'blue':
      return brandLogoBlue
    case 'teal':
      return brandLogoTeal
    default:
      return brandLogoGold
  }
})

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
    if (step.key === 'battery') return firstRun.value && statuses.value.battery === 'denied'
    return firstRun.value && statuses.value[step.key] === 'denied'
  }),
)
const currentStep = computed(() => steps.value[currentIndex.value] ?? steps.value[0] ?? null)
const currentStatus = computed(() => (currentStep.value ? statuses.value[currentStep.value.key] : 'unavailable'))
const totalOnboardingSteps = computed(() => steps.value.length + (firstRun.value ? 2 : 0))
const progress = computed(() => {
  if (totalOnboardingSteps.value === 0) return 100
  const completedPermissions = languageStepActive.value || appearanceStepActive.value ? 0 : currentIndex.value + 1
  const completedLanguage = firstRun.value && !languageStepActive.value ? 1 : 0
  const completedAppearance = firstRun.value && !languageStepActive.value && !appearanceStepActive.value ? 1 : 0
  return Math.round(
    ((completedPermissions + completedLanguage + completedAppearance) / totalOnboardingSteps.value) * 100,
  )
})

function readPermissionGuideSeen(): boolean {
  try {
    return (
      window.localStorage.getItem(GUIDE_STORAGE_KEY) === 'true' ||
      window.localStorage.getItem(LEGACY_STORAGE_KEY) === 'complete'
    )
  } catch {
    return false
  }
}

function markPermissionGuideSeen(): void {
  try {
    window.localStorage.setItem(GUIDE_STORAGE_KEY, 'true')
    window.localStorage.setItem(LEGACY_STORAGE_KEY, 'complete')
  } catch {
    // Storage can be unavailable in hardened WebViews; permission status still works.
  }
}

function refreshStatuses(): void {
  if (!isAndroid.value) return
  statuses.value = readAndroidPermissionStatuses()
  if (currentIndex.value >= steps.value.length) currentIndex.value = Math.max(0, steps.value.length - 1)
  visible.value =
    (languageStepActive.value || appearanceStepActive.value || steps.value.length > 0) && !dismissedThisSession.value
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
  appearanceStepActive.value = true
  currentIndex.value = 0
  visible.value = true
}

async function confirmAppearance(): Promise<void> {
  await preferenceStore.updateAndSave({
    theme: selectedTheme.value,
    colorScheme: selectedColorScheme.value,
    brandLogoStyle: selectedBrandLogoStyle.value,
    androidAppIcon: selectedAndroidAppIcon.value,
  })
  if (isAndroid.value && !setAndroidAppIcon(selectedAndroidAppIcon.value)) {
    logger.warn('AndroidPermissionWelcome.appIcon', 'Failed to apply Android launcher icon')
  }
  appearanceStepActive.value = false
  currentIndex.value = 0
  if (steps.value.length === 0) finish()
  else visible.value = true
}

async function advance(): Promise<void> {
  if (languageStepActive.value) {
    await confirmLanguage()
    return
  }
  if (appearanceStepActive.value) {
    await confirmAppearance()
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
  markPermissionGuideSeen()
  visible.value = false
}

function dismissForNow(): void {
  finish()
}

function onVisibilityOrFocus(): void {
  if (visible.value || isAndroid.value) refreshStatuses()
}

onMounted(() => {
  firstRun.value = !readPermissionGuideSeen()
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
        <h2>
          {{
            languageStepActive
              ? t('preferences.language')
              : appearanceStepActive
                ? t('preferences.appearance-section')
                : t('onboarding.title')
          }}
        </h2>
        <p>
          {{
            languageStepActive
              ? t('preferences.select-language')
              : appearanceStepActive
                ? t('onboarding.appearance-description')
                : t('onboarding.description')
          }}
        </p>
      </div>

      <div v-if="languageStepActive" class="permission-step permission-step--language">
        <div class="permission-step__icon">
          <NIcon :size="26"><LanguageOutline /></NIcon>
        </div>
        <div class="permission-step__copy">
          <select v-model="selectedLocale" :aria-label="t('preferences.select-language')" class="language-select">
            <option v-for="option in localeOptions" :key="option.value" :value="option.value">
              {{ option.label }}
            </option>
          </select>
        </div>
      </div>

      <div v-else-if="appearanceStepActive" class="permission-step permission-step--appearance">
        <div class="onboarding-appearance-group">
          <span class="onboarding-appearance-label">{{ t('preferences.appearance') }}</span>
          <div class="onboarding-theme-options" role="radiogroup" :aria-label="t('preferences.appearance')">
            <button
              v-for="option in themeOptions"
              :key="option.value"
              type="button"
              class="onboarding-theme-option"
              :class="{ active: selectedTheme === option.value }"
              role="radio"
              :aria-checked="selectedTheme === option.value"
              @click="selectedTheme = option.value"
            >
              {{ option.label }}
            </button>
          </div>
        </div>
        <div class="onboarding-appearance-group">
          <span class="onboarding-appearance-label">{{ t('preferences.color-scheme') }}</span>
          <div class="onboarding-color-options" role="radiogroup" :aria-label="t('preferences.color-scheme')">
            <button
              v-for="scheme in COLOR_SCHEMES"
              :key="scheme.id"
              type="button"
              class="onboarding-color-option"
              :class="{ active: selectedColorScheme === scheme.id }"
              :style="{ '--swatch-color': scheme.seed }"
              role="radio"
              :aria-label="t(scheme.labelKey)"
              :aria-checked="selectedColorScheme === scheme.id"
              @click="selectedColorScheme = scheme.id"
            >
              <span aria-hidden="true" />
            </button>
          </div>
        </div>
        <div class="onboarding-appearance-group">
          <span class="onboarding-appearance-label">{{ t('preferences.brand-logo') }}</span>
          <div class="onboarding-logo-selector">
            <img :src="brandLogoPreview" alt="" width="52" height="52" />
            <select v-model="selectedBrandLogoStyle" :aria-label="t('preferences.brand-logo')" class="language-select">
              <option v-for="option in brandLogoOptions" :key="option.value" :value="option.value">
                {{ option.label }}
              </option>
            </select>
          </div>
        </div>
        <div class="onboarding-appearance-group">
          <span class="onboarding-appearance-label">{{ t('preferences.android-app-icon') }}</span>
          <div class="onboarding-logo-selector">
            <img :src="androidAppIconPreview" alt="" width="52" height="52" />
            <select
              v-model="selectedAndroidAppIcon"
              :aria-label="t('preferences.android-app-icon')"
              class="language-select"
            >
              <option v-for="option in androidAppIconOptions" :key="option.value" :value="option.value">
                {{ option.label }}
              </option>
            </select>
          </div>
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
              : appearanceStepActive
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
  max-height: calc(100dvh - 24px);
  padding: 28px;
  overflow-x: hidden;
  overflow-y: auto;
  overscroll-behavior: contain;
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
.language-select {
  width: 100%;
  min-height: 44px;
  padding: 0 42px 0 14px;
  color: var(--m3-on-surface);
  background-color: var(--m3-surface-container-highest);
  border: 1px solid var(--m3-outline);
  border-radius: var(--radius-md);
  font: inherit;
  touch-action: manipulation;
}
.language-select:focus-visible {
  border-color: var(--m3-primary);
  outline: 2px solid color-mix(in srgb, var(--m3-primary) 36%, transparent);
  outline-offset: 2px;
}
.language-select option {
  color: var(--m3-on-surface);
  background: var(--m3-surface-container-high);
}
.permission-step--appearance {
  grid-template-columns: minmax(0, 1fr);
  gap: 18px;
}
.onboarding-appearance-group {
  display: grid;
  gap: 9px;
}
.onboarding-appearance-label {
  color: var(--m3-on-surface);
  font-size: var(--font-size-sm);
  font-weight: 600;
}
.onboarding-theme-options {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 8px;
}
.onboarding-theme-option {
  min-height: 44px;
  padding: 8px 10px;
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-lg);
  background: var(--m3-surface-container-high);
  color: var(--m3-on-surface-variant);
  font: inherit;
  cursor: pointer;
}
.onboarding-theme-option.active {
  border-color: var(--m3-primary);
  background: var(--m3-primary-container);
  color: var(--m3-on-primary-container);
  font-weight: 650;
}
.onboarding-theme-option:focus-visible,
.onboarding-color-option:focus-visible {
  outline: 2px solid var(--m3-primary);
  outline-offset: 2px;
}
.onboarding-color-options {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
}
.onboarding-color-option {
  display: grid;
  place-items: center;
  width: 44px;
  height: 44px;
  padding: 0;
  border: 2px solid transparent;
  border-radius: 50%;
  background: transparent;
  cursor: pointer;
}
.onboarding-color-option > span {
  display: block;
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background: var(--swatch-color);
  box-shadow: inset 0 0 0 1px color-mix(in srgb, var(--m3-on-surface) 18%, transparent);
}
.onboarding-color-option.active {
  border-color: var(--m3-primary);
}
.onboarding-logo-selector {
  display: grid;
  grid-template-columns: 52px minmax(0, 1fr);
  align-items: center;
  gap: 12px;
}
.onboarding-logo-selector img {
  display: block;
  border-radius: 14px;
  box-shadow: 0 4px 14px color-mix(in srgb, var(--m3-shadow) 70%, transparent);
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
