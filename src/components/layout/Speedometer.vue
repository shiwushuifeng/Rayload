<script setup lang="ts">
/**
 * @fileoverview Smart Capsule Speedometer — floating download status capsule
 * with a centered live-status panel for speeds, bulk actions and limits.
 *
 * Capsule states:
 *   IDLE (muted)       — no active tasks, lock-open + ∞
 *   ACTIVE (primary)   — active tasks, lock-open + ∞
 *   LIMITED (tertiary) — limit active, lock-closed + values
 *   DOCKED             — slim speed-only chip flush with the screen edge
 *
 * Interactions:
 *   Left-click  — open the centered live status panel; the capsule hides
 *                 while the panel is open and returns when it closes
 *   Drag edge  — pull the docked speed chip back out into the full capsule
 *   Right-click — same panel (long-press on Android)
 */
import { computed, ref, onMounted, onBeforeUnmount, watch } from 'vue'
import { useOnline } from '@vueuse/core'
import { speedUnitOptions as speedUnits } from '@shared/utils/format'
import { useI18n } from 'vue-i18n'
import { useAppStore } from '@/stores/app'
import { usePreferenceStore } from '@/stores/preference'
import { changeGlobalOption, isEngineReady } from '@/api/aria2'
import { bytesToSize } from '@shared/utils'
import { NIcon, NInputNumber, NSelect, NButton, NSwitch, NText, NCollapseTransition } from 'naive-ui'
import AnimatedNumber from '@/components/common/AnimatedNumber.vue'
import {
  SpeedometerOutline,
  ArrowUpOutline,
  ArrowDownOutline,
  LockClosedOutline,
  TimerOutline,
  PowerOutline,
  RefreshOutline,
  ChevronDownOutline,
  CloseOutline,
  PauseOutline,
  PlayOutline,
} from '@vicons/ionicons5'
import {
  formatLimitBadge,
  parseSpeedLimitValue,
  buildSpeedLimitString,
  toggleSpeedLimit,
  applyCustomLimit,
} from '@/composables/useSpeedLimiter'
import { useAppMessage } from '@/composables/useAppMessage'
import { useEngineRestart } from '@/composables/useEngineRestart'
import { useEngineStore } from '@/stores/engine'
import { useTaskStore } from '@/stores/task'
import { logger } from '@shared/logger'

interface NetworkInformationLike {
  type?: string
  effectiveType?: string
  addEventListener?: (type: 'change', listener: () => void) => void
  removeEventListener?: (type: 'change', listener: () => void) => void
}

const { t } = useI18n()
const appStore = useAppStore()
const preferenceStore = usePreferenceStore()
const message = useAppMessage()
const engineStore = useEngineStore()
const taskStore = useTaskStore()
const { confirmManualRestart } = useEngineRestart()
const online = useOnline()

const stat = computed(() => appStore.stat)
const isIdle = computed(() => stat.value.numActive === 0)
const isLimited = computed(() => !!preferenceStore.config.speedLimitEnabled)
const isScheduleActive = computed(() => !!preferenceStore.config.speedScheduleEnabled)
const shutdownWhenComplete = computed(() => !!preferenceStore.config.shutdownWhenComplete)
/** Clock badge only when schedule is actually effective (both switches ON). */
const isScheduleEffective = computed(() => isLimited.value && isScheduleActive.value)
const formatSpeed = (value: number) => bytesToSize(Math.max(0, Math.round(value)))

const networkConnection =
  typeof navigator === 'undefined'
    ? undefined
    : (navigator as Navigator & { connection?: NetworkInformationLike }).connection
const networkType = ref('')

function refreshNetworkType() {
  networkType.value = networkConnection?.type || networkConnection?.effectiveType || ''
}

const networkStatusLabel = computed(() => {
  if (!online.value) return t('app.speedometer-offline')
  const type = networkType.value
  if (type === 'wifi') return t('app.speedometer-wifi')
  if (['cellular', 'slow-2g', '2g', '3g', '4g'].includes(type)) {
    const generation = type === 'cellular' ? '' : ` · ${type.toUpperCase()}`
    return `${t('app.speedometer-cellular')}${generation}`
  }
  return t('app.speedometer-online')
})

const connectionQuality = computed<{
  label: string
  tone: 'success' | 'waiting' | 'error' | 'muted'
  level: number
}>(() => {
  if (!online.value) return { label: t('app.speedometer-offline'), tone: 'error', level: 0 }
  if (stat.value.numActive === 0) {
    return { label: t('app.speedometer-quality-idle'), tone: 'muted', level: 0 }
  }
  const speed = stat.value.downloadSpeed
  if (speed >= 2 * 1024 * 1024) {
    return { label: t('app.speedometer-quality-excellent'), tone: 'success', level: 4 }
  }
  if (speed >= 512 * 1024) {
    return { label: t('app.speedometer-quality-good'), tone: 'success', level: 3 }
  }
  if (speed > 0) {
    return { label: t('app.speedometer-quality-fair'), tone: 'waiting', level: 2 }
  }
  return { label: t('app.speedometer-quality-weak'), tone: 'error', level: 1 }
})

// ── Draggable floating dock ─────────────────────────────────────────

const DOCK_POSITION_KEY = 'rayload-speedometer-position-v2'
const DOCK_EDGE_GAP = 8
const DOCK_SNAP_THRESHOLD = 28
const COLLAPSED_CAPSULE_WIDTH = 100
const DEFAULT_DOCK_Y_RATIO = 0.8
const dockRef = ref<HTMLElement | null>(null)
const dockPosition = ref<{ x: number; y: number } | null>(null)
const viewportWidth = ref(typeof window === 'undefined' ? 0 : window.innerWidth)
const isDockCollapsed = ref(true)
const dockSide = ref<'left' | 'right'>('left')
const isSnapping = ref(false)
let snapTimer: ReturnType<typeof setTimeout> | null = null

interface DockDragState {
  pointerId: number
  startX: number
  startY: number
  originX: number
  originY: number
  pendingX: number
  pendingY: number
  maxX: number
  maxY: number
  moved: boolean
}

let dockDrag: DockDragState | null = null
let dragFrame = 0
let lastDragEndAt = 0
let lastContextMenuAt = 0
/** Ignore synthetic clicks fired by the activity launch / WebView restore. */
const INTERACTION_READY_DELAY_MS = 450
let interactionReadyAt = 0

