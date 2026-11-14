/** @fileoverview Demo runtime: fakes Tauri IPC so the real frontend runs on sample data. */
import { mockIPC, mockWindows } from '@tauri-apps/api/mocks'
import { emit } from '@tauri-apps/api/event'
import type {
  Aria2File,
  Aria2GlobalStat,
  Aria2RawGlobalStat,
  Aria2Task,
  HistoryRecord,
  TaskSnapshot,
  TaskStatus,
} from '@shared/types'
import type {
  DiscoveredDevice,
  LanDevice,
  LanPairingInfo,
  PeerHistoryStats,
  PeerStatus,
  RemoteChecksum,
  RemoteTask,
} from '@/stores/lan'

const MB = 1024 ** 2
const GB = 1024 ** 3
const DOWNLOAD_DIR = '/storage/emulated/0/Download/Rayload'
const APP_DATA_DIR = '/storage/emulated/0/Android/data/com.shiwushuifeng.rayload/files'

/** Tauri IPC payloads are plain JSON objects; narrow once for field reads. */
function field<T>(payload: unknown, key: string): T | undefined {
  if (!payload || typeof payload !== 'object' || Array.isArray(payload)) return undefined
  return (payload as Record<string, unknown>)[key] as T | undefined
}

function baseName(uri: string): string {
  try {
    return decodeURIComponent(new URL(uri).pathname.split('/').filter(Boolean).pop() ?? '') || 'demo-download.bin'
  } catch {
    return 'demo-download.bin'
  }
}

function isoAgo(days: number): string {
  return new Date(Date.now() - days * 86_400_000).toISOString()
}

function dayAgo(days: number): string {
  return isoAgo(days).slice(0, 10)
}

function demoFile(path: string, length: number, completed = length): Aria2File {
  return {
    index: '1',
    path,
    length: String(Math.round(length)),
    completedLength: String(Math.round(completed)),
    selected: 'true',
    uris: [],
  }
}

interface LocalTask {
  task: Aria2Task
  total: number
  completed: number
  speed: number
  baseSpeed: number
}

function localTask(input: {
  gid: string
  name: string
  dir: string
  total: number
  progress: number
  speed: number
  status: TaskStatus
  seeder?: boolean
  uploadSpeed?: number
  errorMessage?: string
  bittorrent?: Aria2Task['bittorrent']
}): LocalTask {
  const completed = input.total * input.progress
  const task: Aria2Task = {
    gid: input.gid,
    status: input.status,
    totalLength: String(Math.round(input.total)),
    completedLength: String(Math.round(completed)),
    uploadLength: input.seeder ? String(Math.round(completed)) : '0',
    downloadSpeed: String(Math.round(input.speed)),
    uploadSpeed: String(Math.round(input.uploadSpeed ?? 0)),
    connections: input.seeder ? '18' : '12',
    dir: input.dir,
    files: [demoFile(`${input.dir}/${input.name}`, input.total, completed)],
    seeder: input.seeder ? 'true' : 'false',
    errorMessage: input.errorMessage,
    bittorrent: input.bittorrent,
  }
  return { task, total: input.total, completed, speed: input.speed, baseSpeed: input.speed }
}

const localTasks: LocalTask[] = [
  localTask({
    gid: 'demo-video',
    name: 'Dune.Part.Two.2024.2160p.HDR.mkv',
    dir: `${DOWNLOAD_DIR}/Movies`,
    total: 21.8 * GB,
    progress: 0.634,
    speed: 28.4 * MB,
    status: 'active',
  }),
  localTask({
    gid: 'demo-iso',
    name: 'ubuntu-26.04-desktop-amd64.iso',
    dir: `${DOWNLOAD_DIR}/Software`,
    total: 5.1 * GB,
    progress: 0.387,
    speed: 12.6 * MB,
    status: 'active',
  }),
  localTask({
    gid: 'demo-series',
    name: 'The.Last.of.Us.S03E04.1080p.mkv',
    dir: `${DOWNLOAD_DIR}/Series`,
    total: 1.24 * GB,
    progress: 0.812,
    speed: 0,
    status: 'paused',
  }),
  localTask({
    gid: 'demo-seed',
    name: 'Fedora-Workstation-Live-43.x86_64.iso',
    dir: `${DOWNLOAD_DIR}/Software`,
    total: 2.4 * GB,
    progress: 1,
    speed: 0,
    status: 'active',
    seeder: true,
    uploadSpeed: 1.2 * MB,
    bittorrent: {
      info: { name: 'Fedora-Workstation-Live-43.x86_64.iso' },
      state: 'seeding',
      numPeers: '14',
      numSeeds: '6',
    },
  }),
  localTask({
    gid: 'demo-apk',
    name: 'Rayload_1.0.4_arm64.apk',
    dir: `${DOWNLOAD_DIR}/Apps`,
    total: 48.6 * MB,
    progress: 1,
    speed: 0,
    status: 'complete',
  }),
  localTask({
    gid: 'demo-error',
    name: 'Adobe.Photoshop.2026.macOS.dmg',
    dir: `${DOWNLOAD_DIR}/Apps`,
    total: 3.2 * GB,
    progress: 0.216,
    speed: 0,
    status: 'error',
    errorMessage: 'No space left on device',
  }),
].reverse()

