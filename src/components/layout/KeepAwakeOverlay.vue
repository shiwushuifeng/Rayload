<script setup lang="ts">
/** Full-screen keep-awake dashboard driven by the live download statistics. */
import { computed, onBeforeUnmount, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { NIcon } from 'naive-ui'
import { MoonOutline, SunnyOutline } from '@vicons/ionicons5'
import { useAppStore } from '@/stores/app'
import { usePreferenceStore } from '@/stores/preference'
import { useReducedMotion } from '@/composables/useReducedMotion'
import { useTheme } from '@/composables/useTheme'
import { setAndroidKeepScreenOn } from '@/composables/useAndroidNative'
import { triggerHaptic } from '@/composables/useHaptics'
import { bytesToSize } from '@shared/utils'

const { t } = useI18n()
const appStore = useAppStore()
const preferenceStore = usePreferenceStore()
const reducedMotion = useReducedMotion()
const { isDark } = useTheme()

const downloadSpeed = computed(() => bytesToSize(String(appStore.stat.downloadSpeed)))
const activeTasks = computed(() => appStore.stat.numActive)

watch(
  () => appStore.keepAwakeVisible,
  (visible) => setAndroidKeepScreenOn(visible),
  { immediate: true },
)

onBeforeUnmount(() => setAndroidKeepScreenOn(false))

function exit() {
  triggerHaptic()
  appStore.hideKeepAwake()
}

function toggleAppearance() {
  triggerHaptic()
  void preferenceStore.updateAndSave({ theme: isDark.value ? 'light' : 'dark' })
}
</script>

<template>
  <Teleport to="body">
    <Transition name="keep-awake">
      <section
        v-if="appStore.keepAwakeVisible"
        class="keep-awake"
        :class="{ 'is-reduced-motion': reducedMotion, 'is-light': !isDark }"
        role="dialog"
        aria-modal="true"
        :aria-label="t('app.keep-awake-mode')"
        @click="exit"
      >
        <div class="keep-awake__halo" aria-hidden="true" />
        <button
          type="button"
          class="keep-awake__theme"
          :aria-label="t('preferences.appearance')"
          @click.stop="toggleAppearance"
        >
          <NIcon :size="20" aria-hidden="true">
            <SunnyOutline v-if="isDark" />
            <MoonOutline v-else />
          </NIcon>
        </button>
        <div class="keep-awake__center">
          <p class="keep-awake__eyebrow">{{ t('app.keep-awake-mode') }}</p>
          <div class="keep-awake__logo" aria-hidden="true">
            <svg viewBox="0 0 512 512" fill="none">
              <g
                transform="rotate(-35 256 256)"
                stroke="currentColor"
                fill="none"
                stroke-linejoin="miter"
                stroke-linecap="round"
              >
                <path d="M 256 120 L 290 170 L 222 170 Z" fill="currentColor" stroke-width="8" />
                <path d="M 256 410 L 130 290 L 180 290 L 180 240 L 332 240 L 332 290 L 382 290 Z" stroke-width="12" />
                <path d="M 180 240 L 256 410 M 332 240 L 256 410" stroke-width="8" opacity="0.55" />
              </g>
            </svg>
          </div>
          <div class="keep-awake__speed">
            <strong>{{ downloadSpeed }}</strong
            ><span>/s</span>
          </div>
          <p class="keep-awake__label">{{ t('app.keep-awake-speed-label') }}</p>
          <p class="keep-awake__tasks">{{ t('app.keep-awake-active-tasks', { count: activeTasks }) }}</p>
        </div>
        <p class="keep-awake__hint">{{ t('app.keep-awake-exit-hint') }}</p>
      </section>
    </Transition>
  </Teleport>
</template>

<style scoped>
.keep-awake {
  position: fixed;
  inset: 0;
  z-index: 5000;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  box-sizing: border-box;
  padding: calc(env(safe-area-inset-top, 0px) + 32px) 24px calc(env(safe-area-inset-bottom, 0px) + 40px);
  overflow: hidden;
  color: #fff;
  cursor: pointer;
  user-select: none;
  -webkit-tap-highlight-color: transparent;
  background: #050505;
  background:
    radial-gradient(circle at 50% 36%, rgba(139, 92, 246, 0.16) 0%, transparent 46%),
    radial-gradient(circle at 50% 50%, #0b0b0c 0%, #050505 62%, #000 100%);
}

.keep-awake__halo {
  position: absolute;
  top: 50%;
  left: 50%;
  width: min(72vw, 360px);
  aspect-ratio: 1;
  border-radius: 50%;
  background: radial-gradient(circle, rgba(139, 92, 246, 0.26), transparent 68%);
  transform: translate(-50%, -50%);
  filter: blur(18px);
  opacity: 0.7;
  animation: keep-awake-halo 4.4s ease-in-out infinite;
  pointer-events: none;
}

.keep-awake__theme {
  position: absolute;
  top: calc(env(safe-area-inset-top, 0px) + 14px);
  right: 14px;
  z-index: 2;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 44px;
  height: 44px;
  padding: 0;
  border: 1px solid rgb(255 255 255 / 0.14);
  border-radius: var(--radius-full);
  color: rgb(255 255 255 / 0.72);
  background: rgb(255 255 255 / 0.06);
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;
  transition:
    color 180ms ease,
    background-color 180ms ease,
    border-color 180ms ease;
}

.keep-awake__theme:hover {
  color: var(--m3-primary);
  border-color: rgba(139, 92, 246, 0.42);
  background: rgb(255 255 255 / 0.1);
}

/* Light appearance mirrors the app theme instead of forcing a black screen. */
.keep-awake.is-light {
  color: var(--m3-on-surface);
  background: #f6f1f4;
  background:
    radial-gradient(circle at 50% 36%, rgba(139, 92, 246, 0.12) 0%, transparent 48%),
    radial-gradient(circle at 50% 50%, #fdfbfa 0%, #f6f1f4 62%, #eee8ed 100%);
}

.keep-awake.is-light .keep-awake__theme {
  border-color: rgba(0, 0, 0, 0.12);
  color: var(--m3-on-surface-variant);
  background: rgba(255, 255, 255, 0.72);
}

.keep-awake.is-light .keep-awake__theme:hover {
  color: var(--m3-primary);
}

.keep-awake.is-light .keep-awake__eyebrow {
  color: #6d28d9;
}

.keep-awake.is-light .keep-awake__speed {
  color: var(--m3-on-surface);
}

.keep-awake.is-light .keep-awake__speed span {
  color: #7c3aed;
}

.keep-awake.is-light .keep-awake__label,
.keep-awake.is-light .keep-awake__hint {
  color: rgba(29, 27, 30, 0.52);
}

.keep-awake.is-light .keep-awake__tasks {
  color: #7c3aed;
}

.keep-awake.is-light .keep-awake__halo {
  opacity: 0.48;
}

.keep-awake__center {
  position: relative;
  z-index: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
}

.keep-awake__eyebrow {
  margin: 0 0 28px;
  color: #c4b5fd;
  font-size: 11px;
  font-weight: 600;
  letter-spacing: 0.22em;
  text-transform: uppercase;
  opacity: 0.8;
}

.keep-awake__logo {
  width: min(42vw, 168px);
  color: var(--m3-primary);
  filter: drop-shadow(0 0 28px rgba(139, 92, 246, 0.58));
  transform-origin: center;
  animation: keep-awake-breathe 4.4s ease-in-out infinite;
}

.keep-awake__logo svg {
  display: block;
  width: 100%;
  height: auto;
}

.keep-awake__speed {
  display: flex;
  align-items: baseline;
  gap: 4px;
  margin-top: 30px;
  color: #fff;
  font-variant-numeric: tabular-nums;
  line-height: 1;
}

.keep-awake__speed strong {
  font-size: clamp(38px, 11vw, 64px);
  font-weight: 700;
  letter-spacing: -0.03em;
}

.keep-awake__speed span {
  color: #c4b5fd;
  font-size: 20px;
  font-weight: 500;
  opacity: 0.8;
}

.keep-awake__label {
  margin: 10px 0 0;
  color: rgb(255 255 255 / 0.58);
  font-size: var(--font-size-sm);
}

.keep-awake__tasks {
  margin: 10px 0 0;
  color: #ddd6fe;
  font-size: var(--font-size-sm);
  font-weight: 500;
  font-variant-numeric: tabular-nums;
}

.keep-awake__hint {
  position: absolute;
  bottom: calc(env(safe-area-inset-bottom, 0px) + 34px);
  left: 24px;
  right: 24px;
  margin: 0;
  color: rgb(255 255 255 / 0.42);
  font-size: var(--font-size-xs);
  text-align: center;
  letter-spacing: 0.02em;
}

.keep-awake.is-reduced-motion .keep-awake__logo,
.keep-awake.is-reduced-motion .keep-awake__halo {
  animation: none;
}

@keyframes keep-awake-breathe {
  0%,
  100% {
    transform: scale(1);
    filter: drop-shadow(0 0 22px rgba(139, 92, 246, 0.42));
  }
  50% {
    transform: scale(1.045);
    filter: drop-shadow(0 0 38px rgba(139, 92, 246, 0.66));
  }
}

@keyframes keep-awake-halo {
  0%,
  100% {
    opacity: 0.5;
    transform: translate(-50%, -50%) scale(0.94);
  }
  50% {
    opacity: 0.85;
    transform: translate(-50%, -50%) scale(1.05);
  }
}

.keep-awake-enter-active,
.keep-awake-leave-active {
  transition: opacity 0.28s cubic-bezier(0.2, 0, 0, 1);
}

.keep-awake-enter-from,
.keep-awake-leave-to {
  opacity: 0;
}

@media (prefers-reduced-motion: reduce) {
  .keep-awake__logo,
  .keep-awake__halo {
    animation: none;
  }
}
</style>