const dockStyle = computed(() => {
  if (isDockCollapsed.value) {
    const maxY = Math.max(DOCK_EDGE_GAP, window.innerHeight - 46 - DOCK_EDGE_GAP)
    const y = Math.min(
      Math.max(DOCK_EDGE_GAP, dockPosition.value?.y ?? Math.round(window.innerHeight * DEFAULT_DOCK_Y_RATIO)),
      maxY,
    )
    return {
      left: dockSide.value === 'left' ? '0px' : `calc(100vw - ${COLLAPSED_CAPSULE_WIDTH}px)`,
      top: `${y}px`,
      right: 'auto',
      bottom: 'auto',
    }
  }
  if (!dockPosition.value) return undefined
  // Clamp at render time with the live capsule width. The saved coordinate may
  // have been produced while the chip was collapsed, which is much narrower
  // than the expanded capsule and would otherwise push it off-screen.
  const width = capsuleWidth.value || 205
  const maxX = Math.max(DOCK_EDGE_GAP, viewportWidth.value - width - DOCK_EDGE_GAP)
  const maxY = Math.max(DOCK_EDGE_GAP, window.innerHeight - 46 - DOCK_EDGE_GAP)
  return {
    left: `${Math.min(Math.max(DOCK_EDGE_GAP, dockPosition.value.x), maxX)}px`,
    top: `${Math.min(Math.max(DOCK_EDGE_GAP, dockPosition.value.y), maxY)}px`,
    right: 'auto',
    bottom: 'auto',
  }
})

function dockSize() {
  const rect = dockRef.value?.getBoundingClientRect()
  return { width: rect?.width || 205, height: rect?.height || 46 }
}

function clampDockPosition(x: number, y: number) {
  const { width, height } = dockSize()
  const maxX = Math.max(DOCK_EDGE_GAP, window.innerWidth - width - DOCK_EDGE_GAP)
  const maxY = Math.max(DOCK_EDGE_GAP, window.innerHeight - height - DOCK_EDGE_GAP)
  return {
    x: Math.min(Math.max(DOCK_EDGE_GAP, x), maxX),
    y: Math.min(Math.max(DOCK_EDGE_GAP, y), maxY),
  }
}

function restoreDockPosition() {
  try {
    const saved = JSON.parse(localStorage.getItem(DOCK_POSITION_KEY) ?? '') as {
      x?: unknown
      y?: unknown
      collapsed?: unknown
      side?: unknown
    }
    if (typeof saved.x === 'number' && typeof saved.y === 'number') {
      dockPosition.value = clampDockPosition(saved.x, saved.y)
      dockSide.value = saved.side === 'right' ? 'right' : 'left'
      isDockCollapsed.value = saved.collapsed === true
    }
  } catch {
    // Ignore malformed or unavailable local storage.
  }
}

function persistDockPosition() {
  if (!dockPosition.value) return
  try {
    localStorage.setItem(
      DOCK_POSITION_KEY,
      JSON.stringify({
        x: dockPosition.value.x,
        y: dockPosition.value.y,
        collapsed: isDockCollapsed.value,
        side: dockSide.value,
      }),
    )
  } catch {
    // Position persistence is best effort.
  }
}

function handleDockPointerDown(event: PointerEvent) {
  if (event.button !== 0) return
  isSnapping.value = false
  if (snapTimer !== null) {
    clearTimeout(snapTimer)
    snapTimer = null
  }
  if (showPopover.value) showPopover.value = false
  const rect = dockRef.value?.getBoundingClientRect()
  if (!rect) return
  // The docked chip is flush with the edge, so its visual rect — not the old
  // pre-snap coordinates — is the drag origin.
  if (!dockPosition.value || isDockCollapsed.value) {
    dockPosition.value = { x: rect.left, y: rect.top }
  }
  const dragWidth = isDockCollapsed.value ? capsuleWidth.value || 205 : rect.width
  dockDrag = {
    pointerId: event.pointerId,
    startX: event.clientX,
    startY: event.clientY,
    originX: dockPosition.value.x,
    originY: dockPosition.value.y,
    pendingX: dockPosition.value.x,
    pendingY: dockPosition.value.y,
    // Cache the movement bounds once so pointermove never forces a layout read.
    maxX: Math.max(DOCK_EDGE_GAP, window.innerWidth - dragWidth - DOCK_EDGE_GAP),
    maxY: Math.max(DOCK_EDGE_GAP, window.innerHeight - rect.height - DOCK_EDGE_GAP),
    moved: false,
  }
  capsuleRef.value?.setPointerCapture(event.pointerId)
}

function flushDockDrag() {
  if (dragFrame !== 0) {
    cancelAnimationFrame(dragFrame)
    dragFrame = 0
  }
  if (!dockDrag) return
  dockPosition.value = { x: dockDrag.pendingX, y: dockDrag.pendingY }
}

function handleDockPointerMove(event: PointerEvent) {
  if (!dockDrag || event.pointerId !== dockDrag.pointerId) return
  const deltaX = event.clientX - dockDrag.startX
  const deltaY = event.clientY - dockDrag.startY
  if (!dockDrag.moved && Math.hypot(deltaX, deltaY) < 4) return
  dockDrag.moved = true
  // Pulling the docked chip away from the edge restores the full capsule.
  if (isDockCollapsed.value) isDockCollapsed.value = false
  event.preventDefault()
  dockDrag.pendingX = Math.min(Math.max(DOCK_EDGE_GAP, dockDrag.originX + deltaX), dockDrag.maxX)
  dockDrag.pendingY = Math.min(Math.max(DOCK_EDGE_GAP, dockDrag.originY + deltaY), dockDrag.maxY)
  if (dragFrame !== 0) return
  dragFrame = requestAnimationFrame(() => {
    dragFrame = 0
    if (!dockDrag) return
    dockPosition.value = { x: dockDrag.pendingX, y: dockDrag.pendingY }
  })
}

function handleDockPointerUp(event: PointerEvent) {
  if (!dockDrag || event.pointerId !== dockDrag.pointerId) return
  flushDockDrag()
  const moved = dockDrag.moved
  dockDrag = null
  if (capsuleRef.value?.hasPointerCapture(event.pointerId)) capsuleRef.value.releasePointerCapture(event.pointerId)
  if (moved) {
    lastDragEndAt = Date.now()
    snapDockToEdge()
    persistDockPosition()
  }
}

/** Collapse the capsule into a slim speed-only chip when released near an edge. */
function snapDockToEdge() {
  if (!dockPosition.value) return
  const { width } = dockSize()
  const maxX = Math.max(DOCK_EDGE_GAP, window.innerWidth - width - DOCK_EDGE_GAP)
  const leftDistance = dockPosition.value.x - DOCK_EDGE_GAP
  const rightDistance = maxX - dockPosition.value.x
  if (leftDistance > DOCK_SNAP_THRESHOLD && rightDistance > DOCK_SNAP_THRESHOLD) return
  playSnapTransition()
  dockSide.value = leftDistance <= rightDistance ? 'left' : 'right'
  dockPosition.value = {
    x: dockSide.value === 'left' ? DOCK_EDGE_GAP : maxX,
    y: dockPosition.value.y,
  }
  showPopover.value = false
  isDockCollapsed.value = true
}

