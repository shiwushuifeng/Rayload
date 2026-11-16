#!/usr/bin/env node
/** Adds the v1.0.4-beta7 checksum system copy (panel and settings) to every locale. */
import { readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = join(dirname(fileURLToPath(import.meta.url)), '..')
const localesRoot = join(root, 'src', 'shared', 'locales')

const english = {
  'task.checksum-expected-placeholder':
    'Paste a checksum: MD5, SHA-1, SHA-256, SHA-512, BLAKE3, SM3 or CRC32, with or without a sha256: prefix',
  'task.checksum-cancel': 'Cancel calculation',
  'task.remote-removed-offline': 'Device offline: removed from this list, the remote task stays',
  'task.install-apk': 'Install package',
  'task.install-apk-started': 'The package installer is opening',
  'task.install-apk-failed': 'Could not start the package installer',
  'task.preview-media': 'Preview',
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
  'statistics.clear-cache': 'Clear download cache',
  'statistics.clear-cache-hint': 'Remove finished control files and cached data',
  'statistics.clear-cache-done': 'Freed {size}',
  'statistics.storage-analysis': 'Storage space analysis',
  'statistics.storage-analysis-title': 'Storage space',
  'statistics.storage-downloads': 'Downloads',
  'statistics.storage-cache': 'Cache',
  'statistics.storage-data': 'App data',
  'statistics.storage-temp': 'Temp files',
  'statistics.storage-free': 'Free space',
  'statistics.storage-missing': 'Not created yet',
  'statistics.storage-close': 'Close',
}

const translations = {
  'zh-CN': {
    'task.checksum-expected-placeholder':
      '粘贴校验值：支持 MD5、SHA-1、SHA-256、SHA-512、BLAKE3、SM3、CRC32，可带 sha256: 前缀',
    'task.checksum-cancel': '取消计算',
    'task.remote-removed-offline': '设备离线：已从本机列表移除，远端任务保留',
    'task.install-apk': '安装应用',
    'task.install-apk-started': '正在打开安装程序',
    'task.install-apk-failed': '无法启动安装程序',
    'task.preview-media': '预览',
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
    'statistics.clear-cache': '清理下载缓存',
    'statistics.clear-cache-hint': '删除已完成的控制文件与缓存数据',
    'statistics.clear-cache-done': '已释放 {size}',
    'statistics.storage-analysis': '存储空间分析',
    'statistics.storage-analysis-title': '存储空间',
    'statistics.storage-downloads': '下载目录',
    'statistics.storage-cache': '缓存',
    'statistics.storage-data': '应用数据',
    'statistics.storage-temp': '临时文件',
    'statistics.storage-free': '可用空间',
    'statistics.storage-missing': '尚未创建',
    'statistics.storage-close': '关闭',
  },
  'zh-TW': {
    'task.checksum-expected-placeholder':
      '貼上校驗值：支援 MD5、SHA-1、SHA-256、SHA-512、BLAKE3、SM3、CRC32，可帶 sha256: 前置字串',
    'task.checksum-cancel': '取消計算',
    'task.remote-removed-offline': '裝置離線：已從本機清單移除，遠端工作保留',
    'task.install-apk': '安裝應用程式',
    'task.install-apk-started': '正在開啟安裝程式',
    'task.install-apk-failed': '無法啟動安裝程式',
    'task.preview-media': '預覽',
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
    'statistics.clear-cache': '清理下載快取',
    'statistics.clear-cache-hint': '刪除已完成的控制檔與快取資料',
    'statistics.clear-cache-done': '已釋放 {size}',
    'statistics.storage-analysis': '儲存空間分析',
    'statistics.storage-analysis-title': '儲存空間',
    'statistics.storage-downloads': '下載目錄',
    'statistics.storage-cache': '缓存',
    'statistics.storage-data': '應用資料',
    'statistics.storage-temp': '暫存檔案',
    'statistics.storage-free': '可用空間',
    'statistics.storage-missing': '尚未建立',
    'statistics.storage-close': '關閉',
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

// The onboarding engine page no longer shows the default download path box.
const removedKeys = ['onboarding.default-download-path', 'statistics.open-lan', 'statistics.open-history']

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

console.log('Added v1.0.4-beta7 checksum copy to every locale.')
