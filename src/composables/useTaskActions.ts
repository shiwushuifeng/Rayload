/**
 * @fileoverview Composable for task action handler functions.
 *
 * Extracted from TaskView.vue to reduce component script size.
 * Uses dependency injection for all Vue/Pinia dependencies — stores,
 * i18n, dialog, and message are passed in via the options object.
 */
import { finishMedia, saveSession } from '@/api/aria2'
import { canFinishMedia } from '@shared/utils/media'
import { ref, h } from 'vue'
import {
  getTaskUri,
  getTaskName,
  resolveOpenTarget,
  canRestart,
  writeAppClipboardText,
  checkTaskIsSharing,
  getSharingResultLabelKey,
  getTaskSharingKind,
} from '@shared/utils'
import { getErrorMessage } from '@shared/utils/errorMessage'
import { calculateFileDigests } from '@/api/checksum'
import { invoke } from '@tauri-apps/api/core'
import { resolveTaskFilePath, requestFileRecheck } from '@/composables/useTaskPaths'
import { resolveExistingTaskFile } from '@/composables/useTaskFileTarget'
import { logger } from '@shared/logger'
import { openAndroidFile, shareAndroidFile } from '@/composables/useAndroidNative'
import { usePlatform } from '@/composables/usePlatform'
import { NButton, NCheckbox, useDialog } from 'naive-ui'
import type { Aria2Task, AppConfig, TaskDeletionOptions } from '@shared/types'

interface TaskActionsDeps {
  taskStore: {
    pauseTask: (task: Aria2Task) => Promise<unknown>
    resumeTask: (task: Aria2Task) => Promise<unknown>
    finishSharing: (task: Aria2Task) => Promise<unknown>
    removeTask: (task: Aria2Task, options?: TaskDeletionOptions) => Promise<unknown>
    removeTaskRecord: (task: Aria2Task) => Promise<unknown>
    retryTask: (task: Aria2Task) => Promise<unknown>
    redownloadTask: (task: Aria2Task) => Promise<unknown>
    showTaskDetail: (task: Aria2Task) => void
    fetchList: () => Promise<unknown>
    taskList: Aria2Task[]
  }
  requestMagnetSelection: (gid: string) => void
  preferenceConfig: () => AppConfig
  t: (key: string, params?: Record<string, unknown>) => string
  dialog: ReturnType<typeof useDialog>
  message: {
    success: (msg: string) => void
    error: (msg: string) => void
    warning: (msg: string) => void
    info: (msg: string) => void
    progress: (msg: string) => {
      update: (msg: string, type?: 'loading' | 'success' | 'error' | 'warning' | 'info') => void
      finish: (msg: string, type: 'success' | 'error') => void
      destroy: () => void
    }
  }
}

