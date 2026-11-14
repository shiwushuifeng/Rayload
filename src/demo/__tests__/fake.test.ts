import { describe, expect, it } from 'vitest'
import { handleDemoCommand } from '@/demo/fake'
import type { TaskSnapshot } from '@shared/types'
import type { RemoteChecksum, RemoteTask } from '@/stores/lan'

describe('demo IPC backend', () => {
  it('serves animated tasks, remote tasks and remote file operations', () => {
    const snapshot = handleDemoCommand('tasks_refresh') as TaskSnapshot
    expect(snapshot.tasks.some((task) => task.status === 'active')).toBe(true)
    expect(snapshot.stat.downloadSpeed).toBeGreaterThan(0)

    const remote = handleDemoCommand('lan_remote_tasks') as RemoteTask[]
    expect(remote.some((task) => task.seeder)).toBe(true)

    const checksum = handleDemoCommand('lan_remote_checksum', { gid: remote[0].gid }) as RemoteChecksum
    expect(checksum.md5).toHaveLength(32)
    expect(checksum.sha256).toHaveLength(64)

    handleDemoCommand('lan_remote_action', { gid: remote[0].gid, action: 'stop-seeding' })
    expect(remote[0].seeder).toBe(false)
    expect(handleDemoCommand('unknown_command')).toBeUndefined()
  })
})
