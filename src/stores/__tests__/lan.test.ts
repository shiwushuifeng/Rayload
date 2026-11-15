import { beforeEach, describe, expect, it, vi } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'

const invokeMock = vi.hoisted(() => vi.fn())

vi.mock('@tauri-apps/api/core', () => ({
  invoke: (...args: unknown[]) => invokeMock(...args),
}))

vi.mock('@shared/logger', () => ({
  logger: { debug: vi.fn(), warn: vi.fn(), error: vi.fn() },
}))

import { remoteTaskToAria2, useLanStore, type RemoteTask } from '../lan'

const base: RemoteTask = {
  deviceId: 'nas',
  deviceName: 'NAS',
  gid: 'abc',
  url: 'https://example.com/file.zip',
  filename: 'file.zip',
  status: 'active',
  totalLength: 100,
  completedLength: 40,
  downloadSpeed: 2048,
  online: true,
}

describe('remoteTaskToAria2', () => {
  it('maps a remote task onto the local card model', () => {
    const task = remoteTaskToAria2(base)
    expect(task.gid).toBe('remote:nas:abc')
    expect(task.status).toBe('active')
    expect(task.totalLength).toBe('100')
    expect(task.downloadSpeed).toBe('2048')
    expect(task.files[0]?.path).toBe('file.zip')
    expect(task.remote).toMatchObject({ deviceId: 'nas', deviceName: 'NAS', gid: 'abc', online: true })
  })

  it('falls back to a waiting status and the URL name', () => {
    const task = remoteTaskToAria2({ ...base, status: 'unknown', filename: '', url: 'https://example.com/a%20b.mkv' })
    expect(task.status).toBe('waiting')
    expect(task.files[0]?.path).toBe('a b.mkv')
  })

  it('keeps the offline flag', () => {
    expect(remoteTaskToAria2({ ...base, online: false }).remote?.online).toBe(false)
  })

  it('maps remote files, directory and seeding state', () => {
    const task = remoteTaskToAria2({
      ...base,
      status: 'complete',
      dir: '/srv/downloads',
      seeder: true,
      bittorrent: true,
      files: [{ index: '1', path: '/srv/downloads/file.zip', length: 100, completedLength: 100, selected: true }],
    })
    expect(task.dir).toBe('/srv/downloads')
    expect(task.files[0]?.path).toBe('/srv/downloads/file.zip')
    expect(task.files[0]?.selected).toBe('true')
    expect(task.remote).toMatchObject({ seeder: true, bittorrent: true })
  })
})

describe('remote task snapshots', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    invokeMock.mockReset()
  })

  it('keeps a fresh handoff when an older poll response arrives later', async () => {
    const lan = useLanStore()
    let resolveRefresh!: (tasks: RemoteTask[]) => void
    invokeMock.mockImplementationOnce(
      () =>
        new Promise<RemoteTask[]>((resolve) => {
          resolveRefresh = resolve
        }),
    )
    invokeMock.mockResolvedValueOnce({ ...base, gid: 'new' })

    const refreshing = lan.refreshRemoteTasks()
    await lan.handoff('nas', base.url, null)
    expect(lan.remoteTasks.map((task) => task.gid)).toEqual(['new'])

    invokeMock.mockResolvedValue([{ ...base, gid: 'new' }])
    resolveRefresh([])
    await refreshing
    await vi.waitFor(() => expect(lan.remoteTasks.map((task) => task.gid)).toEqual(['new']))
  })
})
