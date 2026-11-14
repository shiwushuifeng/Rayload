<script setup lang="ts">
/** @fileoverview Pair and manage Rayload devices on the local network. */
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import QRCode from 'qrcode'
import { NButton, NCard, NIcon, NInput, NModal, NTag } from 'naive-ui'
import {
  CreateOutline,
  LinkOutline,
  ScanOutline,
  ShieldCheckmarkOutline,
  TrashOutline,
  WifiOutline,
} from '@vicons/ionicons5'
import {
  useLanStore,
  type DiscoveredDevice,
  type LanPairingInfo,
  type LanScanResult,
  type PairResult,
} from '@/stores/lan'
import { useAppMessage } from '@/composables/useAppMessage'
import {
  readAndroidLocalNetworkPermissionStatus,
  requestAndroidLocalNetworkPermission,
  type AndroidPermissionStatus,
} from '@/composables/useAndroidNative'
import { getErrorMessage } from '@shared/utils/errorMessage'
import { deviceArchLabel } from '@/composables/usePlatform'
import { logger } from '@shared/logger'
import QrScanner from '@/components/lan/QrScanner.vue'

interface HostCredentials {
  username: string
  configured: boolean
  name: string
}

interface PairingQr {
  type?: string
  address?: string
  username?: string
  password?: string
}

const { t } = useI18n()
const lan = useLanStore()
const message = useAppMessage()

const address = ref('')
const username = ref('')
const password = ref('')
const pairing = ref(false)
const pending = ref<PairResult | null>(null)

const hostUsername = ref('')
const hostPassword = ref('')
const hostDeviceName = ref('')
const hostConfigured = ref(false)
const savingHost = ref(false)
const savingName = ref(false)
const pairingInfo = ref<LanPairingInfo | null>(null)
const pairingAddress = ref('')
const qrDataUrl = ref('')
const showScanner = ref(false)
const scanning = ref(false)
const scanResults = ref<LanScanResult[]>([])
const discoveredPairing = ref('')
const localNetworkStatus = ref<AndroidPermissionStatus>('unavailable')
let localNetworkRequested = false
let unlistenUnpaired: (() => void) | null = null

const renaming = ref<{ id: string; name: string; note: string } | null>(null)
const renameDraft = ref('')
const noteDraft = ref('')

const confirmedDevices = computed(() => lan.devices)
const pairedAddresses = computed(() => new Set(confirmedDevices.value.map((device) => device.address)))
const pairedFingerprints = computed(
  () => new Set(confirmedDevices.value.map((device) => device.fingerprint).filter(Boolean)),
)

function isPaired(peer: DiscoveredDevice): boolean {
  return (
    pairedAddresses.value.has(peer.address.replace(/\/$/, '')) ||
    Boolean(peer.fingerprint && pairedFingerprints.value.has(peer.fingerprint))
  )
}

function refreshLocalNetworkStatus(): void {
  localNetworkStatus.value = readAndroidLocalNetworkPermissionStatus()
}

function requestLocalNetworkAccess(): void {
  localNetworkRequested = true
  requestAndroidLocalNetworkPermission()
}

function handlePermissionStateChanged(): void {
  const wasGranted = localNetworkStatus.value === 'granted'
  refreshLocalNetworkStatus()
  if (!wasGranted && localNetworkStatus.value === 'granted') {
    void loadPairingInfo()
    void lan.refreshStatuses()
  }
}

async function loadHost(): Promise<void> {
  try {
    const credentials = await invoke<HostCredentials>('lan_credentials')
    hostUsername.value = credentials.username
    hostConfigured.value = credentials.configured
    hostDeviceName.value = credentials.name
  } catch (error) {
    logger.debug('LanDevices.host', error)
  }
}

async function loadPairingInfo(): Promise<void> {
  try {
    pairingInfo.value = await lan.pairingInfo()
    if (!pairingInfo.value.addresses.includes(pairingAddress.value)) {
      pairingAddress.value = pairingInfo.value.addresses[0] ?? ''
    }
    await renderQr()
  } catch (error) {
    logger.debug('LanDevices.pairingInfo', error)
    pairingInfo.value = null
    pairingAddress.value = ''
    qrDataUrl.value = ''
  }
}

