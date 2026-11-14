#!/usr/bin/env node
/** Adds mutual-pairing prompts and the remote save-location hint to every locale. */
import { readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = join(dirname(fileURLToPath(import.meta.url)), '..')
const localesRoot = join(root, 'src', 'shared', 'locales')

const english = {
  'lan-devices.pair-request-title': 'Pairing request',
  'lan-devices.pair-request-body': '{name} wants to pair with this device.',
  'lan-devices.pair-request-approve': 'Approve',
  'lan-devices.pair-request-reject': 'Reject',
  'lan-devices.pair-request-paired': '{name} is now paired.',
  'task.handoff-dir-hint': 'Downloads are saved to the folder configured on {device}.',
}

const translations = {
  'zh-CN': {
    'lan-devices.pair-request-title': '配对请求',
    'lan-devices.pair-request-body': '{name} 请求与这台设备配对。',
    'lan-devices.pair-request-approve': '允许',
    'lan-devices.pair-request-reject': '拒绝',
    'lan-devices.pair-request-paired': '{name} 已配对。',
    'task.handoff-dir-hint': '下载将保存到 {device} 上设置的文件夹。',
  },
  'zh-TW': {
    'lan-devices.pair-request-title': '配對請求',
    'lan-devices.pair-request-body': '{name} 要求與這台裝置配對。',
    'lan-devices.pair-request-approve': '允許',
    'lan-devices.pair-request-reject': '拒絕',
    'lan-devices.pair-request-paired': '{name} 已配對。',
    'task.handoff-dir-hint': '下載將儲存到 {device} 上設定的資料夾。',
  },
  ja: {
    'lan-devices.pair-request-title': 'ペアリング要求',
    'lan-devices.pair-request-body': '{name} がこのデバイスとペアリングを要求しています。',
    'lan-devices.pair-request-approve': '許可',
    'lan-devices.pair-request-reject': '拒否',
    'lan-devices.pair-request-paired': '{name} とペアリングしました。',
    'task.handoff-dir-hint': 'ダウンロードは {device} で設定したフォルダーに保存されます。',
  },
  ko: {
    'lan-devices.pair-request-title': '페어링 요청',
    'lan-devices.pair-request-body': '{name}이(가) 이 기기와 페어링하려고 합니다.',
    'lan-devices.pair-request-approve': '허용',
    'lan-devices.pair-request-reject': '거부',
    'lan-devices.pair-request-paired': '{name} 기기가 페어링되었습니다.',
    'task.handoff-dir-hint': '다운로드는 {device}에 설정된 폴더에 저장됩니다.',
  },
  de: {
    'lan-devices.pair-request-title': 'Kopplungsanfrage',
    'lan-devices.pair-request-body': '{name} möchte dieses Gerät koppeln.',
    'lan-devices.pair-request-approve': 'Zulassen',
    'lan-devices.pair-request-reject': 'Ablehnen',
    'lan-devices.pair-request-paired': '{name} ist jetzt gekoppelt.',
    'task.handoff-dir-hint': 'Downloads werden im auf {device} festgelegten Ordner gespeichert.',
  },
  fr: {
    'lan-devices.pair-request-title': "Demande d'appairage",
    'lan-devices.pair-request-body': "{name} souhaite s'appairer avec cet appareil.",
    'lan-devices.pair-request-approve': 'Autoriser',
    'lan-devices.pair-request-reject': 'Refuser',
    'lan-devices.pair-request-paired': '{name} est maintenant appairé.',
    'task.handoff-dir-hint': 'Les téléchargements sont enregistrés dans le dossier défini sur {device}.',
  },
  es: {
    'lan-devices.pair-request-title': 'Solicitud de vinculación',
    'lan-devices.pair-request-body': '{name} quiere vincularse con este dispositivo.',
    'lan-devices.pair-request-approve': 'Permitir',
    'lan-devices.pair-request-reject': 'Rechazar',
    'lan-devices.pair-request-paired': '{name} ya está vinculado.',
    'task.handoff-dir-hint': 'Las descargas se guardan en la carpeta configurada en {device}.',
  },
  it: {
    'lan-devices.pair-request-title': 'Richiesta di abbinamento',
    'lan-devices.pair-request-body': '{name} vuole abbinarsi a questo dispositivo.',
    'lan-devices.pair-request-approve': 'Consenti',
    'lan-devices.pair-request-reject': 'Rifiuta',
    'lan-devices.pair-request-paired': '{name} è ora abbinato.',
    'task.handoff-dir-hint': 'I download vengono salvati nella cartella impostata su {device}.',
  },
  'pt-BR': {
    'lan-devices.pair-request-title': 'Solicitação de pareamento',
    'lan-devices.pair-request-body': '{name} quer parear com este dispositivo.',
    'lan-devices.pair-request-approve': 'Permitir',
    'lan-devices.pair-request-reject': 'Recusar',
    'lan-devices.pair-request-paired': '{name} agora está pareado.',
    'task.handoff-dir-hint': 'Os downloads são salvos na pasta configurada em {device}.',
  },
  ru: {
    'lan-devices.pair-request-title': 'Запрос на сопряжение',
    'lan-devices.pair-request-body': '{name} хочет сопрячься с этим устройством.',
    'lan-devices.pair-request-approve': 'Разрешить',
    'lan-devices.pair-request-reject': 'Отклонить',
    'lan-devices.pair-request-paired': '{name} теперь сопряжено.',
    'task.handoff-dir-hint': 'Загрузки сохраняются в папку, указанную на {device}.',
  },
  uk: {
    'lan-devices.pair-request-title': 'Запит на спарювання',
    'lan-devices.pair-request-body': '{name} хоче спаруватися з цим пристроєм.',
    'lan-devices.pair-request-approve': 'Дозволити',
    'lan-devices.pair-request-reject': 'Відхилити',
    'lan-devices.pair-request-paired': '{name} тепер спаровано.',
    'task.handoff-dir-hint': 'Завантаження зберігаються в теку, вказану на {device}.',
  },
  pl: {
    'lan-devices.pair-request-title': 'Prośba o sparowanie',
    'lan-devices.pair-request-body': '{name} chce sparować się z tym urządzeniem.',
    'lan-devices.pair-request-approve': 'Zezwól',
    'lan-devices.pair-request-reject': 'Odrzuć',
    'lan-devices.pair-request-paired': '{name} jest teraz sparowane.',
    'task.handoff-dir-hint': 'Pobrane pliki są zapisywane w folderze ustawionym na {device}.',
  },
  nl: {
    'lan-devices.pair-request-title': 'Koppelverzoek',
    'lan-devices.pair-request-body': '{name} wil met dit apparaat koppelen.',
    'lan-devices.pair-request-approve': 'Toestaan',
    'lan-devices.pair-request-reject': 'Weigeren',
    'lan-devices.pair-request-paired': '{name} is nu gekoppeld.',
    'task.handoff-dir-hint': 'Downloads worden opgeslagen in de map die op {device} is ingesteld.',
  },
  ro: {
    'lan-devices.pair-request-title': 'Cerere de asociere',
    'lan-devices.pair-request-body': '{name} dorește să se asocieze cu acest dispozitiv.',
    'lan-devices.pair-request-approve': 'Permite',
    'lan-devices.pair-request-reject': 'Refuză',
    'lan-devices.pair-request-paired': '{name} este acum asociat.',
    'task.handoff-dir-hint': 'Descărcările sunt salvate în folderul configurat pe {device}.',
  },
  hu: {
    'lan-devices.pair-request-title': 'Párosítási kérés',
    'lan-devices.pair-request-body': '{name} párosítani szeretne ezzel az eszközzel.',
    'lan-devices.pair-request-approve': 'Engedélyezés',
    'lan-devices.pair-request-reject': 'Elutasítás',
    'lan-devices.pair-request-paired': '{name} mostantól párosítva van.',
    'task.handoff-dir-hint': 'A letöltések a(z) {device} eszközön beállított mappába kerülnek.',
  },
  tr: {
    'lan-devices.pair-request-title': 'Eşleştirme isteği',
    'lan-devices.pair-request-body': '{name} bu cihazla eşleşmek istiyor.',
    'lan-devices.pair-request-approve': 'İzin ver',
    'lan-devices.pair-request-reject': 'Reddet',
    'lan-devices.pair-request-paired': '{name} artık eşleşti.',
    'task.handoff-dir-hint': 'İndirmeler {device} üzerinde ayarlanan klasöre kaydedilir.',
  },
  ar: {
    'lan-devices.pair-request-title': 'طلب اقتران',
    'lan-devices.pair-request-body': 'يريد {name} الاقتران بهذا الجهاز.',
    'lan-devices.pair-request-approve': 'السماح',
    'lan-devices.pair-request-reject': 'رفض',
    'lan-devices.pair-request-paired': 'تم اقتران {name}.',
    'task.handoff-dir-hint': 'تُحفظ التنزيلات في المجلد المحدد على {device}.',
  },
  fa: {
    'lan-devices.pair-request-title': 'درخواست جفت‌سازی',
    'lan-devices.pair-request-body': '{name} می‌خواهد با این دستگاه جفت شود.',
    'lan-devices.pair-request-approve': 'اجازه',
    'lan-devices.pair-request-reject': 'رد',
    'lan-devices.pair-request-paired': '{name} اکنون جفت شده است.',
    'task.handoff-dir-hint': 'دانلودها در پوشه‌ای که روی {device} تنظیم شده ذخیره می‌شوند.',
  },
  hi: {
    'lan-devices.pair-request-title': 'पेयरिंग अनुरोध',
    'lan-devices.pair-request-body': '{name} इस डिवाइस के साथ पेयर करना चाहता है.',
    'lan-devices.pair-request-approve': 'अनुमति दें',
    'lan-devices.pair-request-reject': 'अस्वीकार करें',
    'lan-devices.pair-request-paired': '{name} अब पेयर हो गया है.',
    'task.handoff-dir-hint': 'डाउनलोड {device} पर सेट फ़ोल्डर में सहेजे जाते हैं.',
  },
  th: {
    'lan-devices.pair-request-title': 'คำขอจับคู่',
    'lan-devices.pair-request-body': '{name} ต้องการจับคู่กับอุปกรณ์นี้',
    'lan-devices.pair-request-approve': 'อนุญาต',
    'lan-devices.pair-request-reject': 'ปฏิเสธ',
    'lan-devices.pair-request-paired': 'จับคู่ {name} แล้ว',
    'task.handoff-dir-hint': 'ดาวน์โหลดจะถูกบันทึกในโฟลเดอร์ที่ตั้งไว้บน {device}',
  },
  vi: {
    'lan-devices.pair-request-title': 'Yêu cầu ghép nối',
    'lan-devices.pair-request-body': '{name} muốn ghép nối với thiết bị này.',
    'lan-devices.pair-request-approve': 'Cho phép',
    'lan-devices.pair-request-reject': 'Từ chối',
    'lan-devices.pair-request-paired': '{name} đã được ghép nối.',
    'task.handoff-dir-hint': 'Tệp tải xuống được lưu vào thư mục đã đặt trên {device}.',
  },
  id: {
    'lan-devices.pair-request-title': 'Permintaan penyandingan',
    'lan-devices.pair-request-body': '{name} ingin menyandingkan dengan perangkat ini.',
    'lan-devices.pair-request-approve': 'Izinkan',
    'lan-devices.pair-request-reject': 'Tolak',
    'lan-devices.pair-request-paired': '{name} kini tersanding.',
    'task.handoff-dir-hint': 'Unduhan disimpan ke folder yang diatur di {device}.',
  },
  bg: {
    'lan-devices.pair-request-title': 'Заявка за сдвояване',
    'lan-devices.pair-request-body': '{name} иска да се сдвои с това устройство.',
    'lan-devices.pair-request-approve': 'Разреши',
    'lan-devices.pair-request-reject': 'Откажи',
    'lan-devices.pair-request-paired': '{name} вече е сдвоено.',
    'task.handoff-dir-hint': 'Изтеглянията се запазват в папката, зададена на {device}.',
  },
  el: {
    'lan-devices.pair-request-title': 'Αίτημα σύζευξης',
    'lan-devices.pair-request-body': 'Το {name} θέλει να συζευχθεί με αυτή τη συσκευή.',
    'lan-devices.pair-request-approve': 'Αποδοχή',
    'lan-devices.pair-request-reject': 'Απόρριψη',
    'lan-devices.pair-request-paired': 'Το {name} συζεύχθηκε.',
    'task.handoff-dir-hint': 'Οι λήψεις αποθηκεύονται στον φάκελο που έχει οριστεί στο {device}.',
  },
  nb: {
    'lan-devices.pair-request-title': 'Sammenkoblingsforespørsel',
    'lan-devices.pair-request-body': '{name} vil kobles til denne enheten.',
    'lan-devices.pair-request-approve': 'Tillat',
    'lan-devices.pair-request-reject': 'Avslå',
    'lan-devices.pair-request-paired': '{name} er nå sammenkoblet.',
    'task.handoff-dir-hint': 'Nedlastinger lagres i mappen som er angitt på {device}.',
  },
  ca: {
    'lan-devices.pair-request-title': "Sol·licitud d'aparellament",
    'lan-devices.pair-request-body': '{name} vol aparellar-se amb aquest dispositiu.',
    'lan-devices.pair-request-approve': 'Permet',
    'lan-devices.pair-request-reject': 'Rebutja',
    'lan-devices.pair-request-paired': '{name} ja està aparellat.',
    'task.handoff-dir-hint': 'Les baixades es desen a la carpeta configurada a {device}.',
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
console.log('Updated mutual pairing copy in every locale.')
