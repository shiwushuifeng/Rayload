/** @fileoverview Session-scoped rolling download speed samples for the statistics chart. */
import { readonly, ref } from 'vue'

export interface SpeedSample {
  at: number
  bytesPerSecond: number
}

export const SPEED_SAMPLE_INTERVAL_MS = 2000
export const SPEED_SAMPLE_LIMIT = 90

const samples = ref<SpeedSample[]>([])
const startedAt = ref(0)

function recordSpeed(bytesPerSecond: number): void {
  const value = Number.isFinite(bytesPerSecond) ? Math.max(0, Math.round(bytesPerSecond)) : 0
  const next = [...samples.value, { at: Date.now(), bytesPerSecond: value }]
  samples.value = next.slice(-SPEED_SAMPLE_LIMIT)
  if (startedAt.value === 0) startedAt.value = Date.now()
}

function resetSpeedTrend(): void {
  samples.value = []
  startedAt.value = 0
}

/** Keeps a rolling window alive across route transitions for the current app session. */
export function useSpeedTrend() {
  return {
    samples: readonly(samples),
    startedAt: readonly(startedAt),
    recordSpeed,
    resetSpeedTrend,
  }
}