function playSnapTransition() {
  isSnapping.value = true
  if (snapTimer !== null) clearTimeout(snapTimer)
  snapTimer = setTimeout(() => {
    isSnapping.value = false
    snapTimer = null
  }, 380)
}

function handleDockResize() {
  viewportWidth.value = window.innerWidth
  if (dockPosition.value) dockPosition.value = clampDockPosition(dockPosition.value.x, dockPosition.value.y)
}

// ── Limit badge display ─────────────────────────────────────────────

const dlLimitBadge = computed(() => formatLimitBadge(preferenceStore.config.maxOverallDownloadLimit))
const ulLimitBadge = computed(() => formatLimitBadge(preferenceStore.config.maxOverallUploadLimit))

// ── Adaptive width (grow instant, shrink debounced) ─────────────────
// The outer capsule (.speedometer) uses a JS-driven `width` so that CSS
// `transition: width` animates both expansion and contraction smoothly.
// A separate inner wrapper (.capsule-content) is observed by
// ResizeObserver — it has NO width constraint, so its borderBoxSize
// always reflects true content width (no circular dependency).
// Shrinking is debounced (800 ms) to prevent jitter from speed fluctuations.

const capsuleRef = ref<HTMLElement | null>(null)
const contentRef = ref<HTMLElement | null>(null)
const capsuleWidth = ref(0)
let shrinkTimer: ReturnType<typeof setTimeout> | null = null
let resizeObserver: ResizeObserver | null = null

const SHRINK_DELAY_MS = 1000

const capsuleStyle = computed(() => {
  if (isDockCollapsed.value) return { width: `${COLLAPSED_CAPSULE_WIDTH}px` }
  return capsuleWidth.value > 0 ? { width: `${capsuleWidth.value}px` } : undefined
})

function cancelShrink() {
  if (shrinkTimer !== null) {
    clearTimeout(shrinkTimer)
    shrinkTimer = null
  }
}

onMounted(() => {
  showPopover.value = false
  limitExpanded.value = false
  interactionReadyAt = Date.now() + INTERACTION_READY_DELAY_MS
  window.__rayloadDismissSpeedometerPanel = () => {
    if (!showPopover.value) return false
    showPopover.value = false
    return true
  }
  if (!contentRef.value || !capsuleRef.value) return
  const capsuleEl = capsuleRef.value

  resizeObserver = new ResizeObserver((entries) => {
    // Collapsed mode uses a fixed width; resume measuring after expanding.
    if (isDockCollapsed.value) return
    // Compute padding+border each tick so CSS changes are reflected immediately
    const cs = getComputedStyle(capsuleEl)
    const hPad =
      parseFloat(cs.paddingLeft) +
      parseFloat(cs.paddingRight) +
      parseFloat(cs.borderLeftWidth) +
      parseFloat(cs.borderRightWidth)

    for (const entry of entries) {
      const contentWidth = entry.borderBoxSize?.[0]?.inlineSize ?? entry.contentRect.width
      const totalWidth = Math.ceil(contentWidth + hPad)

      if (totalWidth > capsuleWidth.value) {
        // Growing — apply immediately
        cancelShrink()
        capsuleWidth.value = totalWidth
        handleDockResize()
      } else if (totalWidth < capsuleWidth.value - 2 && shrinkTimer === null) {
        // Shrinking — debounce to prevent jitter (2px hysteresis)
        const target = totalWidth
        shrinkTimer = setTimeout(() => {
          capsuleWidth.value = target
          shrinkTimer = null
          handleDockResize()
        }, SHRINK_DELAY_MS)
      }
    }
  })
  resizeObserver.observe(contentRef.value)
  refreshNetworkType()
  networkConnection?.addEventListener?.('change', refreshNetworkType)
  requestAnimationFrame(restoreDockPosition)
  window.addEventListener('resize', handleDockResize)
})

onBeforeUnmount(() => {
  delete window.__rayloadDismissSpeedometerPanel
  if (dragFrame !== 0) {
    cancelAnimationFrame(dragFrame)
    dragFrame = 0
  }
  cancelShrink()
  if (snapTimer !== null) clearTimeout(snapTimer)
  resizeObserver?.disconnect()
  resizeObserver = null
  networkConnection?.removeEventListener?.('change', refreshNetworkType)
  window.removeEventListener('resize', handleDockResize)
  document.removeEventListener('pointerdown', handleDocumentPointerDown, true)
})

// ── Popover state ───────────────────────────────────────────────────

const showPopover = ref(false)
const limitExpanded = ref(false)
const bulkAction = ref<'pause' | 'resume' | null>(null)
const popoverDlValue = ref(0)
const popoverDlUnit = ref('K')
const popoverUlValue = ref(0)
const popoverUlUnit = ref('K')

const panelRef = ref<HTMLElement | null>(null)

function handleDocumentPointerDown(event: PointerEvent) {
  if (!showPopover.value) return
  const target = event.target instanceof Element ? event.target : null
  if (!target) return
  if (panelRef.value?.contains(target) || capsuleRef.value?.contains(target)) return
  // The scrim dismisses on click so the pointer release cannot fall through.
  if (target.closest('.speedometer-scrim')) return
  // Naive UI teleports select menus outside the panel.
  if (target.closest('.n-base-select-menu, .n-select-menu, .v-binder-follower-content')) return
  showPopover.value = false
}

watch(showPopover, (open) => {
  if (open) {
    document.addEventListener('pointerdown', handleDocumentPointerDown, true)
  } else {
    limitExpanded.value = false
    document.removeEventListener('pointerdown', handleDocumentPointerDown, true)
  }
})

const speedUnitOptions = computed(speedUnits)

function openPopover() {
  const dl = parseSpeedLimitValue(preferenceStore.config.maxOverallDownloadLimit)
  const ul = parseSpeedLimitValue(preferenceStore.config.maxOverallUploadLimit)
  popoverDlValue.value = dl.num
  popoverDlUnit.value = dl.unit
  popoverUlValue.value = ul.num
  popoverUlUnit.value = ul.unit
  limitExpanded.value = false
  showPopover.value = true
}

// ── Dependency injection for composable calls ───────────────────────

function makeDeps() {
  return {
    changeGlobalOption,
    updateAndSave: (partial: Partial<typeof preferenceStore.config>) => preferenceStore.updateAndSave(partial),
  }
}

// ── Left-click: open live status ────────────────────────────────────

async function handleClick() {
  if (Date.now() < interactionReadyAt) return
  if (Date.now() - lastContextMenuAt < 500) return
  if (showPopover.value) {
    showPopover.value = false
    return
  }
  if (isDockCollapsed.value) {
    openPopover()
    return
  }
  if (Date.now() - lastDragEndAt < 300) return
  openPopover()
}

// ── Right-click: open configuration popover ─────────────────────────

