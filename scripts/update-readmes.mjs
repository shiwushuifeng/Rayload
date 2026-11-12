#!/usr/bin/env node
/** Maintains the GitHub README language picker and translated quick guides. */

import { existsSync, readFileSync, writeFileSync } from 'node:fs'
import { join, resolve } from 'node:path'

const root = resolve(import.meta.dirname, '..')
const catalog = JSON.parse(readFileSync(join(root, 'src/shared/locales/catalog.json'), 'utf8'))
const readmeCopy = JSON.parse(
  readFileSync(join(root, 'scripts/readme-translations.json'), 'utf8'),
)
const forceQuickGuides = process.argv.includes('--force')
const startMarker = '<!-- language-selector:start -->'
const endMarker = '<!-- language-selector:end -->'

const translations = {
  'zh-TW': {
    title: 'Rayload — Android 下載器',
    summary:
      'Rayload 是 Rayburst 的獨立非官方 Android 移植版，專注於 Android 平台，提供行動介面、系統分享、背景下載與本機 Aria2 JSON-RPC。',
    install: '安裝最新的 ARM64 APK',
  },
  ja: {
    title: 'Rayload — Android ダウンローダー',
    summary:
      'Rayload は Rayburst を基にした独立・非公式の Android 版です。モバイル向け UI、共有、バックグラウンドダウンロード、ローカル Aria2 JSON-RPC を提供します。',
    install: '最新の ARM64 APK をインストール',
  },
  ko: {
    title: 'Rayload — Android 다운로더',
    summary:
      'Rayload는 Rayburst 기반의 독립적인 비공식 Android 포트입니다. 모바일 UI, 시스템 공유, 백그라운드 다운로드 및 로컬 Aria2 JSON-RPC를 제공합니다.',
    install: '최신 ARM64 APK 설치',
  },
  fr: {
    title: 'Rayload — téléchargeur Android',
    summary:
      'Rayload est un port Android indépendant et non officiel de Rayburst. Il propose une interface mobile, le partage système, les téléchargements en arrière-plan et Aria2 JSON-RPC en local.',
    install: 'Installer le dernier APK ARM64',
  },
  de: {
    title: 'Rayload — Android-Downloader',
    summary:
      'Rayload ist ein unabhängiger, inoffizieller Android-Port von Rayburst mit mobiler Oberfläche, Systemfreigabe, Hintergrunddownloads und lokalem Aria2 JSON-RPC.',
    install: 'Aktuelle ARM64-APK installieren',
  },
  es: {
    title: 'Rayload — descargador para Android',
    summary:
      'Rayload es un port independiente y no oficial de Rayburst para Android, con interfaz móvil, uso compartido del sistema, descargas en segundo plano y Aria2 JSON-RPC local.',
    install: 'Instalar el APK ARM64 más reciente',
  },
  'pt-BR': {
    title: 'Rayload — gerenciador de downloads para Android',
    summary:
      'O Rayload é um porte independente e não oficial do Rayburst para Android, com interface móvel, compartilhamento do sistema, downloads em segundo plano e Aria2 JSON-RPC local.',
    install: 'Instalar o APK ARM64 mais recente',
  },
  tr: {
    title: 'Rayload — Android indirme yöneticisi',
    summary:
      'Rayload, Rayburst tabanlı bağımsız ve resmî olmayan bir Android portudur. Mobil arayüz, sistem paylaşımı, arka plan indirmeleri ve yerel Aria2 JSON-RPC sunar.',
    install: 'En son ARM64 APK kurulumu',
  },
  ar: {
    title: 'Rayload — مدير تنزيلات Android',
    summary:
      'Rayload نسخة Android مستقلة وغير رسمية مبنية على Rayburst، وتوفر واجهة للهاتف والمشاركة من النظام والتنزيل في الخلفية وخدمة Aria2 JSON-RPC محلية.',
    install: 'تثبيت أحدث ملف APK لمعمارية ARM64',
  },
  bg: {
    title: 'Rayload — Android мениджър за изтегляния',
    summary:
      'Rayload е независим неофициален Android порт на Rayburst с мобилен интерфейс, системно споделяне, фонови изтегляния и локален Aria2 JSON-RPC.',
    install: 'Инсталиране на най-новия ARM64 APK',
  },
  ca: {
    title: 'Rayload — gestor de baixades per a Android',
    summary:
      'Rayload és un port independent i no oficial de Rayburst per a Android, amb interfície mòbil, compartició del sistema, baixades en segon pla i Aria2 JSON-RPC local.',
    install: "Instal·lar l'APK ARM64 més recent",
  },
  el: {
    title: 'Rayload — λήψεις για Android',
    summary:
      'Το Rayload είναι μια ανεξάρτητη ανεπίσημη μεταφορά του Rayburst για Android με κινητή διεπαφή, κοινή χρήση συστήματος, λήψεις στο παρασκήνιο και τοπικό Aria2 JSON-RPC.',
    install: 'Εγκατάσταση του πιο πρόσφατου APK ARM64',
  },
  fa: {
    title: 'Rayload — دانلودر Android',
    summary:
      'Rayload یک نسخه مستقل و غیررسمی از Rayburst برای Android است و رابط موبایل، اشتراک‌گذاری سیستمی، دانلود پس‌زمینه و Aria2 JSON-RPC محلی را ارائه می‌دهد.',
    install: 'نصب جدیدترین APK ARM64',
  },
  hu: {
    title: 'Rayload — Android letöltőkezelő',
    summary:
      'A Rayload a Rayburst független, nem hivatalos Android-portja mobil felülettel, rendszerszintű megosztással, háttérletöltésekkel és helyi Aria2 JSON-RPC-vel.',
    install: 'A legújabb ARM64 APK telepítése',
  },
  hi: {
    title: 'Rayload — Android डाउनलोडर',
    summary:
      'Rayload, Rayburst का एक स्वतंत्र और अनौपचारिक Android पोर्ट है जिसमें मोबाइल इंटरफ़ेस, सिस्टम शेयर, बैकग्राउंड डाउनलोड और स्थानीय Aria2 JSON-RPC शामिल हैं।',
    install: 'नवीनतम ARM64 APK इंस्टॉल करें',
  },
  id: {
    title: 'Rayload — pengunduh Android',
    summary:
      'Rayload adalah port Android independen dan tidak resmi dari Rayburst dengan antarmuka seluler, berbagi sistem, unduhan latar belakang, dan Aria2 JSON-RPC lokal.',
    install: 'Instal APK ARM64 terbaru',
  },
  it: {
    title: 'Rayload — download manager per Android',
    summary:
      'Rayload è un port Android indipendente e non ufficiale di Rayburst con interfaccia mobile, condivisione di sistema, download in background e Aria2 JSON-RPC locale.',
    install: "Installa l'APK ARM64 più recente",
  },
  nb: {
    title: 'Rayload — Android-nedlaster',
    summary:
      'Rayload er en uavhengig, uoffisiell Android-port av Rayburst med mobilgrensesnitt, systemdeling, bakgrunnsnedlastinger og lokal Aria2 JSON-RPC.',
    install: 'Installer den nyeste ARM64-APK-en',
  },
  nl: {
    title: 'Rayload — Android-downloader',
    summary:
      'Rayload is een onafhankelijke, onofficiële Android-port van Rayburst met een mobiele interface, systeem delen, achtergronddownloads en lokale Aria2 JSON-RPC.',
    install: 'Nieuwste ARM64-APK installeren',
  },
  pl: {
    title: 'Rayload — menedżer pobierania dla Androida',
    summary:
      'Rayload to niezależny, nieoficjalny port Rayburst na Androida z mobilnym interfejsem, udostępnianiem systemowym, pobieraniem w tle i lokalnym Aria2 JSON-RPC.',
    install: 'Zainstaluj najnowszy plik APK ARM64',
  },
  ro: {
    title: 'Rayload — manager de descărcări pentru Android',
    summary:
      'Rayload este un port Android independent și neoficial al Rayburst, cu interfață mobilă, partajare prin sistem, descărcări în fundal și Aria2 JSON-RPC local.',
    install: 'Instalează cel mai recent APK ARM64',
  },
  th: {
    title: 'Rayload — โปรแกรมดาวน์โหลดสำหรับ Android',
    summary:
      'Rayload เป็นพอร์ต Android แบบอิสระและไม่เป็นทางการของ Rayburst พร้อมอินเทอร์เฟซสำหรับมือถือ การแชร์ผ่านระบบ ดาวน์โหลดเบื้องหลัง และ Aria2 JSON-RPC ภายในเครื่อง',
    install: 'ติดตั้ง APK ARM64 ล่าสุด',
  },
  uk: {
    title: 'Rayload — завантажувач для Android',
    summary:
      'Rayload — незалежний неофіційний Android-порт Rayburst із мобільним інтерфейсом, системним поширенням, фоновими завантаженнями та локальним Aria2 JSON-RPC.',
    install: 'Встановити найновіший ARM64 APK',
  },
  vi: {
    title: 'Rayload — trình tải xuống Android',
    summary:
      'Rayload là bản chuyển độc lập, không chính thức của Rayburst cho Android với giao diện di động, chia sẻ hệ thống, tải xuống nền và Aria2 JSON-RPC cục bộ.',
    install: 'Cài đặt APK ARM64 mới nhất',
  },
}

