import { afterEach, describe, expect, it } from 'vitest'
import {
  clearDeviceColor,
  identityHue,
  resolveDeviceAccent,
  resolveDeviceContrastAccent,
  setDeviceColor,
} from '../deviceColors'

const IDs = ['nas-primary', 'phone-qi', 'laptop-office']

describe('device accent colors', () => {
  afterEach(() => {
    for (const id of IDs) clearDeviceColor(id)
    clearDeviceColor('nas-primary', 'NAS')
  })

  it('derives a stable hue from the device identity', () => {
    expect(identityHue('nas-primary')).toBe(identityHue('nas-primary'))
    expect(identityHue('nas-primary')).toBeGreaterThanOrEqual(0)
    expect(identityHue('nas-primary')).toBeLessThan(360)
    // Different devices should not collapse onto one hue.
    const hues = new Set(IDs.map((id) => identityHue(id)))
    expect(hues.size).toBeGreaterThan(1)
  })

  it('resolves the same accent for every identity of one device', () => {
    const byId = resolveDeviceAccent('nas-primary', 'NAS')
    const byName = resolveDeviceAccent(undefined, 'nas-primary')
    expect(byId).toBe(byName)
    expect(byId).toContain('color-mix')
  })

  it('uses the stored color and still resolves it through the theme mix', () => {
    setDeviceColor('nas-primary', '#ff3366', 'NAS')
    expect(resolveDeviceAccent('nas-primary')).toContain('#ff3366')
    expect(resolveDeviceAccent('nas-primary', 'NAS')).toBe(resolveDeviceAccent('nas-primary'))
  })

  it('gives the card gradient a hue that clashes with the rail', () => {
    expect(resolveDeviceContrastAccent('nas-primary')).not.toBe(resolveDeviceAccent('nas-primary'))
    expect(resolveDeviceContrastAccent('nas-primary')).toContain('color-mix')
  })

  it('falls back to the theme accent when nothing identifies the device', () => {
    expect(resolveDeviceAccent(undefined, null)).toBe('var(--m3-primary)')
  })
})