const historyRecords: HistoryRecord[] = [
  {
    gid: 'history-archive',
    name: '2026-年度设计归档.zip',
    uri: 'https://example.com/2026-design-archive.zip',
    dir: `${DOWNLOAD_DIR}/Archives`,
    total_length: 8.42 * GB,
    status: 'complete',
    task_type: 'uri',
    added_at: isoAgo(2),
    created_at: isoAgo(2),
    completed_at: isoAgo(1),
  },
  {
    gid: 'history-raw',
    name: 'Sony_A7M5_RAW_Samples.zip',
    uri: 'https://example.com/sony-raw-samples.zip',
    dir: `${DOWNLOAD_DIR}/Archives`,
    total_length: 3.6 * GB,
    status: 'complete',
    task_type: 'uri',
    added_at: isoAgo(3),
    created_at: isoAgo(3),
    completed_at: isoAgo(2),
  },
  {
    gid: 'history-blender',
    name: 'Blender-4.5.3-Release.torrent',
    uri: 'https://example.com/blender-4.5.3.torrent',
    dir: `${DOWNLOAD_DIR}/Software`,
    total_length: 328 * MB,
    status: 'complete',
    task_type: 'bt',
    added_at: isoAgo(4),
    created_at: isoAgo(4),
    completed_at: isoAgo(3),
  },
  {
    gid: 'history-movie',
    name: 'Interstellar.2014.1080p.BluRay.mkv',
    uri: 'https://example.com/interstellar.mkv',
    dir: `${DOWNLOAD_DIR}/Movies`,
    total_length: 12.6 * GB,
    status: 'complete',
    task_type: 'uri',
    added_at: isoAgo(5),
    created_at: isoAgo(5),
    completed_at: isoAgo(4),
  },
  {
    gid: 'history-server',
    name: 'Ubuntu-25.10-Server.iso',
    uri: 'https://example.com/ubuntu-25.10-server.iso',
    dir: `${DOWNLOAD_DIR}/Software`,
    total_length: 2.9 * GB,
    status: 'complete',
    task_type: 'uri',
    added_at: isoAgo(6),
    created_at: isoAgo(6),
    completed_at: isoAgo(5),
  },
  {
    gid: 'history-broken',
    name: 'broken-archive.7z',
    uri: 'https://example.com/broken-archive.7z',
    dir: `${DOWNLOAD_DIR}/Archives`,
    total_length: 720 * MB,
    status: 'error',
    task_type: 'uri',
    added_at: isoAgo(7),
    created_at: isoAgo(7),
    completed_at: isoAgo(7),
  },
]

const devices: LanDevice[] = [
  {
    id: 'mac',
    name: 'MacBook Pro M4',
    note: '书房',
    arch: 'aarch64',
    address: '192.168.1.8',
    fingerprint: 'RL-DEMO-MAC-8C21',
    online: true,
  },
  {
    id: 'nas',
    name: 'Home NAS',
    note: '媒体库',
    arch: 'x86_64',
    address: '192.168.1.20',
    fingerprint: 'RL-DEMO-NAS-4A90',
    online: true,
  },
  {
    id: 'office',
    name: 'Office PC',
    note: '',
    arch: 'x86_64',
    address: '192.168.1.42',
    fingerprint: 'RL-DEMO-PC-71B3',
    online: false,
  },
]

