<script setup lang="ts">
/** @fileoverview Touch gestures for one task card: long press, tap and swipe actions. */
import { computed, onBeforeUnmount, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { NIcon } from 'naive-ui'
import { PauseOutline, PlayOutline, ShareSocialOutline, TrashOutline } from '@vicons/ionicons5'
import { triggerHaptic } from '@/composables/useHaptics'
import { usePlatform } from '@/composables/usePlatform'
import { useReducedMotion } from '@/composables/useReducedMotion'
import { TASK_STATUS } from '@shared/constants'
import type { Aria2Task } from '@shared/types'

const props = withDefaults(defineProps<{ task: Aria2Task; selectionMode: boolean; disabled?: boolean }>(), {
  disabled: false,
})
const emit = defineEmits<{
  activate: [event: MouseEvent]
  'long-press': [task: Aria2Task]
  'swipe-primary': [task: Aria2Task]
  'swipe-delete': [task: Aria2Task]
  'swipe-share': [task: Aria2Task]
}>()

const { t } = useI18n()
const reduceMotion = useReducedMotion()
const SWIPE_THRESHOLD = 48
const MAX_OFFSET = 104
const LONG_PRESS_MS = 520
// A real thumb wobbles before it travels sideways, so the drag needs a dead zone
// and a forgiving axis decision. An emulator drag is a perfectly straight line
// and hides both problems, which is why beta6 passed on the AVD but not on a
// physical device.
const DRAG_DEAD_ZONE = 12
const AXIS_DOMINANCE = 1.2
const LONG_PRESS_CANCEL_DISTANCE = 8

const offset = ref(0)
const dragging = ref(false)
const direction = ref<'primary' | 'delete' | null>(null)
const hapticFired = ref(false)
const suppressClickUntil = ref(0)
const dismissing = ref(false)
const restoring = ref(false)
const exitFrom = ref(0)
const interactionTarget =
  'button, a, input, textarea, select, [data-no-drag], [data-task-drag-handle], .task-item-actions'
const rowRef = ref<HTMLElement | null>(null)

let pointerId: number | null = null
let pointerCaptured = false
let startX = 0
let startY = 0
let longPressTimer: number | null = null
let longPressTriggered = false
let dismissTimer: number | null = null
let restoreTimer: number | null = null

const contentStyle = computed<Record<string, string>>(() => {
  const style: Record<string, string> = {}
  if (dismissing.value) {
    style['--swipe-exit-from'] = `${exitFrom.value}px`
  } else if (!restoring.value) {
    style.transform = `translate3d(${offset.value}px, 0, 0)`
    style.transition = dragging.value ? 'none' : 'transform 220ms cubic-bezier(0.2, 0, 0, 1)'
  }
  return style
})

type PrimaryAction = 'send-local' | 'share' | 'resume' | 'pause'

const { isAndroid } = usePlatform()
/** Finished local files can be shared through the system sheet, same as the card menu. */
const canShareFinished = computed(
  () =>
    !props.task.remote &&
    isAndroid.value &&
    (props.task.status === TASK_STATUS.COMPLETE || props.task.status === TASK_STATUS.REMOVED),
)
const primaryAction = computed<PrimaryAction | null>(() => {
  if (props.task.remote) return 'send-local'
  if (canShareFinished.value) return 'share'
  if (props.task.status === TASK_STATUS.PAUSED) return 'resume'
  if (props.task.status === TASK_STATUS.ACTIVE || props.task.status === TASK_STATUS.WAITING) return 'pause'
  return null
})
const primaryDisabled = computed(() => primaryAction.value === null)
const primaryIcon = computed(() => {
  switch (primaryAction.value) {
    case 'share':
      return ShareSocialOutline
    case 'resume':
      return PlayOutline
    case 'pause':
      return PauseOutline
    default:
      return PlayOutline
  }
})
const primaryLabel = computed(() => {
  switch (primaryAction.value) {
    case 'send-local':
      return t('task.remote-send-local')
    case 'share':
      return t('task.share-file')
    case 'resume':
      return t('task.resume-task')
    default:
      return t('task.pause-task')
  }
})

function actionStyle(action: 'primary' | 'delete') {
  const visible = direction.value === action
  const progress = Math.min(1, Math.abs(offset.value) / SWIPE_THRESHOLD)
  return {
    opacity: visible ? Math.max(0.35, progress) : 0,
    transform: `translate3d(${(1 - progress) * (action === 'primary' ? -14 : 14)}px, 0, 0)`,
  }
}

function clearLongPressTimer() {
  if (longPressTimer !== null) {
    window.clearTimeout(longPressTimer)
    longPressTimer = null
  }
}

function capturePointer(id: number) {
  if (pointerCaptured) return
  try {
    rowRef.value?.setPointerCapture(id)
    pointerCaptured = true
  } catch {
    // Pointer capture can be rejected for synthetic events; moves still work.
  }
}

function releasePointer(id: number | null) {
  if (id === null || !pointerCaptured) return
  pointerCaptured = false
  try {
    rowRef.value?.releasePointerCapture(id)
  } catch {
    // Capture may already be gone, for example after pointercancel.
  }
}

function clearDismissTimers() {
  if (dismissTimer !== null) {
    window.clearTimeout(dismissTimer)
    dismissTimer = null
  }
  if (restoreTimer !== null) {
    window.clearTimeout(restoreTimer)
    restoreTimer = null
  }
}

function resetGesture() {
  clearLongPressTimer()
  clearDismissTimers()
  releasePointer(pointerId)
  pointerId = null
  pointerCaptured = false
  dragging.value = false
  direction.value = null
  hapticFired.value = false
  dismissing.value = false
  restoring.value = false
  offset.value = 0
}

function onPointerDown(event: PointerEvent) {
  if (props.selectionMode || props.disabled || event.pointerType === 'mouse' || event.button !== 0) return
  const target = event.target
  if (target instanceof Element && target.closest(interactionTarget)) return

  pointerId = event.pointerId
  // Capture is taken only once a horizontal drag is locked in. Capturing on
  // pointerdown hands the whole gesture to the row, which cancels the list
  // scroller and, on Android, makes the WebView emit pointercancel.
  startX = event.clientX
  startY = event.clientY
  longPressTriggered = false
  clearLongPressTimer()
  longPressTimer = window.setTimeout(() => {
    longPressTimer = null
    longPressTriggered = true
    suppressClickUntil.value = Date.now() + 700
    triggerHaptic()
    emit('long-press', props.task)
  }, LONG_PRESS_MS)
}

function onPointerMove(event: PointerEvent) {
  if (event.pointerId !== pointerId || longPressTriggered) return
  const deltaX = event.clientX - startX
  const deltaY = event.clientY - startY
  const absX = Math.abs(deltaX)
  const absY = Math.abs(deltaY)
  if (!dragging.value) {
    // Any real movement proves the finger is not resting for a long press.
    if (absX > LONG_PRESS_CANCEL_DISTANCE || absY > LONG_PRESS_CANCEL_DISTANCE) {
      clearLongPressTimer()
    }
    if (absX < DRAG_DEAD_ZONE && absY < DRAG_DEAD_ZONE) return
    // Leave clearly vertical gestures to the list scroller. The dominance ratio
    // matters: a horizontal swipe usually starts with a few pixels of vertical
    // travel, so a strict |dy| >= |dx| test cancels most real swipes.
    if (absY > absX * AXIS_DOMINANCE) {
      pointerId = null
      return
    }
    if (absX < DRAG_DEAD_ZONE) return
    clearLongPressTimer()
    dragging.value = true
    capturePointer(event.pointerId)
  }
  offset.value = Math.max(-MAX_OFFSET, Math.min(MAX_OFFSET, deltaX))
  if (primaryDisabled.value && offset.value > 0) offset.value = 0
  direction.value = offset.value < 0 ? 'delete' : 'primary'
  if (!hapticFired.value && Math.abs(offset.value) >= SWIPE_THRESHOLD) {
    hapticFired.value = true
    triggerHaptic()
  }
}

function finishSwipe() {
  const action = direction.value
  const shouldAct = Math.abs(offset.value) >= SWIPE_THRESHOLD
  const releaseOffset = offset.value
  suppressClickUntil.value = Date.now() + 480
  dragging.value = false
  direction.value = null
  releasePointer(pointerId)
  pointerId = null
  if (!shouldAct || !action) {
    offset.value = 0
    return
  }
  if (action === 'primary') {
    offset.value = 0
    if (primaryAction.value === 'share') emit('swipe-share', props.task)
    else emit('swipe-primary', props.task)
    return
  }
  if (reduceMotion.value) {
    offset.value = 0
    emit('swipe-delete', props.task)
    return
  }

  clearDismissTimers()
  exitFrom.value = releaseOffset
  offset.value = 0
  dismissing.value = true
  emit('swipe-delete', props.task)
  dismissTimer = window.setTimeout(() => {
    dismissTimer = null
    dismissing.value = false
    restoring.value = true
    restoreTimer = window.setTimeout(() => {
      restoring.value = false
      restoreTimer = null
    }, 420)
  }, 560)
}

function onPointerUp(event: PointerEvent) {
  if (event.pointerId !== pointerId) return
  releasePointer(event.pointerId)
  clearLongPressTimer()
  if (longPressTriggered) {
    resetGesture()
    longPressTriggered = false
    return
  }
  if (dragging.value) {
    finishSwipe()
    return
  }
  resetGesture()
}

function onPointerCancel(event: PointerEvent) {
  if (event.pointerId !== pointerId) return
  releasePointer(event.pointerId)
  resetGesture()
}

/**
 * The WebView can revoke capture mid-gesture, for example when the scroller
 * takes over or the app is backgrounded. Without this handler the offset would
 * stay stuck at its last dragged position.
 */
function onLostPointerCapture(event: PointerEvent) {
  if (event.pointerId !== pointerId) return
  pointerCaptured = false
  if (longPressTriggered) {
    resetGesture()
    longPressTriggered = false
    return
  }
  if (dragging.value) {
    finishSwipe()
    return
  }
  resetGesture()
}

function onContextMenu(event: MouseEvent) {
  if (dragging.value || Date.now() < suppressClickUntil.value) event.preventDefault()
}

function onClick(event: MouseEvent) {
  if (Date.now() < suppressClickUntil.value || longPressTriggered) {
    event.preventDefault()
    event.stopPropagation()
    longPressTriggered = false
    return
  }
  const target = event.target
  if (target instanceof Element && target.closest(interactionTarget)) return
  emit('activate', event)
}

onBeforeUnmount(() => {
  clearLongPressTimer()
  clearDismissTimers()
})
</script>

<template>
  <div
    ref="rowRef"
    class="task-swipe-row"
    :class="{ 'task-swipe-row--selecting': selectionMode }"
    @pointerdown="onPointerDown"
    @pointermove="onPointerMove"
    @pointerup="onPointerUp"
    @pointercancel="onPointerCancel"
    @lostpointercapture="onLostPointerCapture"
    @contextmenu="onContextMenu"
    @click="onClick"
  >
    <div
      v-if="!primaryDisabled"
      class="task-swipe-action task-swipe-action--primary"
      :style="actionStyle('primary')"
      aria-hidden="true"
    >
      <NIcon :size="20"><component :is="primaryIcon" /></NIcon>
      <span>{{ primaryLabel }}</span>
    </div>
    <div class="task-swipe-action task-swipe-action--delete" :style="actionStyle('delete')" aria-hidden="true">
      <NIcon :size="20"><TrashOutline /></NIcon>
      <span>{{ t('task.delete-task') }}</span>
    </div>
    <div
      class="task-swipe-content"
      :class="{
        'task-swipe-content--dismissing': dismissing,
        'task-swipe-content--restoring': restoring,
      }"
      :style="contentStyle"
    >
      <slot />
    </div>
  </div>
