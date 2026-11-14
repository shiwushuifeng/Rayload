#!/usr/bin/env node
/** Adds the LAN permission and all-device history copy to every locale. */
import { readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = join(dirname(fileURLToPath(import.meta.url)), '..')
const localesRoot = join(root, 'src', 'shared', 'locales')

const english = {
  'lan-devices.local-network-title': 'Local network access',
  'lan-devices.local-network-hint':
    'Android 17 requires local network access before Rayload can discover and pair devices.',
  'lan-devices.local-network-grant': 'Allow access',
  'statistics.all-history-hint': 'Completed downloads across every paired device',
  'statistics.all-trend-hint': 'Aggregated from local and paired-device download history',
  'statistics.all-categories-hint': 'Completed downloads grouped by category across all devices',
  'preferences.permission-local-network': 'Local network access',
  'preferences.permission-local-network-hint':
    'Required on Android 17 to discover and pair devices on the same network',
}

const translations = {
  'zh-CN': {
    'lan-devices.local-network-title': '局域网访问权限',
    'lan-devices.local-network-hint': 'Android 17 需要允许访问局域网，才能发现设备并完成配对。',
    'lan-devices.local-network-grant': '允许访问',
    'statistics.all-history-hint': '所有已绑定设备的历史完成任务',
    'statistics.all-trend-hint': '汇总本机与已绑定设备的下载历史',
    'statistics.all-categories-hint': '按类别汇总所有设备的已完成下载',
    'preferences.permission-local-network': '局域网访问',
    'preferences.permission-local-network-hint': 'Android 17 上用于发现并配对同一网络中的设备',
  },
  'zh-TW': {
    'lan-devices.local-network-title': '區域網路存取權限',
    'lan-devices.local-network-hint': 'Android 17 需要允許存取區域網路，才能探索裝置並完成配對。',
    'lan-devices.local-network-grant': '允許存取',
    'statistics.all-history-hint': '所有已綁定裝置的歷史完成任務',
    'statistics.all-trend-hint': '彙整本機與已綁定裝置的下載歷史',
    'statistics.all-categories-hint': '依類別彙整所有裝置的已完成下載',
    'preferences.permission-local-network': '區域網路存取',
    'preferences.permission-local-network-hint': 'Android 17 上用於探索並配對同一網路中的裝置',
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
console.log('Updated LAN permission and all-device history copy in every locale.')
