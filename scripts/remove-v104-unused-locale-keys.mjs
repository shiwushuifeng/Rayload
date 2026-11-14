#!/usr/bin/env node
/** Drops locale keys whose controls were removed. */
import { readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = join(dirname(fileURLToPath(import.meta.url)), '..')
const localesRoot = join(root, 'src', 'shared', 'locales')
const removed = [
  ['statistics', 'open-extensions'],
  ['statistics', 'coming-soon'],
  ['plugins', 'sample-installed'],
]

for (const locale of readdirSync(localesRoot)) {
  const directory = join(localesRoot, locale)
  if (!statSync(directory).isDirectory()) continue
  const path = join(directory, 'messages.json')
  const messages = JSON.parse(readFileSync(path, 'utf8'))
  for (const [namespace, key] of removed) delete messages[namespace]?.[key]
  writeFileSync(path, `${JSON.stringify(messages, null, 2)}\n`)
}
console.log('Removed unused locale keys.')
