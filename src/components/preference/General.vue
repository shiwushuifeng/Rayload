<script setup lang="ts">
/**
 * @fileoverview General preference tab: system info, language, how downloads arrive (browser
 * extension, clipboard), notifications, update, appearance, startup & tray, default programs.
 */
import { ref, computed, watch, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { usePreferenceStore } from '@/stores/preference'
import { useAppStore } from '@/stores/app'
import { usePreferenceForm } from '@/composables/usePreferenceForm'
import { useEngineStore } from '@/stores/engine'
import { useFloatWindowRelaunch } from '@/composables/useFloatWindowRelaunch'
import { invoke } from '@tauri-apps/api/core'
import { relaunch } from '@tauri-apps/plugin-process'
import { arch as osArch, locale as getLocale, version as osVersion } from '@tauri-apps/plugin-os'
import { usePlatform } from '@/composables/usePlatform'
import { setAndroidAppIcon } from '@/composables/useAndroidNative'
import { getVersion as getAppVersion } from '@tauri-apps/api/app'
import { resolveSystemLocale } from '@shared/utils/locale'
import { loadLocale } from '@/composables/useLocale'
import { isSupportedLocale, LOCALE_CATALOG, SUPPORTED_LOCALES } from '@shared/localeCatalog'
import { logger } from '@shared/logger'
import { writeAppClipboardText } from '@shared/utils'
import { buildGeneralForm, transformGeneralForStore } from '@/composables/useGeneralPreference'
import { COLOR_SCHEMES, CUSTOM_COLOR_SCHEME_ID, FLOAT_WINDOW_MODE_LABELS, FLOAT_WINDOW_MODES } from '@shared/constants'
import { normalizeCustomColorScheme } from '@shared/utils/colorSchemeConfig'
import { useAppMessage } from '@/composables/useAppMessage'
import {
  NForm,
  NFormItem,
  NSelect,
  NSwitch,
  NButton,
  NDivider,
  NText,
  NCollapseTransition,
  NSpace,
  NTag,
  NRadioGroup,
  NRadioButton,
  NColorPicker,
  NIcon,
  NModal,
} from 'naive-ui'
import PreferenceActionRegistration from './PreferenceActionRegistration.vue'
import MTooltip from '@/components/common/MTooltip.vue'
import { CloudDownloadOutline } from '@vicons/ionicons5'
import type { UpdateChannel } from '@shared/types'
import PreferenceHintLabel from './PreferenceHintLabel.vue'
import PreferenceCheckboxGrid from './PreferenceCheckboxGrid.vue'
import PreferenceAssociations from './PreferenceAssociations.vue'
import NavigationCustomization from './NavigationCustomization.vue'
import AndroidUpdateCheck from './AndroidUpdateCheck.vue'
import { UI_SCALE_STEPS } from '@/composables/useInterfaceScale'

const { t, locale } = useI18n()
const preferenceStore = usePreferenceStore()
const appStore = useAppStore()
const message = useAppMessage()
const { isMac, isLinux, isAndroid, platformLabel, archLabel: getArchLabel } = usePlatform()

// ─── System info card ────────────────────────────────────────────────
const sysArch = ref('')
const sysOsVersion = ref('')
const sysAppVersion = ref('')
const detectedLocaleCode = ref('en-US')
const archLabelDisplay = computed(() => getArchLabel(sysArch.value))

async function copyVersionToClipboard(text: string, label: string) {
  try {
    await writeAppClipboardText(text)
    message.success(t('about.version-copied', { label }))
  } catch (e) {
    logger.debug('General.clipboard', `writeText failed: ${e}`)
  }
}
const checkIntervalOptions = [
  { label: t('preferences.interval-every-startup'), value: 0 },
  { label: t('preferences.interval-daily'), value: 24 },
  { label: t('preferences.interval-weekly'), value: 168 },
  { label: t('preferences.interval-monthly'), value: 720 },
  { label: t('preferences.interval-semi-annual'), value: 4320 },
  { label: t('preferences.interval-yearly'), value: 8760 },
]

const CUSTOM_COLOR_SWATCHES = ['#F59E0B', '#2563EB', '#14B8A6', '#DC2626', '#9333EA', '#4B5563']

function handleUpdateStableOnly(value: boolean) {
  form.value.updateStableOnly = value
  void commitInstant({ updateStableOnly: value }, () => preferenceStore.updateAndSave({ updateStableOnly: value }))
}

function buildForm() {
  return buildGeneralForm(preferenceStore.config)
}

const offerFloatWindowRelaunch = useFloatWindowRelaunch()

const languagePrompt = ref<{ title: string; content: string; restart: string; later: string } | null>(null)

async function dismissLanguagePrompt(restart: boolean): Promise<void> {
  languagePrompt.value = null
  if (!restart) return
  await engineStore.stop('appRelaunch')
  relaunch()
}

const { form, isDirty, handleSave, handleReset, patchSnapshot, resetSnapshot, commitInstant } = usePreferenceForm({
  buildForm,
  transformForStore: transformGeneralForStore,
  afterSave: async (f, prevConfig) => {
    // Locale change → restart prompt
    const prevLocale = prevConfig.locale || 'auto'
    if (f.locale !== prevLocale) {
      // Determine the actual target locale for bilingual dialog rendering.
      const targetLocale = isSupportedLocale(f.locale)
        ? f.locale
        : resolveSystemLocale(detectedLocaleCode.value, SUPPORTED_LOCALES)
      const isEn = targetLocale === 'en-US'
      // Locale messages are lazy-loaded — pull in the target locale so the
      // dialog can render in it (falls back to English if loading fails).
      if (!isEn) await loadLocale(targetLocale)
      const tt = (key: string) => t(key, {}, { locale: targetLocale })
      languagePrompt.value = {
        title: tt('preferences.language-changed-title'),
        content: tt('preferences.language-changed-content'),
        restart: tt('preferences.language-changed-restart'),
        later: tt('preferences.language-changed-later'),
      }
    }

    // Sync autostart state immediately on save
    if (!isAndroid.value && f.openAtLogin !== !!prevConfig.openAtLogin) {
      try {
        await invoke('set_autostart', { enabled: !!f.openAtLogin })
      } catch (e) {
        logger.error('General.autostart', e)
      }
    }

    if (isAndroid.value && f.androidAppIcon !== prevConfig.androidAppIcon) {
      if (!setAndroidAppIcon(f.androidAppIcon)) {
        message.warning(t('preferences.android-app-icon-failed'))
      }
    }

    if (f.floatWindow !== prevConfig.floatWindow) await offerFloatWindowRelaunch(f.floatWindow)
  },
})

// ── Download capture ─────────────────────────────────────────────────
type ClipboardType = 'http' | 'sftp' | 'magnet' | 'ed2k' | 'thunder' | 'btHash'
const clipboardTypes: ClipboardType[] = ['http', 'sftp', 'magnet', 'ed2k', 'thunder', 'btHash']
const clipboardTypeOptions = computed(() => [
  { label: t('preferences.clipboard-http'), value: 'http' },
  { label: t('preferences.clipboard-sftp'), value: 'sftp' },
  { label: t('preferences.clipboard-magnet'), value: 'magnet' },
  { label: t('preferences.clipboard-ed2k'), value: 'ed2k' },
  { label: t('preferences.clipboard-thunder'), value: 'thunder' },
  { label: t('preferences.clipboard-bt-hash'), value: 'btHash' },
])
const clipboardFieldByType: Record<ClipboardType, keyof typeof form.value> = {
  http: 'clipboardHttp',
  sftp: 'clipboardSftp',
  magnet: 'clipboardMagnet',
  ed2k: 'clipboardEd2k',
  thunder: 'clipboardThunder',
  btHash: 'clipboardBtHash',
}
const selectedClipboardTypes = computed<string[]>({
  get: () => clipboardTypes.filter((type) => !!form.value[clipboardFieldByType[type]]),
  set: (types) => {
    const selected = new Set(types)
    for (const type of clipboardTypes) {
      form.value[clipboardFieldByType[type]] = selected.has(type)
    }
  },
})

const floatWindowOptions = computed(() =>
  FLOAT_WINDOW_MODES.map((value) => ({ value, label: t(FLOAT_WINDOW_MODE_LABELS[value]) })),
)

// Note: the legacy one-shot locale sync watcher has been removed.
// With 'auto' as an explicit option, there is no async race condition
// to handle — the form correctly initialises with 'auto' from config.

// ── Instant color-scheme application ─────────────────────────────────
function handlePresetColorScheme(scheme: (typeof COLOR_SCHEMES)[number]): void {
  if (form.value.colorScheme === scheme.id) return

  form.value.colorScheme = scheme.id
  preferenceStore.updateAndSave({ colorScheme: scheme.id, customColorScheme: form.value.customColorScheme })
  patchSnapshot({ colorScheme: scheme.id } as Partial<typeof form.value>)
  message.success(t('preferences.color-scheme-switched', { name: t(scheme.labelKey) }))
}

async function handleCustomColorChange(value: string | null): Promise<void> {
  const color = normalizeCustomColorScheme(value)
  if (form.value.colorScheme === CUSTOM_COLOR_SCHEME_ID && form.value.customColorScheme === color) return

  form.value.customColorScheme = color
  form.value.colorScheme = CUSTOM_COLOR_SCHEME_ID
  patchSnapshot({ colorScheme: CUSTOM_COLOR_SCHEME_ID, customColorScheme: color } as Partial<typeof form.value>)
  await preferenceStore.updateAndSave({ colorScheme: CUSTOM_COLOR_SCHEME_ID, customColorScheme: color })
}

function handleCustomColorComplete(value: string): void {
  const color = normalizeCustomColorScheme(value)
  message.success(t('preferences.color-scheme-switched', { name: color }))
}

// ── Instant theme application ────────────────────────────────────────
watch(
  () => form.value.theme,
  (newTheme, oldTheme) => {
    if (!newTheme || newTheme === oldTheme) return
    preferenceStore.updateAndSave({ theme: newTheme as 'auto' | 'light' | 'dark' })
    patchSnapshot({ theme: newTheme } as Partial<typeof form.value>)
  },
)

// ── Closing the main window: ask, minimize to tray, or quit ─────────
type CloseAction = 'ask' | 'tray' | 'quit'
const closeAction = computed<CloseAction>({
  get: () => (form.value.minimizeToTrayOnClose ? 'tray' : form.value.quitOnClose ? 'quit' : 'ask'),
  set: (action) => {
    form.value.minimizeToTrayOnClose = action === 'tray'
    form.value.quitOnClose = action === 'quit'
  },
})
const closeActionOptions = computed(() => [
  { label: t('preferences.close-action-ask'), value: 'ask' },
  { label: t('app.minimize-to-tray'), value: 'tray' },
  { label: t('app.quit-app'), value: 'quit' },
])

// ── Lightweight mode ↔ Minimize-to-tray linkage ─────────────────────
watch(
  () => form.value.lightweightMode,
  (enabled) => {
    if (enabled && !form.value.minimizeToTrayOnClose) closeAction.value = 'tray'
  },
)
watch(
  () => form.value.minimizeToTrayOnClose,
  (enabled) => {
    if (!enabled && form.value.lightweightMode) {
      form.value.lightweightMode = false
    }
  },
)

const localeOptions = LOCALE_CATALOG.map(({ code, label }) => ({ label, value: code }))

/** Dynamic label for the 'auto' option. */
const autoLocaleLabel = computed(() => {
  return locale.value === 'en-US' ? t('preferences.follow-system') : `${t('preferences.follow-system')} · Follow System`
})

/** Full locale options with 'Follow System' prepended as the first choice. */
const fullLocaleOptions = computed(() => [{ label: autoLocaleLabel.value, value: 'auto' }, ...localeOptions])

// Interface size applies at once, like the zoom shortcuts that change it too.
const uiScaleOptions = UI_SCALE_STEPS.map((step) => ({ label: `${Math.round(step * 100)}%`, value: step }))
function handleUiScaleChange(scale: number) {
  void preferenceStore.updateAndSave({ uiScale: scale })
}

const themeOptions = computed(() => [
  { label: t('preferences.theme-auto'), value: 'auto' },
  { label: t('preferences.theme-light'), value: 'light' },
  { label: t('preferences.theme-dark'), value: 'dark' },
])

const brandLogoOptions = computed(() => [
  { label: t('preferences.brand-logo-auto'), value: 'auto' },
  { label: t('preferences.brand-logo-gold'), value: 'gold' },
  { label: t('preferences.brand-logo-violet'), value: 'violet' },
  { label: t('preferences.brand-logo-blue'), value: 'blue' },
  { label: t('preferences.brand-logo-teal'), value: 'teal' },
])

const androidAppIconOptions = computed(() => [
  { label: t('preferences.android-app-icon-gold'), value: 'gold' },
  { label: t('preferences.android-app-icon-purple'), value: 'purple' },
  { label: t('preferences.android-app-icon-blue'), value: 'blue' },
  { label: t('preferences.android-app-icon-teal'), value: 'teal' },
])

const taskCardModeOptions = computed(() => [
  { label: t('preferences.task-card-mode-full'), value: 'full' },
  { label: t('preferences.task-card-mode-compact'), value: 'compact' },
])

function handleCheckUpdate() {
  appStore.requestUpdateCheck()
}

const engineStore = useEngineStore()

onMounted(async () => {
  try {
    sysArch.value = osArch()
  } catch (e) {
    logger.debug('General.arch', e)
  }
  try {
    sysOsVersion.value = osVersion()
  } catch (e) {
    logger.debug('General.osVersion', e)
  }
  try {
    sysAppVersion.value = await getAppVersion()
  } catch (e) {
    logger.debug('General.appVersion', e)
  }
  try {
    const raw = (await getLocale()) || 'en-US'
    detectedLocaleCode.value = resolveSystemLocale(raw, SUPPORTED_LOCALES)
  } catch (e) {
    logger.debug('General.detectLocale', e)
  }
  resetSnapshot()
})
</script>

<template>
  <div class="preference-form-wrapper">
    <div class="preference-form-scroll">
      <NForm label-placement="left" label-align="left" label-width="260px" size="small" class="form-preference">
        <NFormItem :label="t('about.app-version')">
          <NSpace :size="8" align="center">
            <MTooltip>
              <template #trigger>
                <button
                  class="sysinfo-ver-badge"
                  @click="
                    copyVersionToClipboard(
                      `Rayload v${sysAppVersion} · ${platformLabel} · ${archLabelDisplay}`,
                      'Rayload',
                    )
                  "
                >
                  <span class="sysinfo-ver-value">v{{ sysAppVersion || '\u2014' }}</span>
                  <svg class="sysinfo-ver-copy" width="14" height="14" viewBox="0 0 24 24" fill="none">
                    <rect x="9" y="9" width="13" height="13" rx="2" stroke="currentColor" stroke-width="2" />
                    <path
                      d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"
                      stroke="currentColor"
                      stroke-width="2"
                    />
                  </svg>
                </button>
              </template>
              {{ t('about.click-to-copy') }}
            </MTooltip>
            <NTag type="info" round size="medium">{{ platformLabel }}</NTag>
            <NTag type="success" round size="medium">{{ archLabelDisplay }}</NTag>
          </NSpace>
        </NFormItem>

        <AndroidUpdateCheck
          v-if="isAndroid"
          :stable-only="form.updateStableOnly"
          @update:stable-only="handleUpdateStableOnly"
        />

        <!-- ② Language -->
        <NDivider title-placement="left">
          {{ locale === 'en-US' ? t('preferences.language') : `${t('preferences.language')} · Language` }}
        </NDivider>
        <NFormItem
          :label="
            locale === 'en-US'
              ? t('preferences.select-language')
              : `${t('preferences.select-language')} · Select Language`
          "
        >
          <NSelect
            v-model:value="form.locale"
            :options="fullLocaleOptions"
            class="pref-control-auto pref-control-language"
          />
        </NFormItem>

        <template v-if="!isAndroid">
          <!-- Links that reach Rayburst through the clipboard -->
          <NDivider title-placement="left">{{ t('preferences.download-capture-section') }}</NDivider>
          <NFormItem>
            <template #label>
              <PreferenceHintLabel
                :label="t('preferences.clipboard-auto-detect')"
                :hint="t('preferences.clipboard-filter-hint')"
              />
            </template>
            <NSwitch v-model:value="form.clipboardEnable" />
          </NFormItem>
          <NCollapseTransition :show="form.clipboardEnable">
            <NFormItem label=" ">
              <PreferenceCheckboxGrid v-model:value="selectedClipboardTypes" :options="clipboardTypeOptions" />
            </NFormItem>
            <NFormItem v-if="form.clipboardHttp" :label="t('preferences.clipboard-http-files-only')">
              <NSwitch v-model:value="form.clipboardHttpFilesOnly" />
            </NFormItem>
            <NFormItem>
              <template #label>
                <PreferenceHintLabel
                  :label="t('preferences.clipboard-background')"
                  :hint="t('preferences.clipboard-background-hint')"
                />
              </template>
              <NSwitch v-model:value="form.clipboardBackground" />
            </NFormItem>
          </NCollapseTransition>
        </template>

        <template v-if="!isAndroid">
          <!-- ③ Auto Update -->
          <NDivider title-placement="left">{{ t('preferences.auto-update') }}</NDivider>
          <NFormItem :label="t('preferences.auto-check-update')">
            <NSwitch v-model:value="form.autoCheckUpdate" />
          </NFormItem>
          <NCollapseTransition :show="form.autoCheckUpdate" class="collapse-indent">
            <NFormItem :label="t('preferences.check-frequency')">
              <NSelect
                v-model:value="form.autoCheckUpdateInterval"
                :options="checkIntervalOptions"
                class="pref-control-auto"
              />
            </NFormItem>
          </NCollapseTransition>
          <NFormItem :label="t('preferences.update-channel')">
            <NRadioGroup
              v-model:value="form.updateChannel"
              size="small"
              @update:value="
                (v: UpdateChannel) =>
                  commitInstant({ updateChannel: v }, () => preferenceStore.updateAndSave({ updateChannel: v }))
              "
            >
              <NRadioButton value="stable">{{ t('preferences.update-channel-stable') }}</NRadioButton>
              <NRadioButton value="beta">{{ t('preferences.update-channel-beta') }}</NRadioButton>
            </NRadioGroup>
          </NFormItem>
          <NFormItem :label="t('preferences.last-check-update-time')">
            <div class="pref-inline-row">
              <NButton size="small" @click="handleCheckUpdate">
                <template #icon>
                  <NIcon :size="14"><CloudDownloadOutline /></NIcon>
                </template>
                {{ t('app.check-updates-now') }}
              </NButton>
              <NText v-if="preferenceStore.config.lastCheckUpdateTime" depth="3" class="pref-inline-row__meta">
                {{ new Date(preferenceStore.config.lastCheckUpdateTime).toLocaleString() }}
              </NText>
              <NText v-else depth="3" class="pref-inline-row__meta">—</NText>
            </div>
          </NFormItem>
        </template>
        <!-- ④ Appearance -->
        <NDivider title-placement="left">{{ t('preferences.appearance-section') }}</NDivider>
        <NFormItem :label="t('preferences.appearance')">
          <NSelect v-model:value="form.theme" :options="themeOptions" class="pref-control-auto" />
        </NFormItem>
        <NFormItem :label="t('preferences.color-scheme')">
          <div class="color-scheme-picker">
            <MTooltip v-for="scheme in COLOR_SCHEMES" :key="scheme.id">
              <template #trigger>
                <button
                  class="color-swatch"
                  :aria-label="t(scheme.labelKey)"
                  :class="{ active: form.colorScheme === scheme.id }"
                  :style="{ '--swatch-color': scheme.seed }"
                  @click="handlePresetColorScheme(scheme)"
                >
                  <svg v-if="form.colorScheme === scheme.id" class="swatch-check" viewBox="0 0 16 16" fill="none">
                    <path
                      d="M4 8.5L6.5 11L12 5"
                      stroke="white"
                      stroke-width="2"
                      stroke-linecap="round"
                      stroke-linejoin="round"
                    />
                  </svg>
                </button>
              </template>
              {{ t(scheme.labelKey) }}
            </MTooltip>
          </div>
        </NFormItem>
        <NFormItem :label="t('preferences.custom-color-scheme')">
          <div class="custom-color-picker-wrap">
            <NColorPicker
              :value="form.customColorScheme"
              :modes="['hex']"
              :show-alpha="false"
              :show-preview="true"
              :swatches="CUSTOM_COLOR_SWATCHES"
              class="custom-color-picker"
              @update:value="handleCustomColorChange"
              @complete="handleCustomColorComplete"
            />
          </div>
        </NFormItem>
        <NFormItem :label="t('preferences.brand-logo')">
          <NSelect v-model:value="form.brandLogoStyle" :options="brandLogoOptions" class="pref-control-auto" />
        </NFormItem>
        <NFormItem v-if="isAndroid" :label="t('preferences.android-app-icon')">
          <NSelect v-model:value="form.androidAppIcon" :options="androidAppIconOptions" class="pref-control-auto" />
          <div class="pref-field-hint">
            <NText depth="3">{{ t('preferences.android-app-icon-restart-hint') }}</NText>
          </div>
        </NFormItem>
        <NFormItem v-if="!isAndroid">
          <template #label>
            <PreferenceHintLabel :label="t('preferences.ui-scale')" :hint="t('preferences.ui-scale-hint')" />
          </template>
          <NSelect
            :value="preferenceStore.config.uiScale"
            :options="uiScaleOptions"
            class="pref-control-auto"
            @update:value="handleUiScaleChange"
          />
        </NFormItem>
        <NFormItem :label="t('preferences.task-card-mode')">
          <NRadioGroup v-model:value="form.taskCardMode">
            <NRadioButton v-for="option in taskCardModeOptions" :key="option.value" :value="option.value">
              {{ option.label }}
            </NRadioButton>
          </NRadioGroup>
        </NFormItem>
        <NFormItem :label="t('preferences.show-logo-when-empty')">
          <NSwitch v-model:value="form.showLogoWhenEmpty" />
        </NFormItem>
        <NFormItem v-if="isAndroid">
          <template #label>
            <PreferenceHintLabel
              :label="t('preferences.show-speedometer')"
              :hint="t('preferences.show-speedometer-hint')"
            />
          </template>
          <NSwitch v-model:value="form.showSpeedometer" />
        </NFormItem>
        <NFormItem :label="t('preferences.reduce-motion')">
          <NSwitch v-model:value="form.reduceMotion" />
        </NFormItem>
        <NFormItem :label="t('preferences.sidebar-task-counts')">
          <NSwitch v-model:value="form.sidebarTaskCounts" />
        </NFormItem>
        <NFormItem>
          <template #label>
            <PreferenceHintLabel
              :label="t('preferences.iec-size-units')"
              :hint="t('preferences.iec-size-units-hint')"
            />
          </template>
          <NSwitch v-model:value="form.iecSizeUnits" />
        </NFormItem>
        <NFormItem v-if="isLinux">
          <template #label>
            <PreferenceHintLabel
              :label="t('preferences.system-title-bar')"
              :hint="t('preferences.system-title-bar-hint')"
            />
          </template>
          <NSwitch v-model:value="form.systemTitleBar" />
        </NFormItem>
        <NFormItem v-if="isMac" :label="t('preferences.dock-badge-speed')">
          <NSwitch v-model:value="form.dockBadgeSpeed" />
        </NFormItem>

        <NavigationCustomization v-if="isAndroid" />

        <!-- ⑪ Startup & Tray -->
        <NDivider title-placement="left">{{ t('preferences.startup-behavior') }}</NDivider>
        <template v-if="!isAndroid">
          <NFormItem :label="t('preferences.open-at-login')">
            <NSwitch v-model:value="form.openAtLogin" />
          </NFormItem>
          <NCollapseTransition :show="form.openAtLogin" class="collapse-indent">
            <NFormItem :label="t('preferences.auto-hide-window')">
              <NSwitch v-model:value="form.autoHideWindow" />
            </NFormItem>
          </NCollapseTransition>
          <NFormItem :label="t('preferences.keep-window-state')">
            <NSwitch v-model:value="form.keepWindowState" />
          </NFormItem>
        </template>
        <NFormItem :label="t('preferences.auto-resume-all')">
          <NSwitch v-model:value="form.resumeAllWhenAppLaunched" />
        </NFormItem>
        <template v-if="!isAndroid">
          <NDivider title-placement="left">{{ t('preferences.tray-and-dock') }}</NDivider>
          <NFormItem>
            <template #label>
              <PreferenceHintLabel
                :label="t('preferences.show-tray-icon')"
                :hint="t('preferences.show-tray-icon-hint')"
              />
            </template>
            <NSwitch v-model:value="form.showTrayIcon" />
          </NFormItem>
          <NFormItem :label="t('preferences.close-action')">
            <NSelect v-model:value="closeAction" :options="closeActionOptions" class="pref-control-auto" />
          </NFormItem>
          <NFormItem v-if="isMac" :label="t('preferences.hide-dock-on-minimize')">
            <NSwitch v-model:value="form.hideDockOnMinimize" />
          </NFormItem>
          <NFormItem v-if="isMac || isLinux" :label="t('preferences.tray-speedometer')">
            <NSwitch v-model:value="form.traySpeedometer" />
          </NFormItem>
        </template>
        <template v-if="!isAndroid">
          <NFormItem :label="t('preferences.show-progress-bar')">
            <NSwitch v-model:value="form.showProgressBar" />
          </NFormItem>
          <NFormItem>
            <template #label>
              <PreferenceHintLabel :label="t('preferences.float-window')" :hint="t('preferences.float-window-hint')" />
            </template>
            <NSelect v-model:value="form.floatWindow" :options="floatWindowOptions" class="pref-control-auto" />
          </NFormItem>
          <NFormItem>
            <template #label>
              <PreferenceHintLabel
                :label="t('preferences.lightweight-mode')"
                :hint="t('preferences.lightweight-mode-hint')"
              />
            </template>
            <NSwitch v-model:value="form.lightweightMode" />
          </NFormItem>
          <NFormItem
            v-if="form.lightweightMode && form.floatWindow !== 'off'"
            :label="t('preferences.float-window-in-lightweight')"
          >
            <NSwitch v-model:value="form.floatWindowInLightweight" />
          </NFormItem>
          <PreferenceAssociations />
        </template>
      </NForm>
    </div>
    <PreferenceActionRegistration :is-dirty="isDirty" @save="handleSave" @discard="handleReset" />
    <NModal :show="languagePrompt !== null" :mask-closable="false" :closable="false" :z-index="1500">
      <section v-if="languagePrompt" class="language-dialog" role="alertdialog" aria-modal="true">
        <h2>{{ languagePrompt.title }}</h2>
        <p>{{ languagePrompt.content }}</p>
        <div class="language-dialog__actions">
          <NButton @click="dismissLanguagePrompt(false)">{{ languagePrompt.later }}</NButton>
          <NButton type="primary" @click="dismissLanguagePrompt(true)">{{ languagePrompt.restart }}</NButton>
        </div>
      </section>
    </NModal>
  </div>
</template>

<style scoped>
.language-dialog {
  display: flex;
  flex-direction: column;
  gap: 12px;
  width: min(360px, calc(100vw - 32px));
  padding: 20px;
  color: var(--m3-on-surface);
  background: var(--m3-surface-container-high);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-xl);
  box-shadow: 0 18px 60px var(--m3-shadow);
}
.language-dialog h2 {
  margin: 0;
  font-size: 1.05rem;
}
.language-dialog p {
  margin: 0;
  color: var(--m3-on-surface-variant);
  line-height: 1.5;
}
.language-dialog__actions {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  margin-top: 4px;
}
.pref-control-language {
  min-width: 260px;
}

