#!/usr/bin/env node
/** Refines the v1.0.3 onboarding: personalization, engine presets, skip, and downloader intro. */
import { readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = join(dirname(fileURLToPath(import.meta.url)), '..')
const localesRoot = join(root, 'src', 'shared', 'locales')

const english = {
  'welcome-description':
    'A fast, private Android downloader powered by Aria2 Next. Supports HTTP, BitTorrent, magnet, ED2K and remote push.',
  'feature-multi-protocol': 'HTTP · BT · Magnet · ED2K',
  'feature-batch': 'Batch links and tasks',
  'feature-background': 'Stable background downloads',
  'feature-private': 'Data stays on this device',
  'personalize-title': 'Make it yours',
  'personalize-description': 'Choose language, appearance and accent color now; change them anytime in Settings.',
  'engine-title': 'Tune the download engine',
  'engine-description': 'Use a quick preset or fine-tune concurrency and connections before your first download.',
  'engine-section': 'Engine',
  'engine-concurrent': 'Concurrent downloads',
  'engine-concurrent-hint': 'How many tasks run at the same time (1–100)',
  'engine-connections': 'Connections per task',
  'engine-connections-hint': 'Parallel server connections per task (1–256)',
  'engine-bittorrent': 'BitTorrent and magnets',
  'engine-bittorrent-hint': 'Enable BT, magnet links and torrent files',
  'engine-preset': 'Quick preset',
  'preset-lite': 'Light',
  'preset-balanced': 'Balanced',
  'preset-fast': 'Fast',
  'archive-section': 'Auto archive',
  skip: 'Skip',
}

const translations = {
  'zh-CN': {
    'welcome-description':
      '基于 Aria2 Next 的快速、私密 Android 下载器，支持 HTTP、BitTorrent、磁力、ED2K 与远程推送。',
    'feature-multi-protocol': 'HTTP · BT · 磁力 · ED2K',
    'feature-batch': '批量链接与多任务',
    'feature-background': '后台稳定下载',
    'feature-private': '数据仅留在本机',
    'personalize-title': '个性化外观',
    'personalize-description': '现在选择语言、主题与配色，之后可随时在设置中修改。',
    'engine-title': '调整下载引擎',
    'engine-description': '可使用推荐预设，也可手动设置并发任务与连接数。',
    'engine-section': '引擎',
    'engine-concurrent': '同时下载任务数',
    'engine-concurrent-hint': '同时运行的任务数量（1–100）',
    'engine-connections': '单任务连接数',
    'engine-connections-hint': '每个任务连接服务器的线程数（1–256）',
    'engine-bittorrent': 'BitTorrent 与磁力链接',
    'engine-bittorrent-hint': '启用 BT、磁力链接与种子文件支持',
    'engine-preset': '推荐预设',
    'preset-lite': '省流',
    'preset-balanced': '均衡',
    'preset-fast': '极速',
    'archive-section': '自动归档',
    skip: '跳过',
  },
  'zh-TW': {
    'welcome-description':
      '基於 Aria2 Next 的快速、私密 Android 下載器，支援 HTTP、BitTorrent、磁力、ED2K 與遠端推送。',
    'feature-multi-protocol': 'HTTP · BT · 磁力 · ED2K',
    'feature-batch': '批次連結與多任務',
    'feature-background': '背景穩定下載',
    'feature-private': '資料僅留在本機',
    'personalize-title': '個人化外觀',
    'personalize-description': '現在選擇語言、主題與配色，之後可隨時在設定中修改。',
    'engine-title': '調整下載引擎',
    'engine-description': '可使用推薦預設，也可手動設定並行任務與連線數。',
    'engine-section': '引擎',
    'engine-concurrent': '同時下載任務數',
    'engine-concurrent-hint': '同時執行的任務數量（1–100）',
    'engine-connections': '單任務連線數',
    'engine-connections-hint': '每個任務連線伺服器的執行緒數（1–256）',
    'engine-bittorrent': 'BitTorrent 與磁力連結',
    'engine-bittorrent-hint': '啟用 BT、磁力連結與種子檔案支援',
    'engine-preset': '推薦預設',
    'preset-lite': '省流',
    'preset-balanced': '均衡',
    'preset-fast': '極速',
    'archive-section': '自動歸檔',
    skip: '跳過',
  },
}

const removedKeys = ['consent', 'consent-hint', 'core-title', 'core-description', 'auto-archive-note']

for (const locale of readdirSync(localesRoot)) {
  const directory = join(localesRoot, locale)
  if (!statSync(directory).isDirectory()) continue
  const path = join(directory, 'messages.json')
  const messages = JSON.parse(readFileSync(path, 'utf8'))
  messages.onboarding ??= {}
  Object.assign(messages.onboarding, english, translations[locale] ?? {})
  for (const key of removedKeys) delete messages.onboarding[key]
  writeFileSync(path, `${JSON.stringify(messages, null, 2)}\n`)
}

console.log('✓ v1.0.3 wave-three onboarding localization updated for all locales')
