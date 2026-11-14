#!/usr/bin/env node
/** Adds the local LAN identity fingerprint label to every locale. */
import { readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = join(dirname(fileURLToPath(import.meta.url)), '..')
const localesRoot = join(root, 'src', 'shared', 'locales')

const labels = {
  'zh-CN': '本机指纹',
  'zh-TW': '本機指紋',
  ja: 'このデバイスの指紋',
  ko: '이 기기의 지문',
  de: 'Fingerabdruck dieses Geräts',
  fr: 'Empreinte de cet appareil',
  es: 'Huella de este dispositivo',
  it: 'Impronta di questo dispositivo',
  'pt-BR': 'Impressão digital deste dispositivo',
  ru: 'Отпечаток этого устройства',
  uk: 'Відбиток цього пристрою',
  pl: 'Odcisk tego urządzenia',
  nl: 'Vingerafdruk van dit apparaat',
  ro: 'Amprenta acestui dispozitiv',
  hu: 'Ennek az eszköznek az ujjlenyomata',
  tr: 'Bu cihazın parmak izi',
  ar: 'بصمة هذا الجهاز',
  fa: 'اثر انگشت این دستگاه',
  hi: 'इस डिवाइस की फ़िंगरप्रिंट',
  th: 'ลายนิ้วมือของอุปกรณ์นี้',
  vi: 'Dấu vân tay của thiết bị này',
  id: 'Sidik jari perangkat ini',
  bg: 'Отпечатък на това устройство',
  el: 'Αποτύπωμα αυτής της συσκευής',
  nb: 'Fingeravtrykk for denne enheten',
  ca: "Empremta d'aquest dispositiu",
}

for (const locale of readdirSync(localesRoot)) {
  const directory = join(localesRoot, locale)
  if (!statSync(directory).isDirectory()) continue
  const path = join(directory, 'messages.json')
  const messages = JSON.parse(readFileSync(path, 'utf8'))
  messages['lan-devices'] ??= {}
  messages['lan-devices']['local-fingerprint'] =
    labels[locale] ?? "This device's fingerprint"
  writeFileSync(path, `${JSON.stringify(messages, null, 2)}\n`)
}
console.log('Added the local fingerprint label to every locale.')