function readmePath(code) {
  return code === 'en-US' ? 'README.md' : `README.${code}.md`
}

function linkFor(locale, currentCode) {
  if (currentCode === 'en-US') {
    return locale === 'en-US' ? 'README.md' : `docs/README.${locale}.md`
  }
  return locale === 'en-US' ? '../README.md' : `README.${locale}.md`
}

function selectorFor(code) {
  const links = Object.entries(catalog).map(([locale, metadata]) => {
    const labelText = locale === 'en-US' ? 'English + 简体中文' : metadata.label
    const label = locale === code ? `**${labelText}**` : labelText
    return `[${label}](${linkFor(locale, code)})`
  })
  return `${startMarker}
<details>
<summary>🌐 Language / 语言 / 語言 / Язык</summary>

${links.join(' · ')}

</details>
${endMarker}`
}

function addSelector(content, code) {
  const selector = selectorFor(code)
  const blockPattern = new RegExp(`${startMarker}[\\s\\S]*?${endMarker}`, 'u')
  const withSelector = blockPattern.test(content)
    ? content.replace(blockPattern, selector)
    : content.replace(/^(# .+\r?\n)/u, `$1\n${selector}\n`)
  const localizedLinks = withSelector.replace(
    /\[English README\]\(README\.md\)/gu,
    '[English README](../README.md)',
  )
  return localizedLinks.replace(
    /^\[English\]\(README\.md\) · \[简体中文\]\(README\.zh-CN\.md\) · \[Русский\]\(README\.ru\.md\)\n\n/mu,
    '',
  )
}

function quickGuide(code) {
  const text = translations[code]
  const copy = readmeCopy[code]
  if (!text) throw new Error(`Missing README translation metadata for ${code}`)
  if (!copy) throw new Error(`Missing README copy for ${code}`)
  return `# ${text.title}

${selectorFor(code)}

> ${copy.vision}

${text.summary}

## v1.0.2 Highlights

${copy.highlights.map((item) => `- ${item}`).join('\n')}

## ${text.install}

${copy.download} [GitHub Releases](https://github.com/shiwushuifeng/Rayload/releases/latest)

### Quick start

${copy.quickStart.map((item, index) => `${index + 1}. ${item}`).join('\n')}

- Minimum: Android 7.0 / API 24
- Target: Android 17 / API 37
- ABI: \`arm64-v8a\`

## Software requirements

| Software | Required version |
| -------- | ---------------- |
| Node.js | 22 or newer |
| pnpm | \`11.5.2\` |
| Rust | stable with \`aarch64-linux-android\` |
| Java | JDK 21 |
| Android SDK | Platform 37 |
| Android Build Tools | 37.0.0 |
| Android NDK | \`29.0.14206865\` |
| Git | current stable |
| GitHub CLI | optional, only for releases |

Full build and browser integration instructions are available in the
[English README](../README.md).

## Troubleshooting and FAQ

${copy.faq.map((item) => `### ${item.q}\n\n${item.a}`).join('\n\n')}

## Roadmap

[View the full roadmap](../ROADMAP.md) — ${copy.roadmap}.

## License

Rayload is licensed under the [GPL-3.0 License](../LICENSE). It is based on
[Rayburst](https://github.com/AnInsomniacy/rayburst) by **AnInsomniacy**, whose
original work is licensed under the MIT License. The bundled Aria2 Next
(\`aria2-next\`) engine is licensed under **GPL-2.0-or-later**. See
[NOTICE](../NOTICE) for all third-party attributions and license notices.
Rayload is an independent, unofficial port.

> ⭐ ${copy.star}

> ❤️ Like this project? [Sponsor Rayload](../SPONSOR.md)
`
}

function normalizeContent(content, code) {
  let result = content
    .replace(
      /Rayload is based on \[Rayburst\]\(https:\/\/github\.com\/AnInsomniacy\/rayburst\) by\n\*\*AnInsomniacy\*\* and follows the \[MIT License\]\(LICENSE\)\. It is an independent,\nunofficial port\./gu,
      'Rayload is distributed under the [GPL-3.0 License](../LICENSE). It is based on\n[Rayburst](https://github.com/AnInsomniacy/rayburst) by **AnInsomniacy**, whose\noriginal work is licensed under the MIT License; see [NOTICE](../NOTICE) for\nthird-party attributions. Rayload is an independent, unofficial port.',
    )
    .replace(/\[MIT License\]\(LICENSE\)/gu, '[GPL-3.0 License](../LICENSE)')
    .replace(/\[лицензии MIT\]\(LICENSE\)/gu, '[лицензии GPL-3.0](../LICENSE)')
    .replace(
      /Rayload is distributed under the \[GPL-3\.0 License\]\(\.\.\/LICENSE\)\. It is based on\n\[Rayburst\]\(https:\/\/github\.com\/AnInsomniacy\/rayburst\) by \*\*AnInsomniacy\*\*, whose\noriginal work is licensed under the MIT License; see \[NOTICE\]\(\.\.\/NOTICE\) for\nthird-party attributions\. Rayload is an independent, unofficial port\./gu,
      'Rayload is licensed under the [GPL-3.0 License](../LICENSE). It is based on\n[Rayburst](https://github.com/AnInsomniacy/rayburst) by **AnInsomniacy**, whose\noriginal work is licensed under the MIT License. The bundled Aria2 Next\n(`aria2-next`) engine is licensed under **GPL-2.0-or-later**. See\n[NOTICE](../NOTICE) for all third-party attributions and license notices.\nRayload is an independent, unofficial port.',
    )
  if (!result.includes('SPONSOR.md')) {
    const sponsor =
      code === 'zh-CN'
        ? '> ❤️ 喜欢这个项目？欢迎 [赞助与支持 (Sponsor)](../SPONSOR.md)'
        : code === 'zh-TW'
          ? '> ❤️ 喜歡這個項目？歡迎 [贊助與支持 (Sponsor)](../SPONSOR.md)'
          : '> ❤️ Like this project? [Sponsor Rayload](../SPONSOR.md)'
    result = `${result.trimEnd()}\n\n${sponsor}\n`
  }
  return result
}

for (const code of Object.keys(catalog)) {
  const path = join(root, code === 'en-US' ? readmePath(code) : join('docs', readmePath(code)))
  if (!existsSync(path) || (forceQuickGuides && translations[code])) {
    writeFileSync(path, normalizeContent(quickGuide(code), code), 'utf8')
    continue
  }
  writeFileSync(path, normalizeContent(addSelector(readFileSync(path, 'utf8'), code), code), 'utf8')
}

console.log(`Updated ${Object.keys(catalog).length} README language editions`)
