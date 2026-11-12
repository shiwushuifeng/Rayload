#!/usr/bin/env node
/** Adds v1.0.3 wave-five onboarding archive category copy to every locale. */
import { readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = join(dirname(fileURLToPath(import.meta.url)), '..')
const localesRoot = join(root, 'src', 'shared', 'locales')

const english = {
  'onboarding.archive-folder-title': 'Folder name',
  'onboarding.archive-category-hint': 'Tap a category to include or exclude it. Long-press to rename its folder.',
  'statistics.trend-hint': 'Aggregated live from your local download history',
  'statistics.categories-hint': 'Completed downloads grouped by file category',
  'statistics.placeholder-tag': 'Live',
  'statistics.shortcuts-title': 'Quick actions',
  'statistics.trend-empty': 'No completed downloads in the last 7 days.',
  'statistics.category-tasks': '{count} tasks',
  'statistics.live-speed': 'Live speed',
  'statistics.live-speed-hint': 'Current total download speed reported by Aria2',
  'statistics.total-cumulative': 'Total downloaded',
  'statistics.total-cumulative-hint': 'Completed history plus this session',
  'statistics.category-other': 'Other',
}

const translations = {
  'zh-CN': {
    'onboarding.archive-folder-title': '文件夹名称',
    'onboarding.archive-category-hint': '点按分类可启用或停用，长按可修改文件夹名称。',
    'statistics.trend-hint': '根据本地下载历史实时聚合',
    'statistics.categories-hint': '按文件分类统计已完成的下载',
    'statistics.placeholder-tag': '实时',
    'statistics.shortcuts-title': '快捷入口',
    'statistics.trend-empty': '近 7 天暂无已完成的任务。',
    'statistics.category-tasks': '{count} 个任务',
    'statistics.live-speed': '实时网速',
    'statistics.live-speed-hint': 'Aria2 当前总下载速度',
    'statistics.total-cumulative': '总下载',
    'statistics.total-cumulative-hint': '历史累计 + 本次会话',
    'statistics.category-other': '其他',
  },
  'zh-TW': {
    'onboarding.archive-folder-title': '資料夾名稱',
    'onboarding.archive-category-hint': '點按分類可啟用或停用，長按可修改資料夾名稱。',
    'statistics.trend-hint': '根據本機下載歷史即時聚合',
    'statistics.categories-hint': '依檔案分類統計已完成的下載',
    'statistics.placeholder-tag': '即時',
    'statistics.shortcuts-title': '快捷入口',
    'statistics.trend-empty': '近 7 天尚無已完成的任務。',
    'statistics.category-tasks': '{count} 個任務',
    'statistics.live-speed': '即時網速',
    'statistics.live-speed-hint': 'Aria2 目前總下載速度',
    'statistics.total-cumulative': '總下載',
    'statistics.total-cumulative-hint': '歷史累計 + 本次工作階段',
    'statistics.category-other': '其他',
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
console.log('Updated onboarding category copy in every locale.')
