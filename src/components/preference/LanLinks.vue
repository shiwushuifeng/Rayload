<script setup lang="ts">
/** @fileoverview LAN connection panel: local address, RPC port, secret and QR payload. */
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { NAlert, NButton, NCard, NIcon, NSpin } from 'naive-ui'
import QRCode from 'qrcode'
import { CopyOutline, RefreshOutline, WifiOutline } from '@vicons/ionicons5'
import { usePreferenceStore } from '@/stores/preference'
import { usePlatform } from '@/composables/usePlatform'
import { useAppMessage } from '@/composables/useAppMessage'
import { readAndroidLocalIp } from '@/composables/useAndroidNative'
import { writeAppClipboardText } from '@shared/utils'
import { ANDROID_ENGINE_RPC_PORT } from '@shared/constants'
import { logger } from '@shared/logger'

const { t } = useI18n()
const preferenceStore = usePreferenceStore()
const { isAndroid } = usePlatform()
const message = useAppMessage()

const localIp = ref('')
const qrDataUrl = ref('')
const qrError = ref('')
const rendering = ref(false)

const port = computed(() => (isAndroid.value ? ANDROID_ENGINE_RPC_PORT : preferenceStore.config.rpcListenPort))
const secret = computed(() => preferenceStore.config.rpcSecret || '')
const endpoint = computed(() => (localIp.value ? `http://${localIp.value}:${port.value}/jsonrpc` : ''))
const payload = computed(() =>
  JSON.stringify({
    type: 'rayload-rpc',
    host: localIp.value,
    port: port.value,
    secret: secret.value,
    endpoint: endpoint.value,
  }),
)

async function refresh() {
  localIp.value = isAndroid.value ? readAndroidLocalIp() : detectDesktopIp()
  await renderQr()
}

function detectDesktopIp(): string {
  // The LAN panel is primarily an Android surface; desktop uses loopback
  // unless a native host address becomes available later.
  return '127.0.0.1'
}

async function renderQr() {
  if (!localIp.value || !secret.value) {
    qrDataUrl.value = ''
    qrError.value = ''
    return
  }
  rendering.value = true
  try {
    qrDataUrl.value = await QRCode.toDataURL(payload.value, {
      margin: 1,
      width: 320,
      errorCorrectionLevel: 'M',
      color: { dark: '#241c33', light: '#fffaf0' },
    })
    qrError.value = ''
  } catch (error) {
    logger.warn('LanLinks.qr', error)
    qrDataUrl.value = ''
    qrError.value = t('preferences.config-qr-too-large')
  } finally {
    rendering.value = false
  }
}

async function copyValue(value: string, successKey: string) {
  if (!value) return
  try {
    await writeAppClipboardText(value)
    message.success(t(successKey))
  } catch (error) {
    logger.warn('LanLinks.copy', error)
    message.error(t('task.checksum-copy-failed'))
  }
}

function onWindowFocus() {
  void refresh()
}

onMounted(() => {
  void refresh()
  window.addEventListener('focus', onWindowFocus)
})

onBeforeUnmount(() => {
  window.removeEventListener('focus', onWindowFocus)
})
</script>

