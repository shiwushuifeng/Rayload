#!/usr/bin/env node
/** Adds the v1.0.4 LAN device, remote task and F-Droid copy to every locale. */
import { readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = join(dirname(fileURLToPath(import.meta.url)), '..')
const localesRoot = join(root, 'src', 'shared', 'locales')

const english = {
  'navigation.lan-devices': 'LAN devices',
  'preferences.fdroid-update-hint': 'Updates are delivered by the F-Droid client.',
  'statistics.tab-this-device': 'This device',
  'statistics.tab-all-devices': 'All devices',
  'statistics.all-devices-title': 'All devices',
  'statistics.all-devices-hint': 'Live download speed and active tasks from every paired device.',
  'statistics.this-device': 'This device',
  'statistics.device-active-tasks': '{count} active',
  'task.handoff-target': 'Target device',
  'task.handoff-local': 'This device',
  'task.handoff-sent': 'Sent to {device}',
  "task.remote-badge": "Remote {'@'}{name}",
  'task.remote-offline': 'Device offline, status unavailable',
  'task.remote-removed': 'Remote task removed',
  'lan-devices.title': 'LAN devices',
  'lan-devices.description': 'Pair with another Rayload device on the same network to send downloads to it.',
  'lan-devices.host-title': 'This device',
  'lan-devices.host-hint': 'Other devices use these credentials to pair with this one.',
  'lan-devices.username': 'Username',
  'lan-devices.host-password': 'New password',
  'lan-devices.host-save': 'Save credentials',
  'lan-devices.host-ready': 'Admin credentials configured',
  'lan-devices.host-saved': 'Admin credentials saved',
  'lan-devices.add-title': 'Add a device',
  'lan-devices.address': 'IP address or host',
  'lan-devices.password': 'Password',
  'lan-devices.pair': 'Pair',
  'lan-devices.fingerprint-title': 'Verify fingerprint',
  'lan-devices.fingerprint-hint': 'Compare this fingerprint with {name} before trusting it.',
  'lan-devices.fingerprint-confirm': 'Fingerprint matches',
  'lan-devices.paired-title': 'Paired devices',
  'lan-devices.empty': 'No paired devices yet.',
  'lan-devices.online': 'Online',
  'lan-devices.offline': 'Offline',
  'lan-devices.forget': 'Remove',
  'lan-devices.paired': '{name} paired',
}

const translations = {
  'zh-CN': {
    'navigation.lan-devices': '局域网设备',
    'preferences.fdroid-update-hint': '请前往 F-Droid 客户端更新。',
    'statistics.tab-this-device': '本机',
    'statistics.tab-all-devices': '全部设备',
    'statistics.all-devices-title': '全部设备',
    'statistics.all-devices-hint': '查看所有已绑定设备的实时下载速度与活动任务数。',
    'statistics.this-device': '本机',
    'statistics.device-active-tasks': '{count} 个活动任务',
    'task.handoff-target': '目标设备',
    'task.handoff-local': '本机',
    'task.handoff-sent': '已成功发送至 {device}',
    "task.remote-badge": "远程 {'@'}{name}",
    'task.remote-offline': '设备离线，无法获取状态',
    'task.remote-removed': '远程任务已移除',
    'lan-devices.title': '局域网设备',
    'lan-devices.description': '与同一网络中的另一台 Rayload 设备配对，把下载任务发送过去。',
    'lan-devices.host-title': '本机',
    'lan-devices.host-hint': '其他设备使用这组账号密码与本机配对。',
    'lan-devices.username': '管理账号',
    'lan-devices.host-password': '新密码',
    'lan-devices.host-save': '保存账号密码',
    'lan-devices.host-ready': '管理账号已配置',
    'lan-devices.host-saved': '管理账号已保存',
    'lan-devices.add-title': '添加设备',
    'lan-devices.address': 'IP 地址或主机名',
    'lan-devices.password': '密码',
    'lan-devices.pair': '配对',
    'lan-devices.fingerprint-title': '核对指纹',
    'lan-devices.fingerprint-hint': '请与 {name} 上显示的指纹核对后再信任。',
    'lan-devices.fingerprint-confirm': '指纹一致',
    'lan-devices.paired-title': '已配对设备',
    'lan-devices.empty': '还没有已配对的设备。',
    'lan-devices.online': '在线',
    'lan-devices.offline': '离线',
    'lan-devices.forget': '移除',
    'lan-devices.paired': '{name} 配对成功',
  },
  'zh-TW': {
    'navigation.lan-devices': '區域網路裝置',
    'preferences.fdroid-update-hint': '請前往 F-Droid 用戶端更新。',
    'statistics.tab-this-device': '本機',
    'statistics.tab-all-devices': '全部裝置',
    'statistics.all-devices-title': '全部裝置',
    'statistics.all-devices-hint': '查看所有已綁定裝置的即時下載速度與活動任務數。',
    'statistics.this-device': '本機',
    'statistics.device-active-tasks': '{count} 個活動任務',
    'task.handoff-target': '目標裝置',
    'task.handoff-local': '本機',
    'task.handoff-sent': '已成功傳送至 {device}',
    "task.remote-badge": "遠端 {'@'}{name}",
    'task.remote-offline': '裝置離線，無法取得狀態',
    'task.remote-removed': '遠端任務已移除',
    'lan-devices.title': '區域網路裝置',
    'lan-devices.description': '與同一網路中的另一台 Rayload 裝置配對，將下載任務傳送過去。',
    'lan-devices.host-title': '本機',
    'lan-devices.host-hint': '其他裝置使用這組帳號密碼與本機配對。',
    'lan-devices.username': '管理帳號',
    'lan-devices.host-password': '新密碼',
    'lan-devices.host-save': '儲存帳號密碼',
    'lan-devices.host-ready': '管理帳號已設定',
    'lan-devices.host-saved': '管理帳號已儲存',
    'lan-devices.add-title': '新增裝置',
    'lan-devices.address': 'IP 位址或主機名稱',
    'lan-devices.password': '密碼',
    'lan-devices.pair': '配對',
    'lan-devices.fingerprint-title': '核對指紋',
    'lan-devices.fingerprint-hint': '請與 {name} 上顯示的指紋核對後再信任。',
    'lan-devices.fingerprint-confirm': '指紋一致',
    'lan-devices.paired-title': '已配對裝置',
    'lan-devices.empty': '還沒有已配對的裝置。',
    'lan-devices.online': '線上',
    'lan-devices.offline': '離線',
    'lan-devices.forget': '移除',
    'lan-devices.paired': '{name} 配對成功',
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
console.log('Updated LAN and F-Droid copy in every locale.')
