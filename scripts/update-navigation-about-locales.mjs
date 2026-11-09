#!/usr/bin/env node
/** Batch updates the navigation-customization, Android file-open, and About copy. */

import { readdirSync, readFileSync, writeFileSync } from 'node:fs'
import { join, resolve } from 'node:path'

const root = resolve(import.meta.dirname, '..')
const localesRoot = join(root, 'src/shared/locales')

const translations = {
  'en-US': {
    navigation: 'Bottom Navigation',
    navigationHint: 'Choose which destinations stay visible. Settings always remains available.',
    fileFallback: 'The file is saved in the Download/Rayload folder.',
    acknowledgementLabel: 'Open-source acknowledgement',
    acknowledgement:
      '{label}: Thanks to AnInsomniacy for developing the open-source Rayburst project. Rayload is an unofficial Android port of it, independently maintained and released under the MIT open-source license.',
  },
  'zh-CN': {
    navigation: '导航栏自定义',
    navigationHint: '选择要保留在底部的页面。设置入口始终可用。',
    fileFallback: '文件已保存在 Download/Rayload 目录',
    acknowledgementLabel: '开源致谢',
    acknowledgement:
      '{label}：感谢 AnInsomniacy 开发的开源项目 Rayburst。Rayload 是基于其进行 Android 平台移植与独立维护的非官方版本，遵循 MIT 开源协议。',
  },
  'zh-TW': {
    navigation: '導覽列自訂',
    navigationHint: '選擇要保留在底部的頁面。設定入口始終可用。',
    fileFallback: '檔案已儲存在 Download/Rayload 目錄',
    acknowledgementLabel: '開源致謝',
    acknowledgement:
      '{label}：感謝 AnInsomniacy 開發的開源專案 Rayburst。Rayload 是基於其進行 Android 平台移植與獨立維護的非官方版本，遵循 MIT 開源授權條款。',
  },
  ar: {
    navigation: 'شريط التنقل السفلي',
    navigationHint: 'اختر الوجهات التي تبقى ظاهرة في الأسفل. تظل الإعدادات متاحة دائمًا.',
    fileFallback: 'تم حفظ الملف في مجلد Download/Rayload.',
    acknowledgementLabel: 'شكر للمشروع مفتوح المصدر',
    acknowledgement:
      '{label}: نشكر AnInsomniacy على تطوير مشروع Rayburst مفتوح المصدر. Rayload نسخة Android غير رسمية منه، تُصان بشكل مستقل وتُصدر بموجب ترخيص MIT مفتوح المصدر.',
  },
  bg: {
    navigation: 'Долна навигация',
    navigationHint: 'Изберете кои раздели да останат видими отдолу. Настройките винаги са достъпни.',
    fileFallback: 'Файлът е запазен в папка Download/Rayload.',
    acknowledgementLabel: 'Благодарност за отворения код',
    acknowledgement:
      '{label}: Благодарим на AnInsomniacy за разработването на проекта с отворен код Rayburst. Rayload е неофициален Android порт, поддържан независимо и издаден под MIT лиценз.',
  },
  ca: {
    navigation: 'Navegació inferior',
    navigationHint: 'Trieu quines destinacions es mantenen visibles a baix. La configuració sempre és accessible.',
    fileFallback: 'El fitxer s’ha desat a la carpeta Download/Rayload.',
    acknowledgementLabel: 'Agraïment al codi obert',
    acknowledgement:
      '{label}: Gràcies a AnInsomniacy per desenvolupar el projecte de codi obert Rayburst. Rayload és un port Android no oficial, mantingut de manera independent i publicat sota la llicència MIT.',
  },
  de: {
    navigation: 'Untere Navigation',
    navigationHint: 'Wähle, welche Ziele unten sichtbar bleiben. Einstellungen bleiben immer verfügbar.',
    fileFallback: 'Die Datei wurde im Ordner Download/Rayload gespeichert.',
    acknowledgementLabel: 'Dank an Open Source',
    acknowledgement:
      '{label}: Danke an AnInsomniacy für die Entwicklung des Open-Source-Projekts Rayburst. Rayload ist ein inoffizieller Android-Port, unabhängig gepflegt und unter der MIT-Open-Source-Lizenz veröffentlicht.',
  },
  el: {
    navigation: 'Κάτω πλοήγηση',
    navigationHint: 'Επιλέξτε ποιοι προορισμοί παραμένουν ορατοί στο κάτω μέρος. Οι ρυθμίσεις είναι πάντα διαθέσιμες.',
    fileFallback: 'Το αρχείο αποθηκεύτηκε στον φάκελο Download/Rayload.',
    acknowledgementLabel: 'Ευχαριστίες ανοικτού κώδικα',
    acknowledgement:
      '{label}: Ευχαριστούμε τον AnInsomniacy για την ανάπτυξη του έργου ανοικτού κώδικα Rayburst. Το Rayload είναι μια ανεπίσημη μεταφορά για Android, με ανεξάρτητη συντήρηση και άδεια MIT.',
  },
  es: {
    navigation: 'Navegación inferior',
    navigationHint: 'Elige qué destinos permanecen visibles abajo. Ajustes siempre estará disponible.',
    fileFallback: 'El archivo se ha guardado en la carpeta Download/Rayload.',
    acknowledgementLabel: 'Agradecimiento al código abierto',
    acknowledgement:
      '{label}: Gracias a AnInsomniacy por desarrollar el proyecto de código abierto Rayburst. Rayload es un port no oficial para Android, mantenido de forma independiente y publicado bajo la licencia MIT.',
  },
  fa: {
    navigation: 'پیمایش پایین',
    navigationHint: 'انتخاب کنید کدام مقصدها در پایین نمایش داده شوند. تنظیمات همیشه در دسترس است.',
    fileFallback: 'فایل در پوشه Download/Rayload ذخیره شده است.',
    acknowledgementLabel: 'سپاس از پروژه متن‌باز',
    acknowledgement:
      '{label}: از AnInsomniacy برای توسعه پروژه متن‌باز Rayburst سپاسگزاریم. Rayload یک نسخه غیررسمی Android از آن است که به‌صورت مستقل نگهداری و با مجوز MIT منتشر می‌شود.',
  },
  fr: {
    navigation: 'Navigation inférieure',
    navigationHint: 'Choisissez les destinations visibles en bas. Les réglages restent toujours accessibles.',
    fileFallback: 'Le fichier est enregistré dans le dossier Download/Rayload.',
    acknowledgementLabel: 'Remerciements open source',
    acknowledgement:
      '{label} : Merci à AnInsomniacy d’avoir développé le projet open source Rayburst. Rayload est un port Android non officiel, maintenu indépendamment et publié sous licence MIT.',
  },
  hi: {
    navigation: 'निचला नेविगेशन',
    navigationHint: 'चुनें कि कौनसे पेज नीचे दिखाई दें। सेटिंग्स हमेशा उपलब्ध रहती हैं।',
    fileFallback: 'फ़ाइल Download/Rayload फ़ोल्डर में सहेजी गई है।',
    acknowledgementLabel: 'ओपन-सोर्स आभार',
    acknowledgement:
      '{label}: ओपन-सोर्स प्रोजेक्ट Rayburst विकसित करने के लिए AnInsomniacy का धन्यवाद। Rayload इसका अनौपचारिक Android पोर्ट है, जिसे स्वतंत्र रूप से बनाए रखा जाता है और MIT लाइसेंस के अंतर्गत जारी किया जाता है।',
  },
  hu: {
    navigation: 'Alsó navigáció',
    navigationHint: 'Válaszd ki, mely célok maradjanak láthatók alul. A beállítások mindig elérhetők.',
    fileFallback: 'A fájl a Download/Rayload mappába került.',
    acknowledgementLabel: 'Nyílt forráskódú köszönet',
    acknowledgement:
      '{label}: Köszönjük AnInsomniacynak a Rayburst nyílt forráskódú projekt kidolgozását. A Rayload annak nem hivatalos Android-portja, függetlenül karbantartva és MIT-licenc alatt kiadva.',
  },
  id: {
    navigation: 'Navigasi bawah',
    navigationHint: 'Pilih halaman yang tetap terlihat di bawah. Pengaturan selalu tersedia.',
    fileFallback: 'File telah disimpan di folder Download/Rayload.',
    acknowledgementLabel: 'Ucapan terima kasih open source',
    acknowledgement:
      '{label}: Terima kasih kepada AnInsomniacy yang mengembangkan proyek open source Rayburst. Rayload adalah port Android tidak resmi, dikelola secara independen, dan dirilis dengan lisensi MIT.',
  },
  it: {
    navigation: 'Navigazione inferiore',
    navigationHint: 'Scegli quali destinazioni restano visibili in basso. Le impostazioni sono sempre disponibili.',
    fileFallback: 'Il file è stato salvato nella cartella Download/Rayload.',
    acknowledgementLabel: 'Ringraziamenti open source',
    acknowledgement:
      '{label}: Grazie ad AnInsomniacy per aver sviluppato il progetto open source Rayburst. Rayload è un port Android non ufficiale, mantenuto in modo indipendente e distribuito con licenza MIT.',
  },
  ja: {
    navigation: '下部ナビゲーション',
    navigationHint: '下部に表示するページを選択します。設定は常に利用できます。',
    fileFallback: 'ファイルは Download/Rayload フォルダーに保存されています。',
    acknowledgementLabel: 'オープンソースへの謝辞',
    acknowledgement:
      '{label}: オープンソースプロジェクト Rayburst を開発した AnInsomniacy に感謝します。Rayload はその非公式 Android 移植版であり、独立して維持され、MIT ライセンスで公開されています。',
  },
  ko: {
    navigation: '하단 탐색',
    navigationHint: '아래에 표시할 페이지를 선택하세요. 설정은 항상 사용할 수 있습니다.',
    fileFallback: '파일이 Download/Rayload 폴더에 저장되었습니다.',
    acknowledgementLabel: '오픈 소스 감사',
    acknowledgement:
      '{label}: 오픈 소스 프로젝트 Rayburst를 개발한 AnInsomniacy에게 감사드립니다. Rayload는 이를 기반으로 독립 유지보수되는 비공식 Android 포팅 버전이며 MIT 라이선스를 따릅니다.',
  },
  nb: {
    navigation: 'Bunnnavigasjon',
    navigationHint: 'Velg hvilke sider som skal være synlige nederst. Innstillinger er alltid tilgjengelig.',
    fileFallback: 'Filen er lagret i Download/Rayload-mappen.',
    acknowledgementLabel: 'Tak til åpen kildekode',
    acknowledgement:
      '{label}: Takk til AnInsomniacy for utviklingen av åpen kildekode-prosjektet Rayburst. Rayload er en uoffisiell Android-port som vedlikeholdes uavhengig og publiseres under MIT-lisensen.',
  },
  nl: {
    navigation: 'Onderste navigatie',
    navigationHint: 'Kies welke bestemmingen onderaan zichtbaar blijven. Instellingen blijven altijd beschikbaar.',
    fileFallback: 'Het bestand is opgeslagen in de map Download/Rayload.',
    acknowledgementLabel: 'Dank aan open source',
    acknowledgement:
      '{label}: Dank aan AnInsomniacy voor de ontwikkeling van het opensourceproject Rayburst. Rayload is een onofficiële Android-port, onafhankelijk onderhouden en uitgegeven onder de MIT-licentie.',
  },
  pl: {
    navigation: 'Dolna nawigacja',
    navigationHint: 'Wybierz strony widoczne na dole. Ustawienia pozostają zawsze dostępne.',
    fileFallback: 'Plik zapisano w folderze Download/Rayload.',
    acknowledgementLabel: 'Podziękowania dla open source',
    acknowledgement:
      '{label}: Dziękujemy AnInsomniacy za rozwój projektu open source Rayburst. Rayload jest nieoficjalnym portem na Androida, utrzymywanym niezależnie i publikowanym na licencji MIT.',
  },
  'pt-BR': {
    navigation: 'Navegação inferior',
    navigationHint: 'Escolha quais páginas permanecem visíveis abaixo. Configurações sempre fica disponível.',
    fileFallback: 'O arquivo foi salvo na pasta Download/Rayload.',
    acknowledgementLabel: 'Agradecimento ao código aberto',
    acknowledgement:
      '{label}: Agradecemos a AnInsomniacy por desenvolver o projeto de código aberto Rayburst. O Rayload é um porte não oficial para Android, mantido de forma independente e publicado sob a licença MIT.',
  },
  ro: {
    navigation: 'Navigare inferioară',
    navigationHint: 'Alege ce pagini rămân vizibile jos. Setările sunt întotdeauna disponibile.',
    fileFallback: 'Fișierul a fost salvat în folderul Download/Rayload.',
    acknowledgementLabel: 'Mulțumiri pentru open source',
    acknowledgement:
      '{label}: Mulțumim lui AnInsomniacy pentru dezvoltarea proiectului open source Rayburst. Rayload este un port Android neoficial, întreținut independent și publicat sub licența MIT.',
  },
  ru: {
    navigation: 'Нижняя навигация',
    navigationHint: 'Выберите разделы, которые останутся внизу. Настройки всегда доступны.',
    fileFallback: 'Файл сохранён в папке Download/Rayload.',
    acknowledgementLabel: 'Благодарность открытому коду',
    acknowledgement:
      '{label}: Благодарим AnInsomniacy за разработку проекта с открытым исходным кодом Rayburst. Rayload — неофициальный Android-порт, независимо поддерживаемый и распространяемый по лицензии MIT.',
  },
  th: {
    navigation: 'แถบนำทางด้านล่าง',
    navigationHint: 'เลือกหน้าที่จะแสดงด้านล่าง การตั้งค่าจะพร้อมใช้งานเสมอ',
    fileFallback: 'บันทึกไฟล์ไว้ในโฟลเดอร์ Download/Rayload แล้ว',
    acknowledgementLabel: 'ขอบคุณโอเพนซอร์ส',
    acknowledgement:
      '{label}: ขอบคุณ AnInsomniacy ที่พัฒนาโครงการโอเพนซอร์ส Rayburst Rayload เป็นพอร์ต Android อย่างไม่เป็นทางการ ดูแลรักษาอย่างอิสระ และเผยแพร่ภายใต้สัญญาอนุญาต MIT',
  },
  tr: {
    navigation: 'Alt gezinme',
    navigationHint: 'Altta görünür kalacak sayfaları seçin. Ayarlar her zaman kullanılabilir.',
    fileFallback: 'Dosya Download/Rayload klasörüne kaydedildi.',
    acknowledgementLabel: 'Açık kaynağa teşekkür',
    acknowledgement:
      '{label}: Açık kaynak Rayburst projesini geliştirdiği için AnInsomniacy’ye teşekkür ederiz. Rayload, bağımsız olarak sürdürülen ve MIT lisansıyla yayımlanan resmî olmayan bir Android portudur.',
  },
  uk: {
    navigation: 'Нижня навігація',
    navigationHint: 'Виберіть сторінки, які залишаться внизу. Налаштування завжди доступні.',
    fileFallback: 'Файл збережено в папці Download/Rayload.',
    acknowledgementLabel: 'Подяка відкритому коду',
    acknowledgement:
      '{label}: Дякуємо AnInsomniacy за розробку проєкту з відкритим кодом Rayburst. Rayload — неофіційний Android-порт, який незалежно підтримується та розповсюджується за ліцензією MIT.',
  },
  vi: {
    navigation: 'Điều hướng dưới',
    navigationHint: 'Chọn các trang vẫn hiển thị ở dưới. Cài đặt luôn khả dụng.',
    fileFallback: 'Tệp đã được lưu trong thư mục Download/Rayload.',
    acknowledgementLabel: 'Tri ân mã nguồn mở',
    acknowledgement:
      '{label}: Cảm ơn AnInsomniacy đã phát triển dự án mã nguồn mở Rayburst. Rayload là bản chuyển Android không chính thức, được duy trì độc lập và phát hành theo giấy phép MIT.',
  },
}

