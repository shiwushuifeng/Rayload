#!/usr/bin/env python3
"""Add the v1.0.2 Android icon refresh hint to every frontend locale."""

import json
from pathlib import Path

LOCALES_DIR = Path("src/shared/locales")
KEY = "android-app-icon-restart-hint"

TRANSLATIONS = {
    "ar": "إذا لم يتغير الرمز فورًا، فأعد تشغيل Rayload أو الشاشة الرئيسية.",
    "bg": "Ако иконата не се обнови веднага, рестартирайте Rayload или началния екран.",
    "ca": "Si la icona no canvia immediatament, reinicia Rayload o la pantalla d'inici.",
    "de": "Wenn das Symbol nicht sofort aktualisiert wird, starten Sie Rayload oder den Startbildschirm neu.",
    "el": "Αν το εικονίδιο δεν αλλάξει αμέσως, επανεκκινήστε το Rayload ή την αρχική οθόνη.",
    "en-US": "If the icon does not change immediately, restart Rayload or your home screen.",
    "es": "Si el icono no cambia de inmediato, reinicia Rayload o la pantalla de inicio.",
    "fa": "اگر نماد فوراً تغییر نکرد، Rayload یا صفحه اصلی را دوباره راه‌اندازی کنید.",
    "fr": "Si l’icône ne change pas immédiatement, redémarrez Rayload ou l’écran d’accueil.",
    "hi": "यदि आइकन तुरंत नहीं बदलता है, तो Rayload या होम स्क्रीन को पुनः प्रारंभ करें।",
    "hu": "Ha az ikon nem frissül azonnal, indítsa újra a Rayloadot vagy a kezdőképernyőt.",
    "id": "Jika ikon tidak langsung berubah, mulai ulang Rayload atau layar utama.",
    "it": "Se l’icona non cambia subito, riavvia Rayload o la schermata Home.",
    "ja": "アイコンがすぐに反映されない場合は、アプリまたはホーム画面を再起動してください。",
    "ko": "아이콘이 즉시 바뀌지 않으면 앱이나 홈 화면을 다시 시작하세요.",
    "nb": "Hvis ikonet ikke endres umiddelbart, start Rayload eller startskjermen på nytt.",
    "nl": "Als het pictogram niet meteen verandert, herstart Rayload of je startscherm.",
    "pl": "Jeśli ikona nie zmieni się od razu, uruchom ponownie Rayload lub ekran główny.",
    "pt-BR": "Se o ícone não mudar imediatamente, reinicie o Rayload ou a tela inicial.",
    "ro": "Dacă pictograma nu se schimbă imediat, repornește Rayload sau ecranul principal.",
    "ru": "Если значок не обновился сразу, перезапустите Rayload или главный экран.",
    "th": "หากไอคอนไม่เปลี่ยนทันที ให้รีสตาร์ท Rayload หรือหน้าจอหลัก",
    "tr": "Simge hemen değişmezse Rayload’u veya ana ekranı yeniden başlatın.",
    "uk": "Якщо значок не оновився одразу, перезапустіть Rayload або головний екран.",
    "vi": "Nếu biểu tượng không thay đổi ngay, hãy khởi động lại Rayload hoặc màn hình chính.",
    "zh-CN": "若更换图标后未立即生效，请尝试重启软件或手机桌面。",
    "zh-TW": "若更換圖示後未立即生效，請嘗試重新啟動軟體或手機桌面。",
}

for locale, message in TRANSLATIONS.items():
    path = LOCALES_DIR / locale / "messages.json"
    messages = json.loads(path.read_text(encoding="utf-8"))
    preferences = messages["preferences"]
    preferences[KEY] = message
    path.write_text(json.dumps(messages, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")

print(f"Updated {len(TRANSLATIONS)} locale files with {KEY}.")
