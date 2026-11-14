import { beforeEach, describe, expect, it, vi } from 'vitest'

const mockPlatform = vi.hoisted(() => vi.fn<() => string>())
vi.mock('@tauri-apps/plugin-os', () => ({ type: mockPlatform }))

beforeEach(() => {
  vi.resetModules()
  mockPlatform.mockReset()
})

describe('Platform labels', () => {
  it('labels remote device architectures without local platform bias', async () => {
    const { deviceArchLabel } = await import('../usePlatform')
    expect([deviceArchLabel('aarch64'), deviceArchLabel('x86_64'), deviceArchLabel('riscv64')]).toEqual([
      'ARM64',
      'x64',
      'riscv64',
    ])
  })

  it.each([
    ['macos', 'macOS', true, false, false, 'Apple Silicon', 'Intel'],
    ['windows', 'Windows', false, true, false, 'ARM64', 'x64'],
    ['linux', 'Linux', false, false, true, 'ARM64', 'x64'],
    ['android', 'Android', false, false, false, 'ARM64', 'x64'],
  ])('describes %s', async (platform, label, mac, windows, linux, arm, intel) => {
    mockPlatform.mockReturnValue(platform)
    const { usePlatform } = await import('../usePlatform')
    const state = usePlatform()
    expect([state.platformLabel.value, state.isMac.value, state.isWindows.value, state.isLinux.value]).toEqual([
      label,
      mac,
      windows,
      linux,
    ])
    expect([state.archLabel('aarch64'), state.archLabel('x86_64')]).toEqual([arm, intel])
    expect(state.isAndroid.value).toBe(platform === 'android')
    expect(usePlatform().platform).toBe(state.platform)
    expect(mockPlatform).toHaveBeenCalledOnce()
  })
})