const remoteTasks: RemoteTask[] = [
  {
    deviceId: 'nas',
    deviceName: 'Home NAS',
    gid: 'remote-archive',
    url: 'https://example.com/2026-design-archive.zip',
    filename: '2026-年度设计归档.zip',
    status: 'complete',
    totalLength: 8.42 * GB,
    completedLength: 8.42 * GB,
    downloadSpeed: 0,
    online: true,
    dir: '/volume1/Media/2026-年度设计归档',
    files: [
      {
        index: '1',
        path: '2026-年度设计归档/Design_Assets_2026.zip',
        length: 8.1 * GB,
        completedLength: 8.1 * GB,
        selected: true,
      },
      { index: '2', path: '2026-年度设计归档/README.pdf', length: 12 * MB, completedLength: 12 * MB, selected: true },
    ],
    seeder: true,
    bittorrent: true,
  },
  {
    deviceId: 'mac',
    deviceName: 'MacBook Pro M4',
    gid: 'remote-movie',
    url: 'https://example.com/interstellar-remastered.mkv',
    filename: 'Interstellar.2014.REMASTERED.1080p.mkv',
    status: 'active',
    totalLength: 12.6 * GB,
    completedLength: 5.67 * GB,
    downloadSpeed: 8.4 * MB,
    online: true,
    dir: '/Users/demo/Movies',
    files: [
      {
        index: '1',
        path: 'Interstellar.2014.REMASTERED.1080p.mkv',
        length: 12.6 * GB,
        completedLength: 5.67 * GB,
        selected: true,
      },
    ],
    bittorrent: true,
  },
  {
    deviceId: 'office',
    deviceName: 'Office PC',
    gid: 'remote-pack',
    url: 'https://example.com/rayload-demo-pack.zip',
    filename: 'rayload-demo-pack.zip',
    status: 'complete',
    totalLength: 1.86 * GB,
    completedLength: 1.86 * GB,
    downloadSpeed: 0,
    online: false,
    dir: 'D:/Downloads/Rayload',
    files: [
      { index: '1', path: 'rayload-demo-pack.zip', length: 1.86 * GB, completedLength: 1.86 * GB, selected: true },
    ],
  },
]

const statuses: PeerStatus[] = [
  { id: 'mac', name: 'MacBook Pro M4', online: true, downloadSpeed: 8.4 * MB, activeTasks: 1 },
  { id: 'nas', name: 'Home NAS', online: true, downloadSpeed: 0, activeTasks: 0 },
  { id: 'office', name: 'Office PC', online: false, downloadSpeed: 0, activeTasks: 0 },
]

const historyStats: PeerHistoryStats[] = [
  {
    id: 'mac',
    online: true,
    totalBytes: 186 * GB,
    totalTasks: 74,
    days: [4, 9, 6, 12, 8, 15, 11].map((value, index) => ({
      date: dayAgo(6 - index),
      bytes: value * GB,
      tasks: value,
    })),
    extensions: [
      { extension: 'mkv', bytes: 102 * GB, tasks: 31 },
      { extension: 'zip', bytes: 54 * GB, tasks: 22 },
      { extension: 'dmg', bytes: 30 * GB, tasks: 9 },
    ],
  },
  {
    id: 'nas',
    online: true,
    totalBytes: 412 * GB,
    totalTasks: 168,
    days: [18, 22, 17, 26, 24, 31, 29].map((value, index) => ({
      date: dayAgo(6 - index),
      bytes: value * GB,
      tasks: value,
    })),
    extensions: [
      { extension: 'mkv', bytes: 268 * GB, tasks: 96 },
      { extension: 'iso', bytes: 84 * GB, tasks: 28 },
      { extension: 'zip', bytes: 60 * GB, tasks: 44 },
    ],
  },
  {
    id: 'office',
    online: false,
    totalBytes: 96 * GB,
    totalTasks: 41,
    days: [2, 3, 1, 4, 2, 5, 3].map((value, index) => ({ date: dayAgo(6 - index), bytes: value * GB, tasks: value })),
    extensions: [
      { extension: 'zip', bytes: 48 * GB, tasks: 19 },
      { extension: 'pdf', bytes: 24 * GB, tasks: 12 },
      { extension: 'exe', bytes: 24 * GB, tasks: 10 },
    ],
  },
]

