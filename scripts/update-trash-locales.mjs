#!/usr/bin/env node
/** Adds the Android trash UI keys to every supported frontend locale. */

import { readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs'
import { join, resolve } from 'node:path'

const root = resolve(import.meta.dirname, '..')
const localesRoot = join(root, 'src/shared/locales')

const english = {
  navigation: {
    trash: 'Trash',
  },
  trash: {
    title: 'Trash',
    subtitle: 'Files moved to Trash can be restored to their original download folder.',
    empty: 'Trash is empty',
    loading: 'Loading Trash…',
    refresh: 'Refresh Trash',
    restore: 'Restore',
    'restore-success': 'Restored "{name}"',
    'restore-failed': 'Could not restore the file',
    'delete-permanently': 'Delete Permanently',
    'delete-confirm-title': 'Delete Permanently',
    'delete-confirm': 'Permanently delete "{name}"? This cannot be undone.',
    'delete-success': 'File permanently deleted',
    'delete-failed': 'Could not permanently delete the file',
    'load-failed': 'Could not load Trash',
    retry: 'Retry',
    'item-count': '{count} items',
    file: 'File',
    folder: 'Folder',
  },
}

const zhCN = {
  navigation: { trash: '回收站' },
  trash: {
    title: '回收站',
    subtitle: '移到回收站的文件可以恢复到原来的下载文件夹。',
    empty: '回收站为空',
    loading: '正在读取回收站…',
    refresh: '刷新回收站',
    restore: '恢复',
    'restore-success': '已恢复“{name}”',
    'restore-failed': '恢复文件失败',
    'delete-permanently': '彻底删除',
    'delete-confirm-title': '彻底删除文件',
    'delete-confirm': '确定要彻底删除“{name}”吗？此操作无法撤销。',
    'delete-success': '文件已彻底删除',
    'delete-failed': '彻底删除文件失败',
    'load-failed': '无法读取回收站',
    retry: '重试',
    'item-count': '共 {count} 项',
    file: '文件',
    folder: '文件夹',
  },
}

const zhTW = {
  navigation: { trash: '回收站' },
  trash: {
    title: '回收站',
    subtitle: '移至回收站的檔案可以還原到原本的下載資料夾。',
    empty: '回收站是空的',
    loading: '正在讀取回收站…',
    refresh: '重新整理回收站',
    restore: '還原',
    'restore-success': '已還原「{name}」',
    'restore-failed': '還原檔案失敗',
    'delete-permanently': '永久刪除',
    'delete-confirm-title': '永久刪除檔案',
    'delete-confirm': '確定要永久刪除「{name}」嗎？此操作無法復原。',
    'delete-success': '檔案已永久刪除',
    'delete-failed': '永久刪除檔案失敗',
    'load-failed': '無法讀取回收站',
    retry: '重試',
    'item-count': '共 {count} 項',
    file: '檔案',
    folder: '資料夾',
  },
}

const localeMessages = {
  'zh-CN': zhCN,
  'zh-TW': zhTW,
}

for (const locale of readdirSync(localesRoot)) {
  const localeDir = join(localesRoot, locale)
  if (!statSync(localeDir).isDirectory()) continue
  const messagesPath = join(localeDir, 'messages.json')
  const messages = JSON.parse(readFileSync(messagesPath, 'utf8'))
  const additions = localeMessages[locale] ?? english
  messages.navigation = { ...messages.navigation, ...additions.navigation }
  messages.trash = { ...messages.trash, ...additions.trash }
  writeFileSync(messagesPath, `${JSON.stringify(messages, null, 2)}\n`)
}

console.log('✓ trash localization added to all frontend locales')
