<script setup lang="ts">
/** @fileoverview Central Android permission status and authorization page. */
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { NAlert, NButton, NIcon, NSwitch, NTag } from 'naive-ui'
import {
  BatteryHalfOutline,
  CheckmarkCircleOutline,
  ClipboardOutline,
  CloseCircleOutline,
  CubeOutline,
  FolderOpenOutline,
  NotificationsOutline,
} from '@vicons/ionicons5'
import {
  useAndroidPermissions,
  ANDROID_PERMISSION_KEYS,
  type AndroidPermissionKey,
} from '@/composables/useAndroidPermissions'
import { usePreferenceStore } from '@/stores/preference'
import { useAppMessage } from '@/composables/useAppMessage'
import { syncAndroidNotificationMode } from '@/composables/useAndroidNative'
import type { AppConfig } from '@shared/types'
import NotificationAccess from './NotificationAccess.vue'

const { t } = useI18n()
const { statuses, supported, missing, authorizing, request, requestAll, refresh } = useAndroidPermissions()
const preferenceStore = usePreferenceStore()
const message = useAppMessage()

async function saveNotification(patch: Partial<AppConfig>): Promise<void> {
  const saved = await preferenceStore.updateAndSave(patch)
  if (!saved) message.error(t('preferences.save-fail-message'))
}

function booleanSetting(key: keyof AppConfig) {
  return computed({
    get: () => Boolean(preferenceStore.config[key]),
    set: (value: boolean) => void saveNotification({ [key]: value } as Partial<AppConfig>),
  })
}

const taskNotification = booleanSetting('taskNotification')
const notifyOnStart = booleanSetting('notifyOnStart')
const notifyOnComplete = booleanSetting('notifyOnComplete')
const notifyOnError = booleanSetting('notifyOnError')
const notificationIncludeName = booleanSetting('notificationIncludeName')
const notificationKeepAlive = computed({
  get: () => preferenceStore.config.notificationKeepAlive,
  set: (value: boolean) => {
    void saveNotification({ notificationKeepAlive: value })
    syncAndroidNotificationMode(value)
  },
})

const rows = computed(() =>
  ANDROID_PERMISSION_KEYS.map((key) => ({
    key,
    icon: iconFor(key),
    title: t(`preferences.permission-${key}`),
    hint: t(`preferences.permission-${key}-hint`),
    status: statuses.value[key],
  })),
)

function iconFor(key: AndroidPermissionKey) {
  if (key === 'storage') return FolderOpenOutline
  if (key === 'notification') return NotificationsOutline
  if (key === 'install') return CubeOutline
  if (key === 'clipboard') return ClipboardOutline
  return BatteryHalfOutline
}

function statusLabel(key: AndroidPermissionKey): string {
  switch (statuses.value[key]) {
    case 'granted':
      return t('preferences.permission-granted')
    case 'denied':
      return t('preferences.permission-denied')
    default:
      return t('preferences.permission-unavailable')
  }
}
</script>