function handleContextMenu(e: MouseEvent) {
  e.preventDefault()
  if (Date.now() < interactionReadyAt) return
  lastContextMenuAt = Date.now()
  if (isDockCollapsed.value) {
    openPopover()
    return
  }
  openPopover()
}

// ── Apply custom limit from popover ─────────────────────────────────

async function handleApply() {
  if (!isEngineReady()) return

  // Reject 0/0 — at least one direction must have a non-zero limit
  if (popoverDlValue.value === 0 && popoverUlValue.value === 0) {
    message.warning(t('app.speedometer-enter-values'))
    return
  }

  const dlStr = buildSpeedLimitString(popoverDlValue.value, popoverDlUnit.value)
  const ulStr = buildSpeedLimitString(popoverUlValue.value, popoverUlUnit.value)

  try {
    await applyCustomLimit(dlStr, ulStr, makeDeps())
    showPopover.value = false
    message.success(t('app.speedometer-limit-applied'))
  } catch (e) {
    logger.error('Speedometer.applyLimit', e)
  }
}

const limitToggling = ref(false)

async function handleLimitToggle(enabled: boolean) {
  if (enabled === isLimited.value || limitToggling.value || !engineStore.isReady) return
  limitToggling.value = true
  try {
    const result = await toggleSpeedLimit(preferenceStore.config, makeDeps())
    if (result === 'needs-config') {
      limitExpanded.value = true
      message.info(t('app.speedometer-needs-config-settings'))
      return
    }
    message.success(t(result === 'enabled' ? 'app.speedometer-limit-applied' : 'app.speedometer-limit-removed'))
  } catch (error) {
    logger.error('Speedometer.toggleLimit', error)
  } finally {
    limitToggling.value = false
  }
}

async function handlePauseAll() {
  if (bulkAction.value) return
  bulkAction.value = 'pause'
  try {
    await taskStore.pauseAllTask()
    message.success(t('task.pause-all-task-success'))
  } catch (error) {
    logger.error('Speedometer.pauseAll', error)
    message.error(t('task.pause-all-task-fail'))
  } finally {
    bulkAction.value = null
  }
}

async function handleResumeAll() {
  if (bulkAction.value) return
  bulkAction.value = 'resume'
  try {
    await taskStore.resumeAllTask()
    message.success(t('task.resume-all-task-success'))
  } catch (error) {
    logger.error('Speedometer.resumeAll', error)
    message.error(t('task.resume-all-task-fail'))
  } finally {
    bulkAction.value = null
  }
}

// ── Schedule toggle from popover ───────────────────────────────────────
async function handleScheduleToggle(enabled: boolean) {
  await preferenceStore.updateAndSave({ speedScheduleEnabled: enabled })
  message.success(t(enabled ? 'app.schedule-enabled' : 'app.schedule-disabled'))
}

// ── Shutdown toggle from popover ─────────────────────────────────────
function handleShutdownToggle(enabled: boolean) {
  void preferenceStore.updateAndSave({ shutdownWhenComplete: enabled })
}

function handleRestartEngine() {
  showPopover.value = false
  confirmManualRestart()
}
</script>

