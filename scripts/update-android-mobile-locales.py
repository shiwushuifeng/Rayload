#!/usr/bin/env python3
"""Add Android port notice and double-back strings to every locale."""

import json
from pathlib import Path

LOCALES_DIR = Path("src/shared/locales")

TRANSLATIONS = {
    "ar": (
        "هذا التطبيق نسخة Android غير رسمية من Rayburst. المؤلف الأصلي: AnInsomniacy، ويخضع لترخيص المصدر المفتوح الأصلي.",
        "اضغط مرة أخرى للخروج",
    ),
    "bg": (
        "Това приложение е неофициален Android порт на Rayburst. Оригинален автор: AnInsomniacy, под оригиналния лиценз за отворен код.",
        "Натиснете отново за изход",
    ),
    "ca": (
        "Aquesta aplicació és un port Android no oficial de Rayburst. Autor original: AnInsomniacy, sota la llicència de codi obert original.",
        "Torneu a prémer per sortir",
    ),
    "de": (
        "Diese App ist ein inoffizieller Android-Port von Rayburst. Ursprünglicher Autor: AnInsomniacy, veröffentlicht unter der ursprünglichen Open-Source-Lizenz.",
        "Zum Beenden erneut zurück drücken",
    ),
    "el": (
        "Αυτή η εφαρμογή είναι ανεπίσημη μεταφορά του Rayburst για Android. Αρχικός δημιουργός: AnInsomniacy, με την αρχική άδεια ανοικτού κώδικα.",
        "Πατήστε ξανά για έξοδο",
    ),
    "en-US": (
        "This app is an unofficial Android port of Rayburst. Original author: AnInsomniacy, released under the original open-source license.",
        "Press back again to exit",
    ),
    "es": (
        "Esta aplicación es un port no oficial de Rayburst para Android. Autor original: AnInsomniacy, bajo la licencia de código abierto original.",
        "Pulse atrás de nuevo para salir",
    ),
    "fa": (
        "این برنامه یک پورت غیررسمی Android از Rayburst است. نویسنده اصلی: AnInsomniacy، تحت مجوز متن‌باز اصلی.",
        "برای خروج دوباره برگشت را بزنید",
    ),
    "fr": (
        "Cette application est un port Android non officiel de Rayburst. Auteur original : AnInsomniacy, sous la licence open source d’origine.",
        "Appuyez à nouveau sur Retour pour quitter",
    ),
    "hi": (
        "यह ऐप Rayburst का अनौपचारिक Android पोर्ट है। मूल लेखक: AnInsomniacy, मूल ओपन-सोर्स लाइसेंस के अंतर्गत।",
        "बाहर निकलने के लिए फिर से बैक दबाएँ",
    ),
    "hu": (
        "Ez az alkalmazás a Rayburst nem hivatalos Android-portja. Eredeti szerző: AnInsomniacy, az eredeti nyílt forráskódú licenc alatt.",
        "A kilépéshez nyomja meg újra a vissza gombot",
    ),
    "id": (
        "Aplikasi ini adalah port Android tidak resmi dari Rayburst. Penulis asli: AnInsomniacy, di bawah lisensi open-source asli.",
        "Tekan kembali lagi untuk keluar",
    ),
    "it": (
        "Questa app è un port Android non ufficiale di Rayburst. Autore originale: AnInsomniacy, secondo la licenza open source originale.",
        "Premi di nuovo Indietro per uscire",
    ),
    "ja": (
        "このアプリは Rayburst の非公式 Android 移植版です。原作者：AnInsomniacy、元のオープンソースライセンスに従います。",
        "終了するにはもう一度戻るを押してください",
    ),
    "ko": (
        "이 앱은 Rayburst의 비공식 Android 포팅 버전입니다. 원저작자: AnInsomniacy, 원래 오픈 소스 라이선스를 따릅니다.",
        "종료하려면 뒤로를 한 번 더 누르세요",
    ),
    "nb": (
        "Denne appen er en uoffisiell Android-port av Rayburst. Opprinnelig forfatter: AnInsomniacy, under den opprinnelige åpen kildekode-lisensen.",
        "Trykk tilbake igjen for å avslutte",
    ),
    "nl": (
        "Deze app is een onofficiële Android-port van Rayburst. Oorspronkelijke auteur: AnInsomniacy, onder de oorspronkelijke open-source licentie.",
        "Druk nogmaals op terug om af te sluiten",
    ),
    "pl": (
        "Ta aplikacja jest nieoficjalnym portem Rayburst na Androida. Oryginalny autor: AnInsomniacy, zgodnie z oryginalną licencją open source.",
        "Naciśnij ponownie Wstecz, aby wyjść",
    ),
    "pt-BR": (
        "Este aplicativo é um porte não oficial do Rayburst para Android. Autor original: AnInsomniacy, sob a licença de código aberto original.",
        "Pressione Voltar novamente para sair",
    ),
    "ro": (
        "Această aplicație este un port Android neoficial al Rayburst. Autor original: AnInsomniacy, conform licenței open-source originale.",
        "Apăsați din nou Înapoi pentru a ieși",
    ),
    "ru": (
        "Это неофициальный Android-порт Rayburst. Оригинальный автор: AnInsomniacy, в соответствии с исходной лицензией открытого исходного кода.",
        "Нажмите «Назад» ещё раз для выхода",
    ),
    "th": (
        "แอปนี้เป็นพอร์ต Android อย่างไม่เป็นทางการของ Rayburst ผู้เขียนต้นฉบับ: AnInsomniacy ภายใต้สัญญาอนุญาตโอเพนซอร์สเดิม",
        "กดย้อนกลับอีกครั้งเพื่อออก",
    ),
    "tr": (
        "Bu uygulama Rayburst'un resmî olmayan Android portudur. Orijinal yazar: AnInsomniacy, özgün açık kaynak lisansı kapsamındadır.",
        "Çıkmak için tekrar geri basın",
    ),
    "uk": (
        "Це неофіційний Android-порт Rayburst. Оригінальний автор: AnInsomniacy, відповідно до початкової ліцензії відкритого коду.",
        "Натисніть «Назад» ще раз, щоб вийти",
    ),
    "vi": (
        "Đây là bản chuyển Android không chính thức của Rayburst. Tác giả gốc: AnInsomniacy, theo giấy phép mã nguồn mở gốc.",
        "Nhấn Quay lại lần nữa để thoát",
    ),
    "zh-CN": (
        "本应用为 Rayburst 的非官方 Android 移植版，原作者：AnInsomniacy，遵循原开源协议。",
        "再按一次返回键退出",
    ),
    "zh-TW": (
        "本應用為 Rayburst 的非官方 Android 移植版，原作者：AnInsomniacy，遵循原開源授權條款。",
        "再按一次返回鍵退出",
    ),
}


for locale, (port_notice, press_back) in TRANSLATIONS.items():
    path = LOCALES_DIR / locale / "messages.json"
    messages = json.loads(path.read_text(encoding="utf-8"))
    messages["about"]["android-port-notice"] = port_notice
    messages["app"]["press-back-again"] = press_back
    path.write_text(
        json.dumps(messages, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )

print(f"Updated {len(TRANSLATIONS)} locales")
