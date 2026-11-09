<script setup lang="ts">
/** @fileoverview Lightweight Android permission onboarding and battery prompt. */
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { NButton, NIcon, NModal } from 'naive-ui'
import { BatteryHalfOutline, CheckmarkCircleOutline, FolderOpenOutline, NotificationsOutline } from '@vicons/ionicons5'
import { usePlatform } from '@/composables/usePlatform'
import {
  readAndroidPermissionStatuses,
  requestAndroidBatteryOptimizationExemption,
  requestAndroidNotificationPermission,
  requestAndroidStoragePermission,
  type AndroidPermissionStatus,
} from '@/composables/useAndroidNative'

type PermissionKey = 'storage' | 'notification' | 'battery'

interface PermissionStep {
  key: PermissionKey
  title: string
  icon: typeof FolderOpenOutline
}

const STORAGE_KEY = 'rayload-onboarding-v1'
const { t } = useI18n()
const { isAndroid } = usePlatform()
const visible = ref(false)
const firstRun = ref(true)
const dismissedThisSession = ref(false)
const currentIndex = ref(0)
const statuses = ref<Record<PermissionKey, AndroidPermissionStatus>>({
  storage: 'unavailable',
  notification: 'unavailable',
  battery: 'unavailable',
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
    if (step.key === 'battery') return statuses.value.battery === 'denied'
    return firstRun.value && statuses.value[step.key] === 'denied'
  }),
)
const currentStep = computed(() => steps.value[currentIndex.value] ?? steps.value[0] ?? null)
const currentStatus = computed(() => (currentStep.value ? statuses.value[currentStep.value.key] : 'unavailable'))
const progress = computed(() =>
  steps.value.length === 0 ? 0 : Math.round(((currentIndex.value + 1) / steps.value.length) * 100),
)

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
  visible.value = steps.value.length > 0 && !dismissedThisSession.value
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

function advance(): void {
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
        <h2>{{ t('onboarding.title') }}</h2>
        <p>{{ t('onboarding.description') }}</p>
      </div>

      <div v-if="currentStep" class="permission-step">
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
        <NButton quaternary @click="dismissForNow">{{ t('onboarding.later') }}</NButton>
        <NButton type="primary" @click="advance">
          {{
            currentStatus === 'granted'
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