<template>
  <div ref="dockRef" class="speedometer-dock" :class="{ 'is-snapping': isSnapping }" :style="dockStyle">
    <div
      ref="capsuleRef"
      :class="[
        'speedometer',
        {
          idle: isIdle,
          limited: isLimited,
          'is-collapsed': isDockCollapsed,
          'is-panel-open': showPopover,
          'dock-left': dockSide === 'left',
          'dock-right': dockSide === 'right',
        },
      ]"
      :style="capsuleStyle"
      draggable="true"
      @click="handleClick"
      @contextmenu="handleContextMenu"
      @dragstart.prevent
      @pointerdown="handleDockPointerDown"
      @pointermove="handleDockPointerMove"
      @pointerup="handleDockPointerUp"
      @pointercancel="handleDockPointerUp"
    >
      <Transition name="lock-pop" appear>
        <div v-if="!isDockCollapsed && isScheduleEffective" class="clock-pill">
          <NIcon :size="12"><TimerOutline /></NIcon>
        </div>
      </Transition>
      <Transition name="lock-pop" appear>
        <div v-if="!isDockCollapsed && isLimited && !isScheduleEffective" class="lock-pill">
          <NIcon :size="12"><LockClosedOutline /></NIcon>
        </div>
      </Transition>
      <div ref="contentRef" :class="['capsule-content', { 'is-collapsed': isDockCollapsed }]">
        <div class="mode">
          <i>
            <NIcon :size="22"><SpeedometerOutline /></NIcon>
          </i>
        </div>
        <div class="value">
          <div class="speed-row upload">
            <div class="speed-text">
              <NIcon :size="10" class="speed-arrow"><ArrowUpOutline /></NIcon>
              <em><AnimatedNumber :value="stat.uploadSpeed" :formatter="formatSpeed" />/s</em>
            </div>
            <div class="limit-zone">
              <span class="limit-sep">┊</span>
              <Transition name="limit-fade" mode="out-in">
                <span v-if="isLimited" :key="'ul-' + ulLimitBadge" class="limit-value">{{ ulLimitBadge }}</span>
                <span v-else key="ul-inf" class="limit-value limit-inf">∞</span>
              </Transition>
            </div>
          </div>
          <div class="speed-row download">
            <div class="speed-text">
              <NIcon :size="10" class="speed-arrow"><ArrowDownOutline /></NIcon>
              <span class="speed-value"><AnimatedNumber :value="stat.downloadSpeed" :formatter="formatSpeed" /></span>
              <span class="speed-unit">/s</span>
            </div>
            <div class="limit-zone">
              <span class="limit-sep">┊</span>
              <Transition name="limit-fade" mode="out-in">
                <span v-if="isLimited" :key="'dl-' + dlLimitBadge" class="limit-value">{{ dlLimitBadge }}</span>
                <span v-else key="dl-inf" class="limit-value limit-inf">∞</span>
              </Transition>
            </div>
          </div>
        </div>
      </div>
      <Transition name="chip-swap">
        <div v-if="isDockCollapsed" class="collapsed-chip" aria-hidden="true">
          <NIcon :size="11" class="collapsed-arrow"><ArrowDownOutline /></NIcon>
          <span class="collapsed-value"
            ><AnimatedNumber :value="stat.downloadSpeed" :formatter="formatSpeed" /><em>/s</em></span
          >
        </div>
      </Transition>
    </div>

    <Teleport to="body">
      <Transition name="speed-scrim">
        <div v-if="showPopover" class="speedometer-scrim" aria-hidden="true" @click="showPopover = false" />
      </Transition>
      <Transition name="speed-panel">
        <div v-if="showPopover" ref="panelRef" class="speedometer-panel" role="dialog" aria-modal="true">
          <!-- Live status and speed-limit panel -->
          <div class="limit-panel">
            <div class="panel-head">
              <span class="panel-eyebrow">{{ t('app.speedometer-live-status') }}</span>
              <div class="panel-head-actions">
                <span class="network-pill" :class="{ 'is-offline': !online }">
                  <i class="network-dot" aria-hidden="true" />
                  {{ networkStatusLabel }}
                </span>
                <NButton
                  quaternary
                  circle
                  size="tiny"
                  class="panel-close"
                  :aria-label="t('app.close')"
                  @click="showPopover = false"
                >
                  <template #icon>
                    <NIcon :size="16"><CloseOutline /></NIcon>
                  </template>
                </NButton>
              </div>
            </div>

            <div class="speed-hero">
              <div class="hero-block">
                <span class="hero-label">{{ t('task.task-download-speed') }}</span>
                <strong class="hero-value"
                  ><AnimatedNumber :value="stat.downloadSpeed" :formatter="formatSpeed" /><em>/s</em></strong
                >
              </div>
              <div class="hero-block hero-block--secondary">
                <span class="hero-label">{{ t('task.task-upload-speed') }}</span>
                <span class="hero-value-secondary"
                  ><AnimatedNumber :value="stat.uploadSpeed" :formatter="formatSpeed" /><em>/s</em></span
                >
              </div>
            </div>

            <div class="status-grid">
              <div class="status-cell">
                <span class="status-label">{{ t('task.status-active') }}</span>
                <strong class="status-value">{{ stat.numActive }}</strong>
              </div>
              <div class="status-cell">
                <span class="status-label">{{ t('task.status-waiting') }}</span>
                <strong class="status-value">{{ stat.numWaiting }}</strong>
              </div>
              <div class="status-cell status-cell--wide">
                <span class="status-label">{{ t('app.speedometer-connection-quality') }}</span>
                <span class="quality-value" :class="'quality-' + connectionQuality.tone">
                  <span class="quality-bars" aria-hidden="true">
                    <i v-for="level in 4" :key="level" :class="{ on: level <= connectionQuality.level }" />
                  </span>
                  {{ connectionQuality.label }}
                </span>
              </div>
            </div>

            <div class="quick-actions">
              <NButton
                size="small"
                secondary
                class="quick-action"
                :loading="bulkAction === 'pause'"
                :disabled="!engineStore.isReady || isIdle || bulkAction !== null"
                @click="handlePauseAll"
              >
                <template #icon>
                  <NIcon><PauseOutline /></NIcon>
                </template>
                {{ t('task.pause-all-task') }}
              </NButton>
              <NButton
                size="small"
                secondary
                class="quick-action"
                :loading="bulkAction === 'resume'"
                :disabled="!engineStore.isReady || bulkAction !== null"
                @click="handleResumeAll"
              >
                <template #icon>
                  <NIcon><PlayOutline /></NIcon>
                </template>
                {{ t('task.resume-all-task') }}
              </NButton>
            </div>

            <div class="limit-card">
              <div
                class="limit-card-head"
                role="button"
                tabindex="0"
                :aria-expanded="limitExpanded"
                @click="limitExpanded = !limitExpanded"
                @keydown.enter.prevent="limitExpanded = !limitExpanded"
                @keydown.space.prevent="limitExpanded = !limitExpanded"
              >
                <span class="limit-card-heading">
                  <NIcon :size="14" class="limit-chevron" :class="{ 'is-open': limitExpanded }">
                    <ChevronDownOutline />
                  </NIcon>
                  <span class="limit-card-title">{{ t('app.speedometer-set-limit') }}</span>
                  <span v-if="!limitExpanded" class="limit-summary">
                    <NIcon :size="10"><ArrowDownOutline /></NIcon>
                    {{ isLimited ? dlLimitBadge : '∞' }}
                    <NIcon :size="10"><ArrowUpOutline /></NIcon>
                    {{ isLimited ? ulLimitBadge : '∞' }}
                  </span>
                </span>
                <span class="limit-switch-wrap" @click.stop>
                  <NSwitch
                    :value="isLimited"
                    size="small"
                    :loading="limitToggling"
                    :disabled="!engineStore.isReady || limitToggling"
                    @update:value="handleLimitToggle"
                  />
                </span>
              </div>

              <NCollapseTransition :show="limitExpanded">
                <div class="limit-card-body">
                  <div class="limit-rows">
                    <div class="limit-row">
                      <span class="limit-row-label">
                        <NIcon :size="12"><ArrowDownOutline /></NIcon>
                        {{ t('app.speedometer-download-limit') }}
                      </span>
                      <NInputNumber
                        v-model:value="popoverDlValue"
                        :min="0"
                        :max="65535"
                        :step="1"
                        size="small"
                        class="limit-number"
                      />
                      <NSelect
                        v-model:value="popoverDlUnit"
                        :options="speedUnitOptions"
                        size="small"
                        class="limit-unit"
                      />
                    </div>
                    <div class="limit-row">
                      <span class="limit-row-label">
                        <NIcon :size="12"><ArrowUpOutline /></NIcon>
                        {{ t('app.speedometer-upload-limit') }}
                      </span>
                      <NInputNumber
                        v-model:value="popoverUlValue"
                        :min="0"
                        :max="65535"
                        :step="1"
                        size="small"
                        class="limit-number"
                      />
                      <NSelect
                        v-model:value="popoverUlUnit"
                        :options="speedUnitOptions"
                        size="small"
                        class="limit-unit"
                      />
                    </div>
                  </div>

                  <div class="limit-switch-row">
                    <span class="limit-row-label">
                      <NIcon :size="12"><TimerOutline /></NIcon>
                      {{ t('preferences.speed-schedule-enabled') }}
                    </span>
                    <NSwitch :value="isScheduleActive" size="small" @update:value="handleScheduleToggle" />
                  </div>
                  <Transition name="hint-slide">
                    <NText v-if="isScheduleActive && !isLimited" depth="3" type="warning" class="limit-hint">
                      {{ t('preferences.schedule-needs-limit') }}
                    </NText>
                  </Transition>
                </div>
              </NCollapseTransition>
            </div>

            <div class="panel-extra">
              <div class="limit-switch-row panel-extra-row">
                <span class="limit-row-label">
                  <NIcon :size="12"><PowerOutline /></NIcon>
                  {{ t('preferences.shutdown-when-complete') }}
                </span>
                <NSwitch :value="shutdownWhenComplete" size="small" @update:value="handleShutdownToggle" />
              </div>
              <div class="panel-extra-actions">
                <NButton
                  size="small"
                  block
                  secondary
                  :loading="engineStore.isBusy"
                  :disabled="engineStore.isBusy"
                  @click="handleRestartEngine"
                >
                  <template #icon>
                    <NIcon><RefreshOutline /></NIcon>
                  </template>
                  {{ t('preferences.engine-restart-btn') }}
                </NButton>
                <NButton type="primary" size="small" block @click="handleApply">
                  {{ t('app.speedometer-apply') }}
                </NButton>
              </div>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>
  </div>