<template>
  <div class="permissions-page">
    <header class="permissions-page__header">
      <h2>{{ t('preferences.permissions-title') }}</h2>
      <p>{{ t('preferences.permissions-description') }}</p>
    </header>

    <NAlert v-if="!supported" type="info" :show-icon="false">
      {{ t('preferences.permissions-android-only') }}
    </NAlert>

    <div class="permissions-page__actions">
      <NButton type="primary" :loading="authorizing" :disabled="!supported || missing.length === 0" @click="requestAll">
        {{ authorizing ? t('onboarding.authorizing') : t('preferences.permission-one-click') }}
      </NButton>
      <NButton quaternary :disabled="!supported" @click="refresh">{{ t('preferences.permission-refresh') }}</NButton>
    </div>

    <div class="permissions-page__grid">
      <article v-for="row in rows" :key="row.key" class="permission-card">
        <div class="permission-card__icon">
          <NIcon :size="23"><component :is="row.icon" /></NIcon>
        </div>
        <div class="permission-card__copy">
          <div class="permission-card__title">
            <strong>{{ row.title }}</strong>
            <NTag
              size="small"
              :type="row.status === 'granted' ? 'success' : row.status === 'denied' ? 'warning' : 'default'"
            >
              <template #icon>
                <NIcon>
                  <CheckmarkCircleOutline v-if="row.status === 'granted'" />
                  <CloseCircleOutline v-else />
                </NIcon>
              </template>
              {{ statusLabel(row.key) }}
            </NTag>
          </div>
          <p>{{ row.hint }}</p>
        </div>
        <NButton
          size="small"
          :disabled="!supported || row.status === 'granted' || row.status === 'unavailable'"
          @click="request(row.key)"
        >
          {{ row.status === 'granted' ? t('preferences.permission-granted') : t('preferences.permission-grant') }}
        </NButton>
      </article>
    </div>

    <section class="notification-settings">
      <header class="notification-settings__header">
        <h3>{{ t('preferences.notification-section') }}</h3>
        <p>{{ t('preferences.notification-settings-hint') }}</p>
      </header>

      <div class="notification-row">
        <div class="notification-row__copy">
          <strong>{{ t('preferences.task-completed-notify') }}</strong>
          <p>{{ t('preferences.notification-master-hint') }}</p>
        </div>
        <NSwitch v-model:value="taskNotification" />
      </div>
      <div class="notification-row">
        <div class="notification-row__copy">
          <strong>{{ t('preferences.notify-on-start') }}</strong>
        </div>
        <NSwitch v-model:value="notifyOnStart" :disabled="!taskNotification" />
      </div>
      <div class="notification-row">
        <div class="notification-row__copy">
          <strong>{{ t('preferences.notify-on-complete') }}</strong>
        </div>
        <NSwitch v-model:value="notifyOnComplete" :disabled="!taskNotification" />
      </div>
      <div class="notification-row">
        <div class="notification-row__copy">
          <strong>{{ t('preferences.notify-on-error') }}</strong>
        </div>
        <NSwitch v-model:value="notifyOnError" :disabled="!taskNotification" />
      </div>
      <div class="notification-row">
        <div class="notification-row__copy">
          <strong>{{ t('preferences.notification-include-name') }}</strong>
          <p>{{ t('preferences.notification-include-name-hint') }}</p>
        </div>
        <NSwitch v-model:value="notificationIncludeName" :disabled="!taskNotification" />
      </div>
      <div class="notification-row">
        <div class="notification-row__copy">
          <strong>{{ t('preferences.notification-keep-alive') }}</strong>
          <p>{{ t('preferences.notification-keep-alive-hint') }}</p>
        </div>
        <NSwitch v-model:value="notificationKeepAlive" :disabled="!supported" />
      </div>
      <NotificationAccess />
    </section>
  </div>
</template>

<style scoped>
.permissions-page {
  display: flex;
  flex-direction: column;
  gap: 18px;
  height: 100%;
  max-width: 820px;
  padding: var(--content-gutter);
  overflow-x: hidden;
  overflow-y: auto;
  overscroll-behavior: contain;
  -webkit-overflow-scrolling: touch;
}
.permissions-page > * {
  flex: 0 0 auto;
}
.permissions-page__header h2 {
  margin: 0 0 6px;
  font-size: 20px;
}
.permissions-page__header p,
.permission-card p {
  margin: 0;
  color: var(--m3-on-surface-variant);
  line-height: 1.55;
}
.permissions-page__actions {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}
.permissions-page__grid {
  display: grid;
  gap: 10px;
}
.permission-card {
  display: grid;
  grid-template-columns: 46px minmax(0, 1fr) auto;
  gap: 12px;
  align-items: center;
  padding: 14px;
  background: var(--m3-surface-container);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-lg);
}
.permission-card__icon {
  display: grid;
  width: 46px;
  height: 46px;
  place-items: center;
  color: var(--m3-on-primary-container);
  background: var(--m3-primary-container);
  border-radius: 14px;
}
.permission-card__copy,
.permission-card__title {
  display: grid;
  gap: 5px;
  min-width: 0;
}
.notification-settings {
  display: grid;
  gap: 10px;
  padding-top: 4px;
}
.notification-settings__header h3 {
  margin: 0 0 4px;
  font-size: 16px;
}
.notification-settings__header p,
.notification-row__copy p {
  margin: 0;
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-sm);
  line-height: 1.5;
}
.notification-row {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto;
  gap: 12px;
  align-items: center;
  padding: 12px 14px;
  background: var(--m3-surface-container);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-lg);
}
.notification-row__copy {
  display: grid;
  gap: 4px;
  min-width: 0;
}
.permission-card__title {
  grid-template-columns: minmax(0, 1fr) auto;
  align-items: center;
}
@media (max-width: 620px) {
  .permission-card {
    grid-template-columns: 40px minmax(0, 1fr);
  }
  .permission-card > :last-child {
    grid-column: 1 / -1;
    justify-self: end;
  }
}
</style>
