#!/usr/bin/env node
/** Adds the dedicated onboarding consent-page copy to every locale. */
import { readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = join(dirname(fileURLToPath(import.meta.url)), '..')
const localesRoot = join(root, 'src', 'shared', 'locales')

const english = {
  'onboarding.consent-title': 'Review and accept',
  'onboarding.consent-description':
    'Please read the User Agreement and Privacy Policy before continuing. You must accept them to finish setup.',
  'onboarding.consent-eula-hint': 'Terms of use, licensing and liability',
  'onboarding.consent-privacy-hint': 'How your data stays on your device',
}

const translations = {
  'zh-CN': {
    'onboarding.consent-title': '阅读并同意',
    'onboarding.consent-description': '完成设置前，请阅读《用户使用协议》与《隐私政策》。同意后才能继续。',
    'onboarding.consent-eula-hint': '使用条款、许可与责任说明',
    'onboarding.consent-privacy-hint': '了解你的数据如何保留在本机',
  },
  'zh-TW': {
    'onboarding.consent-title': '閱讀並同意',
    'onboarding.consent-description': '完成設定前，請閱讀《使用者協議》與《隱私權政策》。同意後才能繼續。',
    'onboarding.consent-eula-hint': '使用條款、授權與責任說明',
    'onboarding.consent-privacy-hint': '瞭解您的資料如何保留在本機',
  },
  ja: {
    'onboarding.consent-title': '確認と同意',
    'onboarding.consent-description':
      'セットアップを完了する前に、利用規約とプライバシーポリシーをお読みください。同意が必要です。',
    'onboarding.consent-eula-hint': '利用条件・ライセンス・責任について',
    'onboarding.consent-privacy-hint': 'データが端末内に保持される仕組み',
  },
  ko: {
    'onboarding.consent-title': '확인 및 동의',
    'onboarding.consent-description':
      '설정을 완료하려면 사용자 계약과 개인정보 처리방침을 읽고 동의해야 합니다.',
    'onboarding.consent-eula-hint': '이용 약관, 라이선스 및 책임',
    'onboarding.consent-privacy-hint': '데이터가 기기에 보관되는 방식',
  },
  de: {
    'onboarding.consent-title': 'Prüfen und zustimmen',
    'onboarding.consent-description':
      'Bitte lies die Nutzungsvereinbarung und die Datenschutzerklärung, bevor du fortfährst. Zum Abschluss musst du zustimmen.',
    'onboarding.consent-eula-hint': 'Nutzungsbedingungen, Lizenz und Haftung',
    'onboarding.consent-privacy-hint': 'Wie deine Daten auf dem Gerät bleiben',
  },
  fr: {
    'onboarding.consent-title': 'Vérifier et accepter',
    'onboarding.consent-description':
      "Veuillez lire le Contrat d'utilisation et la Politique de confidentialité avant de continuer. Vous devez les accepter pour terminer la configuration.",
    'onboarding.consent-eula-hint': "Conditions d'utilisation, licence et responsabilité",
    'onboarding.consent-privacy-hint': 'Comment vos données restent sur votre appareil',
  },
  es: {
    'onboarding.consent-title': 'Revisar y aceptar',
    'onboarding.consent-description':
      'Lee el Acuerdo de usuario y la Política de privacidad antes de continuar. Debes aceptarlos para terminar la configuración.',
    'onboarding.consent-eula-hint': 'Condiciones de uso, licencia y responsabilidad',
    'onboarding.consent-privacy-hint': 'Cómo tus datos permanecen en tu dispositivo',
  },
  it: {
    'onboarding.consent-title': 'Rivedi e accetta',
    'onboarding.consent-description':
      "Leggi il Contratto utente e l'Informativa sulla privacy prima di continuare. Devi accettarli per completare la configurazione.",
    'onboarding.consent-eula-hint': "Termini d'uso, licenza e responsabilità",
    'onboarding.consent-privacy-hint': 'Come i tuoi dati restano sul dispositivo',
  },
  'pt-BR': {
    'onboarding.consent-title': 'Revisar e aceitar',
    'onboarding.consent-description':
      'Leia o Contrato do Usuário e a Política de Privacidade antes de continuar. Você precisa aceitá-los para concluir a configuração.',
    'onboarding.consent-eula-hint': 'Termos de uso, licença e responsabilidade',
    'onboarding.consent-privacy-hint': 'Como seus dados permanecem no dispositivo',
  },
  ru: {
    'onboarding.consent-title': 'Проверка и согласие',
    'onboarding.consent-description':
      'Прочитайте Пользовательское соглашение и Политику конфиденциальности, прежде чем продолжить. Чтобы завершить настройку, нужно принять их.',
    'onboarding.consent-eula-hint': 'Условия использования, лицензия и ответственность',
    'onboarding.consent-privacy-hint': 'Как ваши данные остаются на устройстве',
  },
  uk: {
    'onboarding.consent-title': 'Перегляд і згода',
    'onboarding.consent-description':
      'Прочитайте Угоду користувача та Політику конфіденційності, перш ніж продовжити. Щоб завершити налаштування, потрібно їх прийняти.',
    'onboarding.consent-eula-hint': 'Умови використання, ліцензія та відповідальність',
    'onboarding.consent-privacy-hint': 'Як ваші дані залишаються на пристрої',
  },
  pl: {
    'onboarding.consent-title': 'Przegląd i akceptacja',
    'onboarding.consent-description':
      'Przeczytaj Umowę użytkownika i Politykę prywatności przed kontynuowaniem. Aby zakończyć konfigurację, musisz je zaakceptować.',
    'onboarding.consent-eula-hint': 'Warunki użytkowania, licencja i odpowiedzialność',
    'onboarding.consent-privacy-hint': 'Jak Twoje dane pozostają na urządzeniu',
  },
  nl: {
    'onboarding.consent-title': 'Controleren en accepteren',
    'onboarding.consent-description':
      'Lees de Gebruikersovereenkomst en het Privacybeleid voordat je doorgaat. Je moet ze accepteren om de installatie te voltooien.',
    'onboarding.consent-eula-hint': 'Gebruiksvoorwaarden, licentie en aansprakelijkheid',
    'onboarding.consent-privacy-hint': 'Hoe je gegevens op je apparaat blijven',
  },
  ro: {
    'onboarding.consent-title': 'Verifică și acceptă',
    'onboarding.consent-description':
      'Citește Acordul utilizatorului și Politica de confidențialitate înainte de a continua. Trebuie să le accepți pentru a finaliza configurarea.',
    'onboarding.consent-eula-hint': 'Termeni de utilizare, licență și răspundere',
    'onboarding.consent-privacy-hint': 'Cum rămân datele tale pe dispozitiv',
  },
  hu: {
    'onboarding.consent-title': 'Áttekintés és elfogadás',
    'onboarding.consent-description':
      'A folytatás előtt olvasd el a Felhasználói szerződést és az Adatvédelmi irányelveket. A beállítás befejezéséhez el kell fogadnod őket.',
    'onboarding.consent-eula-hint': 'Használati feltételek, licenc és felelősség',
    'onboarding.consent-privacy-hint': 'Hogyan maradnak adataid az eszközön',
  },
  tr: {
    'onboarding.consent-title': 'İncele ve kabul et',
    'onboarding.consent-description':
      "Devam etmeden önce Kullanıcı Sözleşmesi'ni ve Gizlilik Politikası'nı okuyun. Kurulumu tamamlamak için kabul etmeniz gerekir.",
    'onboarding.consent-eula-hint': 'Kullanım koşulları, lisans ve sorumluluk',
    'onboarding.consent-privacy-hint': 'Verilerinizin cihazınızda nasıl kaldığı',
  },
  ar: {
    'onboarding.consent-title': 'المراجعة والقبول',
    'onboarding.consent-description':
      'يرجى قراءة اتفاقية المستخدم وسياسة الخصوصية قبل المتابعة. يجب قبولهما لإكمال الإعداد.',
    'onboarding.consent-eula-hint': 'شروط الاستخدام والترخيص والمسؤولية',
    'onboarding.consent-privacy-hint': 'كيف تبقى بياناتك على جهازك',
  },
  fa: {
    'onboarding.consent-title': 'بررسی و پذیرش',
    'onboarding.consent-description':
      'لطفاً پیش از ادامه، توافق‌نامه کاربر و سیاست حریم خصوصی را بخوانید. برای تکمیل راه‌اندازی باید آن‌ها را بپذیرید.',
    'onboarding.consent-eula-hint': 'شرایط استفاده، مجوز و مسئولیت',
    'onboarding.consent-privacy-hint': 'چگونه داده‌های شما روی دستگاه می‌ماند',
  },
  hi: {
    'onboarding.consent-title': 'समीक्षा करें और स्वीकारें',
    'onboarding.consent-description':
      'जारी रखने से पहले उपयोगकर्ता समझौता और गोपनीयता नीति पढ़ें। सेटअप पूरा करने के लिए आपको इन्हें स्वीकार करना होगा।',
    'onboarding.consent-eula-hint': 'उपयोग की शर्तें, लाइसेंस और दायित्व',
    'onboarding.consent-privacy-hint': 'आपका डेटा आपके डिवाइस पर कैसे रहता है',
  },
  th: {
    'onboarding.consent-title': 'ตรวจสอบและยอมรับ',
    'onboarding.consent-description':
      'โปรดอ่านข้อตกลงผู้ใช้และนโยบายความเป็นส่วนตัวก่อนดำเนินการต่อ คุณต้องยอมรับเพื่อตั้งค่าให้เสร็จสิ้น',
    'onboarding.consent-eula-hint': 'ข้อกำหนดการใช้งาน สัญญาอนุญาต และความรับผิด',
    'onboarding.consent-privacy-hint': 'ข้อมูลของคุณอยู่ในอุปกรณ์อย่างไร',
  },
  vi: {
    'onboarding.consent-title': 'Xem lại và chấp nhận',
    'onboarding.consent-description':
      'Vui lòng đọc Thỏa thuận người dùng và Chính sách quyền riêng tư trước khi tiếp tục. Bạn phải chấp nhận để hoàn tất thiết lập.',
    'onboarding.consent-eula-hint': 'Điều khoản sử dụng, giấy phép và trách nhiệm',
    'onboarding.consent-privacy-hint': 'Cách dữ liệu của bạn lưu trên thiết bị',
  },
  id: {
    'onboarding.consent-title': 'Tinjau dan setujui',
    'onboarding.consent-description':
      'Baca Perjanjian Pengguna dan Kebijakan Privasi sebelum melanjutkan. Anda harus menyetujuinya untuk menyelesaikan penyiapan.',
    'onboarding.consent-eula-hint': 'Ketentuan penggunaan, lisensi, dan tanggung jawab',
    'onboarding.consent-privacy-hint': 'Bagaimana data Anda tetap di perangkat',
  },
  bg: {
    'onboarding.consent-title': 'Преглед и приемане',
    'onboarding.consent-description':
      'Прочетете Потребителското споразумение и Политиката за поверителност, преди да продължите. Трябва да ги приемете, за да завършите настройката.',
    'onboarding.consent-eula-hint': 'Условия за ползване, лиценз и отговорност',
    'onboarding.consent-privacy-hint': 'Как данните ви остават на устройството',
  },
  el: {
    'onboarding.consent-title': 'Έλεγχος και αποδοχή',
    'onboarding.consent-description':
      'Διάβασε τη Συμφωνία χρήστη και την Πολιτική απορρήτου πριν συνεχίσεις. Πρέπει να τις αποδεχτείς για να ολοκληρώσεις τη ρύθμιση.',
    'onboarding.consent-eula-hint': 'Όροι χρήσης, άδεια και ευθύνη',
    'onboarding.consent-privacy-hint': 'Πώς τα δεδομένα σου μένουν στη συσκευή',
  },
  nb: {
    'onboarding.consent-title': 'Se gjennom og godta',
    'onboarding.consent-description':
      'Les Brukeravtalen og Personvernerklæringen før du fortsetter. Du må godta dem for å fullføre oppsettet.',
    'onboarding.consent-eula-hint': 'Bruksvilkår, lisens og ansvar',
    'onboarding.consent-privacy-hint': 'Hvordan dataene dine blir på enheten',
  },
  ca: {
    'onboarding.consent-title': 'Revisa i accepta',
    'onboarding.consent-description':
      "Llegeix el Contracte d'usuari i la Política de privadesa abans de continuar. Has d'acceptar-los per acabar la configuració.",
    'onboarding.consent-eula-hint': "Condicions d'ús, llicència i responsabilitat",
    'onboarding.consent-privacy-hint': 'Com es queden les teves dades al dispositiu',
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
  delete messages.legal?.['consent-prefix']
  delete messages.legal?.['consent-and']
  writeFileSync(path, `${JSON.stringify(messages, null, 2)}\n`)
}
console.log('Updated onboarding consent-page copy in every locale.')