const secondaryCopy = {
  'zh-CN': {
    appearance: '选择浅色或深色外观，并挑选喜欢的品牌配色。',
    sha: 'SHA-256',
    calculating: '正在计算 {fileName} 的 SHA-256…',
    copy: '复制 SHA-256',
    copied: 'SHA-256 已复制',
    failed: '无法计算该文件的 SHA-256',
    logo: '品牌标志',
    logoAuto: '自动（亮色米白金 / 暗色紫罗兰）',
    logoGold: '米白金',
    logoViolet: '紫罗兰',
    logoBlue: '米蓝色',
    logoTeal: '青黑色',
    androidIcon: 'Android 桌面图标',
    androidIconPurple: '经典紫罗兰',
    androidIconGold: '米白金',
    androidIconBlue: '米蓝色',
    androidIconTeal: '青黑色',
    androidIconFailed: '无法切换 Android 桌面图标',
  },
  'zh-TW': {
    appearance: '選擇淺色或深色外觀，並挑選喜歡的品牌配色。',
    sha: 'SHA-256',
    calculating: '正在計算 {fileName} 的 SHA-256…',
    copy: '複製 SHA-256',
    copied: '已複製 SHA-256',
    failed: '無法計算該檔案的 SHA-256',
    logo: '品牌標誌',
    logoAuto: '自動（亮色米白金 / 暗色紫羅蘭）',
    logoGold: '米白金',
    logoViolet: '紫羅蘭',
    logoBlue: '米藍色',
    logoTeal: '青黑色',
    androidIcon: 'Android 桌面圖示',
    androidIconPurple: '經典紫羅蘭',
    androidIconGold: '米白金',
    androidIconBlue: '米藍色',
    androidIconTeal: '青黑色',
    androidIconFailed: '無法切換 Android 桌面圖示',
  },
}

