<script setup lang="ts">
/** @fileoverview About panel with staggered entrance animations and glass effect. */
import { computed, ref, onMounted, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { NButton, NIcon, NModal } from 'naive-ui'
import MTooltip from '@/components/common/MTooltip.vue'
import {
  CloudDownloadOutline,
  DownloadOutline,
  LogoGithub,
  HeartOutline,
  GlobeOutline,
  RocketOutline,
} from '@vicons/ionicons5'
import { useBrandLogo } from '@/composables/useBrandLogo'
import { openUrl as openExternalUrl } from '@tauri-apps/plugin-opener'
import { getVersion } from '@tauri-apps/api/app'
import { arch } from '@tauri-apps/plugin-os'
import { usePlatform } from '@/composables/usePlatform'
import { usePreferenceStore } from '@/stores/preference'
import { exportDiagnosticLogs } from '@/composables/useDiagnosticLogExport'
import { useAppMessage } from '@/composables/useAppMessage'
import { logger } from '@shared/logger'
import { writeAppClipboardText } from '@shared/utils'
import { checkGithubUpdate } from '@/api/githubUpdate'
import { useBuildInfoStore } from '@/stores/build'
import { getErrorMessage } from '@shared/utils/errorMessage'
import { friendlyErrorKind } from '@shared/utils/friendlyError'
import { resolveAppProxyUrl } from '@shared/utils/proxy'
import { PROXY_SCOPES } from '@shared/constants'

const props = defineProps<{ show: boolean }>()
const emit = defineEmits<{ close: [] }>()

const { t } = useI18n()
const { brandLogo } = useBrandLogo()
const message = useAppMessage()
const preferenceStore = usePreferenceStore()
const { archLabel, platformLabel } = usePlatform()
const appVersion = ref('')
const archTag = ref('')
const exportingLogs = ref(false)
const checkingUpdate = ref(false)
const animate = ref(false)

onMounted(async () => {
  appVersion.value = await getVersion()
  try {
    const raw = arch()
    archTag.value = raw === 'aarch64' ? 'arm64' : archLabel(raw).toLowerCase()
  } catch (error) {
    logger.debug('AboutPanel.arch', `arch() unavailable: ${error}`)
  }
})

const versionLabel = computed(() => `v${appVersion.value}${archTag.value ? `-${archTag.value}` : ''}`)

async function exportLogs() {
  if (exportingLogs.value) return
  exportingLogs.value = true
  try {
    await exportDiagnosticLogs({
      t,
      message,
      configuredDir: preferenceStore.config.dir,
    })
  } finally {
    exportingLogs.value = false
  }
}

/* Replay the staggered entrance animation whenever the panel opens. */
watch(
  () => props.show,
  (visible) => {
    if (!visible) return
    animate.value = false
    requestAnimationFrame(() => {
      animate.value = true
    })
  },
)

const techStack = [
  {
    name: 'Tauri 2',
    color: '#D49700',
    /* Lucide app-window — desktop application framework */
    svg: '<rect x="2" y="4" width="20" height="16" rx="2"/><line x1="8" y1="4" x2="8" y2="20"/>',
  },
  {
    name: 'Vue 3',
    color: '#2E8B57',
    /* Lucide layers — layered composition framework */
    svg: '<path d="m12.83 2.18a2 2 0 0 0-1.66 0L2.6 6.08a1 1 0 0 0 0 1.83l8.58 3.91a2 2 0 0 0 1.66 0l8.58-3.9a1 1 0 0 0 0-1.84Z"/><path d="m22 17.65-9.17 4.16a2 2 0 0 1-1.66 0L2 17.65"/><path d="m22 12.65-9.17 4.16a2 2 0 0 1-1.66 0L2 12.65"/>',
  },
  {
    name: 'Rust',
    color: '#DE5623',
    /* Lucide shield — memory safety */
    svg: '<path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 .67-.95l7-2.5a1 1 0 0 1 .67 0l7 2.5A1 1 0 0 1 20 6z"/>',
  },
  {
    name: 'Naive UI',
    color: '#2A9D6E',
    /* Lucide leaf — nature/green brand identity */
    svg: '<path d="M11 20A7 7 0 0 1 9.8 6.9C15.5 4.9 20 .5 20 .5s-4.5 4.5-2.5 10.2A7 7 0 0 1 11 20"/><path d="M2 21c0-3 1.85-5.36 5.08-6C9.5 14.52 12 13 13 12"/>',
  },
]

const links = computed(() => [
  {
    key: 'github',
    label: 'GitHub',
    icon: LogoGithub,
    url: 'https://github.com/shiwushuifeng/',
  },
  {
    key: 'repository',
    label: t('about.repository'),
    icon: GlobeOutline,
    url: 'https://github.com/shiwushuifeng/Rayload',
  },
  {
    key: 'release',
    label: t('about.release'),
    icon: RocketOutline,
    url: 'https://github.com/shiwushuifeng/Rayload/releases',
  },
  {
    key: 'support',
    label: t('about.support'),
    icon: HeartOutline,
    url: 'https://github.com/shiwushuifeng/Rayload/issues',
  },
])

async function copyToClipboard(text: string, label: string) {
  try {
    await writeAppClipboardText(text)
    message.success(t('about.version-copied', { label }))
  } catch (e) {
    logger.debug('AboutPanel.clipboard', `writeText failed: ${e}`)
  }
}

function openUrl(url: string) {
  openExternalUrl(url)
}

async function checkUpdates() {
  const buildInfo = useBuildInfoStore()
  await buildInfo.load()
  if (buildInfo.info.fdroid) {
    message.info(t('preferences.fdroid-update-hint'))
    return
  }
  if (checkingUpdate.value) return
  checkingUpdate.value = true
  try {
    const version = appVersion.value || (await getVersion())
    const proxy = resolveAppProxyUrl(preferenceStore.config.proxy, PROXY_SCOPES.UPDATE_APP)
    const result = await checkGithubUpdate(version, preferenceStore.config.updateStableOnly, proxy)
    await preferenceStore.updateAndSave({ lastCheckUpdateTime: Date.now() })
    if (result.updateAvailable) {
      message.info(t('preferences.update-available', { version: result.version }))
      if (result.releaseUrl) openUrl(result.releaseUrl)
    } else {
      message.success(t('preferences.update-up-to-date'))
    }
  } catch (error) {
    const raw = getErrorMessage(error)
    logger.warn('AboutPanel.checkUpdates', raw)
    message.error(
      friendlyErrorKind(raw) === 'network'
        ? t('preferences.update-network-error')
        : t('preferences.update-check-failed'),
    )
  } finally {
    checkingUpdate.value = false
  }
}
</script>

<template>
  <NModal
    :show="show"
    transform-origin="center"
    @update:show="
      (v: boolean) => {
        if (!v) emit('close')
      }
    "
  >
    <div class="about-glass" :class="{ 'about-enter': animate }">
      <!-- Close button -->
      <button class="about-close" :aria-label="t('app.close')" @click="emit('close')">
        <svg width="14" height="14" viewBox="0 0 14 14" fill="none">
          <path d="M1 1L13 13M13 1L1 13" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
        </svg>
      </button>

      <!-- Logo -->
      <div class="about-logo stagger stagger-1">
        <img :src="brandLogo" alt="Rayload" width="96" height="96" />
      </div>

      <!-- Title -->
      <div class="about-title stagger stagger-2">Rayload</div>

      <!-- Version and platform detection -->
      <div class="about-versions stagger stagger-2">
        <MTooltip>
          <template #trigger>
            <button
              class="version-badge"
              @click="copyToClipboard(`Rayload ${versionLabel} · ${platformLabel}`, 'Rayload')"
            >
              <span class="version-label">{{ t('about.app-version') }}</span>
              <span class="version-value">{{ versionLabel }}</span>
              <svg class="copy-icon" width="14" height="14" viewBox="0 0 24 24" fill="none">
                <rect x="9" y="9" width="13" height="13" rx="2" stroke="currentColor" stroke-width="2" />
                <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1" stroke="currentColor" stroke-width="2" />
              </svg>
            </button>
          </template>
          {{ t('about.click-to-copy') }}
        </MTooltip>
        <div class="about-platform">
          <span class="about-platform__tag">{{ platformLabel }}</span>
          <span v-if="archTag" class="about-platform__tag about-platform__tag--arch">{{ archTag }}</span>
        </div>
      </div>

      <!-- Slogan -->
      <p class="about-desc stagger stagger-3">{{ t('about.tagline') }}</p>

      <!-- Tech Stack -->
      <div class="about-section-label stagger stagger-4">{{ t('about.tech-stack') }}</div>
      <div class="about-tags stagger stagger-4">
        <span v-for="tech in techStack" :key="tech.name" class="about-tag" :style="{ '--tag-color': tech.color }">
          <!-- eslint-disable vue/no-v-html -- tech.svg is static local icon markup -->
          <svg
            class="about-tag-icon"
            width="12"
            height="12"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            v-html="tech.svg"
          />
          <!-- eslint-enable vue/no-v-html -->
          {{ tech.name }}
        </span>
      </div>

      <!-- Links Grid -->
      <div class="about-links stagger stagger-5">
        <a
          v-for="link in links"
          :key="link.key"
          class="about-link-card"
          :href="link.url"
          @click.prevent="openUrl(link.url)"
        >
          <NIcon :size="18" aria-hidden="true"><component :is="link.icon" /></NIcon>
          <span>{{ link.label }}</span>
        </a>
      </div>

      <div class="about-actions stagger stagger-6">
        <NButton size="small" :loading="checkingUpdate" @click="checkUpdates">
          <template #icon>
            <NIcon :size="16"><CloudDownloadOutline /></NIcon>
          </template>
          {{ t('app.check-updates-now') }}
        </NButton>
        <NButton size="small" :loading="exportingLogs" @click="exportLogs">
          <template #icon>
            <NIcon :size="16"><DownloadOutline /></NIcon>
          </template>
          {{ t('preferences.export-diagnostic-logs') }}
        </NButton>
      </div>

      <!-- Footer -->
      <div class="about-footer stagger stagger-6">
        <i18n-t keypath="about.developed-by" tag="div" scope="global" class="about-maintainer">
          <template #author>
            <a class="about-link" @click="openUrl('https://github.com/shiwushuifeng/')">shiwushuifeng</a>
          </template>
        </i18n-t>
        <i18n-t keypath="about.open-source-acknowledgement" tag="p" scope="global">
          <template #label>
            <strong>{{ t('about.open-source-acknowledgement-label') }}</strong>
          </template>
        </i18n-t>
        <p class="about-disclaimer">{{ t('about.disclaimer') }}</p>
      </div>
    </div>
  </NModal>
</template>

<style scoped>
/* ── Glass Container ──────────────────────────────────────────────── */
.about-glass {
  position: relative;
  width: min(440px, calc(100vw - 32px));
  min-width: 0;
  max-height: calc(100dvh - 24px);
  overflow-y: auto;
  overscroll-behavior: contain;
  padding: 32px 28px 24px;
  text-align: center;
  border-radius: var(--radius-xl);
  border: 1px solid var(--m3-outline-variant);
  /* Solid: a backdrop blur makes WebKitGTK flicker on some GPUs (#705). */
  background: var(--m3-surface-container-high);
  box-shadow:
    0 8px 32px var(--m3-shadow),
    0 0 0 1px color-mix(in srgb, var(--m3-on-surface) 8%, transparent);
}

/* ── Close Button ─────────────────────────────────────────────────── */
.about-close {
  position: absolute;
  top: 12px;
  right: 12px;
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  border-radius: var(--radius-md);
  background: transparent;
  color: var(--m3-on-surface-variant);
  cursor: pointer;
  transition: var(--transition-all);
}
.about-close:hover {
  background: var(--m3-surface-container-highest);
  color: var(--m3-on-surface);
}

/* ── Logo ─────────────────────────────────────────────────────────── */
.about-logo img {
  border-radius: 22px;
  box-shadow: 0 4px 20px var(--m3-shadow);
}

/* ── Title ────────────────────────────────────────────────────────── */
.about-title {
  margin-top: 16px;
  font-size: var(--font-size-3xl);
  font-weight: 700;
  letter-spacing: 0.3px;
  color: var(--m3-on-surface);
}
.about-title .accent {
  color: var(--m3-primary);
}

/* ── Version Badges (stacked, prominent) ──────────────────────────── */
.about-versions {
  display: flex;
  flex-direction: column;
  gap: 6px;
  margin-top: 14px;
  padding: 0 12px;
}
.version-badge {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 14px;
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-lg);
  background: var(--about-card-bg);
  cursor: pointer;
  transition: var(--transition-all);
  font-family: var(--font-family-mono);
}
.version-badge:hover {
  border-color: var(--m3-primary);
  background: var(--about-card-hover-bg);
}
.version-badge:hover .copy-icon {
  opacity: 1;
  color: var(--m3-primary);
}
.version-badge:active {
  transform: scale(0.98);
}
.version-label {
  font-size: var(--font-size-xs);
  font-weight: 500;
  color: var(--m3-on-surface-variant);
  letter-spacing: 0.3px;
}
.version-value {
  font-size: var(--font-size-sm);
  font-weight: 700;
  color: var(--m3-on-surface);
  letter-spacing: 0.5px;
  margin-left: auto;
}
.copy-icon {
  opacity: 0.3;
  color: var(--m3-outline);
  transition: var(--transition-all);
  flex-shrink: 0;
}