<template>
  <div class="lan-panel">
    <header class="lan-panel__header">
      <h2>{{ t('preferences.lan-title') }}</h2>
      <p>{{ t('preferences.lan-description') }}</p>
    </header>

    <NAlert v-if="!localIp" type="warning" :show-icon="false">
      {{ t('preferences.lan-no-network') }}
    </NAlert>
    <NAlert v-else-if="!secret" type="warning" :show-icon="false">
      {{ t('preferences.lan-no-secret') }}
    </NAlert>

    <NSpin :show="rendering">
      <div class="lan-panel__grid">
        <NCard class="lan-panel__qr-card" :bordered="false">
          <div class="lan-panel__qr">
            <img v-if="qrDataUrl" :src="qrDataUrl" alt="Rayload RPC configuration QR code" width="240" height="240" />
            <div v-else class="lan-panel__qr-empty">
              <NIcon :size="34"><WifiOutline /></NIcon>
              <span>{{ qrError || t('preferences.lan-qr-placeholder') }}</span>
            </div>
          </div>
          <p class="lan-panel__hint">{{ t('preferences.lan-qr-hint') }}</p>
        </NCard>

        <div class="lan-panel__details">
          <div class="lan-row">
            <span class="lan-row__label">{{ t('preferences.lan-ip') }}</span>
            <code class="lan-row__value">{{ localIp || '—' }}</code>
          </div>
          <div class="lan-row">
            <span class="lan-row__label">{{ t('preferences.lan-port') }}</span>
            <code class="lan-row__value">{{ port }}</code>
          </div>
          <div class="lan-row">
            <span class="lan-row__label">{{ t('preferences.lan-secret') }}</span>
            <code class="lan-row__value lan-row__value--secret">{{ secret || '—' }}</code>
          </div>
          <div v-if="endpoint" class="lan-row">
            <span class="lan-row__label">{{ t('preferences.lan-endpoint') }}</span>
            <code class="lan-row__value">{{ endpoint }}</code>
          </div>
          <div class="lan-panel__actions">
            <NButton size="small" :disabled="!secret" @click="copyValue(secret, 'preferences.lan-secret-copied')">
              <template #icon
                ><NIcon><CopyOutline /></NIcon
              ></template>
              {{ t('preferences.lan-copy-secret') }}
            </NButton>
            <NButton size="small" :disabled="!endpoint" @click="copyValue(endpoint, 'preferences.lan-endpoint-copied')">
              <template #icon
                ><NIcon><CopyOutline /></NIcon
              ></template>
              {{ t('preferences.lan-copy-endpoint') }}
            </NButton>
            <NButton size="small" quaternary :loading="rendering" @click="refresh">
              <template #icon
                ><NIcon><RefreshOutline /></NIcon
              ></template>
              {{ t('preferences.permission-refresh') }}
            </NButton>
          </div>
        </div>
      </div>
    </NSpin>

    <p class="lan-panel__note">{{ t('preferences.lan-coming-soon') }}</p>
  </div>
</template>

<style scoped>
.lan-panel {
  display: flex;
  flex-direction: column;
  gap: 18px;
  height: 100%;
  max-width: 860px;
  padding: var(--content-gutter);
  overflow-x: hidden;
  overflow-y: auto;
  overscroll-behavior: contain;
  -webkit-overflow-scrolling: touch;
}
.lan-panel > * {
  flex: 0 0 auto;
}
.lan-panel__header h2 {
  margin: 0 0 6px;
  font-size: 20px;
}
.lan-panel__header p,
.lan-panel__hint,
.lan-panel__note {
  margin: 0;
  color: var(--m3-on-surface-variant);
  line-height: 1.55;
}
.lan-panel__grid {
  display: grid;
  grid-template-columns: minmax(0, 280px) minmax(0, 1fr);
  gap: 16px;
  align-items: start;
}
.lan-panel__qr-card :deep(.n-card-content) {
  display: grid;
  gap: 10px;
  justify-items: center;
  text-align: center;
}
.lan-panel__qr img {
  display: block;
  width: min(240px, 100%);
  height: auto;
  border-radius: 10px;
}
.lan-panel__qr-empty {
  display: grid;
  gap: 8px;
  place-items: center;
  width: 240px;
  min-height: 240px;
  padding: 18px;
  color: var(--m3-on-surface-variant);
  border: 1px dashed var(--m3-outline-variant);
  border-radius: 10px;
  text-align: center;
}
.lan-panel__details {
  display: grid;
  gap: 10px;
  padding: 14px;
  background: var(--m3-surface-container);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-lg);
}
.lan-row {
  display: grid;
  grid-template-columns: 96px minmax(0, 1fr);
  gap: 10px;
  align-items: center;
  min-height: 44px;
}
.lan-row__label {
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-sm);
}
.lan-row__value {
  min-width: 0;
  overflow-wrap: anywhere;
  font-family: var(--font-family-mono);
  font-size: var(--font-size-sm);
}
.lan-row__value--secret {
  color: var(--m3-primary);
}
.lan-panel__actions {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-top: 4px;
}
.lan-panel__actions :deep(.n-button) {
  min-height: 44px;
}
.lan-panel__note {
  font-size: var(--font-size-sm);
}
@media (max-width: 640px) {
  .lan-panel__grid {
    grid-template-columns: minmax(0, 1fr);
  }
}
</style>
