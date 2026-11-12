#!/usr/bin/env node
/** Removes locale keys that are no longer referenced after the v1.0.3 UI rework. */
import { readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = join(dirname(fileURLToPath(import.meta.url)), '..')
const localesRoot = join(root, 'src', 'shared', 'locales')

const staleKeys = {
  about: ['aria2-version', 'loading', 'unavailable'],
  preferences: ['system-info', 'detected-platform', 'permission-clipboard-note'],
  statistics: ['total-downloaded-hint'],
  onboarding: ['video-folder', 'video-folder-placeholder', 'video-folder-hint'],
}

for (const locale of readdirSync(localesRoot)) {
  const directory = join(localesRoot, locale)
  if (!statSync(directory).isDirectory()) continue
  const path = join(directory, 'messages.json')
  const messages = JSON.parse(readFileSync(path, 'utf8'))
  for (const [namespace, keys] of Object.entries(staleKeys)) {
    const bucket = messages[namespace]
    if (!bucket) continue
    for (const key of keys) delete bucket[key]
  }
  writeFileSync(path, `${JSON.stringify(messages, null, 2)}\n`)
}
console.log('Removed stale v1.0.3 locale keys.')
