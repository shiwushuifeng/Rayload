#!/usr/bin/env node
/** Updates the v1.0.3 polished empty-state copy for every locale. */
import { readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = join(dirname(fileURLToPath(import.meta.url)), '..')
const localesRoot = join(root, 'src', 'shared', 'locales')

const english = {
  'task.empty-hint': 'Everything is ready. Tap + or share a link from your browser to start a fast download.',
  'trash.empty': 'Nothing here yet. Deleted files will be kept here temporarily.',
}

const translations = {
  'zh-CN': {
    'task.empty-hint': '一切准备就绪，点击 + 号或从浏览器分享链接开始极速下载。',
    'trash.empty': '这里空空如也，被删除的文件会暂存在这里。',
  },
  'zh-TW': {
    'task.empty-hint': '一切準備就緒，點擊 + 號或從瀏覽器分享連結開始極速下載。',
    'trash.empty': '這裡空空如也，被刪除的檔案會暫存在這裡。',
  },
}

for (const locale of readdirSync(localesRoot)) {
  const directory = join(localesRoot, locale)
  if (!statSync(directory).isDirectory()) continue
  const path = join(directory, 'messages.json')
  const messages = JSON.parse(readFileSync(path, 'utf8'))
  const copy = { ...english, ...(translations[locale] ?? {}) }
  messages.task ??= {}
  messages.trash ??= {}
  messages.task['empty-hint'] = copy['task.empty-hint']
  messages.trash.empty = copy['trash.empty']
  writeFileSync(path, `${JSON.stringify(messages, null, 2)}\n`)
}

console.log('✓ v1.0.3 empty-state localization updated for all locales')