async function renderQr(): Promise<void> {
  if (!pairingInfo.value || !pairingAddress.value) {
    qrDataUrl.value = ''
    return
  }
  qrDataUrl.value = await QRCode.toDataURL(
    JSON.stringify({
      type: 'rayload-lan',
      version: 1,
      address: pairingAddress.value,
      username: pairingInfo.value.username,
      password: pairingInfo.value.password,
      name: pairingInfo.value.name,
      arch: pairingInfo.value.arch,
    }),
    { margin: 1, width: 280, errorCorrectionLevel: 'M', color: { dark: '#241c33', light: '#fffaf0' } },
  )
}

watch(pairingAddress, () => void renderQr())

async function saveHost(): Promise<void> {
  savingHost.value = true
  try {
    await invoke('lan_set_credentials', { username: hostUsername.value.trim(), password: hostPassword.value })
    hostPassword.value = ''
    hostConfigured.value = Boolean(hostUsername.value.trim())
    message.success(t('lan-devices.host-saved'))
    await loadPairingInfo()
  } catch (error) {
    message.error(getErrorMessage(error))
  } finally {
    savingHost.value = false
  }
}

async function saveDeviceName(): Promise<void> {
  const name = hostDeviceName.value.trim()
  if (savingName.value || !name) return
  savingName.value = true
  try {
    await invoke('lan_set_device_name', { name })
    message.success(t('lan-devices.device-name-saved'))
    await loadPairingInfo()
  } catch (error) {
    message.error(getErrorMessage(error))
  } finally {
    savingName.value = false
  }
}

async function pair(): Promise<void> {
  if (pairing.value || !address.value.trim() || !username.value || !password.value) return
  pairing.value = true
  try {
    pending.value = await lan.pair(address.value.trim(), username.value, password.value)
  } catch (error) {
    message.error(getErrorMessage(error))
  } finally {
    pairing.value = false
  }
}

async function confirm(): Promise<void> {
  if (!pending.value) return
  try {
    await lan.confirm(pending.value.id)
    message.success(t('lan-devices.paired', { name: pending.value.name }))
    pending.value = null
    address.value = ''
    username.value = ''
    password.value = ''
  } catch (error) {
    message.error(getErrorMessage(error))
  }
}

async function cancel(): Promise<void> {
  if (!pending.value) return
  try {
    await lan.forget(pending.value.id)
  } catch (error) {
    logger.debug('LanDevices.cancel', error)
  }
  pending.value = null
}

async function forget(id: string): Promise<void> {
  try {
    await lan.forget(id)
  } catch (error) {
    message.error(getErrorMessage(error))
  }
}

async function pairDiscovered(peer: DiscoveredDevice): Promise<void> {
  if (discoveredPairing.value) return
  discoveredPairing.value = peer.address
  try {
    pending.value = await lan.pairDiscovered(peer.address)
  } catch (error) {
    message.error(getErrorMessage(error))
  } finally {
    discoveredPairing.value = ''
  }
}

async function scanNetwork(): Promise<void> {
  if (scanning.value) return
  scanning.value = true
  try {
    scanResults.value = await lan.scan()
    if (scanResults.value.length === 0) message.info(t('lan-devices.scan-empty'))
  } catch (error) {
    message.error(getErrorMessage(error))
  } finally {
    scanning.value = false
  }
}

function useScanResult(result: LanScanResult): void {
  address.value = result.address
  username.value = hostUsername.value
  scanResults.value = []
}

function handleScan(text: string): void {
  showScanner.value = false
  try {
    const value = JSON.parse(text) as PairingQr
    if (value.type !== 'rayload-lan' || !value.address || !value.username || !value.password) {
      message.error(t('lan-devices.scan-invalid'))
      return
    }
    address.value = value.address
    username.value = value.username
    password.value = value.password
    void pair()
  } catch (error) {
    logger.warn('LanDevices.scan', error)
    message.error(t('lan-devices.scan-invalid'))
  }
}

function openRename(id: string, name: string, note: string): void {
  renaming.value = { id, name, note }
  renameDraft.value = name
  noteDraft.value = note
}

