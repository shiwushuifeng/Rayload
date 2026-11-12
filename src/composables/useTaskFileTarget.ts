/** @fileoverview Finds the local file a task has already written. */
import { invoke } from '@tauri-apps/api/core'
import { resolveOpenTarget } from '@shared/utils/task'
import { logger } from '@shared/logger'
import { resolveTaskFilePath } from '@/composables/useTaskPaths'
import type { Aria2Task } from '@shared/types'

/** Returns the first existing file path belonging to a task. */
export async function resolveExistingTaskFile(task: Aria2Task): Promise<string | null> {
  const candidates = [
    resolveTaskFilePath(task),
    ...(task.files ?? []).map((file) => file.path?.trim() || null),
    await resolveOpenTarget(task),
  ].filter((path): path is string => Boolean(path))

  for (const path of new Set(candidates)) {
    try {
      if (!(await invoke<boolean>('check_path_exists', { path }))) continue
      if (await invoke<boolean>('check_path_is_dir', { path })) continue
      return path
    } catch (error) {
      logger.debug('TaskFileTarget.candidate', error)
    }
  }
  return null
}
