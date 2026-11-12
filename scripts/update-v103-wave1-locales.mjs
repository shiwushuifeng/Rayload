#!/usr/bin/env node
/** Adds v1.0.3 wave-one download UI strings to every frontend locale. */
import { readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = join(dirname(fileURLToPath(import.meta.url)), '..')
const localesRoot = join(root, 'src', 'shared', 'locales')

const english = {
  paste: 'Paste',
  clear: 'Clear',
  'paste-success': 'Pasted {count} link(s)',
  'paste-no-new': 'No new links in the clipboard',
  'clipboard-empty': 'Clipboard is empty',
  'paste-failed': 'Could not read the clipboard',
  'uri-task-count': '{count} task(s) will be created',
  'anti-leech': 'Anti-leech protection',
  'task-tab-checksum': 'Checksums',
  'checksum-file': 'File',
  'checksum-no-file': 'No downloaded file found',
  'checksum-recalculate': 'Recalculate',
  'checksum-computing': 'Computing in the background…',
  'checksum-retry': 'Retry',
  'checksum-copy': 'Copy {algorithm}',
  'checksum-copied': '{algorithm} copied',
  'checksum-copy-failed': 'Could not copy the checksum',
  'checksum-github-title': 'GitHub release verification',
  'checksum-github-refresh': 'Refresh GitHub digest',
  'checksum-github-loading': 'Reading the digest published by GitHub…',
  'checksum-match': 'Match',
  'checksum-mismatch': 'Mismatch',
  'checksum-github-unavailable': 'GitHub verification is unavailable right now',
  'checksum-github-none': 'No GitHub release digest found for this file',
  'checksum-expected': 'Expected checksum',
  'checksum-expected-placeholder': 'Paste MD5, SHA-1, SHA-256 or SHA-512',
  'checksum-expected-invalid': 'This is not a valid checksum',
}

const translations = {
  'zh-CN': {
    paste: '粘贴',
    clear: '清空',
    'paste-success': '已粘贴 {count} 个链接',
    'paste-no-new': '剪贴板中没有新链接',
    'clipboard-empty': '剪贴板为空',
    'paste-failed': '读取剪贴板失败',
    'uri-task-count': '将创建 {count} 个任务',
    'anti-leech': '防盗链',
    'task-tab-checksum': '校验',
    'checksum-file': '文件',
    'checksum-no-file': '未找到已下载文件',
    'checksum-recalculate': '重新计算',
    'checksum-computing': '正在后台计算…',
    'checksum-retry': '重试',
    'checksum-copy': '复制 {algorithm}',
    'checksum-copied': '已复制 {algorithm}',
    'checksum-copy-failed': '复制校验值失败',
    'checksum-github-title': 'GitHub 发布校验',
    'checksum-github-refresh': '刷新 GitHub 校验值',
    'checksum-github-loading': '正在读取 GitHub 发布的校验值…',
    'checksum-match': '匹配',
    'checksum-mismatch': '不匹配',
    'checksum-github-unavailable': '暂时无法使用 GitHub 校验',
    'checksum-github-none': '未找到该文件的 GitHub 发布校验值',
    'checksum-expected': '期望校验值',
    'checksum-expected-placeholder': '粘贴 MD5、SHA-1、SHA-256 或 SHA-512',
    'checksum-expected-invalid': '校验值格式不正确',
  },
  'zh-TW': {
    paste: '貼上',
    clear: '清除',
    'paste-success': '已貼上 {count} 個連結',
    'paste-no-new': '剪貼簿中沒有新連結',
    'clipboard-empty': '剪貼簿為空',
    'paste-failed': '讀取剪貼簿失敗',
    'uri-task-count': '將建立 {count} 個任務',
    'anti-leech': '防盜鏈',
    'task-tab-checksum': '校驗',
    'checksum-file': '檔案',
    'checksum-no-file': '找不到已下載檔案',
    'checksum-recalculate': '重新計算',
    'checksum-computing': '正在背景計算…',
    'checksum-retry': '重試',
    'checksum-copy': '複製 {algorithm}',
    'checksum-copied': '已複製 {algorithm}',
    'checksum-copy-failed': '複製校驗值失敗',
    'checksum-github-title': 'GitHub 發佈校驗',
    'checksum-github-refresh': '重新整理 GitHub 校驗值',
    'checksum-github-loading': '正在讀取 GitHub 發佈的校驗值…',
    'checksum-match': '相符',
    'checksum-mismatch': '不相符',
    'checksum-github-unavailable': '暫時無法使用 GitHub 校驗',
    'checksum-github-none': '找不到此檔案的 GitHub 發佈校驗值',
    'checksum-expected': '預期校驗值',
    'checksum-expected-placeholder': '貼上 MD5、SHA-1、SHA-256 或 SHA-512',
    'checksum-expected-invalid': '校驗值格式不正確',
  },
}

for (const locale of readdirSync(localesRoot)) {
  const directory = join(localesRoot, locale)
  if (!statSync(directory).isDirectory()) continue
  const path = join(directory, 'messages.json')
  const messages = JSON.parse(readFileSync(path, 'utf8'))
  messages.task ??= {}
  Object.assign(messages.task, english, translations[locale] ?? {})
  writeFileSync(path, `${JSON.stringify(messages, null, 2)}\n`)
}

console.log('✓ v1.0.3 wave-one localization added to all frontend locales')
