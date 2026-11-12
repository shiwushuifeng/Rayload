<script setup lang="ts">
/** @fileoverview Three-page Android first-run wizard: personalization, engine, authorization. */
import { computed, nextTick, onMounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { NButton, NIcon, NInput, NInputNumber, NModal, NSwitch, useDialog } from 'naive-ui'
import {
  ArchiveOutline,
  BatteryHalfOutline,
  CheckmarkCircleOutline,
  CloudDownloadOutline,
  CubeOutline,
  DocumentTextOutline,
  FlashOutline,
  FolderOpenOutline,
  ImageOutline,
  LayersOutline,
  LockClosedOutline,
  MusicalNotesOutline,
  NotificationsOutline,
  ShieldCheckmarkOutline,
  SparklesOutline,
  VideocamOutline,
} from '@vicons/ionicons5'
import { useLocale } from '@/composables/useLocale'
import { usePlatform } from '@/composables/usePlatform'
import { usePreferenceStore } from '@/stores/preference'
import {
  useAndroidPermissions,
  ANDROID_PERMISSION_KEYS,
  type AndroidPermissionKey,
} from '@/composables/useAndroidPermissions'
import { BUILTIN_CATEGORY_TEMPLATES, COLOR_SCHEMES, buildDefaultCategories } from '@shared/constants'
import {
  applyArchiveCategorySettings,
  getBuiltInArchiveFolder,
  normalizeArchiveFolderName,
} from '@shared/utils/autoArchive'
import { triggerHaptic } from '@/composables/useHaptics'
import { restartAndroidApp } from '@/composables/useAndroidNative'
import { useTheme } from '@/composables/useTheme'
import { useEngineStore } from '@/stores/engine'
import { relaunch } from '@tauri-apps/plugin-process'
import { isSupportedLocale, LOCALE_CATALOG, type SupportedLocale } from '@shared/localeCatalog'
import { logger } from '@shared/logger'
import { resolveSystemLocale } from '@shared/utils/locale'
import { locale as getSystemLocale } from '@tauri-apps/plugin-os'
import brandLogoGold from '@/assets/logo.svg'

const ONBOARDING_STORAGE_KEY = 'rayload-onboarding-v3'
const { t, locale } = useI18n()
const { isAndroid } = usePlatform()
const { setLocale } = useLocale()
const preferenceStore = usePreferenceStore()
const engineStore = useEngineStore()
const dialog = useDialog()
const { applyTheme } = useTheme()
const { statuses, supported, missing, authorizing, request, requestAll, refresh } = useAndroidPermissions()

const initialTheme = preferenceStore.config.theme
const initialLocale = preferenceStore.locale || 'auto'
const visible = ref(false)
const step = ref(0)
const selectedLocale = ref(preferenceStore.locale || 'auto')
const selectedTheme = ref(preferenceStore.config.theme)
const selectedColorScheme = ref(preferenceStore.config.colorScheme)
const maxConcurrent = ref<number | null>(preferenceStore.config.maxConcurrentDownloads)
const maxConnections = ref<number | null>(preferenceStore.config.streamMaxConnections)
const bittorrentEnabled = ref(preferenceStore.config.bittorrentEnabled)
const autoArchiveEnabled = ref(preferenceStore.config.fileCategoryEnabled)
const initialCategories = preferenceStore.config.fileCategories.length
  ? preferenceStore.config.fileCategories
  : buildDefaultCategories()
const archiveEnabledLabels = ref<string[]>(
  BUILTIN_CATEGORY_TEMPLATES.map((template) => template.label).filter((label) =>
    initialCategories.some((category) => category.label === label),
  ),
)
const archiveFolders = ref<Record<string, string>>(
  Object.fromEntries(
    BUILTIN_CATEGORY_TEMPLATES.map((template) => [
      template.label,
      getBuiltInArchiveFolder(initialCategories, template.label),
    ]),
  ),
)
const editingArchiveLabel = ref<string | null>(null)
const folderDraft = ref('')
const archiveEditorRef = ref<HTMLElement | null>(null)
const saving = ref(false)
const bodyRef = ref<HTMLElement | null>(null)
let touchStart: { x: number; y: number } | null = null

function resetScroll(): void {
  void nextTick(() => bodyRef.value?.scrollTo({ top: 0 }))
}

watch(step, resetScroll)

const ENGINE_PRESETS = [
  { id: 'lite', concurrent: 3, connections: 8 },
  { id: 'balanced', concurrent: 6, connections: 16 },
  { id: 'fast', concurrent: 10, connections: 32 },
] as const

type EnginePresetId = (typeof ENGINE_PRESETS)[number]['id']

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
    ] satisfies Array<{ label: string; value: typeof selectedTheme.value }>,
)

