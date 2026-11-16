#!/usr/bin/env node
/** Adds the v1.0.4-beta7 checksum system copy (panel and settings) to every locale. */
import { readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = join(dirname(fileURLToPath(import.meta.url)), '..')
const localesRoot = join(root, 'src', 'shared', 'locales')

const english = {
  'task.checksum-cancel': 'Cancel calculation',
  'task.checksum-export': 'Export checksum file',
  'task.checksum-export-done': 'Checksum file saved to {path}',
  'task.checksum-export-failed': 'Could not write the checksum file',
  'task.checksum-cached': 'From cache',
  'task.checksum-files-title': 'Files in this torrent',
  'task.checksum-files-calculate': 'Calculate',
  'task.checksum-online-title': 'Published checksums',
  'task.checksum-online-loading': 'Looking up public hash databases',
  'task.checksum-online-none': 'No public database knows this file yet',
  'task.checksum-online-found': 'Known as {name}',
  'task.checksum-online-unavailable': 'Public lookup is unavailable right now',
  'task.checksum-auto-verify-match': '{taskName} matches the published checksum',
  'task.checksum-auto-verify-mismatch': '{taskName} does not match the published checksum',
  'preferences.checksum-title': 'Checksums',
  'preferences.checksum-hint': 'Algorithms, automatic verification and published digests',
  'preferences.checksum-algorithms': 'Algorithms',
  'preferences.checksum-algorithms-hint': 'The detail page computes only the enabled algorithms',
  'preferences.checksum-auto-verify': 'Verify after download',
  'preferences.checksum-auto-verify-hint': 'Compare finished downloads with published digests and warn on a mismatch',
  'preferences.checksum-categories': 'Categories to verify',
  'preferences.checksum-categories-hint': 'Leave every category unchecked to verify all downloads',
  'preferences.checksum-online-lookup': 'Look up published checksums',
  'preferences.checksum-online-lookup-hint': 'Ask GitHub and CIRCL Hash Lookup about GitHub-hosted downloads',
  'preferences.checksum-rules': 'Skip rules',
  'preferences.checksum-rules-hint': 'Deny skips verification for matching source URLs, allow keeps it',
  'preferences.checksum-rule-add': 'Add rule',
  'preferences.checksum-rule-pattern': 'URL pattern',
  'preferences.checksum-rule-regex': 'Regex',
  'preferences.checksum-rule-allow': 'Allow',
  'preferences.checksum-rule-deny': 'Deny',
  'preferences.checksum-cache-clear': 'Clear digest cache',
  'preferences.checksum-cache-cleared': 'Removed {count} cached digests',
  'preferences.checksum-cache-clear-failed': 'Could not clear the digest cache',
}

const translations = {
  'zh-CN': {
    'task.checksum-cancel': '取消计算',
    'task.checksum-export': '导出校验文件',
    'task.checksum-export-done': '校验文件已保存到 {path}',
    'task.checksum-export-failed': '无法写入校验文件',
    'task.checksum-cached': '来自缓存',
    'task.checksum-files-title': '此种子内的文件',
    'task.checksum-files-calculate': '计算',
    'task.checksum-online-title': '已公布的校验值',
    'task.checksum-online-loading': '正在查询公开哈希数据库',
    'task.checksum-online-none': '公开数据库暂无此文件的记录',
    'task.checksum-online-found': '已知名称为 {name}',
    'task.checksum-online-unavailable': '当前无法进行公开查询',
    'task.checksum-auto-verify-match': '{taskName} 与公布校验值一致',
    'task.checksum-auto-verify-mismatch': '{taskName} 与公布校验值不一致',
    'preferences.checksum-title': '校验',
    'preferences.checksum-hint': '算法、自动校验与公布值查询',
    'preferences.checksum-algorithms': '算法',
    'preferences.checksum-algorithms-hint': '详情页只计算已启用的算法',
    'preferences.checksum-auto-verify': '下载后自动校验',
    'preferences.checksum-auto-verify-hint': '与公布值比对已完成的任务，不一致时提醒',
    'preferences.checksum-categories': '参与校验的分类',
    'preferences.checksum-categories-hint': '全部不勾选表示校验所有下载',
    'preferences.checksum-online-lookup': '查询已公布的校验值',
    'preferences.checksum-online-lookup-hint': '针对 GitHub 来源向 GitHub 与 CIRCL Hash Lookup 查询',
    'preferences.checksum-rules': '跳过规则',
    'preferences.checksum-rules-hint': '禁止会跳过匹配来源的校验，允许则保留校验',
    'preferences.checksum-rule-add': '添加规则',
    'preferences.checksum-rule-pattern': 'URL 规则',
    'preferences.checksum-rule-regex': '正则',
    'preferences.checksum-rule-allow': '允许',
    'preferences.checksum-rule-deny': '禁止',
    'preferences.checksum-cache-clear': '清空校验缓存',
    'preferences.checksum-cache-cleared': '已清除 {count} 条缓存校验值',
    'preferences.checksum-cache-clear-failed': '无法清空校验缓存',
  },
  'zh-TW': {
    'task.checksum-cancel': '取消計算',
    'task.checksum-export': '匯出校驗檔',
    'task.checksum-export-done': '校驗檔已儲存到 {path}',
    'task.checksum-export-failed': '無法寫入校驗檔',
    'task.checksum-cached': '來自快取',
    'task.checksum-files-title': '此種子內的檔案',
    'task.checksum-files-calculate': '計算',
    'task.checksum-online-title': '已公布的校驗值',
    'task.checksum-online-loading': '正在查詢公開雜湊資料庫',
    'task.checksum-online-none': '公開資料庫尚無此檔案的紀錄',
    'task.checksum-online-found': '已知名稱為 {name}',
    'task.checksum-online-unavailable': '目前無法進行公開查詢',
    'task.checksum-auto-verify-match': '{taskName} 與公布校驗值一致',
    'task.checksum-auto-verify-mismatch': '{taskName} 與公布校驗值不一致',
    'preferences.checksum-title': '校驗',
    'preferences.checksum-hint': '演算法、自動校驗與公布值查詢',
    'preferences.checksum-algorithms': '演算法',
    'preferences.checksum-algorithms-hint': '詳情頁只計算已啟用的演算法',
    'preferences.checksum-auto-verify': '下載後自動校驗',
    'preferences.checksum-auto-verify-hint': '與公布值比對已完成的工作，不一致時提醒',
    'preferences.checksum-categories': '參與校驗的分類',
    'preferences.checksum-categories-hint': '全部不勾選表示校驗所有下載',
    'preferences.checksum-online-lookup': '查詢已公布的校驗值',
    'preferences.checksum-online-lookup-hint': '針對 GitHub 來源向 GitHub 與 CIRCL Hash Lookup 查詢',
    'preferences.checksum-rules': '略過規則',
    'preferences.checksum-rules-hint': '禁止會略過符合來源的校驗，允許則保留校驗',
    'preferences.checksum-rule-add': '新增規則',
    'preferences.checksum-rule-pattern': 'URL 規則',
    'preferences.checksum-rule-regex': '正規表示式',
    'preferences.checksum-rule-allow': '允許',
    'preferences.checksum-rule-deny': '禁止',
    'preferences.checksum-cache-clear': '清空校驗快取',
    'preferences.checksum-cache-cleared': '已清除 {count} 條快取校驗值',
    'preferences.checksum-cache-clear-failed': '無法清空校驗快取',
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

for (const locale of readdirSync(localesRoot)) {
  const directory = join(localesRoot, locale)
  if (!statSync(directory).isDirectory()) continue
  const messagesPath = join(directory, 'messages.json')
  const messages = JSON.parse(readFileSync(messagesPath, 'utf8'))
  const copy = { ...english, ...(translations[locale] ?? {}) }
  for (const [key, value] of Object.entries(copy)) setPath(messages, key, value)
  writeFileSync(messagesPath, `${JSON.stringify(messages, null, 2)}\n`)
}

console.log('Added v1.0.4-beta7 checksum copy to every locale.')
