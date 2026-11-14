/** @fileoverview LAN peer state, remote tasks and polling for full builds. */
import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { logger } from '@shared/logger'
import type { Aria2Task, TaskStatus } from '@shared/types'

export interface LanDevice {
  id: string
  name: string
  note: string
  arch: string
  address: string
  fingerprint: string
  online: boolean
}

export interface PairResult {
  id: string
  name: string
  fingerprint: string
}

export interface LanPairRequest {
  id: string
  name: string
  fingerprint: string
  arch: string
}

export interface RemoteTask {
  deviceId: string
  deviceName: string
  gid: string
  url: string
  filename: string
  status: string
  totalLength: number
  completedLength: number
  downloadSpeed: number
  online: boolean
  error?: string
}

export interface PeerStatus {
  id: string
  name: string
  online: boolean
  downloadSpeed: number
  activeTasks: number
}

export interface PeerHistoryStats {
  id: string
  online: boolean
  totalBytes: number
  totalTasks: number
  days: Array<{ date: string; bytes: number }>
  extensions: Array<{ extension: string; bytes: number; tasks: number }>
}

export interface DiscoveredDevice {
  name: string
  address: string
  arch: string
  fingerprint?: string
}

export interface LanScanResult {
  address: string
  name: string
  arch: string
  fingerprint: string
}

export interface LanPairingInfo {
  addresses: string[]
  username: string
  password: string
  name: string
  arch: string
  fingerprint: string
}

const TASK_STATUS_VALUES: readonly string[] = ['active', 'waiting', 'paused', 'error', 'complete', 'removed']

function toTaskStatus(value: string): TaskStatus {
  return TASK_STATUS_VALUES.includes(value) ? (value as TaskStatus) : 'waiting'
}

function remoteName(task: RemoteTask): string {
  if (task.filename) return task.filename
  try {
    return decodeURIComponent(new URL(task.url).pathname.split('/').filter(Boolean).pop() ?? '') || task.url
  } catch {
    return task.url
  }
}

/** Renders a remote task with the same card model as local tasks. */
export function remoteTaskToAria2(task: RemoteTask): Aria2Task {
  const name = remoteName(task)
  return {
    gid: `remote:${task.deviceId}:${task.gid}`,
    status: toTaskStatus(task.status),
    totalLength: String(task.totalLength),
    completedLength: String(task.completedLength),
    uploadLength: '0',
    downloadSpeed: String(task.downloadSpeed),
    uploadSpeed: '0',
    connections: '0',
    dir: '',
    files: [
      {
        index: '1',
        path: name,
        length: String(task.totalLength),
        completedLength: String(task.completedLength),
        selected: 'true',
        uris: [],
      },
    ],
    remote: {
      deviceId: task.deviceId,
      deviceName: task.deviceName,
      gid: task.gid,
      online: task.online,
      error: task.error,
    },
  }
}