const permissionRows = computed(() =>
  ANDROID_PERMISSION_KEYS.map((key) => ({
    key,
    icon: permissionIcon(key),
    title: t(`preferences.permission-${key}`),
  })),
)

const authorizingText = computed(() =>
  authorizing.value
    ? t('onboarding.authorizing')
    : missing.value.length
      ? t('onboarding.authorize-all')
      : t('onboarding.permissions-done'),
)

const ARCHIVE_CATEGORY_ICONS: Record<string, typeof VideocamOutline> = {
  'file-category-videos': VideocamOutline,
  'file-category-music': MusicalNotesOutline,
  'file-category-images': ImageOutline,
  'file-category-documents': DocumentTextOutline,
  'file-category-archives': ArchiveOutline,
  'file-category-programs': CubeOutline,
}

const archiveCategories = computed(() =>
  BUILTIN_CATEGORY_TEMPLATES.map((template) => ({
    key: template.label,
    icon: ARCHIVE_CATEGORY_ICONS[template.label] ?? CubeOutline,
    label: t(`preferences.${template.label}`),
    folder: archiveFolders.value[template.label] ?? template.subdirName,
    enabled: archiveEnabledLabels.value.includes(template.label),
  })),
)

const editingArchiveCategory = computed(() =>
  archiveCategories.value.find((category) => category.key === editingArchiveLabel.value),
)

const activePreset = computed<EnginePresetId | null>(
  () =>
    ENGINE_PRESETS.find(
      (preset) => preset.concurrent === maxConcurrent.value && preset.connections === maxConnections.value,
    )?.id ?? null,
)

function permissionIcon(key: AndroidPermissionKey) {
  if (key === 'storage') return FolderOpenOutline
  if (key === 'notification') return NotificationsOutline
  if (key === 'install') return CubeOutline
  return BatteryHalfOutline
}

function readOnboardingSeen(): boolean {
  try {
    return window.localStorage.getItem(ONBOARDING_STORAGE_KEY) === 'complete'
  } catch {
    return false
  }
}

function markOnboardingSeen(): void {
  try {
    window.localStorage.setItem(ONBOARDING_STORAGE_KEY, 'complete')
  } catch {
    // Hardened WebViews can reject storage; the wizard still works for this session.
  }
}

function clampInteger(value: number | null, fallback: number, min: number, max: number): number {
  const numeric = Number(value ?? fallback)
  if (!Number.isFinite(numeric)) return fallback
  return Math.min(max, Math.max(min, Math.round(numeric)))
}

async function applyLocale(): Promise<void> {
  if (selectedLocale.value === 'auto') {
    let target: SupportedLocale = isSupportedLocale(locale.value) ? locale.value : 'en-US'
    try {
      target = resolveSystemLocale((await getSystemLocale()) || 'en-US')
    } catch (error) {
      logger.debug('Onboarding.systemLocale', error)
    }
    await setLocale(target)
    return
  }
  if (isSupportedLocale(selectedLocale.value)) await setLocale(selectedLocale.value)
}

async function finish(): Promise<void> {
  if (saving.value) return
  saving.value = true
  try {
    const baseCategories = preferenceStore.config.fileCategories.length
      ? preferenceStore.config.fileCategories
      : buildDefaultCategories()
    const categories = applyArchiveCategorySettings(baseCategories, {
      enabledLabels: archiveEnabledLabels.value,
      folders: archiveFolders.value,
    })
    await preferenceStore.updateAndSave({
      locale: selectedLocale.value,
      theme: selectedTheme.value,
      colorScheme: selectedColorScheme.value,
      maxConcurrentDownloads: clampInteger(maxConcurrent.value, 6, 1, 100),
      streamMaxConnections: clampInteger(maxConnections.value, 16, 1, 256),
      bittorrentEnabled: bittorrentEnabled.value,
      fileCategoryEnabled: autoArchiveEnabled.value,
      fileCategories: categories,
    })
    await applyLocale()
  } catch (error) {
    logger.warn('Onboarding.save', error)
  } finally {
    saving.value = false
    markOnboardingSeen()
    visible.value = false
  }
  if (selectedLocale.value !== initialLocale) {
    window.setTimeout(() => showLanguageRestartDialog(), 260)
  }
}

