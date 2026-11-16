#!/usr/bin/env node
/** Adds the v1.0.4-beta4 detail tabs and grouped settings navigation copy. */
import { readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = join(dirname(fileURLToPath(import.meta.url)), '..')
const localesRoot = join(root, 'src', 'shared', 'locales')

const english = {
  'task.task-tab-file-details': 'File details',
  'task.task-tab-checksum-files': 'Checksums',
  'task.section-transfer': 'Transfer',
  'task.section-files': 'Files',
  'task.section-options': 'Download options',
  'task.section-peers': 'Peers',
  'task.section-trackers': 'Trackers',
  'preferences.group-account': 'Account & connectivity',
  'preferences.group-account-hint': 'LAN devices, pairing and system permissions',
  'preferences.group-download': 'Downloads',
  'preferences.group-download-hint': 'Downloader, file categories and network',
  'preferences.group-appearance': 'Appearance & experience',
  'preferences.group-appearance-hint': 'Language, theme and interface behavior',
  'preferences.group-advanced': 'Advanced & extensions',
  'preferences.group-advanced-hint': 'Advanced options, plugins and maintenance tools',
}

const translations = {
  'zh-CN': {
    'task.task-tab-file-details': '文件详情',
    'task.task-tab-checksum-files': '校验文件',
    'task.section-transfer': '传输信息',
    'task.section-files': '文件列表',
    'task.section-options': '下载选项',
    'task.section-peers': '节点',
    'task.section-trackers': 'Tracker',
    'preferences.group-account': '账号与互联',
    'preferences.group-account-hint': '局域网设备、配对与系统权限',
    'preferences.group-download': '下载设置',
    'preferences.group-download-hint': '下载器、文件分类与网络',
    'preferences.group-appearance': '外观与体验',
    'preferences.group-appearance-hint': '语言、主题与界面行为',
    'preferences.group-advanced': '高级与扩展',
    'preferences.group-advanced-hint': '高级选项、插件与维护工具',
  },
  'zh-TW': {
    'task.task-tab-file-details': '檔案詳情',
    'task.task-tab-checksum-files': '校驗檔案',
    'task.section-transfer': '傳輸資訊',
    'task.section-files': '檔案列表',
    'task.section-options': '下載選項',
    'task.section-peers': '節點',
    'task.section-trackers': 'Tracker',
    'preferences.group-account': '帳號與互聯',
    'preferences.group-account-hint': '區域網路裝置、配對與系統權限',
    'preferences.group-download': '下載設定',
    'preferences.group-download-hint': '下載器、檔案分類與網路',
    'preferences.group-appearance': '外觀與體驗',
    'preferences.group-appearance-hint': '語言、主題與介面行為',
    'preferences.group-advanced': '進階與擴充',
    'preferences.group-advanced-hint': '進階選項、外掛與維護工具',
  },
}

function setPath(target, dottedKey, value) {
  const parts = dottedKey.split('.')
  let cursor = target
  for (const part of parts.slice(0, -1)) {
    cursor[part] ??= {}
    cursor = cursor[part]
  }
  cursor[parts.at(-1)] = value
}

function deletePath(target, dottedKey) {
  const parts = dottedKey.split('.')
  let cursor = target
  for (const part of parts.slice(0, -1)) {
    cursor = cursor?.[part]
    if (!cursor || typeof cursor !== 'object') return
  }
  delete cursor[parts.at(-1)]
}

const removedKeys = ['task.file-priority', 'task.file-extension', 'task.file-completed', 'task.sort-by']

for (const locale of readdirSync(localesRoot)) {
  const directory = join(localesRoot, locale)
  if (!statSync(directory).isDirectory()) continue
  const messagesPath = join(directory, 'messages.json')
  const messages = JSON.parse(readFileSync(messagesPath, 'utf8'))
  const copy = { ...english, ...(translations[locale] ?? {}) }
  for (const [key, value] of Object.entries(copy)) setPath(messages, key, value)
  for (const key of removedKeys) deletePath(messages, key)
  writeFileSync(messagesPath, `${JSON.stringify(messages, null, 2)}\n`)
}

console.log('Added v1.0.4-beta4 UI copy to every locale.')
