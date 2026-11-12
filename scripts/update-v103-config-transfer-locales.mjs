#!/usr/bin/env node
/** Aligns config transfer copy with the JSON-only transfer flow. */
import { readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = join(dirname(fileURLToPath(import.meta.url)), '..')
const localesRoot = join(root, 'src', 'shared', 'locales')

const english = {
  'preferences.config-transfer-button': 'JSON import / export',
  'preferences.config-export-failed': 'Could not build the configuration export.',
  'preferences.config-import-empty': 'Paste the configuration JSON first.',
}

const translations = {
  'zh-CN': {
    'preferences.config-transfer-button': 'JSON 导入 / 导出',
    'preferences.config-export-failed': '无法生成配置导出。',
    'preferences.config-import-empty': '请先粘贴配置 JSON。',
  },
  'zh-TW': {
    'preferences.config-transfer-button': 'JSON 匯入 / 匯出',
    'preferences.config-export-failed': '無法產生設定匯出。',
    'preferences.config-import-empty': '請先貼上設定 JSON。',
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
console.log('Updated config transfer copy in every locale.')