/* ── System info version badge ─────────────────────────────────────── */
.sysinfo-ver-badge {
  display: inline-flex;
  align-items: center;
  gap: 12px;
  height: 30px;
  padding: 0 10px;
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-md);
  background: var(--about-card-bg);
  cursor: pointer;
  transition: var(--transition-all);
  font-family: var(--font-family-mono);
}
.sysinfo-ver-badge:hover {
  border-color: var(--m3-primary);
  background: var(--about-card-hover-bg);
}
.sysinfo-ver-badge:hover .sysinfo-ver-copy {
  opacity: 0.7;
}
.sysinfo-ver-badge:active {
  transform: scale(0.97);
}
.sysinfo-ver-value {
  font-size: var(--font-size-sm);
  font-weight: 520;
  color: var(--m3-on-surface);
  letter-spacing: 0.3px;
}
.sysinfo-ver-copy {
  opacity: 0.35;
  margin-left: auto;
  color: var(--m3-on-surface-variant);
  transition: var(--transition-all);
  flex-shrink: 0;
}
.sysinfo-ver-badge--muted {
  cursor: default;
}
.sysinfo-ver-badge--muted:hover {
  border-color: var(--m3-outline-variant);
  background: var(--about-card-bg);
}
.sysinfo-ver-muted {
  font-size: var(--font-size-xs);
  font-weight: 500;
  color: var(--m3-outline);
  letter-spacing: 0.3px;
}