</template>

<style scoped>
.speedometer-dock {
  position: fixed;
  left: auto;
  right: 16px;
  bottom: calc(env(safe-area-inset-bottom, 0px) + 96px);
  z-index: 25;
  pointer-events: none;
  will-change: left, top;
}
.speedometer-dock.is-snapping {
  transition:
    left 0.32s cubic-bezier(0.22, 1, 0.36, 1),
    top 0.32s cubic-bezier(0.22, 1, 0.36, 1);
}

/* ── Teleported config panel (centered) ───────────────────────────── */
.speedometer-scrim {
  position: fixed;
  inset: 0;
  z-index: 1890;
  background: color-mix(in srgb, var(--m3-scrim, #000) 36%, transparent);
}
.speed-scrim-enter-active {
  transition: opacity 0.16s ease;
}
.speed-scrim-leave-active {
  transition: opacity 0.12s ease;
}
.speed-scrim-enter-from,
.speed-scrim-leave-to {
  opacity: 0;
}

.speedometer-panel {
  position: fixed;
  top: 50%;
  left: 50%;
  z-index: 1900;
  box-sizing: border-box;
  width: min(340px, calc(100vw - 32px));
  max-height: calc(100dvh - 32px - env(safe-area-inset-top, 0px) - env(safe-area-inset-bottom, 0px));
  padding: 16px;
  overflow-y: auto;
  overscroll-behavior: contain;
  -webkit-overflow-scrolling: touch;
  color: var(--m3-on-surface);
  border: 1px solid var(--m3-outline-variant);
  border-radius: 20px;
  background: var(--m3-surface-container-high);
  transform: translate(-50%, -50%);
  box-shadow:
    0 24px 60px -16px var(--m3-shadow),
    0 4px 16px -4px var(--m3-shadow);
}
.speed-panel-enter-active {
  transition:
    opacity 0.16s ease,
    transform 0.18s cubic-bezier(0.2, 0, 0, 1);
}
.speed-panel-leave-active {
  transition:
    opacity 0.12s ease,
    transform 0.12s ease;
}
.speed-panel-enter-from,
.speed-panel-leave-to {
  opacity: 0;
  transform: translate(-50%, -50%) scale(0.96);
}

/* ── Base: capsule (outer shell receives min-width) ───────────────── */
.speedometer {
  font-size: var(--font-size-xs);
  position: relative;
  z-index: 20;
  display: flex;
  align-items: center;
  box-sizing: border-box;
  min-width: 56px;
  height: 46px;
  padding: 6px 14px 6px 10px;
  border-radius: var(--radius-full);
  cursor: grab;
  user-select: none;
  pointer-events: auto;
  touch-action: none;
  transition:
    width 0.32s cubic-bezier(0.22, 1, 0.36, 1),
    opacity 0.14s ease,
    transform 0.18s cubic-bezier(0.2, 0, 0, 1),
    border-color 0.2s ease,
    background 0.2s ease;
  border: 1px solid var(--m3-outline-variant);
  background: var(--m3-surface-container);
  overflow: visible;
  will-change: width;
}
.speedometer:hover {
  border-color: var(--m3-outline);
}
.speedometer:active {
  cursor: grabbing;
  transform: scale(0.97);
}

/* The floating ball fades away while the centered panel is open. */
.speedometer.is-panel-open {
  opacity: 0;
  transform: scale(0.9);
  pointer-events: none;
}

/* ── Inner content wrapper (observed by ResizeObserver, no min-width) ─ */
.capsule-content {
  display: flex;
  align-items: center;
  width: fit-content;
  gap: 4px;
  padding-right: 2px;
  transition: opacity 0.12s ease;
}

/* ── Edge-collapsed chip: download speed only ───────────────────── */
.speedometer.is-collapsed {
  min-width: 0;
  height: 46px;
  padding: 0;
  overflow: hidden;
  justify-content: center;
  cursor: pointer;
  background: color-mix(in srgb, var(--m3-surface-container) 90%, transparent);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  box-shadow:
    0 3px 14px -4px var(--m3-shadow),
    inset 0 1px 0 color-mix(in srgb, var(--m3-on-surface) 6%, transparent);
}
.speedometer.is-collapsed.dock-left {
  --chip-enter-x: -10px;
  --chip-origin-x: left center;
  border-top-left-radius: 0;
  border-bottom-left-radius: 0;
  border-top-right-radius: 16px;
  border-bottom-right-radius: 16px;
  border-left-color: transparent;
}
.speedometer.is-collapsed.dock-right {
  --chip-enter-x: 10px;
  --chip-origin-x: right center;
  border-top-right-radius: 0;
  border-bottom-right-radius: 0;
  border-top-left-radius: 16px;
  border-bottom-left-radius: 16px;
  border-right-color: transparent;
}
.capsule-content.is-collapsed {
  width: 0;
  padding: 0;
  overflow: hidden;
  opacity: 0;
  pointer-events: none;
}
.collapsed-chip {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 3px;
  padding: 0 10px;
  min-width: 0;
  color: var(--m3-primary);
  font-size: var(--font-size-sm);
  font-weight: 600;
  font-variant-numeric: tabular-nums;
  white-space: nowrap;
  transform-origin: var(--chip-origin-x, center);
  transition: color 0.2s ease;
}
.collapsed-chip::after {
  content: '';
  position: absolute;
  top: 50%;
  width: 3px;
  height: 16px;
  border-radius: 999px;
  background: currentColor;
  opacity: 0.28;
  transform: translateY(-50%);
}
.speedometer.is-collapsed.dock-left .collapsed-chip::after {
  right: 5px;
}
.speedometer.is-collapsed.dock-right .collapsed-chip::after {
  left: 5px;
}
.speedometer.is-collapsed.limited .collapsed-chip {
  color: var(--m3-tertiary);
}
.collapsed-chip .collapsed-arrow {
  flex-shrink: 0;
  opacity: 0.7;
}
.collapsed-value {
  overflow: hidden;
  text-overflow: clip;
  white-space: nowrap;
}
.collapsed-value em {
  margin-left: 1px;
  font-size: 10px;
  font-style: normal;
  font-weight: 500;
  opacity: 0.6;
}
.chip-swap-enter-active {
  transition:
    opacity 0.18s ease 0.04s,
    transform 0.24s cubic-bezier(0.22, 1, 0.36, 1) 0.02s;
}
.chip-swap-leave-active {
  transition:
    opacity 0.1s ease,
    transform 0.14s ease;
}
.chip-swap-enter-from,
.chip-swap-leave-to {
  opacity: 0;
  transform: translateX(var(--chip-enter-x, 0)) scale(0.94);
}

/* Magnetic settle: a soft squash toward the docked edge instead of a
   straight width shrink. */
.speedometer-dock.is-snapping .speedometer.is-collapsed {
  animation: speedometer-dock-settle 0.38s cubic-bezier(0.22, 1, 0.36, 1);
}
.speedometer.is-collapsed.dock-left {
  transform-origin: left center;
}
.speedometer.is-collapsed.dock-right {
  transform-origin: right center;
}
@keyframes speedometer-dock-settle {
  0% {
    transform: scaleX(1);
  }
  35% {
    transform: scaleX(1.045) scaleY(0.98);
  }
  68% {
    transform: scaleX(0.985) scaleY(1.005);
  }
  100% {
    transform: scaleX(1) scaleY(1);
  }
}

/* ── IDLE — compact capsule ────────────────────────────────────── */
.speedometer.idle .mode i {
  color: var(--m3-outline);
  transform: rotate(-15deg);
}
.speedometer.idle .value {
  opacity: 0.55;
}
.speedometer.idle .speed-text {
  flex: 0;
  width: 0;
  opacity: 0;
  gap: 0;
}
.speedometer.idle .speed-row.download {
  color: var(--m3-outline);
}
.speedometer.idle .speed-row.download .speed-arrow {
  color: var(--m3-outline);
}

/* ── LIMITED — tertiary color accent ─────────────────────────────── */
.speedometer.limited {
  border-color: color-mix(in srgb, var(--m3-tertiary) 32%, transparent);
}
.speedometer.limited:hover {
  border-color: var(--m3-tertiary);
}
.speedometer.limited .mode i {
  color: var(--m3-tertiary);
}
.speedometer.limited .speed-row.download {
  color: var(--m3-tertiary);
}
.speedometer.limited .speed-row.download .speed-arrow {
  color: var(--m3-tertiary);
}

/* ── Common ──────────────────────────────────────────────────────── */
.speedometer em {
  font-style: normal;
}
.mode {
  flex-shrink: 0;
}
.mode i {
  font-size: 22px;
  font-style: normal;
  display: flex;
  align-items: center;
  justify-content: center;
  box-sizing: border-box;
  width: 30px;
  height: 30px;
  padding: 2px;
  position: relative;
  color: var(--m3-primary);
  transition:
    transform 0.35s ease,
    color 0.2s ease;
  transform: rotate(0deg);
}
.value {
  overflow: clip visible;
  width: 100%;
  white-space: nowrap;
  text-overflow: ellipsis;
  opacity: 1;
  transition: opacity 0.3s ease;
}
.speed-row {
  display: flex;
  align-items: center;
}
.speed-text {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 3px;
  overflow: hidden;
  min-width: 0;
  transition:
    width 0.4s cubic-bezier(0.4, 0, 0.2, 1),
    opacity 0.3s ease,
    gap 0.3s ease;
}
.speed-arrow {
  flex-shrink: 0;
  opacity: 0.7;
}
.speed-row.upload {
  color: var(--m3-outline);
}
.speed-row.upload em {
  font-style: normal;
  font-size: var(--font-size-2xs);
  line-height: 14px;
}
.speed-row.upload .speed-arrow {
  color: var(--m3-outline);
}
.speed-row.download {
  color: var(--m3-primary);
}
.speed-row.download .speed-text span {
  font-size: var(--font-size-sm);
  line-height: 16px;
  font-weight: 500;
}
.speed-row.download .speed-arrow {
  color: var(--m3-primary);
}
.speed-row.download .limit-value {
  font-size: var(--font-size-sm);
  font-weight: 500;
}

/* ── Lock pill (top-right corner badge, visible when limited) ──────── */
.lock-pill {
  position: absolute;
  top: -8px;
  left: -8px;
  width: 24px;
  height: 24px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: color-mix(in srgb, var(--m3-tertiary) 22%, var(--m3-surface-container));
  color: var(--m3-tertiary);
  font-size: 10px;
  border: 1px solid color-mix(in srgb, var(--m3-tertiary) 30%, transparent);
}

/* ── Clock pill (top-right badge, visible when schedule is active) ── */
.clock-pill {
  position: absolute;
  top: -8px;
  left: -8px;
  width: 24px;
  height: 24px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: color-mix(in srgb, var(--m3-primary) 22%, var(--m3-surface-container));
  color: var(--m3-primary);
  font-size: 10px;
  border: 1px solid color-mix(in srgb, var(--m3-primary) 30%, transparent);
}

/* Lock pill pop animation — bouncy enter, smooth exit */
.lock-pop-enter-active {
  transition:
    transform 0.25s cubic-bezier(0.34, 1.56, 0.64, 1),
    opacity 0.2s ease;
}
.lock-pop-leave-active {
  transition:
    transform 0.25s cubic-bezier(0.4, 0, 1, 1),
    opacity 0.2s ease;
  pointer-events: none;
}
.lock-pop-enter-from,
.lock-pop-leave-to {
  transform: scale(0);
  opacity: 0;
}

/* ── Limit zone (auto width, never truncates) ──────────────────────── */
.limit-zone {
  flex-shrink: 0;
  display: flex;
  align-items: center;
  gap: 2px;
  white-space: nowrap;
}
.limit-sep {
  opacity: 0.3;
  font-size: var(--font-size-2xs);
  flex-shrink: 0;
}
.limit-value {
  font-size: var(--font-size-2xs);
  opacity: 0.65;
  flex-shrink: 0;
  font-weight: 400;
}
.limit-inf {
  opacity: 0.35;
  flex-shrink: 0;
}

/* ── Limit value crossfade transition ────────────────────────────── */
.limit-fade-enter-active,
.limit-fade-leave-active {
  transition: opacity 0.15s cubic-bezier(0.2, 0, 0, 1);
}
.limit-fade-enter-from,
.limit-fade-leave-to {
  opacity: 0;
}

/* ── Popover panel ────────────────────────────────────────────────── */
.limit-panel {
  padding: 0;
  box-sizing: border-box;
  width: 100%;
}

.panel-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
  margin-bottom: 10px;
}

.panel-head-actions {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  flex-shrink: 0;
}

.panel-close {
  flex-shrink: 0;
}

.panel-eyebrow {
  font-size: var(--font-size-2xs);
  font-weight: 600;
  letter-spacing: 0.04em;
  text-transform: uppercase;
  color: var(--m3-on-surface-variant);
  opacity: 0.75;
}

.network-pill {
  display: inline-flex;
  align-items: center;
  gap: 5px;
  padding: 3px 8px;
  border-radius: var(--radius-full);
  font-size: var(--font-size-2xs);
  color: var(--m3-on-surface-variant);
  background: var(--m3-surface-container-high);
  white-space: nowrap;
}

.network-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--m3-status-success);
  box-shadow: 0 0 0 3px color-mix(in srgb, var(--m3-status-success) 16%, transparent);
}

