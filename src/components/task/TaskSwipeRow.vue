<script setup lang="ts">
/**
 * @fileoverview Touch gestures for one task card: long press, tap and swipe actions.
 *
 * Touch input is driven from `touchstart`/`touchmove`/`touchend` instead of
 * pointer events. Android's WebView delivers `pointercancel` as soon as the
 * scroller or SortableJS touches the gesture, which silently aborted every
 * swipe on real devices while emulator drags kept working. A non-passive
 * `touchmove` listener lets the row claim the gesture the moment it locks
 * horizontally, so the list stops scrolling and the swipe survives.
 */
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { NIcon } from 'naive-ui'
import { PauseOutline, PlayOutline, ShareSocialOutline, TrashOutline } from '@vicons/ionicons5'
import { triggerHaptic } from '@/composables/useHaptics'
import { usePlatform } from '@/composables/usePlatform'
import { useReducedMotion } from '@/composables/useReducedMotion'
import { TASK_STATUS } from '@shared/constants'
import { logger } from '@shared/logger'
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
const { isAndroid } = usePlatform()
const reduceMotion = useReducedMotion()
const SWIPE_THRESHOLD = 48
const MAX_OFFSET = 104
const LONG_PRESS_MS = 520
// A real thumb wobbles before it travels sideways, so the drag needs a dead zone
// and a forgiving axis decision. An emulator drag is a perfectly straight line
// and hides both problems.
const DRAG_DEAD_ZONE = 12
const AXIS_DOMINANCE = 1.2
const LONG_PRESS_CANCEL_DISTANCE = 8

type PrimaryAction = 'send-local' | 'share' | 'resume' | 'pause'

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

let active = false
let startX = 0
let startY = 0
let lastDeltaX = 0
let longPressTimer: number | null = null
let longPressTriggered = false
let dismissTimer: number | null = null
let restoreTimer: number | null = null

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
  active = false
  dragging.value = false
  direction.value = null
  hapticFired.value = false
  dismissing.value = false
  restoring.value = false
  offset.value = 0
}

function beginGesture(target: EventTarget | null, x: number, y: number): void {
  if (props.selectionMode || props.disabled) return
  if (target instanceof Element && target.closest(interactionTarget)) return
  if (active) return

  active = true
  startX = x
  startY = y
  lastDeltaX = 0
  longPressTriggered = false
  clearLongPressTimer()
  longPressTimer = window.setTimeout(() => {
    longPressTimer = null
    if (dragging.value) return
    longPressTriggered = true
    suppressClickUntil.value = Date.now() + 700
    triggerHaptic()
    emit('long-press', props.task)
  }, LONG_PRESS_MS)
}

/** Returns true when the move was consumed by the swipe. */
function moveGesture(x: number, y: number): boolean {
  if (!active || longPressTriggered) return false
  const deltaX = x - startX
  const deltaY = y - startY
  const absX = Math.abs(deltaX)
  const absY = Math.abs(deltaY)

  if (!dragging.value) {
    // Any real movement proves the finger is not resting for a long press.
    if (absX > LONG_PRESS_CANCEL_DISTANCE || absY > LONG_PRESS_CANCEL_DISTANCE) clearLongPressTimer()
    if (absX < DRAG_DEAD_ZONE && absY < DRAG_DEAD_ZONE) return false
    // Leave clearly vertical gestures to the list scroller.
    if (absY > absX * AXIS_DOMINANCE) {
      active = false
      return false
    }
    if (absX < DRAG_DEAD_ZONE) return false
    clearLongPressTimer()
    dragging.value = true
    logger.debug('TaskSwipe.lock', `gid=${props.task.gid} dx=${Math.round(deltaX)} dy=${Math.round(deltaY)}`)
  }

  lastDeltaX = Math.max(-MAX_OFFSET, Math.min(MAX_OFFSET, deltaX))
  offset.value = primaryDisabled.value && lastDeltaX > 0 ? 0 : lastDeltaX
  direction.value = offset.value < 0 ? 'delete' : 'primary'
  if (!hapticFired.value && Math.abs(offset.value) >= SWIPE_THRESHOLD) {
    hapticFired.value = true
    triggerHaptic()
  }
  return true
}

function finishGesture(): void {
  if (!active) return
  active = false
  clearLongPressTimer()
  if (longPressTriggered) {
    longPressTriggered = false
    resetGesture()
    return
  }
  if (!dragging.value) {
    resetGesture()
    return
  }

  const action = direction.value
  const shouldAct = Math.abs(offset.value) >= SWIPE_THRESHOLD
  const releaseOffset = offset.value
  suppressClickUntil.value = Date.now() + 480
  dragging.value = false
  direction.value = null
  if (!shouldAct || !action) {
    offset.value = 0
    return
  }
  logger.debug('TaskSwipe.commit', `gid=${props.task.gid} action=${action} offset=${Math.round(releaseOffset)}`)
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

/**
 * An interrupted gesture that already passed the threshold still counts: the
 * scroller or the drag library taking over must not swallow the user's intent.
 */
function interruptGesture(): void {
  if (!dragging.value) {
    resetGesture()
    return
  }
  finishGesture()
}

function onTouchStart(event: TouchEvent) {
  const touch = event.touches[0]
  if (!touch) return
  beginGesture(event.target, touch.clientX, touch.clientY)
}

function onTouchMove(event: TouchEvent) {
  const touch = event.touches[0]
  if (!touch) return
  if (moveGesture(touch.clientX, touch.clientY) && event.cancelable) {
    // Claim the gesture once it is horizontal so the list stops scrolling.
    event.preventDefault()
  }
}

function onTouchEnd() {
  finishGesture()
}

function onTouchCancel() {
  interruptGesture()
}

function onPointerDown(event: PointerEvent) {
  if (event.pointerType === 'touch' || event.button !== 0) return
  beginGesture(event.target, event.clientX, event.clientY)
}

function onPointerMove(event: PointerEvent) {
  if (event.pointerType === 'touch') return
  moveGesture(event.clientX, event.clientY)
}

function onPointerUp(event: PointerEvent) {
  if (event.pointerType === 'touch') return
  finishGesture()
}

function onPointerCancel(event: PointerEvent) {
  if (event.pointerType === 'touch') return
  interruptGesture()
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

function onContextMenu(event: MouseEvent) {
  if (dragging.value || Date.now() < suppressClickUntil.value) event.preventDefault()
}

onMounted(() => {
  const element = rowRef.value
  if (!element) return
  // Non-passive so a horizontal swipe can stop the scroller.
  element.addEventListener('touchstart', onTouchStart, { passive: true })
  element.addEventListener('touchmove', onTouchMove, { passive: false })
  element.addEventListener('touchend', onTouchEnd, { passive: true })
  element.addEventListener('touchcancel', onTouchCancel, { passive: true })
})

onBeforeUnmount(() => {
  const element = rowRef.value
  if (element) {
    element.removeEventListener('touchstart', onTouchStart)
    element.removeEventListener('touchmove', onTouchMove)
    element.removeEventListener('touchend', onTouchEnd)
    element.removeEventListener('touchcancel', onTouchCancel)
  }
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
