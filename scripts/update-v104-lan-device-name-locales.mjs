#!/usr/bin/env node
/** Adds the editable local device name to every locale. */
import { readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = join(dirname(fileURLToPath(import.meta.url)), '..')
const localesRoot = join(root, 'src', 'shared', 'locales')

const english = {
  'device-name-label': 'This device name',
  'device-name-save': 'Save name',
  'device-name-saved': 'Device name saved',
}

const translations = {
  'zh-CN': { 'device-name-label': '本机名称', 'device-name-save': '保存名称', 'device-name-saved': '本机名称已保存' },
  'zh-TW': { 'device-name-label': '本機名稱', 'device-name-save': '儲存名稱', 'device-name-saved': '本機名稱已儲存' },
  ja: { 'device-name-label': 'デバイス名', 'device-name-save': '名前を保存', 'device-name-saved': 'デバイス名を保存しました' },
  ko: { 'device-name-label': '기기 이름', 'device-name-save': '이름 저장', 'device-name-saved': '기기 이름이 저장되었습니다' },
  de: { 'device-name-label': 'Gerätename', 'device-name-save': 'Namen speichern', 'device-name-saved': 'Gerätename gespeichert' },
  fr: { 'device-name-label': "Nom de l'appareil", 'device-name-save': 'Enregistrer le nom', 'device-name-saved': "Nom de l'appareil enregistré" },
  es: { 'device-name-label': 'Nombre del dispositivo', 'device-name-save': 'Guardar nombre', 'device-name-saved': 'Nombre del dispositivo guardado' },
  it: { 'device-name-label': 'Nome dispositivo', 'device-name-save': 'Salva nome', 'device-name-saved': 'Nome dispositivo salvato' },
  'pt-BR': { 'device-name-label': 'Nome do dispositivo', 'device-name-save': 'Salvar nome', 'device-name-saved': 'Nome do dispositivo salvo' },
  ru: { 'device-name-label': 'Имя устройства', 'device-name-save': 'Сохранить имя', 'device-name-saved': 'Имя устройства сохранено' },
  uk: { 'device-name-label': "Ім'я пристрою", 'device-name-save': 'Зберегти ім’я', 'device-name-saved': 'Ім’я пристрою збережено' },
  pl: { 'device-name-label': 'Nazwa urządzenia', 'device-name-save': 'Zapisz nazwę', 'device-name-saved': 'Nazwa urządzenia zapisana' },
  nl: { 'device-name-label': 'Apparaatnaam', 'device-name-save': 'Naam opslaan', 'device-name-saved': 'Apparaatnaam opgeslagen' },
  ro: { 'device-name-label': 'Numele dispozitivului', 'device-name-save': 'Salvează numele', 'device-name-saved': 'Numele dispozitivului a fost salvat' },
  hu: { 'device-name-label': 'Eszköznév', 'device-name-save': 'Név mentése', 'device-name-saved': 'Eszköznév mentve' },
  tr: { 'device-name-label': 'Cihaz adı', 'device-name-save': 'Adı kaydet', 'device-name-saved': 'Cihaz adı kaydedildi' },
  ar: { 'device-name-label': 'اسم الجهاز', 'device-name-save': 'حفظ الاسم', 'device-name-saved': 'تم حفظ اسم الجهاز' },
  fa: { 'device-name-label': 'نام دستگاه', 'device-name-save': 'ذخیره نام', 'device-name-saved': 'نام دستگاه ذخیره شد' },
  hi: { 'device-name-label': 'डिवाइस नाम', 'device-name-save': 'नाम सहेजें', 'device-name-saved': 'डिवाइस नाम सहेजा गया' },
  th: { 'device-name-label': 'ชื่ออุปกรณ์', 'device-name-save': 'บันทึกชื่อ', 'device-name-saved': 'บันทึกชื่ออุปกรณ์แล้ว' },
  vi: { 'device-name-label': 'Tên thiết bị', 'device-name-save': 'Lưu tên', 'device-name-saved': 'Đã lưu tên thiết bị' },
  id: { 'device-name-label': 'Nama perangkat', 'device-name-save': 'Simpan nama', 'device-name-saved': 'Nama perangkat disimpan' },
  bg: { 'device-name-label': 'Име на устройството', 'device-name-save': 'Запази името', 'device-name-saved': 'Името на устройството е запазено' },
  el: { 'device-name-label': 'Όνομα συσκευής', 'device-name-save': 'Αποθήκευση ονόματος', 'device-name-saved': 'Το όνομα συσκευής αποθηκεύτηκε' },
  nb: { 'device-name-label': 'Enhetsnavn', 'device-name-save': 'Lagre navn', 'device-name-saved': 'Enhetsnavn lagret' },
  ca: { 'device-name-label': 'Nom del dispositiu', 'device-name-save': 'Desa el nom', 'device-name-saved': 'Nom del dispositiu desat' },
}

for (const locale of readdirSync(localesRoot)) {
  const directory = join(localesRoot, locale)
  if (!statSync(directory).isDirectory()) continue
  const path = join(directory, 'messages.json')
  const messages = JSON.parse(readFileSync(path, 'utf8'))
  messages['lan-devices'] ??= {}
  Object.assign(messages['lan-devices'], english, translations[locale] ?? {})
  writeFileSync(path, `${JSON.stringify(messages, null, 2)}\n`)
}
console.log('Added the local device name copy to every locale.')
