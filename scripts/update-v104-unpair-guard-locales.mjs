#!/usr/bin/env node
/** Pairing request warning and remote advanced-settings hint for every locale. */
import { readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = join(dirname(fileURLToPath(import.meta.url)), '..')
const localesRoot = join(root, 'src', 'shared', 'locales')

const english = {
  'lan-devices': {
    'pair-request-body':
      '{name} wants to pair with this device. Only allow it if you just started pairing on {name}; otherwise reject.',
  },
  task: {
    'handoff-advanced-unavailable': 'Advanced settings are not sent to another device.',
  },
}

const translations = {
  'zh-CN': {
    'lan-devices': {
      'pair-request-body': '{name} 请求与这台设备配对。如果不是你刚刚在 {name} 上发起配对，请拒绝。',
    },
    task: { 'handoff-advanced-unavailable': '发送到其他设备时不支持高级设置。' },
  },
  'zh-TW': {
    'lan-devices': {
      'pair-request-body': '{name} 要求與這台裝置配對。如果不是你剛剛在 {name} 上發起配對，請拒絕。',
    },
    task: { 'handoff-advanced-unavailable': '傳送到其他裝置時不支援進階設定。' },
  },
  ja: {
    'lan-devices': {
      'pair-request-body':
        '{name} がこのデバイスとのペアリングを求めています。{name} で今しがたペアリングを開始していない場合は拒否してください。',
    },
    task: { 'handoff-advanced-unavailable': '他のデバイスへの送信では詳細設定は使えません。' },
  },
  ko: {
    'lan-devices': {
      'pair-request-body': '{name}에서 이 기기와 페어링을 요청합니다. 방금 {name}에서 페어링을 시작하지 않았다면 거부하세요.',
    },
    task: { 'handoff-advanced-unavailable': '다른 기기로 보낼 때는 고급 설정을 사용할 수 없습니다.' },
  },
  de: {
    'lan-devices': {
      'pair-request-body':
        '{name} möchte dieses Gerät koppeln. Nur zulassen, wenn du die Kopplung gerade auf {name} gestartet hast; sonst ablehnen.',
    },
    task: { 'handoff-advanced-unavailable': 'Erweiterte Einstellungen werden nicht an ein anderes Gerät gesendet.' },
  },
  fr: {
    'lan-devices': {
      'pair-request-body':
        "{name} souhaite associer cet appareil. N'autorisez que si vous venez de lancer l'association sur {name} ; sinon refusez.",
    },
    task: { 'handoff-advanced-unavailable': "Les réglages avancés ne sont pas envoyés à un autre appareil." },
  },
  es: {
    'lan-devices': {
      'pair-request-body':
        '{name} quiere vincularse con este dispositivo. Permítelo solo si acabas de iniciar la vinculación en {name}; si no, recházalo.',
    },
    task: { 'handoff-advanced-unavailable': 'Los ajustes avanzados no se envían a otro dispositivo.' },
  },
  it: {
    'lan-devices': {
      'pair-request-body':
        "{name} vuole associarsi a questo dispositivo. Consenti solo se hai appena avviato l'associazione su {name}; altrimenti rifiuta.",
    },
    task: { 'handoff-advanced-unavailable': 'Le impostazioni avanzate non vengono inviate a un altro dispositivo.' },
  },
  'pt-BR': {
    'lan-devices': {
      'pair-request-body':
        '{name} quer parear com este dispositivo. Permita apenas se você acabou de iniciar o pareamento em {name}; caso contrário, recuse.',
    },
    task: { 'handoff-advanced-unavailable': 'As configurações avançadas não são enviadas para outro dispositivo.' },
  },
  ru: {
    'lan-devices': {
      'pair-request-body':
        '{name} запрашивает сопряжение с этим устройством. Разрешайте, только если вы только что начали сопряжение на {name}; иначе отклоните.',
    },
    task: { 'handoff-advanced-unavailable': 'Дополнительные параметры не передаются на другое устройство.' },
  },
  uk: {
    'lan-devices': {
      'pair-request-body':
        '{name} запитує сполучення з цим пристроєм. Дозволяйте, лише якщо ви щойно почали сполучення на {name}; інакше відхиліть.',
    },
    task: { 'handoff-advanced-unavailable': 'Додаткові параметри не надсилаються на інший пристрій.' },
  },
  pl: {
    'lan-devices': {
      'pair-request-body':
        '{name} chce sparować się z tym urządzeniem. Zezwól tylko, jeśli właśnie rozpoczęto parowanie na {name}; w przeciwnym razie odrzuć.',
    },
    task: { 'handoff-advanced-unavailable': 'Ustawienia zaawansowane nie są wysyłane do innego urządzenia.' },
  },
  nl: {
    'lan-devices': {
      'pair-request-body':
        '{name} wil dit apparaat koppelen. Sta het alleen toe als je net op {name} de koppeling hebt gestart; anders weigeren.',
    },
    task: { 'handoff-advanced-unavailable': 'Geavanceerde instellingen worden niet naar een ander apparaat verzonden.' },
  },
  ro: {
    'lan-devices': {
      'pair-request-body':
        '{name} dorește să se asocieze cu acest dispozitiv. Permite doar dacă ai inițiat chiar acum asocierea pe {name}; altfel refuză.',
    },
    task: { 'handoff-advanced-unavailable': 'Setările avansate nu sunt trimise către un alt dispozitiv.' },
  },
  hu: {
    'lan-devices': {
      'pair-request-body':
        '{name} párosítani szeretne ezzel az eszközzel. Csak akkor engedélyezd, ha épp most indítottál párosítást a(z) {name} eszközön; különben utasítsd el.',
    },
    task: { 'handoff-advanced-unavailable': 'A speciális beállítások nem kerülnek át a másik eszközre.' },
  },
  tr: {
    'lan-devices': {
      'pair-request-body':
        '{name} bu cihazla eşleşmek istiyor. Yalnızca eşleşmeyi az önce {name} üzerinde başlattıysanız izin verin; aksi halde reddedin.',
    },
    task: { 'handoff-advanced-unavailable': 'Gelişmiş ayarlar başka bir cihaza gönderilmez.' },
  },
  ar: {
    'lan-devices': {
      'pair-request-body': 'يطلب {name} الاقتران بهذا الجهاز. اسمح فقط إذا بدأت الاقتران للتو على {name}؛ وإلا فارفض.',
    },
    task: { 'handoff-advanced-unavailable': 'لا يتم إرسال الإعدادات المتقدمة إلى جهاز آخر.' },
  },
  fa: {
    'lan-devices': {
      'pair-request-body':
        '{name} درخواست جفت‌سازی با این دستگاه را دارد. فقط اگر همین حالا جفت‌سازی را روی {name} آغاز کرده‌اید اجازه دهید؛ در غیر این صورت رد کنید.',
    },
    task: { 'handoff-advanced-unavailable': 'تنظیمات پیشرفته به دستگاه دیگر ارسال نمی‌شود.' },
  },
  hi: {
    'lan-devices': {
      'pair-request-body':
        '{name} इस डिवाइस के साथ पेयर करना चाहता है. केवल तभी अनुमति दें जब आपने अभी-अभी {name} पर पेयरिंग शुरू की हो; अन्यथा अस्वीकार करें.',
    },
    task: { 'handoff-advanced-unavailable': 'उन्नत सेटिंग्स किसी अन्य डिवाइस पर नहीं भेजी जाती हैं.' },
  },
  th: {
    'lan-devices': {
      'pair-request-body':
        '{name} ขอจับคู่กับอุปกรณ์นี้ อนุญาตเฉพาะเมื่อคุณเพิ่งเริ่มจับคู่บน {name} เท่านั้น มิฉะนั้นให้ปฏิเสธ',
    },
    task: { 'handoff-advanced-unavailable': 'การตั้งค่าขั้นสูงจะไม่ถูกส่งไปยังอุปกรณ์อื่น' },
  },
  vi: {
    'lan-devices': {
      'pair-request-body':
        '{name} muốn ghép nối với thiết bị này. Chỉ cho phép nếu bạn vừa bắt đầu ghép nối trên {name}; nếu không, hãy từ chối.',
    },
    task: { 'handoff-advanced-unavailable': 'Cài đặt nâng cao không được gửi đến thiết bị khác.' },
  },
  id: {
    'lan-devices': {
      'pair-request-body':
        '{name} ingin memasangkan dengan perangkat ini. Izinkan hanya jika Anda baru saja memulai pemasangan di {name}; jika tidak, tolak.',
    },
    task: { 'handoff-advanced-unavailable': 'Pengaturan lanjutan tidak dikirim ke perangkat lain.' },
  },
  bg: {
    'lan-devices': {
      'pair-request-body':
        '{name} иска сдвояване с това устройство. Разрешавайте само ако току-що сте започнали сдвояване на {name}; в противен случай откажете.',
    },
    task: { 'handoff-advanced-unavailable': 'Разширените настройки не се изпращат към друго устройство.' },
  },
  el: {
    'lan-devices': {
      'pair-request-body':
        'Το {name} ζητά σύζευξη με αυτή τη συσκευή. Να επιτρέπεται μόνο αν μόλις ξεκινήσατε τη σύζευξη στο {name}· διαφορετικά απορρίψτε.',
    },
    task: { 'handoff-advanced-unavailable': 'Οι προηγμένες ρυθμίσεις δεν αποστέλλονται σε άλλη συσκευή.' },
  },
  nb: {
    'lan-devices': {
      'pair-request-body':
        '{name} vil koble til denne enheten. Tillat bare hvis du nettopp startet koblingen på {name}; ellers avslå.',
    },
    task: { 'handoff-advanced-unavailable': 'Avanserte innstillinger sendes ikke til en annen enhet.' },
  },
  ca: {
    'lan-devices': {
      'pair-request-body':
        "{name} vol emparellar-se amb aquest dispositiu. Permet-ho només si acabes d'iniciar l'emparellament a {name}; si no, rebutja-ho.",
    },
    task: { 'handoff-advanced-unavailable': "La configuració avançada no s'envia a un altre dispositiu." },
  },
}

for (const locale of readdirSync(localesRoot)) {
  const directory = join(localesRoot, locale)
  if (!statSync(directory).isDirectory()) continue
  const path = join(directory, 'messages.json')
  const messages = JSON.parse(readFileSync(path, 'utf8'))
  const copy = translations[locale] ?? english
  for (const [namespace, values] of Object.entries(copy)) {
    messages[namespace] ??= {}
    Object.assign(messages[namespace], values)
  }
  writeFileSync(path, `${JSON.stringify(messages, null, 2)}\n`)
}
console.log('Added the unpair guard copy to every locale.')
