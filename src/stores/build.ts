/** @fileoverview Compile-time features of the running build. */
import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { logger } from '@shared/logger'

export interface BuildInfo {
  variant: 'lite' | 'full' | 'fdroid'
  lan: boolean
  pluginEngine: boolean
  fdroid: boolean
}

const LITE: BuildInfo = { variant: 'lite', lan: false, pluginEngine: false, fdroid: false }

export const useBuildInfoStore = defineStore('buildInfo', () => {
  const info = ref<BuildInfo>(LITE)
  let loading: Promise<void> | null = null

  function load(): Promise<void> {
    loading ??= invoke<BuildInfo>('build_info')
      .then((value) => {
        info.value = value
      })
      .catch((error: unknown) => logger.debug('BuildInfo.load', error))
    return loading
  }

  return { info, load }
})