const discovered: DiscoveredDevice[] = [
  { name: 'Studio Mac mini', address: '192.168.1.55', arch: 'aarch64', fingerprint: 'RL-DEMO-STUDIO-55' },
  { name: 'Living Room TV', address: '192.168.1.66', arch: 'aarch64', fingerprint: 'RL-DEMO-TV-66' },
]

const engine = {
  phase: 'running',
  desired: 'running',
  revision: 1,
  operationId: 1,
  attempt: 1,
  maxAttempts: 5,
  cause: 'startup',
  failure: null,
}

const buildInfo = { variant: 'full', lan: true, pluginEngine: true, fdroid: false }

const plugins = [
  {
    file: 'rayload-download-test.json',
    name: 'Download Test',
    version: '1.0.0',
    description: 'Verifies link handling and returns a sample download.',
    kind: 'rule',
    runtime: 'declarative',
    permissions: ['network'],
    grantedPermissions: ['network'],
    size: 1840,
  },
  {
    file: 'media-sniffer.sh',
    name: 'Media Sniffer',
    version: '0.9.2',
    description: 'Extracts direct media links from supported pages.',
    kind: 'script',
    runtime: 'script',
    permissions: ['network', 'modify_headers'],
    grantedPermissions: ['network', 'modify_headers'],
    size: 10240,
  },
]

const storeData = new Map<string, Map<string, unknown>>()
const storeIds = new Map<number, string>()
let nextStoreId = 1

function storeForRid(rid: number): Map<string, unknown> {
  const path = storeIds.get(rid) ?? 'config.json'
  let store = storeData.get(path)
  if (!store) {
    store = new Map()
    storeData.set(path, store)
  }
  return store
}

function currentTask(item: LocalTask): Aria2Task {
  const task = item.task
  const completed = Math.round(item.completed)
  return {
    ...task,
    totalLength: String(Math.round(item.total)),
    completedLength: String(completed),
    downloadSpeed: String(Math.round(item.speed)),
    files: task.files.map((file) => ({
      ...file,
      length: String(Math.round(item.total)),
      completedLength: String(completed),
    })),
  }
}

function stat(): Aria2GlobalStat {
  const active = localTasks.filter((item) => item.task.status === 'active')
  const waiting = localTasks.filter((item) => item.task.status === 'waiting')
  const stopped = localTasks.filter((item) => item.task.status === 'complete' || item.task.status === 'error')
  return {
    downloadSpeed: active.reduce((total, item) => total + (item.task.seeder === 'true' ? 0 : item.speed), 0),
    uploadSpeed: active.reduce((total, item) => total + Number(item.task.uploadSpeed ?? 0), 0),
    numActive: active.length,
    numWaiting: waiting.length,
    numStopped: stopped.length,
    numStoppedTotal: stopped.length + historyRecords.filter((record) => record.status === 'complete').length,
  }
}

function rawStat(): Aria2RawGlobalStat {
  const value = stat()
  return {
    downloadSpeed: String(Math.round(value.downloadSpeed)),
    uploadSpeed: String(Math.round(value.uploadSpeed)),
    numActive: String(value.numActive),
    numWaiting: String(value.numWaiting),
    numStopped: String(value.numStopped),
    numStoppedTotal: String(value.numStoppedTotal),
  }
}

function snapshot(): TaskSnapshot {
  return { stat: stat(), tasks: localTasks.map(currentTask) }
}

function findLocal(gid: string): LocalTask | undefined {
  return localTasks.find((item) => item.task.gid === gid)
}

function findRemote(gid: string): RemoteTask | undefined {
  return remoteTasks.find((task) => task.gid === gid)
}

function removeLocal(gid: string): void {
  const index = localTasks.findIndex((item) => item.task.gid === gid)
  if (index >= 0) localTasks.splice(index, 1)
}

function addLocal(name: string, total: number, speed: number): string {
  const gid = `demo-${Date.now().toString(36)}`
  localTasks.unshift(
    localTask({
      gid,
      name,
      dir: DOWNLOAD_DIR,
      total,
      progress: 0,
      speed,
      status: 'active',
    }),
  )
  return gid
}