async function saveRename(): Promise<void> {
  if (!renaming.value || !renameDraft.value.trim()) return
  try {
    await lan.updateDevice(renaming.value.id, renameDraft.value.trim(), noteDraft.value.trim())
    renaming.value = null
  } catch (error) {
    message.error(getErrorMessage(error))
  }
}

onMounted(async () => {
  refreshLocalNetworkStatus()
  if (localNetworkStatus.value === 'denied' && !localNetworkRequested) {
    requestLocalNetworkAccess()
  }
  window.addEventListener('rayload-permission-state-changed', handlePermissionStateChanged)
  void loadHost()
  void loadPairingInfo()
  void lan.refreshDevices()
  lan.startStatusPolling()
  // The other side removed us: drop its card without waiting for a reload.
  unlistenUnpaired = await listen<string>('lan:unpaired', () => void lan.refreshDevices())
})
onBeforeUnmount(() => {
  window.removeEventListener('rayload-permission-state-changed', handlePermissionStateChanged)
  unlistenUnpaired?.()
  unlistenUnpaired = null
  lan.stopStatusPolling()
})
</script>

<template>
  <div class="lan-devices-view">
    <header class="lan-devices-header">
      <h2>{{ t('lan-devices.title') }}</h2>
      <p>{{ t('lan-devices.description') }}</p>
    </header>

    <NCard v-if="localNetworkStatus === 'denied'" class="lan-card lan-permission" :bordered="false">
      <h3>{{ t('lan-devices.local-network-title') }}</h3>
      <p class="lan-card-hint">{{ t('lan-devices.local-network-hint') }}</p>
      <NButton type="primary" @click="requestLocalNetworkAccess">
        {{ t('lan-devices.local-network-grant') }}
      </NButton>
    </NCard>

    <NCard class="lan-card" :bordered="false">
      <h3>{{ t('lan-devices.host-title') }}</h3>
      <p class="lan-card-hint">{{ t('lan-devices.host-hint') }}</p>
      <div class="lan-form">
        <NInput v-model:value="hostDeviceName" :maxlength="64" :placeholder="t('lan-devices.device-name-label')" />
        <NButton :loading="savingName" :disabled="!hostDeviceName.trim()" @click="saveDeviceName">
          {{ t('lan-devices.device-name-save') }}
        </NButton>
      </div>
      <div class="lan-form">
        <NInput v-model:value="hostUsername" :placeholder="t('lan-devices.username')" />
        <NInput
          v-model:value="hostPassword"
          type="password"
          show-password-on="click"
          :placeholder="t('lan-devices.host-password')"
        />
        <NButton :loading="savingHost" :disabled="!hostUsername.trim() || !hostPassword" @click="saveHost">
          {{ t('lan-devices.host-save') }}
        </NButton>
      </div>
      <NTag v-if="hostConfigured" size="small" :bordered="false" type="success">
        {{ t('lan-devices.host-ready') }}
      </NTag>

      <div v-if="pairingInfo" class="lan-pairing">
        <div class="lan-pairing__qr">
          <img :src="qrDataUrl" alt="Rayload LAN pairing QR code" width="180" height="180" />
        </div>
        <div class="lan-pairing__copy">
          <strong>{{ t('lan-devices.qr-title') }}</strong>
          <p>{{ t('lan-devices.qr-hint') }}</p>
          <div class="lan-address-list">
            <button
              v-for="item in pairingInfo.addresses"
              :key="item"
              type="button"
              class="lan-address-chip"
              :class="{ active: item === pairingAddress }"
              @click="pairingAddress = item"
            >
              <span>{{ item.includes('[') ? 'IPv6' : 'IPv4' }}</span>
              <code>{{ item }}</code>
            </button>
          </div>
          <span>{{ pairingInfo.name }} · {{ deviceArchLabel(pairingInfo.arch) }}</span>
          <div class="lan-host-fingerprint">
            <span>{{ t('lan-devices.local-fingerprint') }}</span>
            <code>{{ pairingInfo.fingerprint }}</code>
          </div>
        </div>
      </div>
    </NCard>

    <NCard class="lan-card" :bordered="false">
      <h3>{{ t('lan-devices.add-title') }}</h3>
      <div class="lan-add-actions">
        <NButton @click="showScanner = true">
          <template #icon
            ><NIcon><ScanOutline /></NIcon
          ></template>
          {{ t('lan-devices.scan-button') }}
        </NButton>
        <NButton :loading="scanning" @click="scanNetwork">
          <template #icon
            ><NIcon><WifiOutline /></NIcon
          ></template>
          {{ t('lan-devices.scan-network') }}
        </NButton>
      </div>

      <div v-if="lan.discovered.length" class="lan-discovered">
        <h4>{{ t('lan-devices.discovered-title') }}</h4>
        <div v-for="peer in lan.discovered" :key="peer.address" class="lan-discovered__row">
          <span class="lan-dot online" aria-hidden="true" />
          <span class="lan-discovered__name">{{ peer.name }}</span>
          <code>{{ deviceArchLabel(peer.arch) || '—' }}</code>
          <code>{{ peer.address }}</code>
          <NButton
            size="small"
            :loading="discoveredPairing === peer.address"
            :disabled="isPaired(peer)"
            @click="pairDiscovered(peer)"
          >
            {{ isPaired(peer) ? t('lan-devices.paired-short') : t('lan-devices.discovered-add') }}
          </NButton>
        </div>
      </div>

      <div v-if="scanResults.length" class="lan-discovered">
        <h4>{{ t('lan-devices.scan-results') }}</h4>
        <button
          v-for="result in scanResults"
          :key="result.address"
          type="button"
          class="lan-discovered__row lan-discovered__row--button"
          @click="useScanResult(result)"
        >
          <span class="lan-dot online" aria-hidden="true" />
          <span class="lan-discovered__name">{{ result.name }}</span>
          <code>{{ deviceArchLabel(result.arch) || '—' }}</code>
          <code>{{ result.address }}</code>
        </button>
      </div>

      <div class="lan-form">
        <NInput v-model:value="address" :placeholder="t('lan-devices.address')" />
        <NInput v-model:value="username" :placeholder="t('lan-devices.username')" />
        <NInput
          v-model:value="password"
          type="password"
          show-password-on="click"
          :placeholder="t('lan-devices.password')"
        />
        <NButton type="primary" :loading="pairing" :disabled="!address.trim() || !username || !password" @click="pair">
          <template #icon>
            <NIcon><LinkOutline /></NIcon>
          </template>
          {{ t('lan-devices.pair') }}
        </NButton>
      </div>
    </NCard>

    <NModal :show="pending !== null" :mask-closable="false" :closable="false" :z-index="1500">
      <section v-if="pending" class="lan-fingerprint" role="alertdialog" aria-modal="true">
        <h3>
          <NIcon :size="18"><ShieldCheckmarkOutline /></NIcon> {{ t('lan-devices.fingerprint-title') }}
        </h3>
        <p class="lan-card-hint">{{ t('lan-devices.fingerprint-hint', { name: pending.name }) }}</p>
        <code class="lan-fingerprint-value">{{ pending.fingerprint }}</code>
        <div class="lan-fingerprint-actions">
          <NButton @click="cancel">{{ t('app.cancel') }}</NButton>
          <NButton type="primary" @click="confirm">{{ t('lan-devices.fingerprint-confirm') }}</NButton>
        </div>
      </section>
    </NModal>

    <section class="lan-list" :aria-label="t('lan-devices.paired-title')">
      <h3>{{ t('lan-devices.paired-title') }}</h3>
      <p v-if="confirmedDevices.length === 0" class="lan-empty">{{ t('lan-devices.empty') }}</p>
      <div v-for="device in confirmedDevices" :key="device.id" class="lan-device-row">
        <span class="lan-device-icon" :class="{ online: device.online }">
          <NIcon :size="18"><WifiOutline /></NIcon>
        </span>
        <div class="lan-device-copy">
          <strong>
            <span class="lan-dot" :class="{ online: device.online }" aria-hidden="true" />
            {{ device.name }}
          </strong>
          <span v-if="device.note" class="lan-device-note">{{ device.note }}</span>
          <span>{{ device.online ? t('lan-devices.online') : t('lan-devices.offline') }}</span>
          <span>{{ device.address }}</span>
        </div>
        <NButton
          size="small"
          quaternary
          :aria-label="t('lan-devices.edit-title')"
          @click="openRename(device.id, device.name, device.note)"
        >
          <template #icon
            ><NIcon><CreateOutline /></NIcon
          ></template>
        </NButton>
        <NButton size="small" quaternary @click="forget(device.id)">
          <template #icon
            ><NIcon><TrashOutline /></NIcon
          ></template>
          {{ t('lan-devices.forget') }}
        </NButton>
      </div>
    </section>

    <NModal
      :show="!!renaming"
      transform-origin="center"
      @update:show="
        (visible: boolean) => {
          if (!visible) renaming = null
        }
      "
    >
      <NCard class="lan-rename" :bordered="false" :title="t('lan-devices.edit-title')">
        <div class="lan-rename__fields">
          <NInput
            v-model:value="renameDraft"
            :placeholder="t('lan-devices.rename-placeholder')"
            @keyup.enter="saveRename"
          />
          <NInput
            v-model:value="noteDraft"
            :maxlength="120"
            :placeholder="t('lan-devices.note-placeholder')"
            @keyup.enter="saveRename"
          />
        </div>
        <div class="lan-fingerprint-actions">
          <NButton @click="renaming = null">{{ t('app.cancel') }}</NButton>
          <NButton type="primary" :disabled="!renameDraft.trim()" @click="saveRename">
            {{ t('app.save') }}
          </NButton>
        </div>
      </NCard>
    </NModal>

    <QrScanner :show="showScanner" @close="showScanner = false" @result="handleScan" />
  </div>
