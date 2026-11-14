<script setup lang="ts">
/** @fileoverview Scrollable task list container with SortableJS drag ordering and Vue list transitions. */
import { ref, computed, watch, nextTick, onMounted, onBeforeUnmount } from 'vue'
import Sortable from 'sortablejs'
import { useTaskStore } from '@/stores/task'
import { usePreferenceStore } from '@/stores/preference'
import { useReducedMotion } from '@/composables/useReducedMotion'
import { usePlatform } from '@/composables/usePlatform'
import { logger } from '@shared/logger'
import TaskItem from './TaskItem.vue'
import TaskCompactItem from './TaskCompactItem.vue'
import TaskSwipeRow from './TaskSwipeRow.vue'
import type { ComponentPublicInstance } from 'vue'
import type { SortableEvent, SortableOptions } from 'sortablejs'
import type { Aria2Task } from '@shared/types'

const emit = defineEmits<{
  pause: [task: Aria2Task]
  resume: [task: Aria2Task]
  retry: [task: Aria2Task]
  redownload: [task: Aria2Task]
  'finish-sharing': [task: Aria2Task]
  'finish-media': [task: Aria2Task]
  delete: [task: Aria2Task]
  'delete-record': [task: Aria2Task]
  'copy-link': [task: Aria2Task]
  'show-info': [task: Aria2Task]
  folder: [task: Aria2Task]
  checksum: [task: Aria2Task]
  'open-file': [task: Aria2Task]
  'share-file': [task: Aria2Task]
  'select-files': [task: Aria2Task]
}>()

const props = withDefaults(defineProps<{ remoteTasks?: Aria2Task[] }>(), { remoteTasks: () => [] })
const taskStore = useTaskStore()
const preferenceStore = usePreferenceStore()
const reduceMotion = useReducedMotion()
const { isAndroid } = usePlatform()

type ListRefTarget = HTMLElement | ComponentPublicInstance | null

const dragOrder = ref<string[] | null>(null)
const taskList = computed(() => {
  if (!dragOrder.value) return taskStore.taskList
  const tasks = new Map(taskStore.taskList.map((task) => [task.gid, task]))
  const ordered: Aria2Task[] = []
  for (const gid of dragOrder.value) {
    const task = tasks.get(gid)
    if (task) ordered.push(task)
    tasks.delete(gid)
  }
  return [...ordered, ...tasks.values()]
})
const displayTasks = computed(() => [...taskList.value, ...props.remoteTasks])
const listRef = ref<ListRefTarget>(null)
const sorting = ref(false)
const containerTransitioning = ref(false)
let lastFloatingRect: DOMRect | null = null
let floatingRectFrame = 0
let sortable: Sortable | null = null
const taskCardComponent = computed(() =>
  preferenceStore.config.taskCardMode === 'compact' ? TaskCompactItem : TaskItem,
)
const pageTransitionKey = computed(() => taskStore.currentList)
const pageSize = computed(() => taskStore.taskPagination.pageSize)
/** How many cards are mounted; grows as the sentinel scrolls into view. */
const renderLimit = ref(pageSize.value)
const visibleTaskList = computed<Aria2Task[]>({
  get: () => displayTasks.value.slice(0, renderLimit.value),
  set(nextPageList) {
    dragOrder.value = [...nextPageList, ...displayTasks.value.slice(nextPageList.length)].map((task) => task.gid)
  },
})
const hasMoreTasks = computed(() => renderLimit.value < displayTasks.value.length)
const sentinelRef = ref<HTMLElement | null>(null)
let loadObserver: IntersectionObserver | null = null

function loadMoreTasks() {
  if (!hasMoreTasks.value) return
  renderLimit.value = Math.min(taskList.value.length, renderLimit.value + pageSize.value)
}

function disconnectLoadObserver() {
  loadObserver?.disconnect()
  loadObserver = null
}

function observeSentinel() {
  disconnectLoadObserver()
  if (!hasMoreTasks.value || typeof IntersectionObserver === 'undefined') return
  const element = sentinelRef.value
  if (!element) return
  loadObserver = new IntersectionObserver(
    (entries) => {
      if (entries.some((entry) => entry.isIntersecting)) loadMoreTasks()
    },
    { rootMargin: '320px 0px' },
  )
  loadObserver.observe(element)
}

onBeforeUnmount(() => {
  disconnectLoadObserver()
  stopFloatingRectTracking()
  destroySortable()
  lastFloatingRect = null
})