function tick(): void {
  for (const item of localTasks) {
    if (item.task.status !== 'active') continue
    if (item.task.seeder === 'true') {
      item.task.uploadLength = String(Number(item.task.uploadLength) + item.speed * 0.9)
      continue
    }
    item.completed = Math.min(item.total * 0.985, item.completed + item.speed * 0.9)
  }
  for (const task of remoteTasks) {
    if (task.status !== 'active') continue
    task.completedLength = Math.min(task.totalLength * 0.985, task.completedLength + task.downloadSpeed * 0.9)
  }
}

function taskFileStates(payload: unknown): Record<string, string> {
  const gids = field<string[]>(payload, 'gids') ?? []
  return Object.fromEntries(gids.map((gid) => [gid, 'available']))
}

function remoteChecksum(gid: string): RemoteChecksum {
  const task = findRemote(gid)
  const seed = (gid + task?.filename).replace(/[^a-z0-9]/giu, '')
  const digest = (length: number): string => (seed + '0123456789abcdef'.repeat(Math.ceil(length / 16))).slice(0, length)
  return { md5: digest(32), sha1: digest(40), sha256: digest(64), sha512: digest(128), size: task?.totalLength ?? 0 }
}

function pluginTest(): Record<string, unknown> {
  return { status: 'ok', matched: 'https://example.com/demo.bin' }
}

