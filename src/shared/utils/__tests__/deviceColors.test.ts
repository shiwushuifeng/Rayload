import { afterEach, describe, expect, it } from 'vitest'
import {
  clearDeviceColor,
  identityColor,
  identityHue,
  resolveDeviceAccent,
  resolveDeviceColor,
  setDeviceColor,
} from '../deviceColors'

const IDS = ['nas-primary', 'phone-qi', 'laptop-office', 'NAS']

describe('device accent colors', () => {
  afterEach(() => {
    for (const id of IDS) clearDeviceColor(id)
  })

  it('derives a stable flat hex color from the device identity', () => {
    expect(identityColor('nas-primary')).toBe(identityColor('nas-primary'))
    expect(identityColor('nas-primary')).toMatch(/^#[0-9a-f]{6}$/)
    expect(identityHue('nas-primary')).toBeGreaterThanOrEqual(0)
    expect(identityHue('nas-primary')).toBeLessThan(360)
    // Different devices should not collapse onto one hue.
    const hues = new Set(['nas-primary', 'phone-qi', 'laptop-office'].map((id) => identityHue(id)))
    expect(hues.size).toBeGreaterThan(1)
  })

  it('resolves the same flat color for every identity of one device', () => {
    expect(resolveDeviceColor('nas-primary', 'NAS')).toBe(resolveDeviceColor('nas-primary'))
    expect(resolveDeviceAccent('nas-primary', 'NAS')).toBe(resolveDeviceColor('nas-primary'))
    // Flat hex, so a color picker can show exactly the displayed value.
    expect(resolveDeviceAccent('nas-primary')).toMatch(/^#[0-9a-f]{6}$/)
  })

  it('returns the explicitly chosen color unchanged', () => {
    setDeviceColor('nas-primary', '#ff3366', 'NAS')
    expect(resolveDeviceColor('nas-primary')).toBe('#ff3366')
    expect(resolveDeviceColor(undefined as never, 'NAS')).toBe('#ff3366')
    expect(resolveDeviceAccent('nas-primary')).toBe('#ff3366')
  })

  it('falls back to the theme accent when nothing identifies the device', () => {
    expect(resolveDeviceColor(undefined, null)).toBeNull()
    expect(resolveDeviceAccent(undefined, null)).toBe('var(--m3-primary)')
  })
})