onMounted(() => {
  void nextTick(() => {
    mountSortable()
    observeSentinel()
  })
})

watch([hasMoreTasks, sentinelRef], () => {
  void nextTick(observeSentinel)
})

function trackFloatingRect() {
  const floating = document.querySelector<HTMLElement>('.task-list-item--floating')
  if (floating?.isConnected) {
    lastFloatingRect = floating.getBoundingClientRect()
  }

  if (sorting.value) {
    floatingRectFrame = requestAnimationFrame(trackFloatingRect)
  }
}

function startFloatingRectTracking() {
  stopFloatingRectTracking()
  lastFloatingRect = null
  floatingRectFrame = requestAnimationFrame(trackFloatingRect)
}

function stopFloatingRectTracking() {
  if (!floatingRectFrame) return
  cancelAnimationFrame(floatingRectFrame)
  floatingRectFrame = 0
}

function animateDropSettle(event: SortableEvent | undefined): Promise<void> {
  const item = event?.item
  if (!lastFloatingRect || !item?.isConnected) return Promise.resolve()

  if (reduceMotion.value) {
    lastFloatingRect = null
    return Promise.resolve()
  }

  const targetRect = item.getBoundingClientRect()
  const deltaX = lastFloatingRect.left - targetRect.left
  const deltaY = lastFloatingRect.top - targetRect.top
  lastFloatingRect = null

  if (Math.abs(deltaX) < 1 && Math.abs(deltaY) < 1) return Promise.resolve()

  item.classList.add('task-list-item--settling')
  item.style.setProperty('--task-drop-x', `${deltaX}px`)
  item.style.setProperty('--task-drop-y', `${deltaY}px`)

  return new Promise((resolve) => {
    requestAnimationFrame(() => {
      item.classList.add('task-list-item--settled')
    })

    window.setTimeout(() => {
      item.classList.remove('task-list-item--settling', 'task-list-item--settled')
      item.style.removeProperty('--task-drop-x')
      item.style.removeProperty('--task-drop-y')
      resolve()
    }, 320)
  })
}

watch(pageTransitionKey, () => {
  dragOrder.value = null
  sorting.value = false
  stopFloatingRectTracking()
  renderLimit.value = pageSize.value
  void nextTick(observeSentinel)
})

watch(pageSize, (size, previous) => {
  renderLimit.value = renderLimit.value === previous ? size : Math.max(renderLimit.value, size)
  void nextTick(observeSentinel)
})

watch(
  () => displayTasks.value.length,
  (length) => {
    if (renderLimit.value > length) renderLimit.value = length
    void nextTick(observeSentinel)
  },
)

watch(
  pageTransitionKey,
  () => {
    if (sorting.value) return
    containerTransitioning.value = true
  },
  { flush: 'sync' },
)

const sortableOptions: SortableOptions = {
  animation: reduceMotion.value ? 0 : 240,
  draggable: '.task-list-item',
  filter: '.task-item-actions, button, a, input, textarea, select, [data-no-drag], .task-list-item--remote',
  handle: isAndroid.value ? '[data-task-drag-handle]' : undefined,
  delay: isAndroid.value ? 260 : 450,
  delayOnTouchOnly: true,
  touchStartThreshold: 4,
  ghostClass: 'task-list-item--ghost',
  chosenClass: 'task-list-item--chosen',
  fallbackClass: 'task-list-item--floating',
  dragClass: 'task-list-item--dragging',
  direction: 'vertical',
  swapThreshold: 0.72,
  invertedSwapThreshold: 0.28,
  invertSwap: false,
  forceFallback: true,
  fallbackOnBody: true,
  fallbackTolerance: 3,
  preventOnFilter: false,
  onStart: () => {
    dragOrder.value = taskStore.taskList.map((task) => task.gid)
    sorting.value = true
    if (!reduceMotion.value) startFloatingRectTracking()
  },
  onUpdate: (event) => {
    if (event.oldIndex === undefined || event.newIndex === undefined || event.oldIndex === event.newIndex) return
    const nextPageList = [...visibleTaskList.value]
    const [task] = nextPageList.splice(event.oldIndex, 1)
    if (!task) return
    nextPageList.splice(event.newIndex, 0, task)
    visibleTaskList.value = nextPageList
  },
  onEnd: async (event) => {
    if (!sorting.value) return
    const gids = visibleTaskList.value.map((task) => task.gid)
    sorting.value = false
    dragOrder.value = null
    stopFloatingRectTracking()
    try {
      const saved = taskStore.saveVisibleTaskOrder(gids)
      await nextTick()
      await Promise.all([animateDropSettle(event), saved])
    } catch (error) {
      logger.warn('TaskList.saveOrder', error)
    }
  },
}

