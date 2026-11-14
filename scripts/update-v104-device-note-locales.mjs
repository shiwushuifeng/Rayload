#!/usr/bin/env node
/** Adds the LAN device note and edit-dialog copy to every locale. */
import { readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = join(dirname(fileURLToPath(import.meta.url)), '..')
const localesRoot = join(root, 'src', 'shared', 'locales')

const english = {
  'lan-devices.edit-title': 'Edit device',
  'lan-devices.note-placeholder': 'Note (optional)',
}

const translations = {
  'zh-CN': { 'lan-devices.edit-title': '编辑设备', 'lan-devices.note-placeholder': '备注（可选）' },
  'zh-TW': { 'lan-devices.edit-title': '編輯裝置', 'lan-devices.note-placeholder': '備註（選填）' },
  ja: { 'lan-devices.edit-title': 'デバイスを編集', 'lan-devices.note-placeholder': 'メモ（任意）' },
  ko: { 'lan-devices.edit-title': '기기 편집', 'lan-devices.note-placeholder': '메모(선택)' },
  de: { 'lan-devices.edit-title': 'Gerät bearbeiten', 'lan-devices.note-placeholder': 'Notiz (optional)' },
  fr: { 'lan-devices.edit-title': "Modifier l'appareil", 'lan-devices.note-placeholder': 'Remarque (facultatif)' },
  es: { 'lan-devices.edit-title': 'Editar dispositivo', 'lan-devices.note-placeholder': 'Nota (opcional)' },
  it: { 'lan-devices.edit-title': 'Modifica dispositivo', 'lan-devices.note-placeholder': 'Nota (facoltativa)' },
  'pt-BR': { 'lan-devices.edit-title': 'Editar dispositivo', 'lan-devices.note-placeholder': 'Observação (opcional)' },
  ru: { 'lan-devices.edit-title': 'Изменить устройство', 'lan-devices.note-placeholder': 'Заметка (необязательно)' },
  uk: { 'lan-devices.edit-title': 'Змінити пристрій', 'lan-devices.note-placeholder': 'Нотатка (необовʼязково)' },
  pl: { 'lan-devices.edit-title': 'Edytuj urządzenie', 'lan-devices.note-placeholder': 'Notatka (opcjonalnie)' },
  nl: { 'lan-devices.edit-title': 'Apparaat bewerken', 'lan-devices.note-placeholder': 'Notitie (optioneel)' },
  ro: { 'lan-devices.edit-title': 'Editează dispozitivul', 'lan-devices.note-placeholder': 'Notă (opțional)' },
  hu: { 'lan-devices.edit-title': 'Eszköz szerkesztése', 'lan-devices.note-placeholder': 'Megjegyzés (nem kötelező)' },
  tr: { 'lan-devices.edit-title': 'Cihazı düzenle', 'lan-devices.note-placeholder': 'Not (isteğe bağlı)' },
  ar: { 'lan-devices.edit-title': 'تعديل الجهاز', 'lan-devices.note-placeholder': 'ملاحظة (اختياري)' },
  fa: { 'lan-devices.edit-title': 'ویرایش دستگاه', 'lan-devices.note-placeholder': 'یادداشت (اختیاری)' },
  hi: { 'lan-devices.edit-title': 'डिवाइस संपादित करें', 'lan-devices.note-placeholder': 'नोट (वैकल्पिक)' },
  th: { 'lan-devices.edit-title': 'แก้ไขอุปกรณ์', 'lan-devices.note-placeholder': 'หมายเหตุ (ไม่บังคับ)' },
  vi: { 'lan-devices.edit-title': 'Chỉnh sửa thiết bị', 'lan-devices.note-placeholder': 'Ghi chú (không bắt buộc)' },
  id: { 'lan-devices.edit-title': 'Edit perangkat', 'lan-devices.note-placeholder': 'Catatan (opsional)' },
  bg: { 'lan-devices.edit-title': 'Редактиране на устройство', 'lan-devices.note-placeholder': 'Бележка (по избор)' },
  el: { 'lan-devices.edit-title': 'Επεξεργασία συσκευής', 'lan-devices.note-placeholder': 'Σημείωση (προαιρετικό)' },
  nb: { 'lan-devices.edit-title': 'Rediger enhet', 'lan-devices.note-placeholder': 'Notat (valgfritt)' },
  ca: { 'lan-devices.edit-title': 'Edita el dispositiu', 'lan-devices.note-placeholder': 'Nota (opcional)' },
}

for (const locale of readdirSync(localesRoot)) {
  const directory = join(localesRoot, locale)
  if (!statSync(directory).isDirectory()) continue
  const path = join(directory, 'messages.json')
  const messages = JSON.parse(readFileSync(path, 'utf8'))
  const copy = { ...english, ...(translations[locale] ?? {}) }
  messages['lan-devices'] ??= {}
  for (const [flatKey, value] of Object.entries(copy)) {
    messages['lan-devices'][flatKey.split('.')[1]] = value
  }
  delete messages['lan-devices'].rename
  delete messages['lan-devices']['rename-save']
  writeFileSync(path, `${JSON.stringify(messages, null, 2)}\n`)
}
console.log('Updated LAN device note copy in every locale.')
