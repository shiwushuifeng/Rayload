#!/usr/bin/env node
/** Adds the v1.0.4-beta5 onboarding, plugin entry and LAN copy. */
import { readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = join(dirname(fileURLToPath(import.meta.url)), '..')
const localesRoot = join(root, 'src', 'shared', 'locales')

const english = {
  'preferences.plugin-management': 'Plugin management',
  'task.remote-target-offline': 'The target device is currently offline',
  'onboarding.skip': 'Skip',
  'onboarding.default-download-path': 'Default download path',
  'onboarding.consent-disclaimer-hint': 'Technical use, copyright and liability',
  'lan-devices.choose-color': 'Choose device color',
}

const translations = {
  'zh-CN': {
    'preferences.plugin-management': '插件管理',
    'task.remote-target-offline': '目标设备当前离线',
    'onboarding.skip': '跳过',
    'onboarding.default-download-path': '默认下载路径',
    'onboarding.consent-disclaimer-hint': '技术用途、版权与责任说明',
    'lan-devices.choose-color': '选择设备颜色',
  },
  'zh-TW': {
    'preferences.plugin-management': '外掛管理',
    'task.remote-target-offline': '目標裝置目前離線',
    'onboarding.skip': '略過',
    'onboarding.default-download-path': '預設下載路徑',
    'onboarding.consent-disclaimer-hint': '技術用途、版權與責任說明',
    'lan-devices.choose-color': '選擇裝置顏色',
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

const removedKeys = ['navigation.extensions', 'plugins.manage']

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

console.log('Added v1.0.4-beta5 UI copy to every locale.')
