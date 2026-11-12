#!/usr/bin/env node
/** Adds the live speed trend copy to every locale. */
import { readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = join(dirname(fileURLToPath(import.meta.url)), '..')
const localesRoot = join(root, 'src', 'shared', 'locales')

const english = {
  'statistics.speed-trend-title': 'Live speed trend',
  'statistics.speed-trend-hint': 'Sampled every 2 seconds, rolling window of {minutes} minutes',
  'statistics.speed-trend-empty': 'Collecting samples. The trend appears after a few seconds.',
  'statistics.speed-trend-peak': 'Peak {speed}',
  'statistics.speed-trend-now': 'now',
}

const translations = {
  'zh-CN': {
    'statistics.speed-trend-title': '实时网速趋势',
    'statistics.speed-trend-hint': '每 2 秒采样，滚动保留最近 {minutes} 分钟',
    'statistics.speed-trend-empty': '正在采样，稍后显示趋势。',
    'statistics.speed-trend-peak': '峰值 {speed}',
    'statistics.speed-trend-now': '现在',
  },
  'zh-TW': {
    'statistics.speed-trend-title': '即時網速趨勢',
    'statistics.speed-trend-hint': '每 2 秒取樣，滾動保留最近 {minutes} 分鐘',
    'statistics.speed-trend-empty': '正在取樣，稍後顯示趨勢。',
    'statistics.speed-trend-peak': '峰值 {speed}',
    'statistics.speed-trend-now': '現在',
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
console.log('Updated live speed trend copy in every locale.')
