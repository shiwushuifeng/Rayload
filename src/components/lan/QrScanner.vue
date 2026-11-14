<script setup lang="ts">
/** Camera QR scanner used by the LAN pairing form. */
import { nextTick, onBeforeUnmount, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import jsQR from 'jsqr'
import { NButton, NIcon, NModal } from 'naive-ui'
import { CameraOutline, CloseOutline } from '@vicons/ionicons5'
import { getErrorMessage } from '@shared/utils/errorMessage'
import { logger } from '@shared/logger'

const props = defineProps<{ show: boolean }>()
const emit = defineEmits<{ close: []; result: [text: string] }>()
const { t } = useI18n()

const video = ref<HTMLVideoElement | null>(null)
const error = ref('')
let stream: MediaStream | null = null
let frame = 0

function stop(): void {
  cancelAnimationFrame(frame)
  frame = 0
  stream?.getTracks().forEach((track) => track.stop())
  stream = null
  if (video.value) video.value.srcObject = null
}

function scanFrame(): void {
  const element = video.value
  if (!stream || !element) return
  if (element.readyState >= 2 && element.videoWidth > 0) {
    const canvas = document.createElement('canvas')
    canvas.width = element.videoWidth
    canvas.height = element.videoHeight
    const context = canvas.getContext('2d', { willReadFrequently: true })
    if (context) {
      context.drawImage(element, 0, 0, canvas.width, canvas.height)
      const image = context.getImageData(0, 0, canvas.width, canvas.height)
      const code = jsQR(image.data, image.width, image.height, { inversionAttempts: 'dontInvert' })
      if (code?.data) {
        emit('result', code.data)
        emit('close')
        return
      }
    }
  }
  frame = requestAnimationFrame(scanFrame)
}

async function start(): Promise<void> {
  error.value = ''
  await nextTick()
  try {
    stream = await navigator.mediaDevices.getUserMedia({
      video: { facingMode: { ideal: 'environment' } },
      audio: false,
    })
    const element = video.value
    if (!element) return
    element.srcObject = stream
    await element.play()
    frame = requestAnimationFrame(scanFrame)
  } catch (reason) {
    logger.warn('QrScanner.camera', reason)
    error.value = getErrorMessage(reason) || t('lan-devices.camera-error')
    stop()
  }
}

watch(
  () => props.show,
  (visible) => {
    if (visible) void start()
    else stop()
  },
  { immediate: true },
)
onBeforeUnmount(stop)
</script>

<template>
  <NModal
    :show="show"
    transform-origin="center"
    @update:show="
      (visible: boolean) => {
        if (!visible) emit('close')
      }
    "
  >
    <div class="qr-scanner" role="dialog" aria-modal="true">
      <header>
        <span>{{ t('lan-devices.scan-title') }}</span>
        <NButton quaternary circle size="small" :aria-label="t('app.close')" @click="emit('close')">
          <template #icon
            ><NIcon :size="16"><CloseOutline /></NIcon
          ></template>
        </NButton>
      </header>
      <div class="qr-scanner__frame">
        <video ref="video" playsinline muted />
        <div v-if="!video || error" class="qr-scanner__hint">
          <NIcon :size="32"><CameraOutline /></NIcon>
          <span>{{ error || t('lan-devices.scan-hint') }}</span>
        </div>
      </div>
    </div>
  </NModal>
</template>

<style scoped>
.qr-scanner {
  width: min(420px, calc(100vw - 28px));
  padding: 14px;
  background: var(--m3-surface-container-high);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-xl);
}
.qr-scanner header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  margin-bottom: 10px;
  font-weight: 600;
}
.qr-scanner__frame {
  position: relative;
  overflow: hidden;
  aspect-ratio: 1;
  background: var(--m3-surface-container-low);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-lg);
}
.qr-scanner video {
  width: 100%;
  height: 100%;
  object-fit: cover;
}
.qr-scanner__hint {
  position: absolute;
  inset: 0;
  display: grid;
  gap: 10px;
  place-content: center;
  padding: 24px;
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-sm);
  text-align: center;
}
</style>