/* ── Color Scheme Swatch Picker ───────────────────────────────────── */
.color-scheme-picker {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}
.color-swatch {
  position: relative;
  width: 26px;
  height: 26px;
  border-radius: 50%;
  border: 2px solid transparent;
  background: var(--swatch-color);
  cursor: pointer;
  transition:
    transform 0.2s cubic-bezier(0.2, 0, 0, 1),
    border-color 0.2s cubic-bezier(0.2, 0, 0, 1),
    box-shadow 0.2s cubic-bezier(0.2, 0, 0, 1);
  display: flex;
  align-items: center;
  justify-content: center;
  outline: none;
  padding: 0;
}
.color-swatch:hover {
  transform: scale(1.18);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
}
.color-swatch:active {
  transform: scale(1.05);
}
.color-swatch.active {
  border-color: var(--m3-on-surface);
  box-shadow:
    0 0 0 2px var(--swatch-color),
    0 2px 8px rgba(0, 0, 0, 0.25);
}
.custom-color-picker-wrap {
  width: 100px;
  flex: 0 0 auto;
  display: inline-block;
}
.custom-color-picker {
  width: 100%;
}
.swatch-check {
  width: 14px;
  height: 14px;
  filter: drop-shadow(0 1px 2px rgba(0, 0, 0, 0.3));
}
.pref-field-hint {
  flex-basis: 100%;
  margin-top: 6px;
  font-size: var(--font-size-xs);
  line-height: 1.5;
}
</style>
