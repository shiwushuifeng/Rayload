import { describe, expect, it } from 'vitest'
import { remoteTaskToAria2, type RemoteTask } from '../lan'

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
})