export function handleDemoCommand(cmd: string, payload?: unknown): unknown {
  switch (cmd) {
    case 'plugin:store|load': {
      const path = field<string>(payload, 'path') ?? 'config.json'
      const rid = nextStoreId++
      storeIds.set(rid, path)
      return rid
    }
    case 'plugin:store|get_store': {
      const path = field<string>(payload, 'path') ?? 'config.json'
      const found = [...storeIds.entries()].find(([, value]) => value === path)
      return found ? found[0] : null
    }
    case 'plugin:store|get': {
      const store = storeForRid(field<number>(payload, 'rid') ?? 0)
      const key = field<string>(payload, 'key') ?? ''
      return [store.get(key) ?? null, store.has(key)]
    }
    case 'plugin:store|set': {
      const store = storeForRid(field<number>(payload, 'rid') ?? 0)
      store.set(field<string>(payload, 'key') ?? '', field<unknown>(payload, 'value'))
      return null
    }
    case 'plugin:store|has':
      return storeForRid(field<number>(payload, 'rid') ?? 0).has(field<string>(payload, 'key') ?? '')
    case 'plugin:store|delete':
      storeForRid(field<number>(payload, 'rid') ?? 0).delete(field<string>(payload, 'key') ?? '')
      return true
    case 'plugin:store|clear':
      storeForRid(field<number>(payload, 'rid') ?? 0).clear()
      return null
    case 'plugin:store|keys':
      return [...storeForRid(field<number>(payload, 'rid') ?? 0).keys()]
    case 'plugin:store|values':
      return [...storeForRid(field<number>(payload, 'rid') ?? 0).values()]
    case 'plugin:store|entries':
      return [...storeForRid(field<number>(payload, 'rid') ?? 0).entries()]
    case 'plugin:store|length':
      return storeForRid(field<number>(payload, 'rid') ?? 0).size
    case 'plugin:store|save':
    case 'plugin:store|reload':
    case 'plugin:store|reset':
      return null

    case 'plugin:os|locale':
      return navigator.language || 'zh-CN'
    case 'plugin:app|version':
      return '1.0.4'
    case 'plugin:app|name':
      return 'Rayload'
    case 'plugin:path|resolve_directory':
      return APP_DATA_DIR
    case 'plugin:path|join':
      return (field<string[]>(payload, 'paths') ?? []).join('/')
    case 'plugin:path|normalize':
      return field<string>(payload, 'path') ?? ''
    case 'plugin:path|resolve':
      return field<string>(payload, 'path') ?? ''
    case 'plugin:path|basename':
      return (field<string>(payload, 'path') ?? '').split('/').pop() ?? ''
    case 'plugin:path|dirname':
      return (field<string>(payload, 'path') ?? '').split('/').slice(0, -1).join('/')
    case 'plugin:path|extname': {
      const name = field<string>(payload, 'path') ?? ''
      const dot = name.lastIndexOf('.')
      return dot > 0 ? name.slice(dot) : ''
    }
    case 'plugin:path|is_absolute':
      return true

    case 'plugin:window|is_maximized':
    case 'plugin:window|is_fullscreen':
      return false
    case 'plugin:window|is_visible':
      return true
    case 'plugin:window|scale_factor':
      return 1

    case 'build_info':
      return buildInfo
    case 'engine_supervisor_state':
    case 'engine_ensure_running':
    case 'engine_restart':
    case 'engine_stop':
    case 'engine_recover_runtime_state':
    case 'engine_cancel':
      return { ...engine }
    case 'tasks_refresh':
      return snapshot()
    case 'aria2_get_global_stat':
      return rawStat()
    case 'aria2_get_version':
      return { version: '2.1.0', enabledFeatures: ['BitTorrent', 'Metalink', 'XML-RPC'] }
    case 'aria2_fetch_active_task_list':
      return localTasks.filter((item) => item.task.status === 'active').map(currentTask)
    case 'aria2_fetch_task_list': {
      const type = field<string>(payload, 'type') ?? 'all'
      const tasks = localTasks.map(currentTask)
      if (type === 'active') return tasks.filter((task) => task.status === 'active')
      if (type === 'waiting') return tasks.filter((task) => task.status === 'waiting' || task.status === 'paused')
      if (type === 'stopped') return tasks.filter((task) => task.status === 'complete' || task.status === 'error')
      return tasks
    }
    case 'aria2_fetch_task_item': {
      const gid = field<string>(payload, 'gid') ?? ''
      return currentTask(findLocal(gid) ?? localTasks[0])
    }
    case 'aria2_fetch_task_item_with_peers': {
      const gid = field<string>(payload, 'gid') ?? ''
      return { ...currentTask(findLocal(gid) ?? localTasks[0]), peers: [] }
    }
    case 'aria2_get_files': {
      const gid = field<string>(payload, 'gid') ?? ''
      return findLocal(gid)?.task.files ?? []
    }
    case 'aria2_get_option':
      return {}
    case 'aria2_get_bt_trackers':
      return []
    case 'aria2_pause':
    case 'aria2_force_pause': {
      const item = findLocal(field<string>(payload, 'gid') ?? '')
      if (item) {
        item.task.status = 'paused'
        item.speed = 0
      }
      return 'OK'
    }
    case 'aria2_unpause': {
      const item = findLocal(field<string>(payload, 'gid') ?? '')
      if (item?.task.status === 'paused') {
        item.task.status = 'active'
        item.speed = item.baseSpeed
      }
      return 'OK'
    }
    case 'aria2_force_pause_all':
      for (const item of localTasks) {
        if (item.task.status === 'active') {
          item.task.status = 'paused'
          item.speed = 0
        }
      }
      return 'OK'
    case 'aria2_resume_eligible':
      return { resumed: 0, blocked: 0 }
    case 'aria2_save_session':
      return 'OK'
    case 'aria2_finish_sharing': {
      const item = findLocal(field<string>(payload, 'gid') ?? '')
      if (item) {
        item.task.seeder = 'false'
        item.task.status = 'complete'
        item.task.uploadSpeed = '0'
      }
      return 'OK'
    }
    case 'aria2_delete_task':
    case 'aria2_force_remove':
    case 'aria2_remove_download_result': {
      const gid = field<string>(payload, 'gid') ?? ''
      removeLocal(gid)
      return gid
    }
    case 'aria2_batch_delete_tasks': {
      const tasks = field<Array<{ gid: string }>>(payload, 'tasks') ?? []
      for (const task of tasks) removeLocal(task.gid)
      return { succeeded: tasks.map((task) => task.gid), failed: [] }
    }
    case 'aria2_batch_finish_sharing': {
      const gids = field<string[]>(payload, 'gids') ?? []
      for (const gid of gids) {
        const item = findLocal(gid)
        if (item) {
          item.task.seeder = 'false'
          item.task.status = 'complete'
        }
      }
      return { succeeded: gids, failed: [] }
    }
    case 'aria2_add_uri': {
      const uris = field<string[]>(payload, 'uris') ?? []
      const options = field<Record<string, string>>(payload, 'options') ?? {}
      const uri = uris[0] ?? ''
      return addLocal(options.out || baseName(uri), 2.2 * GB, 9.6 * MB)
    }
    case 'aria2_add_torrent': {
      const options = field<Record<string, string>>(payload, 'options') ?? {}
      return addLocal(options.out || 'Rayload.Demo.Torrent.iso', 4.8 * GB, 15.2 * MB)
    }
    case 'tasks_find_duplicates':
      return []
    case 'tasks_set_detailed':
    case 'aria2_change_global_option':
    case 'aria2_change_option':
    case 'aria2_select_bt_files':
    case 'aria2_defer_bt_selection':
    case 'aria2_force_bt_recheck':
    case 'aria2_replace_bt_trackers':
    case 'aria2_replace_bt_web_seeds':
    case 'aria2_replace_uri':
    case 'aria2_purge_task_records':
      return null

    case 'database_initialize':
    case 'database_reset':
      return null
    case 'database_schema_version':
      return 1
    case 'history_get_records':
      return field<string>(payload, 'status')
        ? historyRecords.filter((record) => record.status === field<string>(payload, 'status'))
        : historyRecords
    case 'history_get_page':
      return { records: historyRecords, total: historyRecords.length }
    case 'history_get_record':
      return historyRecords.find((record) => record.gid === field<string>(payload, 'gid')) ?? null
    case 'history_load_births':
      return historyRecords.map((record) => [record.gid, record.added_at ?? record.created_at ?? isoAgo(1)])
    case 'history_record_birth':
    case 'history_add_record':
      return null
    case 'history_remove_record':
    case 'history_remove_births':
    case 'history_remove_stale':
    case 'history_clear_records': {
      const gid = field<string>(payload, 'gid')
      if (gid) {
        const index = historyRecords.findIndex((record) => record.gid === gid)
        if (index >= 0) historyRecords.splice(index, 1)
      }
      return null
    }
    case 'history_check_integrity':
      return 'ok'

    case 'lan_devices':
      return devices
    case 'lan_remote_tasks':
      return remoteTasks
    case 'lan_device_statuses':
      return statuses
    case 'lan_device_history_stats':
      return historyStats
    case 'lan_discovered':
    case 'lan_scan':
      return discovered
    case 'lan_credentials':
      return { username: 'rayload', configured: true, name: 'Pixel 9 Pro' }
    case 'lan_pairing_info':
      return {
        addresses: ['192.168.1.31'],
        username: 'rayload',
        password: 'demo-pass',
        name: 'Pixel 9 Pro',
        arch: 'aarch64',
        fingerprint: 'RL-DEMO-PIXEL-31A7',
      } satisfies LanPairingInfo
    case 'lan_remote_action': {
      const gid = field<string>(payload, 'gid') ?? ''
      const action = field<string>(payload, 'action') ?? ''
      const task = findRemote(gid)
      if (task) {
        if (action === 'pause') {
          task.status = 'paused'
          task.downloadSpeed = 0
        } else if (action === 'unpause') {
          task.status = 'active'
          task.downloadSpeed = 9.2 * MB
        } else if (action === 'stop-seeding') {
          task.seeder = false
          task.status = 'complete'
        } else if (action === 'remove') {
          remoteTasks.splice(remoteTasks.indexOf(task), 1)
        }
      }
      return null
    }
    case 'lan_remote_fetch': {
      const gid = field<string>(payload, 'gid') ?? ''
      const task = findRemote(gid)
      return addLocal(task?.filename ?? 'remote-download.bin', task?.totalLength ?? 1.5 * GB, 11.4 * MB)
    }
    case 'lan_remote_checksum':
      return remoteChecksum(field<string>(payload, 'gid') ?? '')
    case 'lan_handoff': {
      const deviceId = field<string>(payload, 'deviceId') ?? 'mac'
      const url = field<string>(payload, 'url') ?? ''
      const device = devices.find((item) => item.id === deviceId)
      const task: RemoteTask = {
        deviceId,
        deviceName: device?.name ?? 'Rayload',
        gid: `remote-${Date.now().toString(36)}`,
        url,
        filename: field<string>(payload, 'filename') ?? baseName(url),
        status: 'active',
        totalLength: 2.2 * GB,
        completedLength: 0.18 * GB,
        downloadSpeed: 9.1 * MB,
        online: true,
        dir: '/home/demo/Downloads',
        files: [
          {
            index: '1',
            path: field<string>(payload, 'filename') ?? baseName(url),
            length: 2.2 * GB,
            completedLength: 0.18 * GB,
            selected: true,
          },
        ],
      }
      remoteTasks.unshift(task)
      return task
    }
    case 'lan_set_credentials':
    case 'lan_set_device_name':
    case 'lan_forget_device':
    case 'lan_rename_device':
    case 'lan_set_device_note':
    case 'lan_confirm_device':
    case 'lan_pair_approve':
    case 'lan_pair_reject':
      return null
    case 'lan_pair':
      return { id: 'mac', name: 'MacBook Pro M4', fingerprint: 'RL-DEMO-MAC-8C21' }
    case 'lan_pair_discovered':
      return { id: 'mac', name: 'MacBook Pro M4', fingerprint: 'RL-DEMO-MAC-8C21' }
    case 'lan_pair_requests':
      return []

    case 'plugin_list':
      return plugins
    case 'plugin_sample_manifest':
      return JSON.stringify({
        name: 'rayload-download-test',
        version: '1.0.0',
        type: 'rule',
        permissions: ['network'],
        allowedDomains: ['example.com'],
      })
    case 'plugin_test':
      return pluginTest()
    case 'plugin_save':
    case 'plugin_save_script':
    case 'plugin_remove':
      return null

    case 'is_autostart_launch':
    case 'peek_pending_deep_links_silent':
    case 'peek_pending_external_inputs_silent':
    case 'clipboard_baseline_wanted':
    case 'gpu_acceleration_unstable':
      return false
    case 'take_android_shared_links':
    case 'take_pending_frontend_actions':
      return []
    case 'take_pending_deep_links':
      return { urls: [], silent: false }
    case 'take_pending_external_inputs':
      return { inputs: [], silent: false }
    case 'get_ed2k_bootstrap_status':
      return { serverMetModified: null, nodesDatModified: null }
    case 'check_path_exists':
      return false
    case 'task_file_states':
      return taskFileStates(payload)
    case 'get_system_proxy':
      return { server: '', bypass: '', unsupportedBypass: [], isSocks: false }
    case 'disk_free_space':
      return 128 * GB
    case 'write_settings_backup_file':
    case 'clear_log_file':
    case 'replace_system_config':
    case 'refresh_runtime_config':
    case 'sync_autostart':
    case 'set_autostart':
    case 'set_dock_visible':
    case 'update_tray_menu_labels':
    case 'update_menu_labels':
    case 'minimize_to_tray':
    case 'add_window_prepare':
    case 'cancel_shutdown':
    case 'set_app_log_level':
    case 'send_app_system_notification':
      return null

    default:
      return undefined
  }
}