function showLanguageRestartDialog(): void {
  dialog.info({
    title: t('preferences.language-changed-title'),
    content: t('preferences.language-changed-content'),
    positiveText: t('preferences.language-changed-restart'),
    negativeText: t('preferences.language-changed-later'),
    zIndex: 12000,
    onPositiveClick: async () => {
      await engineStore.stop('appRelaunch')
      if (isAndroid.value) restartAndroidApp()
      else await relaunch()
    },
  })
}

function dismissForNow(): void {
  restorePreviewTheme()
  visible.value = false
}

function skipAll(): void {
  restorePreviewTheme()
  markOnboardingSeen()
  visible.value = false
}

function previewTheme(value: typeof selectedTheme.value): void {
  selectedTheme.value = value
  applyTheme(value)
}

function restorePreviewTheme(): void {
  applyTheme(initialTheme)
}

function toggleArchiveCategory(label: string): void {
  if (archiveEnabledLabels.value.includes(label)) {
    archiveEnabledLabels.value = archiveEnabledLabels.value.filter((item) => item !== label)
  } else {
    archiveEnabledLabels.value = [...archiveEnabledLabels.value, label]
  }
  if (editingArchiveLabel.value === label && !archiveEnabledLabels.value.includes(label)) {
    editingArchiveLabel.value = null
  }
}

let archiveLongPressTimer: number | null = null
let archiveLongPressFired = false

function clearArchiveLongPress(): void {
  if (archiveLongPressTimer !== null) {
    window.clearTimeout(archiveLongPressTimer)
    archiveLongPressTimer = null
  }
}

function startArchiveLongPress(label: string): void {
  archiveLongPressFired = false
  clearArchiveLongPress()
  archiveLongPressTimer = window.setTimeout(() => {
    archiveLongPressTimer = null
    archiveLongPressFired = true
    triggerHaptic()
    openArchiveFolderEditor(label)
  }, 480)
}

function handleArchiveClick(label: string): void {
  if (archiveLongPressFired) {
    archiveLongPressFired = false
    return
  }
  toggleArchiveCategory(label)
}

function openArchiveFolderEditor(label: string): void {
  const template = BUILTIN_CATEGORY_TEMPLATES.find((item) => item.label === label)
  if (!template) return
  editingArchiveLabel.value = label
  folderDraft.value = archiveFolders.value[label] ?? template.subdirName
  void nextTick(() => archiveEditorRef.value?.scrollIntoView({ block: 'nearest', behavior: 'smooth' }))
}

function closeArchiveFolderEditor(): void {
  editingArchiveLabel.value = null
}

function commitArchiveFolder(): void {
  const label = editingArchiveLabel.value
  if (!label) return
  const template = BUILTIN_CATEGORY_TEMPLATES.find((item) => item.label === label)
  folderDraft.value = normalizeArchiveFolderName(folderDraft.value, template?.subdirName)
  archiveFolders.value = { ...archiveFolders.value, [label]: folderDraft.value }
  editingArchiveLabel.value = null
}

function applyPreset(id: EnginePresetId): void {
  const preset = ENGINE_PRESETS.find((item) => item.id === id)
  if (!preset) return
  maxConcurrent.value = preset.concurrent
  maxConnections.value = preset.connections
}

function next(): void {
  if (step.value < 3) step.value += 1
}

function back(): void {
  if (step.value > 0) step.value -= 1
}

function onTouchStart(event: TouchEvent): void {
  const point = event.changedTouches[0]
  touchStart = point ? { x: point.clientX, y: point.clientY } : null
}

function onTouchEnd(event: TouchEvent): void {
  const start = touchStart
  touchStart = null
  const point = event.changedTouches[0]
  if (!start || !point) return
  const dx = point.clientX - start.x
  const dy = point.clientY - start.y
  if (Math.abs(dx) < 56 || Math.abs(dx) <= Math.abs(dy)) return
  if (dx < 0) next()
  else back()
}

onMounted(() => {
  if (!isAndroid.value) return
  if (readOnboardingSeen()) return
  window.setTimeout(() => {
    refresh()
    visible.value = true
    resetScroll()
  }, 350)
})
</script>

