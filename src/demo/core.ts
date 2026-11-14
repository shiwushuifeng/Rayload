/** @fileoverview Demo replacement for the app's Tauri invoke import. */
import { handleDemoCommand } from './fake'

export * from '@tauri-core-real'

/** Every app command goes to the demo table instead of the native engine. */
export async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  return handleDemoCommand(cmd, args) as T
}
