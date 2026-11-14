<script setup lang="ts">
/** @fileoverview Approves incoming LAN pairing requests from other devices. */
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { useI18n } from 'vue-i18n'
import { NButton, NIcon, NModal } from 'naive-ui'
import { ShieldCheckmarkOutline } from '@vicons/ionicons5'
import { useLanStore, type LanPairRequest } from '@/stores/lan'
import { useAppMessage } from '@/composables/useAppMessage'
import { logger } from '@shared/logger'
import { getErrorMessage } from '@shared/utils/errorMessage'

const { t } = useI18n()
const lan = useLanStore()
const message = useAppMessage()

const queue = ref<LanPairRequest[]>([])
const busy = ref(false)
const current = computed(() => queue.value[0] ?? null)
let unlistenRequest: (() => void) | null = null
let unlistenPaired: (() => void) | null = null
let expiry: number | undefined

// The backend drops unanswered requests after 90 seconds; mirror that so the
// dialog cannot outlive the request it describes.
const DECISION_WINDOW_MS = 90_000

function armExpiry(): void {
  window.clearTimeout(expiry)
  if (queue.value.length === 0) return
  expiry = window.setTimeout(() => {
    queue.value.shift()
    armExpiry()
  }, DECISION_WINDOW_MS)
}

function enqueue(request: LanPairRequest): void {
  if (!queue.value.some((item) => item.id === request.id)) queue.value.push(request)
  armExpiry()
}

async function decide(approved: boolean): Promise<void> {
  const request = current.value
  if (!request || busy.value) return
  busy.value = true
  try {
    if (approved) await lan.approvePair(request.id)
    else await lan.rejectPair(request.id)
  } catch (error) {
    // The dialog closes either way, so a silent failure looks like a decision.
    logger.debug('LanPair.decide', error)
    message.error(getErrorMessage(error))
  } finally {
    queue.value.shift()
    busy.value = false
    armExpiry()
  }
}

onMounted(async () => {
  try {
    for (const request of await lan.pairRequests()) enqueue(request)
  } catch (error) {
    logger.debug('LanPair.pending', error)
  }
  unlistenRequest = await listen<LanPairRequest>('lan:pair-request', ({ payload }) => enqueue(payload))
  unlistenPaired = await listen<string>('lan:paired', ({ payload }) => {
    void lan.refreshDevices()
    message.success(t('lan-devices.pair-request-paired', { name: payload }))
  })
})

onUnmounted(() => {
  window.clearTimeout(expiry)
  unlistenRequest?.()
  unlistenPaired?.()
})
</script>

<template>
  <NModal :show="current !== null" :mask-closable="false" :closable="false" :z-index="1600">
    <section
      v-if="current"
      class="pair-request"
      role="alertdialog"
      aria-modal="true"
      :aria-label="t('lan-devices.pair-request-title')"
    >
      <header class="pair-request__head">
        <NIcon :size="22"><ShieldCheckmarkOutline /></NIcon>
        <h2>{{ t('lan-devices.pair-request-title') }}</h2>
      </header>
      <p class="pair-request__body">{{ t('lan-devices.pair-request-body', { name: current.name }) }}</p>
      <div class="pair-request__fingerprint">
        <span class="pair-request__label">{{ t('lan-devices.fingerprint-title') }}</span>
        <code>{{ current.fingerprint }}</code>
      </div>
      <p class="pair-request__hint">{{ t('lan-devices.fingerprint-hint', { name: current.name }) }}</p>
      <div class="pair-request__actions">
        <NButton :disabled="busy" @click="decide(false)">{{ t('lan-devices.pair-request-reject') }}</NButton>
        <NButton type="primary" :loading="busy" @click="decide(true)">
          {{ t('lan-devices.pair-request-approve') }}
        </NButton>
      </div>
    </section>
  </NModal>
</template>

<style scoped>
.pair-request {
  display: flex;
  flex-direction: column;
  gap: 12px;
  width: min(420px, calc(100vw - 28px));
  padding: 20px;
  color: var(--m3-on-surface);
  background: var(--m3-surface-container-high);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-xl);
  box-shadow: 0 18px 60px var(--m3-shadow);
}

.pair-request__head {
  display: flex;
  align-items: center;
  gap: 10px;
  color: var(--m3-primary);
}

.pair-request__head h2 {
  margin: 0;
  font-size: 1.05rem;
  color: var(--m3-on-surface);
}

.pair-request__body {
  margin: 0;
  color: var(--m3-on-surface-variant);
}

.pair-request__fingerprint {
  display: flex;
  flex-direction: column;
  gap: 6px;
  padding: 10px 12px;
  background: var(--m3-surface-container-low);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-sm);
}

.pair-request__label {
  font-size: 0.8rem;
  color: var(--m3-on-surface-variant);
}

.pair-request__fingerprint code {
  display: block;
  font-size: 13px;
  line-height: 1.5;
  overflow-wrap: anywhere;
}

.pair-request__hint {
  margin: 0;
  font-size: 0.82rem;
  color: var(--m3-on-surface-variant);
}

.pair-request__actions {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}
</style>