<template>
  <NModal
    :show="visible"
    :mask-closable="false"
    :close-on-esc="false"
    :block-scroll="true"
    :z-index="1500"
    transform-origin="center"
  >
    <section
      class="onboarding"
      role="dialog"
      aria-modal="true"
      :aria-label="t('onboarding.title')"
      @touchstart.passive="onTouchStart"
      @touchend.passive="onTouchEnd"
    >
      <div class="onboarding__accent" />
      <div class="onboarding__topbar">
        <div class="onboarding__progress" aria-hidden="true">
          <span v-for="index in 4" :key="index" :class="{ active: index - 1 <= step }" />
        </div>
        <button type="button" class="onboarding__skip" @click="skipAll">{{ t('onboarding.skip') }}</button>
      </div>

      <div ref="bodyRef" class="onboarding__body">
        <Transition name="onboarding-page" mode="out-in">
          <div :key="step" class="onboarding__page" :class="{ 'onboarding__page--welcome': step === 0 }">
            <template v-if="step === 0">
              <div class="onboarding__hero">
                <div class="onboarding__icon-shell">
                  <img :src="brandLogoGold" alt="" width="56" height="56" class="onboarding__logo" />
                </div>
                <div class="onboarding__heading">
                  <span class="onboarding__eyebrow"
                    ><NIcon :size="14"><SparklesOutline /></NIcon> Rayload</span
                  >
                  <h2>{{ t('onboarding.welcome-title') }}</h2>
                  <p>{{ t('onboarding.welcome-description') }}</p>
                </div>
              </div>
              <div class="onboarding__features">
                <div class="onboarding__feature">
                  <NIcon :size="18"><CloudDownloadOutline /></NIcon>
                  <span>{{ t('onboarding.feature-multi-protocol') }}</span>
                </div>
                <div class="onboarding__feature">
                  <NIcon :size="18"><LayersOutline /></NIcon>
                  <span>{{ t('onboarding.feature-batch') }}</span>
                </div>
                <div class="onboarding__feature">
                  <NIcon :size="18"><FlashOutline /></NIcon>
                  <span>{{ t('onboarding.feature-background') }}</span>
                </div>
                <div class="onboarding__feature">
                  <NIcon :size="18"><LockClosedOutline /></NIcon>
                  <span>{{ t('onboarding.feature-private') }}</span>
                </div>
              </div>
            </template>

            <template v-else-if="step === 1">
              <div class="onboarding__heading">
                <h2>{{ t('onboarding.personalize-title') }}</h2>
                <p>{{ t('onboarding.personalize-description') }}</p>
              </div>
              <div class="onboarding__card onboarding__card--wide">
                <label class="onboarding__field">
                  <span>{{ t('preferences.language') }}</span>
                  <select v-model="selectedLocale" class="onboarding__select">
                    <option v-for="option in localeOptions" :key="option.value" :value="option.value">
                      {{ option.label }}
                    </option>
                  </select>
                </label>
                <div class="onboarding__field">
                  <span>{{ t('preferences.appearance') }}</span>
                  <div class="onboarding__segmented" role="radiogroup" :aria-label="t('preferences.appearance')">
                    <button
                      v-for="option in themeOptions"
                      :key="option.value"
                      type="button"
                      :class="{ active: selectedTheme === option.value }"
                      role="radio"
                      :aria-checked="selectedTheme === option.value"
                      @click="previewTheme(option.value)"
                    >
                      {{ option.label }}
                    </button>
                  </div>
                </div>
                <div class="onboarding__field">
                  <span>{{ t('preferences.color-scheme') }}</span>
                  <div class="onboarding__swatches" role="radiogroup" :aria-label="t('preferences.color-scheme')">
                    <button
                      v-for="scheme in COLOR_SCHEMES"
                      :key="scheme.id"
                      type="button"
                      class="onboarding__swatch"
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
              </div>
            </template>

            <template v-else-if="step === 2">
              <div class="onboarding__heading">
                <h2>{{ t('onboarding.engine-title') }}</h2>
                <p>{{ t('onboarding.engine-description') }}</p>
              </div>
              <div class="onboarding__card">
                <span class="onboarding__section-label">{{ t('onboarding.engine-section') }}</span>
                <div class="onboarding__preset-row">
                  <span>{{ t('onboarding.engine-preset') }}</span>
                  <div class="onboarding__preset-options">
                    <button
                      v-for="preset in ENGINE_PRESETS"
                      :key="preset.id"
                      type="button"
                      :class="{ active: activePreset === preset.id }"
                      @click="applyPreset(preset.id)"
                    >
                      {{ t(`onboarding.preset-${preset.id}`) }}
                    </button>
                  </div>
                </div>
                <div class="onboarding__engine-grid">
                  <label class="onboarding__field">
                    <span>{{ t('onboarding.engine-concurrent') }}</span>
                    <NInputNumber v-model:value="maxConcurrent" :min="1" :max="100" :show-button="false" />
                    <small>{{ t('onboarding.engine-concurrent-hint') }}</small>
                  </label>
                  <label class="onboarding__field">
                    <span>{{ t('onboarding.engine-connections') }}</span>
                    <NInputNumber v-model:value="maxConnections" :min="1" :max="256" :show-button="false" />
                    <small>{{ t('onboarding.engine-connections-hint') }}</small>
                  </label>
                </div>
                <div class="onboarding__row">
                  <div class="onboarding__row-copy">
                    <strong>{{ t('onboarding.engine-bittorrent') }}</strong>
                    <span>{{ t('onboarding.engine-bittorrent-hint') }}</span>
                  </div>
                  <NSwitch v-model:value="bittorrentEnabled" />
                </div>
              </div>
              <div class="onboarding__card">
                <span class="onboarding__section-label">{{ t('onboarding.archive-section') }}</span>
                <div class="onboarding__row">
                  <div class="onboarding__row-copy">
                    <strong>{{ t('onboarding.auto-archive') }}</strong>
                    <span>{{ t('onboarding.auto-archive-hint') }}</span>
                  </div>
                  <NSwitch v-model:value="autoArchiveEnabled" />
                </div>
                <template v-if="autoArchiveEnabled">
                  <div class="onboarding__archive-grid">
                    <button
                      v-for="category in archiveCategories"
                      :key="category.key"
                      type="button"
                      class="onboarding__archive-item"
                      :class="{ active: category.enabled }"
                      :aria-pressed="category.enabled"
                      :aria-label="category.label"
                      @pointerdown="startArchiveLongPress(category.key)"
                      @pointerup="clearArchiveLongPress"
                      @pointerleave="clearArchiveLongPress"
                      @pointercancel="clearArchiveLongPress"
                      @click="handleArchiveClick(category.key)"
                    >
                      <NIcon :size="18"><component :is="category.icon" /></NIcon>
                      <span class="onboarding__archive-name">{{ category.label }}</span>
                      <span class="onboarding__archive-folder">{{ category.folder }}</span>
                      <NIcon v-if="category.enabled" :size="14" class="onboarding__archive-check">
                        <CheckmarkCircleOutline />
                      </NIcon>
                    </button>
                  </div>
                  <div v-if="editingArchiveCategory" ref="archiveEditorRef" class="onboarding__archive-editor">
                    <span class="onboarding__section-label">
                      {{ t('onboarding.archive-folder-title') }} · {{ editingArchiveCategory.label }}
                    </span>
                    <div class="onboarding__archive-editor-row">
                      <NInput
                        v-model:value="folderDraft"
                        size="small"
                        :placeholder="editingArchiveCategory.folder"
                        @keyup.enter="commitArchiveFolder"
                      />
                      <NButton size="small" @click="closeArchiveFolderEditor">{{ t('app.cancel') }}</NButton>
                      <NButton size="small" type="primary" @click="commitArchiveFolder">{{ t('app.save') }}</NButton>
                    </div>
                  </div>
                  <p class="onboarding__archive-hint">{{ t('onboarding.archive-category-hint') }}</p>
                </template>
              </div>
            </template>

            <template v-else>
              <div class="onboarding__heading">
                <h2>{{ t('onboarding.auth-title') }}</h2>
                <p>{{ t('onboarding.auth-description') }}</p>
              </div>
              <div class="onboarding__permissions">
                <div
                  v-for="row in permissionRows"
                  :key="row.key"
                  class="onboarding__permission"
                  :class="{ 'onboarding__permission--notice': row.key === 'clipboard' }"
                >
                  <div class="onboarding__permission-icon">
                    <NIcon :size="20"><component :is="row.icon" /></NIcon>
                  </div>
                  <div class="onboarding__permission-copy">
                    <strong>{{ row.title }}</strong>
                    <span>{{ t(`preferences.permission-${row.key}-hint`) }}</span>
                  </div>
                  <NIcon v-if="statuses[row.key] === 'granted'" :size="20" class="onboarding__granted">
                    <CheckmarkCircleOutline />
                  </NIcon>
                  <NButton
                    v-else
                    size="small"
                    class="onboarding__permission-button"
                    :disabled="statuses[row.key] === 'unavailable'"
                    @click="request(row.key)"
                  >
                    {{ t('onboarding.allow') }}
                  </NButton>
                </div>
              </div>
              <NButton
                block
                type="primary"
                :loading="authorizing"
                :disabled="!supported || missing.length === 0"
                @click="requestAll"
              >
                {{ authorizingText }}
              </NButton>
              <div class="onboarding__note">
                <NIcon :size="18"><ShieldCheckmarkOutline /></NIcon>
                <span>{{ t('onboarding.privacy-note') }}</span>
              </div>
            </template>
          </div>
        </Transition>
      </div>

      <div class="onboarding__actions">
        <NButton v-if="step === 0" quaternary @click="dismissForNow">{{ t('onboarding.later') }}</NButton>
        <NButton v-else quaternary @click="back">{{ t('onboarding.back') }}</NButton>
        <span class="onboarding__spacer" />
        <NButton v-if="step < 3" type="primary" @click="next">{{ t('onboarding.next') }}</NButton>
        <NButton v-else type="primary" :loading="saving" @click="finish">{{ t('onboarding.finish') }}</NButton>
      </div>
    </section>
  </NModal>
