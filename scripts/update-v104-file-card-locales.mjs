#!/usr/bin/env node
/** Adds the mobile file-card copy actions. */
import { readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = join(dirname(fileURLToPath(import.meta.url)), '..')
const localesRoot = join(root, 'src', 'shared', 'locales')

const english = {
  'task.copy-file-name': 'Copy file name',
  'task.copy-file-path': 'Copy file path',
  'task.file-actions': 'File actions',
}

const translations = {
  'zh-CN': {
    'task.copy-file-name': '复制文件名',
    'task.copy-file-path': '复制文件路径',
    'task.file-actions': '文件操作',
  },
  'zh-TW': {
    'task.copy-file-name': '複製檔案名稱',
    'task.copy-file-path': '複製檔案路徑',
    'task.file-actions': '檔案操作',
  },
}

for (const locale of readdirSync(localesRoot)) {
  const directory = join(localesRoot, locale)
  if (!statSync(directory).isDirectory()) continue
  const path = join(directory, 'messages.json')
  const messages = JSON.parse(readFileSync(path, 'utf8'))
  const copy = { ...english, ...(translations[locale] ?? {}) }
  for (const [flatKey, value] of Object.entries(copy)) {
    const [namespace, key] = flatKey.split('.')
    messages[namespace] ??= {}
    messages[namespace][key] = value
  }
  writeFileSync(path, `${JSON.stringify(messages, null, 2)}\n`)
}
console.log('Updated mobile file-card copy actions.')