watch(reduceMotion, (enabled) => {
  const duration = enabled ? 0 : 240
  sortableOptions.animation = duration
  sortable?.option('animation', duration)
})

watch(
  () => taskStore.selectionMode,
  (selecting) => {
    sortable?.option('disabled', selecting)
  },
)

function destroySortable() {
  sortable?.destroy()
  sortable = null
}

function resolveListElement() {
  const target = listRef.value
  if (target instanceof HTMLElement) return target
  const element = target?.$el
  return element instanceof HTMLElement ? element : null
}

function mountSortable() {
  destroySortable()
  const element = resolveListElement()
  if (!element) return
  sortable = Sortable.create(element, sortableOptions)
}

function handlePageSwapBeforeLeave() {
  containerTransitioning.value = true
  destroySortable()
}

function handlePageSwapAfterEnter() {
  containerTransitioning.value = false
  void nextTick(mountSortable)
}

// Long press starts selection. While selecting, a tap toggles the card.
// Ctrl/Cmd-click picks one task and Shift-click a range on desktop.
function handleLongPress(task: Aria2Task) {
  if (task.remote) return
  if (!taskStore.checkedGids.includes(task.gid)) taskStore.setChecked(task.gid, true)
}

function handleItemClick(event: MouseEvent, task: Aria2Task) {
  if (task.remote) return
  if (taskStore.selectionMode) {
    event.preventDefault()
    event.stopPropagation()
    taskStore.toggleChecked(task.gid)
    return
  }
  const picking = event.ctrlKey || event.metaKey || event.shiftKey
  if (!picking) {
    taskStore.clearChecked()
    return
  }
  event.preventDefault()
  event.stopPropagation()
  if (event.shiftKey) {
    taskStore.checkRange(
      task.gid,
      taskList.value.map((item) => item.gid),
    )
  } else taskStore.toggleChecked(task.gid)
}

function handleKeydown(event: KeyboardEvent) {
  const target = event.target as HTMLElement | null
  if (target?.closest('input, textarea, [contenteditable="true"], .n-modal, .n-drawer')) return
  if (event.key === 'Escape' && taskStore.checkedGids.length) {
    taskStore.clearChecked()
  } else if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === 'a') {
    event.preventDefault()
    taskStore.checkAll(taskList.value.map((task) => task.gid))
  }
}

onMounted(() => window.addEventListener('keydown', handleKeydown))
onBeforeUnmount(() => window.removeEventListener('keydown', handleKeydown))

function handleCardBeforeLeave(element: Element) {
  if (containerTransitioning.value) return
  if (!(element instanceof HTMLElement)) return
  const height = Math.ceil(element.getBoundingClientRect().height || element.offsetHeight)
  element.classList.add('task-list-item--collapsing')
  element.style.setProperty('--task-list-card-leave-height', `${height}px`)
}
</script>

<template>
  <div class="task-list">
    <Transition
      name="task-page-swap"
      mode="out-in"
      appear
      @before-leave="handlePageSwapBeforeLeave"
      @after-enter="handlePageSwapAfterEnter"
      @enter-cancelled="handlePageSwapAfterEnter"
      @leave-cancelled="handlePageSwapAfterEnter"
    >
      <TransitionGroup
        :key="pageTransitionKey"
        ref="listRef"
        tag="div"
        :css="!sorting && !containerTransitioning"
        name="task-list-card"
        class="task-list-inner"
        @before-leave="handleCardBeforeLeave"
      >
        <div
          v-for="item in visibleTaskList"
          :key="item.gid"
          class="task-list-item"
          :class="{
            'task-list-item--checked': taskStore.checkedGids.includes(item.gid),
            'task-list-item--remote': item.remote,
          }"
        >
          <TaskSwipeRow
            :task="item"
            :selection-mode="taskStore.selectionMode"
            :disabled="taskStore.resubmittingGids.includes(item.gid)"
            @activate="handleItemClick($event, item)"
            @long-press="handleLongPress"
            @swipe-pause="emit('pause', item)"
            @swipe-delete="emit('delete', item)"
          >
            <component
              :is="item.remote ? TaskItem : taskCardComponent"
              :task="item"
              :action-pending="taskStore.resubmittingGids.includes(item.gid)"
              @pause="emit('pause', item)"
              @resume="emit('resume', item)"
              @retry="emit('retry', item)"
              @redownload="emit('redownload', item)"
              @finish-sharing="emit('finish-sharing', item)"
              @finish-media="emit('finish-media', item)"
              @delete="emit('delete', item)"
              @delete-record="emit('delete-record', item)"
              @copy-link="emit('copy-link', item)"
              @show-info="emit('show-info', item)"
              @folder="emit('folder', item)"
              @checksum="emit('checksum', item)"
              @open-file="emit('open-file', item)"
              @share-file="emit('share-file', item)"
              @select-files="emit('select-files', item)"
            />
          </TaskSwipeRow>
        </div>
      </TransitionGroup>
    </Transition>
    <div v-if="hasMoreTasks" ref="sentinelRef" class="task-list-sentinel" aria-hidden="true" />
  </div>
