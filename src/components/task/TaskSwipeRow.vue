<script setup lang="ts">
/** @fileoverview Touch gestures for one task card: long press, tap and swipe actions. */
import { computed, onBeforeUnmount, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { NIcon } from 'naive-ui'
import { PauseOutline, TrashOutline } from '@vicons/ionicons5'
import { triggerHaptic } from '@/composables/useHaptics'
import { useReducedMotion } from '@/composables/useReducedMotion'
import type { Aria2Task } from '@shared/types'

const props = withDefaults(defineProps<{ task: Aria2Task; selectionMode: boolean; disabled?: boolean }>(), {
  disabled: false,
})
const emit = defineEmits<{
  activate: [event: MouseEvent]
  'long-press': [task: Aria2Task]
  'swipe-pause': [task: Aria2Task]
  'swipe-delete': [task: Aria2Task]
}>()

const { t } = useI18n()
const reduceMotion = useReducedMotion()
const SWIPE_THRESHOLD = 64
const MAX_OFFSET = 104
const LONG_PRESS_MS = 480

const offset = ref(0)
const dragging = ref(false)
const direction = ref<'pause' | 'delete' | null>(null)
const hapticFired = ref(false)
const suppressClickUntil = ref(0)
const dismissing = ref(false)
const restoring = ref(false)
const exitFrom = ref(0)
const interactionTarget =
  'button, a, input, textarea, select, [data-no-drag], [data-task-drag-handle], .task-item-actions'

let pointerId: number | null = null
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

function actionStyle(action: 'pause' | 'delete') {
  const visible = direction.value === action
  const progress = Math.min(1, Math.abs(offset.value) / SWIPE_THRESHOLD)
  return {
    opacity: visible ? Math.max(0.35, progress) : 0,
    transform: `translate3d(${(1 - progress) * (action === 'pause' ? -14 : 14)}px, 0, 0)`,
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
  pointerId = null
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
  if (!dragging.value) {
    if (Math.abs(deltaY) > 8 && Math.abs(deltaY) >= Math.abs(deltaX)) {
      clearLongPressTimer()
      pointerId = null
      return
    }
    if (Math.abs(deltaX) < 8 || Math.abs(deltaX) <= Math.abs(deltaY)) return
    clearLongPressTimer()
    dragging.value = true
  }
  offset.value = Math.max(-MAX_OFFSET, Math.min(MAX_OFFSET, deltaX))
  direction.value = offset.value < 0 ? 'delete' : 'pause'
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
  pointerId = null
  if (!shouldAct || !action) {
    offset.value = 0
    return
  }
  if (action === 'pause') {
    offset.value = 0
    emit('swipe-pause', props.task)
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
  resetGesture()
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
    class="task-swipe-row"
    :class="{ 'task-swipe-row--selecting': selectionMode }"
    @pointerdown="onPointerDown"
    @pointermove="onPointerMove"
    @pointerup="onPointerUp"
    @pointercancel="onPointerCancel"
    @click="onClick"
  >
    <div class="task-swipe-action task-swipe-action--pause" :style="actionStyle('pause')" aria-hidden="true">
      <NIcon :size="20"><PauseOutline /></NIcon>
      <span>{{ t('task.pause-task') }}</span>
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
.task-swipe-action--pause {
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