let installed = false

/** Installs the fake IPC layer and starts the animated snapshot pump. */
export function installDemoBackend(): void {
  if (installed) return
  installed = true
  try {
    window.localStorage.setItem('rayload-onboarding-v3', 'complete')
    window.localStorage.setItem('rayload-whats-new-seen', '1.0.4')
    window.localStorage.setItem(
      'rayload.bottom-navigation.visibility.v2',
      JSON.stringify({
        all: true,
        progress: true,
        failed: true,
        completed: true,
        'keep-awake': true,
        statistics: true,
        'lan-devices': true,
        plugins: true,
        trash: true,
        about: true,
      }),
    )
  } catch {
    // Hardened WebViews can reject storage; the demo still renders.
  }
  // A real Tauri runtime freezes its internals; demo builds then only replace
  // the app's invoke import and keep native window/event IPC alive.
  if (!('__TAURI_INTERNALS__' in window)) {
    window.__TAURI_OS_PLUGIN_INTERNALS__ = {
      eol: '\n',
      os_type: 'android',
      platform: 'android',
      family: 'unix',
      version: '17',
      arch: 'aarch64',
      exe_extension: '',
    }
    mockWindows('main')
    mockIPC((cmd, payload) => handleDemoCommand(cmd, payload), { shouldMockEvents: true })
  }
  window.setInterval(() => {
    tick()
    void emit('tasks:snapshot', snapshot()).catch(() => undefined)
  }, 900)
}