</template>

<style scoped>
.task-list {
  --task-list-bottom-safety: 96px;
  padding: 16px var(--content-gutter);
  min-height: 100%;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
}
/*
 * Floating speed-ball safety area — only when cards are present.
 * A ::after pseudo-element participates in flex layout, reliably
 * letting the final card scroll above the floating widgets without
 * forcing short lists to show a scrollbar.
 */
.task-list-inner:not(:empty)::after {
  content: '';
  display: block;
  height: var(--task-list-bottom-safety);
}

/* ── Task card layer ─────────────────────────────────────────────────── */
.task-list-inner {
  flex: 1;
  position: relative;
  z-index: 1;
}
.task-list-sentinel {
  width: 100%;
  height: 1px;
  flex: 0 0 1px;
}
.task-page-swap-enter-active {
  transition:
    opacity 0.2s cubic-bezier(0.2, 0, 0, 1),
    transform 0.2s cubic-bezier(0.2, 0, 0, 1);
}
.task-page-swap-leave-active {
  pointer-events: none;
  transition:
    opacity 0.15s cubic-bezier(0.3, 0, 0.8, 0.15),
    transform 0.15s cubic-bezier(0.3, 0, 0.8, 0.15);
}
.task-page-swap-enter-from,
.task-page-swap-leave-to {
  opacity: 0;
  transform: scale(0.98);
}
.task-list-item {
  position: relative;
  margin-bottom: 16px;
}
.task-list-card-move,
.task-list-card-enter-active {
  transition:
    transform 260ms ease,
    opacity 180ms ease;
}
.task-list-card-enter-from {
  opacity: 0;
  transform: scale(0.98);
}
.task-list-card-leave-to {
  opacity: 0;
  transform: translate3d(-26px, 0, 0) scale(0.985);
}
.task-list-card-leave-active {
  transition:
    transform 320ms cubic-bezier(0.34, 1.18, 0.64, 1),
    opacity 220ms ease;
  pointer-events: none;
}
.task-list-card-leave-active.task-list-item--collapsing {
  height: var(--task-list-card-leave-height);
  overflow: hidden;
  transition:
    height 300ms cubic-bezier(0.2, 0, 0, 1),
    margin-bottom 300ms cubic-bezier(0.2, 0, 0, 1),
    opacity 220ms ease,
    transform 320ms cubic-bezier(0.34, 1.18, 0.64, 1);
}
.task-list-card-leave-to.task-list-item--collapsing {
  height: 0;
  margin-bottom: 0;
  opacity: 0;
  transform: translate3d(-26px, 0, 0) scale(0.985);
}
.task-list-item--checked {
  border-radius: var(--radius-sm);
  outline: 2px solid var(--m3-primary);
  outline-offset: 2px;
}
.task-list-item--ghost {
  overflow: hidden;
  opacity: 0;
}
.task-list-item--floating {
  opacity: 1 !important;
  filter: none !important;
  pointer-events: none;
  transition: none !important;
}
.task-list-item--dragging {
  opacity: 1 !important;
}
.task-list-item--settling {
  z-index: 3;
  transform: translate3d(var(--task-drop-x), var(--task-drop-y), 0);
  will-change: transform;
}
.task-list-item--settling.task-list-item--settled {
  transform: translate3d(0, 0, 0);
  transition: transform 300ms ease;
}
</style>
