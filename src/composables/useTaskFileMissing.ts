/** Shared native availability state; cards never inspect individual paths. */
import { computed, onBeforeUnmount, shallowRef, watch, type ComputedRef } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { recheckTrigger } from '@/composables/useTaskPaths'
import { logger } from '@shared/logger'
import type { Aria2Task } from '@shared/types'

export type TaskFileState = 'available' | 'missing' | 'inaccessible' | 'unknown'
const states = shallowRef<Record<string, TaskFileState>>({})
const visible = new Map<string, number>()
let scheduled = false
let revision = 0
export function updateTaskFileStates(value: Record<string, TaskFileState>): void {
  revision++
  states.value = value
}

function inspect(): void {
  if (scheduled) return
  scheduled = true
  queueMicrotask(async () => {
    scheduled = false
    const gids = [...visible.keys()]
    const startedAt = revision
    try {
      const result = await invoke<Record<string, TaskFileState>>('task_file_states', { gids })
      if (revision === startedAt) updateTaskFileStates(result)
    } catch (error) {
      logger.debug('TaskFiles.inspect', error)
    }
  })
}

export function useTaskFileMissing(task: ComputedRef<Aria2Task>) {
  /** Remote cards have no local file to inspect. */
  const trackedGid = computed(() => (task.value.remote ? '' : task.value.gid))
  const release = (gid: string) => {
    const count = (visible.get(gid) ?? 1) - 1
    if (count > 0) visible.set(gid, count)
    else visible.delete(gid)
  }
  watch(
    trackedGid,
    (gid, previous) => {
      if (previous) release(previous)
      if (!gid) return
      visible.set(gid, (visible.get(gid) ?? 0) + 1)
      inspect()
    },
    { immediate: true },
  )
  watch([() => task.value.status, () => task.value.seeder, recheckTrigger], inspect)
  onBeforeUnmount(() => {
    if (trackedGid.value) release(trackedGid.value)
    inspect()
  })
  const fileState = computed(() => (trackedGid.value ? (states.value[trackedGid.value] ?? 'unknown') : 'unknown'))
  const fileMissing = computed(() => fileState.value === 'missing' || fileState.value === 'inaccessible')
  return { fileMissing, fileState }
}
