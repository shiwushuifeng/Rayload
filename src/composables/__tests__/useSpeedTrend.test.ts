/** @fileoverview Coverage for the rolling live-speed sample buffer. */
import { beforeEach, describe, expect, it, vi } from 'vitest'
import { SPEED_SAMPLE_LIMIT, useSpeedTrend } from '../useSpeedTrend'

describe('useSpeedTrend', () => {
  const trend = useSpeedTrend()

  beforeEach(() => {
    trend.resetSpeedTrend()
    vi.restoreAllMocks()
  })

  it('keeps only the newest samples within the window limit', () => {
    for (let index = 0; index < SPEED_SAMPLE_LIMIT + 25; index += 1) {
      trend.recordSpeed(index * 1024)
    }
    expect(trend.samples.value).toHaveLength(SPEED_SAMPLE_LIMIT)
    const last = trend.samples.value[trend.samples.value.length - 1]
    expect(last?.bytesPerSecond).toBe((SPEED_SAMPLE_LIMIT + 24) * 1024)
  })

  it('normalizes invalid and negative readings to zero', () => {
    trend.recordSpeed(Number.NaN)
    trend.recordSpeed(-42)
    expect(trend.samples.value.map((sample) => sample.bytesPerSecond)).toEqual([0, 0])
  })

  it('clears samples and the session start stamp on reset', () => {
    trend.recordSpeed(2048)
    expect(trend.startedAt.value).toBeGreaterThan(0)
    trend.resetSpeedTrend()
    expect(trend.samples.value).toEqual([])
    expect(trend.startedAt.value).toBe(0)
  })
})