</template>

<style scoped>
.onboarding {
  position: relative;
  display: flex;
  flex-direction: column;
  width: min(460px, calc(100vw - 28px));
  max-height: calc(100dvh - 16px - env(safe-area-inset-bottom, 0px));
  padding: clamp(16px, 4.4vw, 24px);
  overflow: hidden;
  color: var(--m3-on-surface);
  background: var(--m3-surface-container-high);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-xl);
  box-shadow: 0 18px 60px var(--m3-shadow);
}
.onboarding__body {
  flex: 1;
  min-height: 0;
  margin-inline: -4px;
  padding-inline: 4px;
  overflow-y: auto;
  overscroll-behavior: contain;
  scrollbar-width: thin;
}
.onboarding__accent {
  position: absolute;
  inset: 0 0 auto;
  height: 4px;
  background: linear-gradient(90deg, var(--m3-primary), var(--m3-tertiary));
}
.onboarding__topbar {
  position: relative;
  z-index: 2;
  display: grid;
  flex-shrink: 0;
  grid-template-columns: minmax(0, 1fr) auto;
  gap: 12px;
  align-items: center;
  padding-bottom: 8px;
  margin-bottom: 10px;
  background: var(--m3-surface-container-high);
}
.onboarding__progress {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 6px;
}
.onboarding__progress span {
  height: 4px;
  background: var(--m3-surface-container-highest);
  border-radius: var(--radius-full);
}
.onboarding__progress span.active {
  background: var(--m3-primary);
}
.onboarding__skip {
  min-height: 44px;
  padding: 0 10px;
  color: var(--m3-on-surface-variant);
  background: transparent;
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-full);
  font: inherit;
  font-size: var(--font-size-xs);
  cursor: pointer;
}
.onboarding__skip:hover {
  color: var(--m3-on-surface);
  background: var(--m3-surface-container-highest);
}
.onboarding__page {
  display: grid;
  gap: 12px;
}
.onboarding__page--welcome {
  justify-items: center;
  text-align: center;
}
.onboarding__hero {
  display: grid;
  gap: 10px;
  justify-items: center;
}
.onboarding__icon-shell {
  display: grid;
  width: 84px;
  height: 84px;
  place-items: center;
  background:
    radial-gradient(circle at 30% 20%, color-mix(in srgb, var(--m3-primary) 34%, transparent), transparent 62%),
    var(--m3-surface-container);
  border: 1px solid var(--m3-outline-variant);
  border-radius: 24px;
  box-shadow: 0 14px 32px color-mix(in srgb, var(--m3-shadow) 55%, transparent);
}
.onboarding__logo {
  display: block;
  border-radius: 20px;
}
.onboarding__heading h2 {
  margin: 0 0 5px;
  font-size: clamp(19px, 5.2vw, 23px);
  font-weight: 650;
}
.onboarding__heading p {
  margin: 0;
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-sm);
  line-height: 1.5;
}
.onboarding__eyebrow {
  display: inline-flex;
  gap: 6px;
  align-items: center;
  margin-bottom: 4px;
  color: var(--m3-primary);
  font-size: var(--font-size-sm);
  font-weight: 650;
}
.onboarding__features {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 6px;
  width: 100%;
  text-align: left;
}
.onboarding__feature {
  display: flex;
  gap: 8px;
  align-items: center;
  min-height: 38px;
  padding: 6px 8px;
  color: var(--m3-on-surface-variant);
  background: var(--m3-surface-container);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-md);
  font-size: var(--font-size-xs);
  line-height: 1.35;
}
.onboarding__feature svg {
  flex: none;
  color: var(--m3-primary);
}
.onboarding__field {
  display: grid;
  gap: 6px;
  width: 100%;
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-sm);
}
.onboarding__field small {
  color: var(--m3-on-surface-variant);
  line-height: 1.5;
}
.onboarding__select {
  width: 100%;
  min-height: 44px;
  padding: 0 38px 0 14px;
  color: var(--m3-on-surface);
  background-color: var(--m3-surface-container-highest);
  border: 1px solid var(--m3-outline);
  border-radius: var(--radius-md);
  font: inherit;
}
.onboarding__select option {
  color: var(--m3-on-surface);
  background: var(--m3-surface-container-high);
}
.onboarding__card {
  display: grid;
  gap: 12px;
  width: 100%;
  padding: 12px;
  text-align: left;
  background: var(--m3-surface-container);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-lg);
}
.onboarding__card--wide {
  text-align: left;
}
.onboarding__section-heading {
  display: grid;
  gap: 4px;
}
.onboarding__section-heading span,
.onboarding__row-copy span,
.onboarding__note,
.onboarding__permission-copy span {
  color: var(--m3-on-surface-variant);
  line-height: 1.5;
}
.onboarding__section-label {
  color: var(--m3-primary);
  font-size: var(--font-size-xs);
  font-weight: 700;
  letter-spacing: 0.04em;
  text-transform: uppercase;
}
.onboarding__segmented {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 7px;
}
.onboarding__segmented button {
  min-height: 44px;
  padding: 6px 8px;
  color: var(--m3-on-surface-variant);
  background: var(--m3-surface-container-highest);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-md);
  font: inherit;
  cursor: pointer;
}
.onboarding__segmented button.active {
  color: var(--m3-on-primary-container);
  background: var(--m3-primary-container);
  border-color: var(--m3-primary);
  font-weight: 650;
}
.onboarding__swatches {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}
.onboarding__swatch {
  display: grid;
  width: 44px;
  height: 44px;
  padding: 0;
  place-items: center;
  background: transparent;
  border: 2px solid transparent;
  border-radius: 50%;
  cursor: pointer;
}
.onboarding__swatch > span {
  display: block;
  width: 28px;
  height: 28px;
  background: var(--swatch-color);
  border-radius: 50%;
  box-shadow: inset 0 0 0 1px color-mix(in srgb, var(--m3-on-surface) 18%, transparent);
}
.onboarding__swatch.active {
  border-color: var(--m3-primary);
}
.onboarding__engine-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 14px;
}
.onboarding__preset-row {
  display: grid;
  gap: 8px;
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-sm);
}
.onboarding__preset-options {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 7px;
}
.onboarding__preset-options button {
  min-height: 44px;
  padding: 6px 8px;
  color: var(--m3-on-surface-variant);
  background: var(--m3-surface-container-highest);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-md);
  font: inherit;
  cursor: pointer;
}
.onboarding__preset-options button.active {
  color: var(--m3-on-primary-container);
  background: var(--m3-primary-container);
  border-color: var(--m3-primary);
  font-weight: 650;
}
.onboarding__row {
  display: flex;
  gap: 16px;
  align-items: center;
  justify-content: space-between;
}
.onboarding__row-copy {
  display: grid;
  gap: 4px;
}
.onboarding__note {
  display: flex;
  gap: 9px;
  align-items: flex-start;
  padding: 9px 12px;
  background: color-mix(in srgb, var(--m3-primary) 8%, var(--m3-surface-container));
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-lg);
}
.onboarding__permissions {
  display: grid;
  gap: 7px;
}
.onboarding__permission {
  display: grid;
  grid-template-columns: 34px minmax(0, 1fr) auto;
  gap: 9px;
  align-items: center;
  padding: 9px;
  background: var(--m3-surface-container);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-lg);
}
.onboarding__permission-icon {
  display: grid;
  width: 34px;
  height: 34px;
  place-items: center;
  color: var(--m3-on-primary-container);
  background: var(--m3-primary-container);
  border-radius: 10px;
}
.onboarding__permission-copy {
  display: grid;
  gap: 2px;
  min-width: 0;
  font-size: var(--font-size-sm);
}
.onboarding__permission-button {
  min-height: 44px;
}
.onboarding__granted {
  color: var(--m3-success);
}
.onboarding__archive-grid {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 6px;
}
.onboarding__archive-item {
  position: relative;
  display: flex;
  flex-wrap: wrap;
  gap: 4px 6px;
  align-items: center;
  min-height: 44px;
  padding: 4px 8px;
  color: var(--m3-on-surface-variant);
  text-align: left;
  cursor: pointer;
  user-select: none;
  -webkit-touch-callout: none;
  background: var(--m3-surface-container-low);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-sm);
  font-size: var(--font-size-sm);
  transition:
    color 0.18s ease,
    border-color 0.18s ease,
    background-color 0.18s ease;
}
.onboarding__archive-item.active {
  color: var(--m3-on-primary-container);
  background: var(--m3-primary-container);
  border-color: var(--m3-primary);
}
.onboarding__archive-name {
  font-weight: 500;
}
.onboarding__archive-folder {
  flex-basis: 100%;
  overflow: hidden;
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-xs);
  text-overflow: ellipsis;
  white-space: nowrap;
  opacity: 0.75;
}
.onboarding__archive-item.active .onboarding__archive-folder {
  color: var(--m3-on-primary-container);
  opacity: 0.85;
}
.onboarding__archive-check {
  margin-left: auto;
  color: var(--m3-primary);
}
.onboarding__archive-editor {
  display: grid;
  gap: 8px;
  padding: 10px 12px;
  margin-top: 10px;
  background: var(--m3-surface-container-low);
  border: 1px solid var(--m3-primary);
  border-radius: var(--radius-sm);
}
.onboarding__archive-editor-row {
  display: flex;
  gap: 8px;
  align-items: center;
}
.onboarding__archive-editor-row :deep(.n-input) {
  flex: 1;
  min-width: 0;
}
.onboarding__archive-hint {
  margin: 8px 0 0;
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-xs);
  line-height: 1.5;
}
.onboarding__actions {
  display: flex;
  flex-shrink: 0;
  gap: 8px;
  align-items: center;
  margin-top: 8px;
  padding-top: 8px;
  padding-bottom: 10px;
  background: var(--m3-surface-container-high);
}
.onboarding__actions :deep(.n-button) {
  min-height: 44px;
}
.onboarding__spacer {
  flex: 1;
}
.onboarding-page-enter-active,
.onboarding-page-leave-active {
  transition:
    opacity 180ms cubic-bezier(0.2, 0, 0, 1),
    transform 180ms cubic-bezier(0.2, 0, 0, 1);
}
.onboarding-page-enter-from {
  opacity: 0;
  transform: translateX(16px);
}
.onboarding-page-leave-to {
  opacity: 0;
  transform: translateX(-12px);
}
@media (max-width: 460px) {
  .onboarding__field small {
    display: none;
  }
  .onboarding__permission {
    grid-template-columns: 30px minmax(0, 1fr) auto;
    gap: 8px;
    padding: 6px 8px;
  }
  .onboarding__permission-icon {
    width: 30px;
    height: 30px;
    border-radius: 9px;
  }
  .onboarding__permission-copy {
    font-size: var(--font-size-xs);
  }
  .onboarding__permission:not(.onboarding__permission--notice) .onboarding__permission-copy span {
    display: none;
  }
  .onboarding__permission--notice .onboarding__permission-copy span {
    display: -webkit-box;
    overflow: hidden;
    -webkit-box-orient: vertical;
    -webkit-line-clamp: 2;
  }
}
@media (max-height: 780px) {
  .onboarding__icon-shell {
    width: 68px;
    height: 68px;
    border-radius: 20px;
  }
  .onboarding__logo {
    width: 48px;
    height: 48px;
  }
  .onboarding__feature {
    min-height: 34px;
  }
  .onboarding__card {
    gap: 10px;
  }
  .onboarding__note {
    padding: 7px 10px;
  }
}
@media (max-width: 420px) {
  .onboarding__engine-grid {
    grid-template-columns: 1fr;
  }
  .onboarding__archive-grid {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
}
</style>