export function useTaskActions(deps: TaskActionsDeps) {
  const { taskStore, preferenceConfig, t, dialog, message, requestMagnetSelection } = deps
  const { isAndroid } = usePlatform()

  const deleteFilesLabel = () =>
    t(
      preferenceConfig().fileDeletionMode === 'permanent'
        ? 'task.delete-local-files-permanent-label'
        : 'task.delete-local-files-trash-label',
    )

  function handlePauseTask(task: Aria2Task) {
    const taskName = getTaskName(task, { defaultName: 'Unknown' })
    taskStore
      .pauseTask(task)
      .then(() => message.success(t('task.pause-task-success', { taskName })))
      .catch((e) => {
        logger.warn('TaskView.pauseTask', getErrorMessage(e))
        message.error(t('task.pause-task-fail', { taskName }))
      })
  }

  function handleResumeTask(task: Aria2Task) {
    const taskName = getTaskName(task, { defaultName: 'Unknown' })
    taskStore
      .resumeTask(task)
      .then((resumed) => {
        if (resumed !== false) message.success(t('task.resume-task-success', { taskName }))
      })
      .catch((e) => {
        logger.warn('TaskView.resumeTask', getErrorMessage(e))
        message.error(t('task.resume-task-fail', { taskName }))
      })
  }

  function handleRetryTask(task: Aria2Task) {
    const taskName = getTaskName(task, { defaultName: 'Unknown' })
    if (!canRestart(task)) {
      message.warning(t('task.restart-not-available'))
      return
    }
    taskStore
      .retryTask(task)
      .then(() => message.success(t('task.retry-task-success', { taskName })))
      .catch((error) => {
        logger.warn('TaskView.retryTask', getErrorMessage(error))
        message.error(t('task.retry-task-fail', { taskName }))
      })
  }

  function handleRedownloadTask(task: Aria2Task) {
    const taskName = getTaskName(task, { defaultName: 'Unknown' })
    if (!canRestart(task)) {
      message.warning(t('task.restart-not-available'))
      return
    }
    taskStore
      .redownloadTask(task)
      .then(() => message.success(t('task.restart-task-success', { taskName })))
      .catch((error) => {
        logger.warn('TaskView.redownloadTask', getErrorMessage(error))
        message.error(t('task.restart-task-fail', { taskName }))
      })
  }

  const finishingMedia = new Set<string>()
  async function handleFinishMedia(task: Aria2Task) {
    if (!canFinishMedia(task) || finishingMedia.has(task.gid)) return
    finishingMedia.add(task.gid)
    try {
      await finishMedia(task.gid)
      await saveSession()
      await taskStore.fetchList()
    } catch (error) {
      logger.warn('TaskView.finishMedia', getErrorMessage(error))
      message.error(getErrorMessage(error))
    } finally {
      finishingMedia.delete(task.gid)
    }
  }

  function handleFinishSharing(task: Aria2Task) {
    const taskName = getTaskName(task, { defaultName: 'Unknown' })
    const kind = getTaskSharingKind(task)
    if (!kind) return
    taskStore
      .finishSharing(task)
      .then(() => message.success(t(getSharingResultLabelKey(kind, 'success'), { taskName })))
      .catch((error) => {
        logger.warn('TaskView.finishSharing', getErrorMessage(error))
        message.error(t(getSharingResultLabelKey(kind, 'fail'), { taskName }))
      })
  }

  function handleDeleteTask(task: Aria2Task) {
    const config = preferenceConfig()
    const noConfirm = config.noConfirmBeforeDeleteTask
    if (noConfirm) {
      const alsoDeleteFiles = config.deleteFilesWhenSkipConfirm
      taskStore
        .removeTask(task, { deleteMode: alsoDeleteFiles ? config.fileDeletionMode : undefined })
        .catch((error: unknown) => {
          logger.error('TaskView.deleteTask', error)
          message.error(t('task.delete-task-fail', { taskName: getTaskName(task, { defaultName: 'Unknown' }) }))
        })
      return
    }
    const deleteFiles = ref(false)
    const name = getTaskName(task, { defaultName: 'Unknown' })
    const d = dialog.error({
      title: t('task.delete-task'),
      content: () =>
        h('div', {}, [
          h('p', { class: 'technical-text-wrap', style: 'margin: 0 0 12px;' }, name),
          h(
            NCheckbox,
            {
              checked: deleteFiles.value,
              'onUpdate:checked': (v: boolean) => {
                deleteFiles.value = v
              },
            },
            { default: deleteFilesLabel },
          ),
        ]),
      positiveText: t('app.yes'),
      negativeText: t('app.no'),
      onPositiveClick: async () => {
        d.loading = true
        d.negativeButtonProps = { disabled: true }
        d.closable = false
        d.maskClosable = false
        try {
          await taskStore.removeTask(task, { deleteMode: deleteFiles.value ? config.fileDeletionMode : undefined })
          message.success(t('task.delete-task-success', { taskName: name }))
        } catch (e) {
          logger.error('TaskView.deleteTask', e)
          message.error(t('task.delete-task-fail', { taskName: name }))
        }
      },
    })
  }

  const handleDeleteRecord = handleDeleteTask

  async function handleCopyLink(task: Aria2Task) {
    const uri = getTaskUri(task).trim()
    if (!uri) {
      message.warning(t('task.copy-link-unavailable'))
      return
    }
    try {
      await writeAppClipboardText(uri)
      message.success(t('task.copy-link-success'))
    } catch (e) {
      logger.warn('TaskView.copyLink', getErrorMessage(e))
      message.error(t('task.copy-link-unavailable'))
    }
  }

  function handleShowInfo(task: Aria2Task) {
    taskStore.showTaskDetail(task)
  }

  async function handleShowInFolder(task: Aria2Task) {
    if (isAndroid.value) {
      if (task.status !== 'complete' && task.status !== 'removed' && !checkTaskIsSharing(task)) return
      await handleOpenAndroidFile(task)
      return
    }

    const files = task.files || []
    if (files.length === 0) return

    // Resolve correct path — archived location takes priority over aria2 original
    const filePath = resolveTaskFilePath(task)

    if (!filePath) return
    try {
      const fileExists = await invoke<boolean>('check_path_exists', { path: filePath })
      if (fileExists) {
        await invoke('show_item_in_dir', { path: filePath })
        message.success(t('task.open-folder-success'))
        return
      }
      // Fallback: file missing but BT folder or download dir may still exist
      const fallback = await resolveOpenTarget(task)
      if (fallback) {
        const fallbackExists = await invoke<boolean>('check_path_exists', { path: fallback })
        if (fallbackExists) {
          await invoke('show_item_in_dir', { path: fallback })
          message.success(t('task.open-folder-success'))
          return
        }
      }
      message.warning(t('task.file-not-exist'))
      requestFileRecheck()
    } catch (e) {
      logger.warn('TaskView.showInFolder', e instanceof Error ? e.message : JSON.stringify(e))
      message.warning(t('task.file-not-exist'))
      requestFileRecheck()
    }
  }

  /**
   * Android cannot reveal arbitrary absolute paths without a broad SAF grant.
   * The folder action therefore opens a readable file with ACTION_VIEW.
   */
  async function handleOpenAndroidFile(task: Aria2Task) {
    let target: string | null
    try {
      target = await resolveExistingTaskFile(task)
    } catch (error) {
      logger.warn('TaskView.openAndroidFile', getErrorMessage(error))
      target = null
    }
    if (!target) {
      message.warning(t('task.file-not-exist'))
      requestFileRecheck()
      return
    }
    if (openAndroidFile(target)) {
      message.success(t('task.open-file-success'))
      return
    }
    message.info(t('task.android-file-open-fallback'))
  }

  async function handleVerifyChecksum(task: Aria2Task) {
    if (task.status !== 'complete' && task.status !== 'removed' && !checkTaskIsSharing(task)) return

    let target: string | null
    try {
      target = await resolveExistingTaskFile(task)
    } catch (error) {
      logger.warn('TaskView.verifyChecksum.resolve', getErrorMessage(error))
      target = null
    }
    if (!target) {
      message.warning(t('task.file-not-exist'))
      requestFileRecheck()
      return
    }

    const fileName = target.split(/[\\/]/).pop() || target
    const pending = message.progress(t('task.sha256-calculating', { fileName }))
    try {
      const digests = await calculateFileDigests(target)
      pending.destroy()
      const rows: Array<[string, string]> = [
        ['MD5', digests.md5],
        ['SHA-1', digests.sha1],
        ['SHA-256', digests.sha256],
        ['SHA-512', digests.sha512],
      ]
      dialog.info({
        title: t('task.checksum-title'),
        content: () =>
          h('div', { class: 'technical-text-wrap', style: 'display:grid;gap:14px' }, [
            h('p', { style: 'margin:0;color:var(--m3-on-surface-variant)' }, fileName),
            ...rows.map(([label, value]) =>
              h('div', { style: 'display:grid;gap:4px' }, [
                h('strong', { style: 'font-size:var(--font-size-sm)' }, label),
                h(
                  'code',
                  {
                    style:
                      'display:block;overflow-wrap:anywhere;user-select:text;font-family:var(--font-family-mono);line-height:1.6',
                  },
                  value,
                ),
                h(
                  NButton,
                  {
                    size: 'tiny',
                    style: 'justify-self:start',
                    onClick: async () => {
                      try {
                        await writeAppClipboardText(value)
                        message.success(t('task.checksum-copied', { algorithm: label }))
                      } catch (error) {
                        logger.warn('TaskView.verifyChecksum.copy', getErrorMessage(error))
                        message.error(t('task.checksum-copy-failed'))
                      }
                    },
                  },
                  { default: () => t('task.checksum-copy', { algorithm: label }) },
                ),
              ]),
            ),
          ]),
        positiveText: t('app.close'),
        zIndex: 12000,
      })
    } catch (error) {
      pending.destroy()
      logger.warn('TaskView.verifyChecksum', getErrorMessage(error))
      message.error(t('task.checksum-failed'))
    }
  }

  async function handleOpenFile(task: Aria2Task) {
    const target = await resolveOpenTarget(task)
    if (!target) return
    try {
      const fileExists = await invoke<boolean>('check_path_exists', { path: target })
      if (!fileExists) {
        message.warning(t('task.file-not-exist'))
        requestFileRecheck()
        return
      }
      const isDir = await invoke<boolean>('check_path_is_dir', { path: target })
      await invoke('open_path_normalized', { path: target })
      message.success(t(isDir ? 'task.open-file-is-folder' : 'task.open-file-success'))
    } catch (e) {
      logger.warn('TaskView.openFile error', e instanceof Error ? e.message : JSON.stringify(e))
      message.warning(t('task.file-not-exist'))
      requestFileRecheck()
    }
  }

  async function handleShareFile(task: Aria2Task) {
    const candidates = [
      resolveTaskFilePath(task),
      ...(task.files ?? []).map((file) => file.path?.trim() || null),
      await resolveOpenTarget(task),
    ].filter((path): path is string => Boolean(path))
    const uniqueCandidates = [...new Set(candidates)]
    if (uniqueCandidates.length === 0) {
      message.warning(t('task.file-not-exist'))
      requestFileRecheck()
      return
    }
    try {
      // The native bridge checks readability and converts the path through
      // FileProvider before opening the system share sheet.
      if (!uniqueCandidates.some((path) => shareAndroidFile(path))) {
        message.warning(t('task.file-not-exist'))
        requestFileRecheck()
        return
      }
      message.success(t('task.share-file-success'))
    } catch (e) {
      logger.warn('TaskView.shareFile', getErrorMessage(e))
      message.error(t('task.share-file-failed'))
    }
  }

  function handleSelectFiles(task: Aria2Task) {
    requestMagnetSelection(task.gid)
  }

  return {
    handlePauseTask,
    handleResumeTask,
    handleRetryTask,
    handleRedownloadTask,
    handleFinishSharing,
    handleFinishMedia,
    handleDeleteTask,
    handleDeleteRecord,
    handleCopyLink,
    handleShowInfo,
    handleShowInFolder,
    handleVerifyChecksum,
    handleOpenFile,
    handleShareFile,
    handleSelectFiles,
  }
}
