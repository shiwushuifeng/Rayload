#!/usr/bin/env node
/** Labels the new Settings > Plugins tab with the existing navigation wording. */
import { readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = join(dirname(fileURLToPath(import.meta.url)), '..')
const localesRoot = join(root, 'src', 'shared', 'locales')

for (const locale of readdirSync(localesRoot)) {
  const directory = join(localesRoot, locale)
  if (!statSync(directory).isDirectory()) continue
  const path = join(directory, 'messages.json')
  const messages = JSON.parse(readFileSync(path, 'utf8'))
  messages.preferences ??= {}
  messages.preferences.plugins = messages.navigation?.plugins ?? 'Plugins'
  writeFileSync(path, `${JSON.stringify(messages, null, 2)}\n`)
}
console.log('Added the Plugins settings tab label to every locale.')
