/** @fileoverview Reserves the local JSON store for future download statistics. */
import { invoke } from '@tauri-apps/api/core'
import { appDataDir, join } from '@tauri-apps/api/path'
import { logger } from '@shared/logger'

export interface DailyDownloadStat {
  date: string
  downloadedBytes: number
  completedTasks: number
}

export interface StatisticsHistoryFile {
  version: 1
  daily: DailyDownloadStat[]
  categories: Record<string, number>
}

function isoDay(offset: number): string {
  const date = new Date()
  date.setDate(date.getDate() + offset)
  return date.toISOString().slice(0, 10)
}

/** Creates the versioned skeleton once; real aggregation arrives in v1.0.4. */
export async function ensureStatisticsHistoryFile(): Promise<void> {
  try {
    const path = await join(await appDataDir(), 'statistics-history.json')
    if (await invoke<boolean>('check_path_exists', { path })) return
    const skeleton: StatisticsHistoryFile = {
      version: 1,
      daily: Array.from({ length: 7 }, (_, index) => ({
        date: isoDay(index - 6),
        downloadedBytes: 0,
        completedTasks: 0,
      })),
      categories: {},
    }
    await invoke('write_settings_backup_file', {
      path,
      content: `${JSON.stringify(skeleton, null, 2)}\n`,
    })
  } catch (error) {
    logger.debug('StatisticsHistory.ensure', error)
  }
}