/* ── Description ──────────────────────────────────────────────────── */
.about-desc {
  margin: 16px auto 0;
  max-width: 320px;
  font-size: var(--font-size-sm);
  line-height: 1.7;
  color: var(--m3-on-surface-variant);
}
/* ── Section Label ────────────────────────────────────────────────── */
.about-section-label {
  margin-top: 20px;
  font-size: var(--font-size-2xs);
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 2px;
  color: var(--m3-outline);
}

/* ── Tech Tags ────────────────────────────────────────────────────── */
.about-tags {
  margin-top: 10px;
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 8px;
}
.about-tag {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  font-size: var(--font-size-2xs);
  font-weight: 600;
  padding: 3px 10px;
  border-radius: var(--radius-lg);
  color: var(--tag-color);
  background: var(--about-card-bg);
  border: 1px solid color-mix(in srgb, var(--tag-color) 40%, transparent);
  letter-spacing: 0.3px;
}
.about-tag svg {
  flex-shrink: 0;
}
.about-tag-icon {
  opacity: 0.7;
}

/* ── Link Cards (2×2 Grid) ────────────────────────────────────────── */
.about-links {
  margin-top: 20px;
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 8px;
}
.about-link-card {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  min-height: 40px;
  text-decoration: none;
  padding: 10px 0;
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-lg);
  background: var(--about-card-bg);
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-xs);
  font-weight: 500;
  cursor: pointer;
  transition: var(--transition-all);
}
.about-link-card:hover {
  border-color: var(--m3-primary);
  color: var(--m3-primary);
  background: var(--about-card-hover-bg);
}
.about-link-card:focus-visible,
.about-link:focus-visible {
  outline: 2px solid var(--m3-primary);
  outline-offset: 3px;
}