</template>

<style scoped>
.lan-devices-view {
  box-sizing: border-box;
  display: grid;
  align-content: start;
  gap: 14px;
  width: 100%;
  height: 100%;
  max-width: 860px;
  margin-inline: auto;
  padding: 18px var(--content-gutter) 40px;
  overflow-y: auto;
  overscroll-behavior: contain;
  -webkit-overflow-scrolling: touch;
}
.lan-permission {
  border-color: color-mix(in srgb, var(--m3-error) 40%, var(--m3-outline-variant));
}
.lan-devices-header h2 {
  margin: 0 0 4px;
  font-size: 20px;
}
.lan-devices-header p,
.lan-card-hint,
.lan-empty {
  margin: 0;
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-sm);
}
.lan-card {
  background: var(--m3-surface-container);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-lg);
}
.lan-card h3,
.lan-list h3,
.lan-discovered h4 {
  margin: 0 0 8px;
  font-size: 15px;
}
.lan-form {
  display: grid;
  gap: 8px;
  margin: 12px 0;
}
.lan-add-actions,
.lan-fingerprint-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}
.lan-fingerprint-actions {
  justify-content: flex-end;
  margin-top: 12px;
}
.lan-pairing {
  display: grid;
  grid-template-columns: auto minmax(0, 1fr);
  gap: 16px;
  align-items: center;
  margin-top: 14px;
}
.lan-pairing__qr {
  padding: 8px;
  background: #fffaf0;
  border-radius: var(--radius-md);
}
.lan-pairing__copy {
  display: grid;
  gap: 6px;
  min-width: 0;
}
.lan-pairing__copy p {
  margin: 0;
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-sm);
}
.lan-pairing__copy code,
.lan-discovered code {
  overflow: hidden;
  color: var(--m3-on-surface-variant);
  font-size: 12px;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.lan-address-list {
  display: grid;
  gap: 6px;
}
.lan-address-chip {
  display: grid;
  grid-template-columns: auto minmax(0, 1fr);
  gap: 8px;
  align-items: center;
  min-height: 44px;
  padding: 6px 10px;
  color: var(--m3-on-surface);
  text-align: left;
  background: var(--m3-surface-container-low);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-md);
}
.lan-address-chip span {
  color: var(--m3-on-surface-variant);
  font-size: 11px;
  font-weight: 600;
  letter-spacing: 0.04em;
}
.lan-address-chip.active {
  border-color: var(--m3-primary);
  background: color-mix(in srgb, var(--m3-primary-container) 55%, var(--m3-surface-container-low));
}
.lan-address-chip code {
  white-space: normal;
  overflow-wrap: anywhere;
}
.lan-discovered {
  display: grid;
  gap: 6px;
  margin: 14px 0;
}
.lan-discovered__row {
  display: grid;
  grid-template-columns: 10px minmax(0, 1fr) auto auto auto;
  gap: 10px;
  align-items: center;
  width: 100%;
  min-height: 44px;
  padding: 6px 10px;
  color: var(--m3-on-surface);
  text-align: left;
  background: var(--m3-surface-container-low);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-md);
}
.lan-discovered__row code {
  max-width: 220px;
}
.lan-discovered__row--button:hover {
  background: var(--m3-surface-container-high);
}
.lan-discovered__name {
  overflow: hidden;
  font-weight: 600;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.lan-dot {
  display: inline-block;
  width: 8px;
  height: 8px;
  flex: none;
  background: var(--m3-outline);
  border-radius: 50%;
}
.lan-dot.online {
  background: color-mix(in srgb, var(--m3-primary) 62%, var(--m3-outline));
}
.lan-fingerprint {
  display: flex;
  flex-direction: column;
  gap: 12px;
  width: min(420px, calc(100vw - 28px));
  padding: 20px;
  background: var(--m3-surface-container-high);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-xl);
  box-shadow: 0 18px 60px var(--m3-shadow);
}
.lan-fingerprint h3 {
  display: flex;
  gap: 8px;
  align-items: center;
  margin: 0;
  font-size: 1.05rem;
}
.lan-host-fingerprint {
  display: flex;
  flex-direction: column;
  gap: 4px;
  margin-top: 4px;
  font-size: 12px;
  color: var(--m3-on-surface-variant);
}
.lan-host-fingerprint code {
  overflow-wrap: anywhere;
  color: var(--m3-on-surface);
}
.lan-fingerprint-value {
  display: block;
  padding: 10px 12px;
  overflow-wrap: anywhere;
  background: var(--m3-surface-container-low);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-sm);
  font-size: 13px;
}
.lan-list {
  display: grid;
  gap: 8px;
}
.lan-device-row {
  display: grid;
  grid-template-columns: 40px minmax(0, 1fr) auto auto;
  gap: 10px;
  align-items: center;
  padding: 10px 12px;
  background: var(--m3-surface-container);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-lg);
}
.lan-device-icon {
  display: grid;
  width: 40px;
  height: 40px;
  place-items: center;
  color: var(--m3-on-surface-variant);
  background: var(--m3-surface-container-high);
  border-radius: 12px;
}
.lan-device-icon.online {
  color: color-mix(in srgb, var(--m3-primary) 74%, var(--m3-on-surface-variant));
  background: color-mix(in srgb, var(--m3-primary-container) 74%, var(--m3-surface-container-high));
}
.lan-device-copy {
  display: grid;
  gap: 2px;
  min-width: 0;
}
.lan-device-copy strong {
  display: flex;
  gap: 7px;
  align-items: center;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.lan-device-copy span {
  overflow: hidden;
  color: var(--m3-on-surface-variant);
  font-size: 12px;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.lan-rename {
  width: min(420px, calc(100vw - 28px));
}
.lan-rename__fields {
  display: grid;
  gap: 10px;
}
.lan-device-copy .lan-device-note {
  color: var(--m3-primary);
  font-style: italic;
}
@media (max-width: 540px) {
  .lan-pairing {
    grid-template-columns: 1fr;
    justify-items: center;
    text-align: center;
  }
  .lan-discovered__row {
    grid-template-columns: 10px minmax(0, 1fr) auto;
  }
  .lan-discovered__row:not(.lan-discovered__row--button) code {
    display: none;
  }
  .lan-discovered__row--button code:first-of-type {
    display: none;
  }
  .lan-device-row {
    grid-template-columns: 40px minmax(0, 1fr) auto;
  }
  .lan-device-row .n-button:last-child {
    grid-column: 2 / -1;
    justify-self: end;
  }
}
</style>