.network-pill.is-offline .network-dot {
  background: var(--m3-error);
  box-shadow: 0 0 0 3px color-mix(in srgb, var(--m3-error) 16%, transparent);
}

.speed-hero {
  display: grid;
  grid-template-columns: minmax(0, 1.35fr) minmax(0, 1fr);
  gap: 10px;
  padding: 12px;
  border-radius: 16px;
  border: 1px solid color-mix(in srgb, var(--m3-primary) 16%, var(--m3-outline-variant));
  background: linear-gradient(
    135deg,
    color-mix(in srgb, var(--m3-primary) 12%, var(--m3-surface-container)) 0%,
    var(--m3-surface-container) 72%
  );
}

.hero-block {
  display: flex;
  flex-direction: column;
  gap: 2px;
  min-width: 0;
}

.hero-block--secondary {
  justify-content: center;
  padding-left: 10px;
  border-left: 1px solid color-mix(in srgb, var(--m3-outline-variant) 70%, transparent);
}

.hero-label {
  font-size: var(--font-size-2xs);
  color: var(--m3-on-surface-variant);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.hero-value {
  color: var(--m3-primary);
  font-size: 22px;
  line-height: 1.1;
  font-weight: 700;
  font-variant-numeric: tabular-nums;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.hero-value-secondary {
  color: var(--m3-on-surface-variant);
  font-size: 13px;
  line-height: 1.2;
  font-weight: 600;
  font-variant-numeric: tabular-nums;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.hero-value em,
.hero-value-secondary em {
  margin-left: 2px;
  font-size: 11px;
  font-style: normal;
  font-weight: 500;
  opacity: 0.65;
}

.status-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 6px;
  margin-top: 8px;
}

.status-cell {
  display: flex;
  flex-direction: column;
  gap: 2px;
  min-width: 0;
  padding: 8px 10px;
  border-radius: 12px;
  background: var(--m3-surface-container);
}

.status-cell--wide {
  display: flex;
  grid-column: 1 / -1;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
}

.status-label {
  font-size: var(--font-size-2xs);
  color: var(--m3-on-surface-variant);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.status-value {
  color: var(--m3-on-surface);
  font-size: 16px;
  line-height: 1.1;
  font-variant-numeric: tabular-nums;
}

.quality-value {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  font-size: var(--font-size-xs);
  font-weight: 600;
  white-space: nowrap;
}

.quality-bars {
  display: inline-flex;
  align-items: flex-end;
  gap: 2px;
  height: 10px;
}

.quality-bars i {
  width: 3px;
  border-radius: 2px;
  background: color-mix(in srgb, currentColor 18%, transparent);
}

.quality-bars i:nth-child(1) {
  height: 4px;
}

.quality-bars i:nth-child(2) {
  height: 6px;
}

.quality-bars i:nth-child(3) {
  height: 8px;
}

.quality-bars i:nth-child(4) {
  height: 10px;
}

.quality-bars i.on {
  background: currentColor;
}

.quality-success {
  color: var(--m3-status-success);
}

.quality-waiting {
  color: var(--m3-status-waiting);
}

.quality-error {
  color: var(--m3-error);
}

.quality-muted {
  color: var(--m3-on-surface-variant);
}

.quick-actions {
  display: flex;
  gap: 8px;
  margin-top: 8px;
}

.quick-action {
  flex: 1;
  min-width: 0;
}

.limit-card {
  margin-top: 8px;
  padding: 10px 12px;
  border: 1px solid var(--m3-outline-variant);
  border-radius: 16px;
  background: var(--m3-surface-container-low);
}

.limit-card-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
  min-height: 32px;
  cursor: pointer;
  user-select: none;
  -webkit-tap-highlight-color: transparent;
}

.limit-card-heading {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  flex: 1;
  min-width: 0;
}

.limit-chevron {
  flex-shrink: 0;
  color: var(--m3-on-surface-variant);
  transition: transform 0.2s cubic-bezier(0.2, 0, 0, 1);
}

.limit-chevron.is-open {
  transform: rotate(180deg);
}

.limit-card-title {
  color: var(--m3-on-surface);
  font-size: var(--font-size-sm);
  font-weight: 600;
  white-space: nowrap;
}

.limit-summary {
  display: inline-flex;
  align-items: center;
  gap: 2px;
  margin-left: 2px;
  overflow: hidden;
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-2xs);
  font-variant-numeric: tabular-nums;
  white-space: nowrap;
  opacity: 0.7;
}

.limit-switch-wrap {
  display: flex;
  flex-shrink: 0;
}

.limit-card-body {
  padding-top: 10px;
}

.limit-rows {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.limit-row {
  display: grid;
  grid-template-columns: minmax(0, 1fr) 74px 76px;
  align-items: center;
  gap: 6px;
  min-width: 0;
}

.limit-row-label {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  min-width: 0;
  overflow: hidden;
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-xs);
  white-space: nowrap;
  text-overflow: ellipsis;
}

.limit-number,
.limit-unit {
  width: 100%;
  min-width: 0;
}

.limit-switch-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
  min-height: 32px;
  margin-top: 6px;
  padding-top: 6px;
  border-top: 1px solid color-mix(in srgb, var(--m3-outline-variant) 55%, transparent);
}

