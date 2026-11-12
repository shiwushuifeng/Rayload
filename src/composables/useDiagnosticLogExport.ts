/** @fileoverview Exports diagnostic logs, sharing them on Android and saving on desktop. */
import { invoke } from '@tauri-apps/api/core'
import { appCacheDir, join } from '@tauri-apps/api/path'
import { save as saveDialog } from '@tauri-apps/plugin-dialog'
import { getRayloadNativeBridge, shareAndroidFile } from '@/composables/useAndroidNative'
import { resolveUserVisibleDownloadDir } from '@shared/utils/userVisibleDirectory'

interface DiagnosticExportDeps {
  t: (key: string, params?: Record<string, unknown>) => string
  message: { success: (message: string) => void; error: (message: string) => void }
  configuredDir: string
}

/** Exports logs and hands them to the Android share sheet or a desktop save dialog. */
export async function exportDiagnosticLogs({ t, message, configuredDir }: DiagnosticExportDeps): Promise<void> {
  try {
    const bridge = getRayloadNativeBridge()
    if (bridge) {
      const stamp = new Date().toISOString().replace(/[:.]/g, '-')
      const savePath = await join(await appCacheDir(), `rayload-logs-${stamp}.zip`)
      const zipPath = await invoke<string>('export_diagnostic_logs', { savePath })
      if (!shareAndroidFile(zipPath)) throw new Error('Android share sheet unavailable')
      message.success(t('preferences.export-diagnostic-logs-shared'))
      return
    }

    const resolvedDir = await resolveUserVisibleDownloadDir({ configuredDir })
    const savePath = await saveDialog({
      title: t('preferences.export-diagnostic-logs'),
      defaultPath: await join(resolvedDir.path, 'rayload-logs.zip'),
      filters: [{ name: 'ZIP', extensions: ['zip'] }],
    })
    if (!savePath) return
    const zipPath = await invoke<string>('export_diagnostic_logs', { savePath })
    message.success(t('preferences.export-diagnostic-logs-success', { path: zipPath }))
  } catch {
    message.error(t('preferences.export-diagnostic-logs-failed'))
  }
}
