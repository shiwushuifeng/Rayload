#!/usr/bin/env node
/** Adds legal, onboarding, settings and release-note copy to every locale. */
import { readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = join(dirname(fileURLToPath(import.meta.url)), '..')
const localesRoot = join(root, 'src', 'shared', 'locales')

const english = {
  'about.disclaimer':
    'Rayload is an open-source, tracking-free download tool. Any content obtained through built-in plugins (such as cloud-drive parsing) is unrelated to the developers; please comply with the relevant cloud-drive terms of service.\n\nRayload 是一款开源、无追踪的下载工具。任何基于本软件内置插件（如网盘解析）获取的内容，均与开发者无关，请遵守相关网盘的服务条款。',
  'preferences.downloader': 'Downloader',
  'preferences.categories': 'File categories',
  'legal.eula-title': 'Rayload User Agreement',
  'legal.privacy-title': 'Rayload Privacy Policy',
  'legal.close': 'Close',
  'legal.consent-label': 'I have read and agree to the User Agreement and Privacy Policy',
  'legal.consent-prefix': 'I have read and agree to the',
  'legal.consent-and': 'and',
  'legal.eula-link': 'User Agreement',
  'legal.privacy-link': 'Privacy Policy',
  'legal.technical-title': 'Technical tool only',
  'legal.technical-body':
    'Rayload only provides P2P and HTTP download technology. It does not store, provide, index, or distribute any downloadable content. All content is downloaded directly from third parties chosen by the user.',
  'legal.copyright-title': 'Copyright compliance',
  'legal.copyright-body':
    'Users must ensure they own the rights to, or are otherwise legally authorized to download, all content. Using Rayload to download pirated software, films, music, or other infringing material is strictly prohibited.',
  'legal.lawful-title': 'Lawful use',
  'legal.lawful-body':
    'Rayload must not be used to distribute malware, illegal information, or to violate any applicable local law or regulation.',
  'legal.liability-title': 'Limitation of liability',
  'legal.liability-body':
    'Any legal dispute or copyright claim arising from the use of Rayload is the sole responsibility of the user. The developers accept no joint or vicarious liability.',
  'legal.privacy-local-title': 'Local-first data',
  'legal.privacy-local-body':
    'Rayload has no account system and no telemetry. Task history, settings and logs stay on your device unless you choose to export or share them.',
  'legal.privacy-tracking-title': 'No tracking',
  'legal.privacy-tracking-body':
    'Rayload does not collect analytics, advertising identifiers, or personal information. Update checks contact the release source only to compare version numbers.',
  'legal.privacy-lan-title': 'Local network',
  'legal.privacy-lan-body':
    'LAN pairing exchanges connection details only between devices you explicitly pair. Data is sent directly over your local network and is not uploaded to a developer server.',
  'whats-new.title': "What's new",
  'whats-new.close': 'Got it',
  'whats-new.1-0-4-lan':
    'LAN devices: pair phones and tablets, send tasks remotely, and aggregate duplicate addresses per device.',
  'whats-new.1-0-4-history':
    'Statistics: all-device history totals, trends and per-category breakdowns, with live sparklines.',
  'whats-new.1-0-4-permission':
    'Android 17 local network permission is now requested in-app and listed in Permission settings.',
  'whats-new.1-0-4-settings':
    'Settings are reorganized: Downloads, BitTorrent, ED2K and Network share one Downloader page; file categories have their own page.',
  'whats-new.1-0-4-wizard':
    'Setup wizard now includes navigation customization, floating-orb options and the user agreement.',
  'onboarding.navigation-section': 'Navigation bar',
  'onboarding.navigation-section-hint':
    'Choose which items appear in the sidebar or bottom navigation. Tap an item to show or hide it.',
  'onboarding.nav-hint-all': 'Every task in one list',
  'onboarding.nav-hint-progress': 'Downloads that are currently running',
  'onboarding.nav-hint-failed': 'Tasks that stopped with an error',
  'onboarding.nav-hint-completed': 'Finished downloads',
  'onboarding.nav-hint-keep-awake': 'Keep the screen and engine awake',
  'onboarding.nav-hint-statistics': 'Speed, history and category insights',
  'onboarding.nav-hint-lan-devices': 'Pair and control other devices',
  'onboarding.nav-hint-trash': 'Restore or permanently delete files',
  'onboarding.nav-hint-about': 'Version, licenses and project links',
  'onboarding.disclaimer-title': 'Disclaimer',
  'onboarding.floating-orb-title': 'Floating speed orb',
  'onboarding.floating-orb-hint': 'Show a draggable live speed panel over other apps',
  'plugins.warning-title': 'Third-party plugin risk',
  'plugins.warning-body':
    'This plugin is provided by a third party and may contain unknown code or violate the target website terms of service. Only load it if you trust its source; otherwise do not continue.',
  'plugins.warning-confirm': 'I understand the risk',
  'plugins.test-download-started': 'Download started: {name}',
  'plugins.type-rule': 'Rule plugin',
  'plugins.type-script': 'Script plugin',
  'plugins.review-title': 'Review plugin permissions',
  'plugins.review-confirm': 'Allow and import',
  'plugins.warning-rule':
    'This JSON rule plugin is provided by a third party. Only continue if you trust its source and the domains it declares.',
  'plugins.warning-script':
    'This is a shell script plugin. It runs local code with the permissions shown below and may behave unpredictably. Only load scripts you completely trust.',
  'plugins.allowed-domains': 'Allowed domains',
  'plugins.no-permissions': 'This plugin declares no privileged permissions.',
  'plugins.permission-network-title': 'Access specific domains',
  'plugins.permission-network-body': 'The plugin may send requests only to: {domains}',
  'plugins.permission-cookies-title': 'Read cloud-drive cookies (high risk)',
  'plugins.permission-cookies-body':
    '⚠️ This plugin requests access to your cloud-drive login cookies. Only allow it if you completely trust this plugin.',
  'plugins.permission-modify_headers-title': 'Customize request headers',
  'plugins.permission-modify_headers-body':
    'The plugin may change the User-Agent and Referer to bypass anti-hotlink checks.',
  'plugins.permission-read_clipboard-title': 'Read the clipboard',
  'plugins.permission-read_clipboard-body':
    'The plugin may read clipboard text while it is running. Only allow this for tools you trust.',
}

const translations = {
  'zh-CN': {
    'about.disclaimer':
      'Rayload 是一款开源、无追踪的下载工具。任何基于本软件内置插件（如网盘解析）获取的内容，均与开发者无关，请遵守相关网盘的服务条款。\n\nRayload is an open-source, tracking-free download tool. Any content obtained through built-in plugins (such as cloud-drive parsing) is unrelated to the developers; please comply with the relevant cloud-drive terms of service.',
    'preferences.downloader': '下载器设置',
    'preferences.categories': '文件分类',
    'legal.eula-title': 'Rayload 用户使用协议',
    'legal.privacy-title': 'Rayload 隐私政策',
    'legal.close': '关闭',
    'legal.consent-label': '我已阅读并同意《用户使用协议》与《隐私政策》',
    'legal.consent-prefix': '我已阅读并同意',
    'legal.consent-and': '与',
    'legal.eula-link': '《用户使用协议》',
    'legal.privacy-link': '《隐私政策》',
    'legal.technical-title': '纯技术工具声明',
    'legal.technical-body':
      '本软件仅提供 P2P/HTTP 下载技术，不存储、不提供、不分发任何实质性的下载资源内容。所有内容均由用户自行从第三方来源获取。',
    'legal.copyright-title': '版权合规',
    'legal.copyright-body':
      '用户必须保证其下载的内容拥有合法版权或已获得合法授权，严禁使用本软件下载盗版软件、影视、音乐等侵权内容。',
    'legal.lawful-title': '合法使用',
    'legal.lawful-body':
      '严禁将本软件用于传播恶意软件、非法信息，或用于违反当地法律法规的任何用途。',
    'legal.liability-title': '责任免除',
    'legal.liability-body':
      '用户因使用本软件产生的任何法律纠纷与版权问题，均由用户自行承担，开发者不承担任何连带责任。',
    'legal.privacy-local-title': '数据保存在本机',
    'legal.privacy-local-body':
      '本软件没有账号体系，也不收集遥测数据。任务历史、设置与日志仅保存在您的设备上，除非您主动导出或分享。',
    'legal.privacy-tracking-title': '无追踪',
    'legal.privacy-tracking-body':
      '本软件不收集统计分析、广告标识或个人身份信息。检查更新时仅访问发布源以比较版本号。',
    'legal.privacy-lan-title': '局域网互联',
    'legal.privacy-lan-body':
      '局域网配对仅在您明确配对的设备之间交换连接信息。数据通过本地网络直接传输，不会上传到开发者服务器。',
    'whats-new.title': '更新记录',
    'whats-new.close': '我知道了',
    'whats-new.1-0-4-lan':
      '局域网设备：手机与平板可配对、远程发送任务，同一设备的多个地址会聚合显示。',
    'whats-new.1-0-4-history':
      '数据统计：全部设备历史总量、趋势与分类统计，并新增实时折线图。',
    'whats-new.1-0-4-permission':
      'Android 17 局域网权限已接入应用内申请，并加入权限设置。',
    'whats-new.1-0-4-settings':
      '设置重新整理：下载、BT、ED2K 与网络合并为下载器设置，文件分类独立成页。',
    'whats-new.1-0-4-wizard':
      '首次向导新增导航栏自定义、悬浮球开关与用户协议确认。',
    'onboarding.navigation-section': '导航栏',
    'onboarding.navigation-section-hint': '选择侧边栏或底部导航中显示的项目，点按即可显示或隐藏。',
    'onboarding.nav-hint-all': '在一个列表中查看全部任务',
    'onboarding.nav-hint-progress': '正在进行的下载',
    'onboarding.nav-hint-failed': '因错误停止的任务',
    'onboarding.nav-hint-completed': '已完成的下载',
    'onboarding.nav-hint-keep-awake': '保持屏幕与下载引擎唤醒',
    'onboarding.nav-hint-statistics': '速度、历史与分类统计',
    'onboarding.nav-hint-lan-devices': '配对并控制其他设备',
    'onboarding.nav-hint-trash': '恢复或彻底删除文件',
    'onboarding.nav-hint-about': '版本、许可证与项目链接',
    'onboarding.disclaimer-title': '免责声明',
    'onboarding.floating-orb-title': '悬浮球',
    'onboarding.floating-orb-hint': '在其他应用上方显示可拖动的实时网速面板',
    'plugins.warning-title': '第三方插件风险',
    'plugins.warning-body':
      '该插件由第三方提供，可能包含未知代码或违反目标网站服务条款，请确保您信任该插件来源，否则请勿加载。',
    'plugins.warning-confirm': '我已知晓风险',
    'plugins.test-download-started': '已开始下载：{name}',
    'plugins.type-rule': '规则插件',
    'plugins.type-script': '脚本插件',
    'plugins.review-title': '审查插件权限',
    'plugins.review-confirm': '允许并导入',
    'plugins.warning-rule': '该 JSON 规则插件由第三方提供，请确认您信任其来源及下方声明的域名后再继续。',
    'plugins.warning-script':
      '该插件为 Shell 脚本插件，会以本地代码形式运行并可能拥有下方权限，行为无法完全预测。请仅加载您完全信任的脚本。',
    'plugins.allowed-domains': '允许访问的域名',
    'plugins.no-permissions': '该插件未声明任何特权权限。',
    'plugins.permission-network-title': '访问特定网络',
    'plugins.permission-network-body': '该插件仅可向以下域名发送请求：{domains}',
    'plugins.permission-cookies-title': '读取网盘登录凭证（高危）',
    'plugins.permission-cookies-body':
      '⚠️ 该插件请求读取您的网盘登录凭证（Cookie），请确保完全信任该插件！',
    'plugins.permission-modify_headers-title': '自定义请求头',
    'plugins.permission-modify_headers-body': '该插件可伪造 User-Agent 与 Referer，用于绕过防盗链检查。',
    'plugins.permission-read_clipboard-title': '读取剪贴板',
    'plugins.permission-read_clipboard-body': '该插件在运行时可读取剪贴板文本，请仅对信任的工具授权。',
  },
  'zh-TW': {
    'about.disclaimer':
      'Rayload 是一款開源、無追蹤的下載工具。任何基於本軟體內建外掛（如網盤解析）取得的內容，均與開發者無關，請遵守相關網盤的服務條款。\n\nRayload is an open-source, tracking-free download tool. Any content obtained through built-in plugins (such as cloud-drive parsing) is unrelated to the developers; please comply with the relevant cloud-drive terms of service.',
    'preferences.downloader': '下載器設定',
    'preferences.categories': '檔案分類',
    'legal.eula-title': 'Rayload 使用者使用協議',
    'legal.privacy-title': 'Rayload 隱私政策',
    'legal.close': '關閉',
    'legal.consent-label': '我已閱讀並同意《使用者使用協議》與《隱私政策》',
    'legal.consent-prefix': '我已閱讀並同意',
    'legal.consent-and': '與',
    'legal.eula-link': '《使用者使用協議》',
    'legal.privacy-link': '《隱私政策》',
    'legal.technical-title': '純技術工具聲明',
    'legal.technical-body':
      '本軟體僅提供 P2P/HTTP 下載技術，不儲存、不提供、不分發任何實質性的下載資源內容。所有內容均由使用者自行從第三方來源取得。',
    'legal.copyright-title': '版權合規',
    'legal.copyright-body':
      '使用者必須保證其下載的內容擁有合法版權或已獲得合法授權，嚴禁使用本軟體下載盜版軟體、影視、音樂等侵權內容。',
    'legal.lawful-title': '合法使用',
    'legal.lawful-body':
      '嚴禁將本軟體用於傳播惡意軟體、非法資訊，或用於違反當地法律法規的任何用途。',
    'legal.liability-title': '責任免除',
    'legal.liability-body':
      '使用者因使用本軟體產生的任何法律糾紛與版權問題，均由使用者自行承擔，開發者不承擔任何連帶責任。',
    'legal.privacy-local-title': '資料保存在本機',
    'legal.privacy-local-body':
      '本軟體沒有帳號體系，也不收集遙測資料。任務歷史、設定與日誌僅保存在您的裝置上，除非您主動匯出或分享。',
    'legal.privacy-tracking-title': '無追蹤',
    'legal.privacy-tracking-body':
      '本軟體不收集統計分析、廣告識別碼或個人身分資訊。檢查更新時僅存取發佈來源以比較版本號。',
    'legal.privacy-lan-title': '區域網路互聯',
    'legal.privacy-lan-body':
      '區域網路配對僅在您明確配對的裝置之間交換連線資訊。資料透過本機網路直接傳輸，不會上傳到開發者伺服器。',
    'whats-new.title': '更新記錄',
    'whats-new.close': '我知道了',
    'whats-new.1-0-4-lan':
      '區域網路裝置：手機與平板可配對、遠端傳送任務，同一裝置的多個位址會聚合顯示。',
    'whats-new.1-0-4-history':
      '資料統計：全部裝置歷史總量、趨勢與分類統計，並新增即時折線圖。',
    'whats-new.1-0-4-permission':
      'Android 17 區域網路權限已接入應用內申請，並加入權限設定。',
    'whats-new.1-0-4-settings':
      '設定重新整理：下載、BT、ED2K 與網路合併為下載器設定，檔案分類獨立成頁。',
    'whats-new.1-0-4-wizard':
      '首次嚮導新增導覽列自訂、懸浮球開關與使用者協議確認。',
    'onboarding.navigation-section': '導覽列',
    'onboarding.navigation-section-hint': '選擇側邊欄或底部導覽中顯示的項目，點按即可顯示或隱藏。',
    'onboarding.nav-hint-all': '在一個清單中查看全部任務',
    'onboarding.nav-hint-progress': '正在進行的下載',
    'onboarding.nav-hint-failed': '因錯誤停止的任務',
    'onboarding.nav-hint-completed': '已完成的下載',
    'onboarding.nav-hint-keep-awake': '保持螢幕與下載引擎喚醒',
    'onboarding.nav-hint-statistics': '速度、歷史與分類統計',
    'onboarding.nav-hint-lan-devices': '配對並控制其他裝置',
    'onboarding.nav-hint-trash': '還原或徹底刪除檔案',
    'onboarding.nav-hint-about': '版本、授權與專案連結',
    'onboarding.disclaimer-title': '免責聲明',
    'onboarding.floating-orb-title': '懸浮球',
    'onboarding.floating-orb-hint': '在其他應用程式上方顯示可拖曳的即時網速面板',
    'plugins.warning-title': '第三方外掛風險',
    'plugins.warning-body':
      '此外掛由第三方提供，可能包含未知程式碼或違反目標網站服務條款，請確保您信任此外掛來源，否則請勿載入。',
    'plugins.warning-confirm': '我已知曉風險',
    'plugins.test-download-started': '已開始下載：{name}',
    'plugins.type-rule': '規則外掛',
    'plugins.type-script': '腳本外掛',
    'plugins.review-title': '審查外掛權限',
    'plugins.review-confirm': '允許並匯入',
    'plugins.warning-rule': '該 JSON 規則外掛由第三方提供，請確認您信任其來源及下方宣告的網域後再繼續。',
    'plugins.warning-script':
      '該外掛為 Shell 腳本外掛，會以本機程式碼形式執行並可能擁有下方權限，行為無法完全預測。請僅載入您完全信任的腳本。',
    'plugins.allowed-domains': '允許存取的網域',
    'plugins.no-permissions': '該外掛未宣告任何特權權限。',
    'plugins.permission-network-title': '存取特定網路',
    'plugins.permission-network-body': '該外掛僅可向以下網域傳送請求：{domains}',
    'plugins.permission-cookies-title': '讀取網盤登入憑證（高風險）',
    'plugins.permission-cookies-body':
      '⚠️ 該外掛請求讀取您的網盤登入憑證（Cookie），請確保完全信任該外掛！',
    'plugins.permission-modify_headers-title': '自訂請求標頭',
    'plugins.permission-modify_headers-body': '該外掛可偽造 User-Agent 與 Referer，用於繞過防盜鏈檢查。',
    'plugins.permission-read_clipboard-title': '讀取剪貼簿',
    'plugins.permission-read_clipboard-body': '該外掛執行時可讀取剪貼簿文字，請僅對信任的工具授權。',
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
  delete messages.onboarding?.['later']
  delete messages.onboarding?.['skip']
  delete messages.plugins?.['warning-title']
  delete messages.plugins?.['warning-body']
  writeFileSync(path, `${JSON.stringify(messages, null, 2)}\n`)
}
console.log('Updated legal, onboarding, settings and release-note copy in every locale.')