const englishSecondaryCopy = {
  appearance: 'Choose a light or dark appearance and a brand color scheme.',
  sha: 'SHA-256',
  calculating: 'Calculating SHA-256 for {fileName}…',
  copy: 'Copy SHA-256',
  copied: 'SHA-256 copied',
  failed: 'Could not calculate the file SHA-256',
  logo: 'Brand logo',
  logoAuto: 'Auto (cream-gold on light, violet on dark)',
  logoGold: 'Cream & Gold',
  logoViolet: 'Violet',
  logoBlue: 'Cream & Blue',
  logoTeal: 'Teal & Black',
  androidIcon: 'Android Home Icon',
  androidIconPurple: 'Classic Violet',
  androidIconGold: 'Cream & Gold',
  androidIconBlue: 'Cream & Blue',
  androidIconTeal: 'Teal & Black',
  androidIconFailed: 'Could not change the Android Home icon',
}

for (const locale of readdirSync(localesRoot, { withFileTypes: true })) {
  if (!locale.isDirectory()) continue
  const values = translations[locale.name]
  const secondary = secondaryCopy[locale.name] ?? englishSecondaryCopy
  if (!values) throw new Error(`Missing translations for ${locale.name}`)

  const path = join(localesRoot, locale.name, 'messages.json')
  const messages = JSON.parse(readFileSync(path, 'utf8'))
  delete messages.about['android-port-notice']
  delete messages.about['original-work']
  messages.about['open-source-acknowledgement-label'] = values.acknowledgementLabel
  messages.about['open-source-acknowledgement'] = values.acknowledgement
  messages.preferences['navigation-customization'] = values.navigation
  messages.preferences['navigation-customization-hint'] = values.navigationHint
  messages.onboarding['appearance-description'] = secondary.appearance
  messages.task['android-file-open-fallback'] = values.fileFallback
  messages.task.sha256 = secondary.sha
  messages.task['sha256-calculating'] = secondary.calculating
  messages.task['sha256-copy'] = secondary.copy
  messages.task['sha256-copied'] = secondary.copied
  messages.task['sha256-failed'] = secondary.failed
  messages.preferences['brand-logo'] = secondary.logo
  messages.preferences['brand-logo-auto'] = secondary.logoAuto
  messages.preferences['brand-logo-gold'] = secondary.logoGold
  messages.preferences['brand-logo-violet'] = secondary.logoViolet
  messages.preferences['brand-logo-blue'] = secondary.logoBlue
  messages.preferences['brand-logo-teal'] = secondary.logoTeal
  messages.preferences['android-app-icon'] = secondary.androidIcon
  messages.preferences['android-app-icon-purple'] = secondary.androidIconPurple
  messages.preferences['android-app-icon-gold'] = secondary.androidIconGold
  messages.preferences['android-app-icon-blue'] = secondary.androidIconBlue
  messages.preferences['android-app-icon-teal'] = secondary.androidIconTeal
  messages.preferences['android-app-icon-failed'] = secondary.androidIconFailed
  writeFileSync(path, `${JSON.stringify(messages, null, 2)}\n`)
}

console.log(`Updated navigation and About copy in ${Object.keys(translations).length} locales`)