.limit-rows + .limit-switch-row {
  margin-top: 10px;
}

.limit-hint {
  display: block;
  margin-top: 4px;
  font-size: var(--font-size-2xs);
}

.panel-extra {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-top: 8px;
  padding: 10px 12px;
  border: 1px solid var(--m3-outline-variant);
  border-radius: 16px;
  background: var(--m3-surface-container-low);
}

.panel-extra-row {
  min-height: 28px;
  margin-top: 0;
  padding-top: 0;
  border-top: none;
}

.panel-extra-actions {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

/* ── Hint slide-fade transition ───────────────────────────────────── */
.hint-slide-enter-active,
.hint-slide-leave-active {
  transition:
    opacity 0.25s cubic-bezier(0.2, 0, 0, 1),
    transform 0.25s cubic-bezier(0.2, 0, 0, 1),
    max-height 0.25s cubic-bezier(0.2, 0, 0, 1);
  overflow: hidden;
}
.hint-slide-enter-from,
.hint-slide-leave-to {
  opacity: 0;
  transform: translateY(-8px);
  max-height: 0;
}
.hint-slide-enter-to,
.hint-slide-leave-from {
  max-height: 40px;
}
</style>

<style scoped>
/* Keep the shared footer usable at the minimum window width. */
@container footer (max-width: 400px) {
  .speedometer {
    max-width: 128px;
  }
  .speedometer:not(.idle) .mode,
  .speedometer:not(.idle) .limit-zone {
    display: none;
  }
}
</style>