/* ── Footer ───────────────────────────────────────────────────────── */
.about-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  justify-content: center;
  margin-top: 18px;
}
.about-footer {
  margin-top: 20px;
  display: flex;
  flex-direction: column;
  gap: 7px;
  font-size: var(--font-size-xs);
  line-height: 1.55;
  color: var(--m3-outline);
}
.about-maintainer {
  color: var(--m3-on-surface-variant);
  font-weight: 600;
}
.about-footer p {
  margin: 0;
  text-wrap: pretty;
}
.about-disclaimer {
  white-space: pre-line;
}
.about-footer strong {
  color: var(--m3-primary);
  font-weight: 700;
}
.about-link {
  color: var(--m3-primary);
  cursor: pointer;
  text-decoration: none;
}
.about-link:hover {
  text-decoration: underline;
}
.about-platform {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  justify-content: center;
  margin-top: 8px;
}
.about-platform__tag {
  padding: 3px 9px;
  color: var(--m3-on-surface-variant);
  font-size: 11px;
  letter-spacing: 0.02em;
  background: var(--m3-surface-container-highest);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-full);
}
.about-platform__tag--arch {
  color: var(--m3-on-primary-container);
  background: var(--m3-primary-container);
  border-color: transparent;
}
.version-error {
  margin-left: auto;
  font-size: var(--font-size-xs);
  font-weight: 500;
  color: var(--m3-outline);
  letter-spacing: 0.3px;
}

