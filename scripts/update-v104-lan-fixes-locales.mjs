#!/usr/bin/env node
/** Adds the v1.0.4 LAN settings, device dashboard and plugin test copy. */
import { readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = join(dirname(fileURLToPath(import.meta.url)), '..')
const localesRoot = join(root, 'src', 'shared', 'locales')

const english = {
  'statistics.all-devices-online': 'Devices online',
  'statistics.all-devices-speed': 'Total live speed',
  'statistics.all-devices-active': 'Active tasks',
  'plugins.sample': 'Install sample',
  'plugins.sample-installed': '{name} installed',
  'plugins.test': 'Test',
  'plugins.test-passed': '{name} test passed',
  'plugins.test-failed': 'Plugin test failed: {error}',
}

const translations = {
  'zh-CN': {
    'statistics.all-devices-online': '在线设备',
    'statistics.all-devices-speed': '全部设备实时速度',
    'statistics.all-devices-active': '活动任务',
    'plugins.sample': '安装示例插件',
    'plugins.sample-installed': '已安装 {name}',
    'plugins.test': '测试',
    'plugins.test-passed': '{name} 测试通过',
    'plugins.test-failed': '插件测试失败：{error}',
  },
  'zh-TW': {
    'statistics.all-devices-online': '線上裝置',
    'statistics.all-devices-speed': '全部裝置即時速度',
    'statistics.all-devices-active': '活動任務',
    'plugins.sample': '安裝範例外掛',
    'plugins.sample-installed': '已安裝 {name}',
    'plugins.test': '測試',
    'plugins.test-passed': '{name} 測試通過',
    'plugins.test-failed': '外掛測試失敗：{error}',
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
  delete messages.statistics['device-active-tasks']
  writeFileSync(path, `${JSON.stringify(messages, null, 2)}\n`)
}
console.log('Updated LAN settings, device dashboard and plugin test copy.')