export const useLanStore = defineStore('lan', () => {
  const devices = ref<LanDevice[]>([])
  const discovered = ref<DiscoveredDevice[]>([])
  const remoteTasks = ref<RemoteTask[]>([])
  const statuses = ref<PeerStatus[]>([])
  const historyStats = ref<PeerHistoryStats[]>([])
  let taskTimer: number | undefined
  let statusTimer: number | undefined
  let historyTimer: number | undefined
  let statusPending = false
  let historyPending = false
  let discoverPending = false
  let remotePending = false

  async function refreshDevices(): Promise<void> {
    try {
      devices.value = await invoke<LanDevice[]>('lan_devices')
    } catch (error) {
      logger.debug('Lan.refreshDevices', error)
    }
  }

  async function refreshRemoteTasks(): Promise<void> {
    if (remotePending) return
    remotePending = true
    try {
      remoteTasks.value = await invoke<RemoteTask[]>('lan_remote_tasks')
    } catch (error) {
      logger.debug('Lan.refreshRemoteTasks', error)
    } finally {
      remotePending = false
    }
  }

  async function refreshStatuses(): Promise<void> {
    if (statusPending) return
    statusPending = true
    try {
      statuses.value = await invoke<PeerStatus[]>('lan_device_statuses')
    } catch (error) {
      logger.debug('Lan.refreshStatuses', error)
    } finally {
      statusPending = false
    }
  }

  async function refreshHistoryStats(): Promise<void> {
    if (historyPending) return
    historyPending = true
    try {
      const previous = new Map(historyStats.value.map((item) => [item.id, item]))
      const result = await invoke<PeerHistoryStats[]>('lan_device_history_stats')
      historyStats.value = result.map((item) => (item.online ? item : (previous.get(item.id) ?? item)))
    } catch (error) {
      logger.debug('Lan.refreshHistoryStats', error)
    } finally {
      historyPending = false
    }
  }

  async function refreshDiscovered(): Promise<void> {
    if (discoverPending) return
    discoverPending = true
    try {
      discovered.value = await invoke<DiscoveredDevice[]>('lan_discovered')
    } catch (error) {
      logger.debug('Lan.refreshDiscovered', error)
    } finally {
      discoverPending = false
    }
  }

  function startTaskPolling(): void {
    if (taskTimer !== undefined) return
    void Promise.all([refreshDevices(), refreshRemoteTasks()])
    taskTimer = window.setInterval(() => {
      void refreshRemoteTasks()
    }, 2500)
  }

  function stopTaskPolling(): void {
    if (taskTimer !== undefined) window.clearInterval(taskTimer)
    taskTimer = undefined
  }

  function startStatusPolling(): void {
    if (statusTimer !== undefined) return
    void refreshDevices()
    void refreshStatuses()
    void refreshHistoryStats()
    void refreshDiscovered()
    statusTimer = window.setInterval(() => {
      void refreshStatuses()
      void refreshDiscovered()
    }, 3000)
    historyTimer = window.setInterval(() => {
      void refreshHistoryStats()
    }, 30000)
  }

  function stopStatusPolling(): void {
    if (statusTimer !== undefined) window.clearInterval(statusTimer)
    if (historyTimer !== undefined) window.clearInterval(historyTimer)
    statusTimer = undefined
    historyTimer = undefined
  }

  function pair(address: string, username: string, password: string): Promise<PairResult> {
    return invoke('lan_pair', { address, username, password })
  }

  function pairDiscovered(address: string): Promise<PairResult> {
    return invoke('lan_pair_discovered', { address })
  }

  function pairRequests(): Promise<LanPairRequest[]> {
    return invoke<LanPairRequest[]>('lan_pair_requests')
  }

  function approvePair(id: string): Promise<void> {
    return invoke('lan_pair_approve', { id })
  }

  function rejectPair(id: string): Promise<void> {
    return invoke('lan_pair_reject', { id })
  }

  async function confirm(id: string): Promise<void> {
    await invoke('lan_confirm_device', { id })
    await refreshDevices()
  }

  async function forget(id: string): Promise<void> {
    await invoke('lan_forget_device', { id })
    await Promise.all([refreshDevices(), refreshRemoteTasks(), refreshHistoryStats()])
  }

  async function updateDevice(id: string, name: string, note: string): Promise<void> {
    await invoke('lan_rename_device', { id, name })
    await invoke('lan_set_device_note', { id, note })
    await refreshDevices()
  }

  function pairingInfo(): Promise<LanPairingInfo> {
    return invoke<LanPairingInfo>('lan_pairing_info')
  }

  function scan(): Promise<LanScanResult[]> {
    return invoke<LanScanResult[]>('lan_scan')
  }

  function handoff(deviceId: string, url: string, filename: string | null): Promise<RemoteTask> {
    return invoke('lan_handoff', { deviceId, url, filename })
  }

  async function remoteAction(id: string, gid: string, action: 'pause' | 'unpause' | 'remove'): Promise<void> {
    await invoke('lan_remote_action', { deviceId: id, gid, action })
    await refreshRemoteTasks()
  }

  return {
    devices,
    discovered,
    remoteTasks,
    statuses,
    historyStats,
    refreshDevices,
    refreshRemoteTasks,
    refreshStatuses,
    refreshHistoryStats,
    refreshDiscovered,
    startTaskPolling,
    stopTaskPolling,
    startStatusPolling,
    stopStatusPolling,
    pair,
    pairDiscovered,
    pairRequests,
    approvePair,
    rejectPair,
    confirm,
    forget,
    updateDevice,
    pairingInfo,
    scan,
    handoff,
    remoteAction,
  }
})