/* ── Staggered Entrance Animation ─────────────────────────────────── */
.stagger {
  opacity: 0;
  transform: translateY(12px);
}
.about-enter .stagger {
  animation: about-fade-up 0.4s cubic-bezier(0.25, 0.46, 0.45, 0.94) forwards;
}
.about-enter .stagger-1 {
  animation-delay: 0.05s;
}
.about-enter .stagger-2 {
  animation-delay: 0.12s;
}
.about-enter .stagger-3 {
  animation-delay: 0.18s;
}
.about-enter .stagger-4 {
  animation-delay: 0.24s;
}
.about-enter .stagger-5 {
  animation-delay: 0.3s;
}
.about-enter .stagger-6 {
  animation-delay: 0.36s;
}

@keyframes about-fade-up {
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* ── Spinner ──────────────────────────────────────────────────────── */
@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}
.spinner {
  animation: spin 0.8s linear infinite;
  flex-shrink: 0;
  will-change: transform;
  contain: layout style paint;
}
.version-loading {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-left: auto;
  font-size: var(--font-size-xs);
  font-weight: 500;
  color: var(--m3-outline);
  letter-spacing: 0.3px;
}
.version-badge--loading {
  cursor: default;
}
.version-badge--loading:hover {
  border-color: var(--m3-outline-variant);
  background: var(--about-card-bg);
}

/* ── Version Swap Transition ──────────────────────────────────────── */
.version-swap-enter-active {
  transition:
    opacity 0.25s cubic-bezier(0.2, 0, 0, 1),
    transform 0.25s cubic-bezier(0.2, 0, 0, 1);
}
.version-swap-leave-active {
  transition:
    opacity 0.15s cubic-bezier(0.3, 0, 0.8, 0.15),
    transform 0.15s cubic-bezier(0.3, 0, 0.8, 0.15);
}
.version-swap-enter-from {
  opacity: 0;
  transform: translateY(6px);
}
.version-swap-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}
</style>