</template>

<style scoped>
.task-swipe-row {
  position: relative;
  overflow: hidden;
  border-radius: var(--radius-sm);
  touch-action: pan-y;
  -webkit-touch-callout: none;
  -webkit-user-select: none;
  user-select: none;
}
.task-swipe-row--selecting {
  touch-action: manipulation;
}
.task-swipe-content {
  position: relative;
  z-index: 1;
  will-change: transform;
}
.task-swipe-content--dismissing {
  animation: task-swipe-dismiss 300ms cubic-bezier(0.34, 1.16, 0.64, 1) forwards;
}
.task-swipe-content--restoring {
  animation: task-swipe-restore 420ms cubic-bezier(0.34, 1.12, 0.64, 1);
}
@keyframes task-swipe-dismiss {
  0% {
    opacity: 1;
    transform: translate3d(var(--swipe-exit-from), 0, 0) scale(1);
  }
  62% {
    opacity: 0.92;
    transform: translate3d(calc(-100% - 20px), 0, 0) scale(0.986);
  }
  78% {
    opacity: 0.78;
    transform: translate3d(calc(-100% - 10px), 0, 0) scale(0.993);
  }
  100% {
    opacity: 0;
    transform: translate3d(calc(-100% - 28px), 0, 0) scale(0.98);
  }
}
@keyframes task-swipe-restore {
  0% {
    opacity: 0;
    transform: translate3d(calc(-100% - 28px), 0, 0) scale(0.98);
  }
  38% {
    opacity: 1;
  }
  100% {
    opacity: 1;
    transform: translate3d(0, 0, 0) scale(1);
  }
}
.task-swipe-action {
  position: absolute;
  top: 0;
  bottom: 0;
  display: flex;
  align-items: center;
  gap: 6px;
  width: 52%;
  padding: 0 20px;
  font-size: var(--font-size-sm);
  font-weight: 600;
  pointer-events: none;
  transition:
    opacity 120ms linear,
    transform 180ms cubic-bezier(0.2, 0, 0, 1);
}
.task-swipe-action--primary {
  left: 0;
  justify-content: flex-start;
  color: var(--m3-status-success);
  background: color-mix(in srgb, var(--m3-status-success) 18%, var(--m3-surface-container));
}
.task-swipe-action--delete {
  right: 0;
  justify-content: flex-end;
  color: var(--m3-error);
  background: color-mix(in srgb, var(--m3-error) 18%, var(--m3-surface-container));
}
</style>
