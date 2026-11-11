#!/usr/bin/env python3
"""Add the keep-awake dashboard copy to every supported locale."""

import json
from pathlib import Path

LOCALES_DIR = Path("src/shared/locales")

COPY = {
    "ar": (
        "تم تفعيل وضع إبقاء الشاشة مضاءة. يُنصح بتوصيل الطاقة.",
        "وضع إبقاء الشاشة مضاءة",
        "إجمالي سرعة التنزيل",
        "التنزيلات النشطة: {count}",
        "انقر في أي مكان للخروج",
        "إبقاء الشاشة مضاءة",
    ),
    "bg": (
        "Режимът за постоянно включен екран е активиран. Препоръчва се свързване към захранване.",
        "Режим включен екран",
        "Обща скорост на изтегляне",
        "Активни изтегляния: {count}",
        "Докоснете навсякъде, за да излезете",
        "Поддържай екрана включен",
    ),
    "ca": (
        "El mode de pantalla sempre activa està activat. Es recomana connectar-lo al corrent.",
        "Mode de pantalla activa",
        "Velocitat total de descàrrega",
        "Descàrregues actives: {count}",
        "Toca a qualsevol lloc per sortir",
        "Mantén la pantalla encesa",
    ),
    "de": (
        "Bildschirm-an-Modus aktiviert. Netzbetrieb wird empfohlen.",
        "Bildschirm-an-Modus",
        "Gesamte Download-Geschwindigkeit",
        "Aktive Downloads: {count}",
        "Zum Beenden beliebig tippen",
        "Bildschirm anlassen",
    ),
    "el": (
        "Η λειτουργία διατήρησης οθόνης ενεργής είναι ενεργή. Συνιστάται σύνδεση με τροφοδοσία.",
        "Λειτουργία ενεργής οθόνης",
        "Συνολική ταχύτητα λήψης",
        "Ενεργές λήψεις: {count}",
        "Πατήστε οπουδήποτε για έξοδο",
        "Διατήρηση οθόνης ενεργής",
    ),
    "en-US": (
        "Keep-awake mode is on. Connecting to power is recommended.",
        "Keep-awake mode",
        "Total download speed",
        "Active downloads: {count}",
        "Tap anywhere to exit keep-awake mode",
        "Keep Awake",
    ),
    "es": (
        "Modo de pantalla siempre activa activado. Se recomienda conectarlo a la corriente.",
        "Modo pantalla activa",
        "Velocidad total de descarga",
        "Descargas activas: {count}",
        "Toca en cualquier lugar para salir",
        "Mantener pantalla encendida",
    ),
    "fa": (
        "حالت روشن ماندن صفحه فعال شد. اتصال به برق توصیه می‌شود.",
        "حالت روشن ماندن صفحه",
        "سرعت کل دانلود",
        "دانلودهای فعال: {count}",
        "برای خروج هر جا را لمس کنید",
        "روشن نگه داشتن صفحه",
    ),
    "fr": (
        "Le mode veille désactivée est activé. Il est conseillé de brancher l'appareil.",
        "Mode écran toujours allumé",
        "Vitesse de téléchargement totale",
        "Téléchargements actifs : {count}",
        "Touchez n'importe où pour quitter",
        "Écran toujours allumé",
    ),
    "hi": (
        "स्क्रीन चालू रखने वाला मोड सक्षम है। बिजली से जोड़ने की सलाह दी जाती है।",
        "स्क्रीन चालू मोड",
        "कुल डाउनलोड गति",
        "सक्रिय डाउनलोड: {count}",
        "बाहर निकलने के लिए कहीं भी टैप करें",
        "स्क्रीन चालू रखें",
    ),
    "hu": (
        "Az ébren tartás mód bekapcsolva. Ajánlott az áramforrásra csatlakoztatás.",
        "Ébren tartás mód",
        "Teljes letöltési sebesség",
        "Aktív letöltések: {count}",
        "Koppintson bárhová a kilépéshez",
        "Képernyő bekapcsolva tartása",
    ),
    "id": (
        "Mode layar tetap menyala diaktifkan. Sebaiknya hubungkan ke daya.",
        "Mode layar menyala",
        "Kecepatan unduh total",
        "Unduhan aktif: {count}",
        "Ketuk di mana saja untuk keluar",
        "Layar tetap menyala",
    ),
    "it": (
        "Modalità schermo sempre acceso attiva. Si consiglia di collegare l'alimentazione.",
        "Modalità schermo acceso",
        "Velocità di download totale",
        "Download attivi: {count}",
        "Tocca in qualsiasi punto per uscire",
        "Mantieni schermo acceso",
    ),
    "ja": (
        "スリープ防止モードをオンにしました。電源接続をおすすめします。",
        "スリープ防止モード",
        "現在の合計ダウンロード速度",
        "ダウンロード中: {count} 件",
        "画面の任意の場所をタップして終了",
        "画面スリープ防止",
    ),
    "ko": (
        "화면 켜짐 유지 모드가 켜졌습니다. 전원 연결을 권장합니다.",
        "화면 켜짐 유지",
        "현재 총 다운로드 속도",
        "다운로드 중: {count}개",
        "화면 아무 곳이나 눌러 종료",
        "화면 켜짐 유지",
    ),
    "nb": (
        "Modus for alltid på skjerm er aktivert. Det anbefales å koble til strøm.",
        "Skjerm på-modus",
        "Total nedlastingshastighet",
        "Aktive nedlastinger: {count}",
        "Trykk hvor som helst for å avslutte",
        "Hold skjermen på",
    ),
    "nl": (
        "Scherm-aan-modus ingeschakeld. Aansluiten op stroom wordt aanbevolen.",
        "Scherm aan-modus",
        "Totale downloadsnelheid",
        "Actieve downloads: {count}",
        "Tik ergens om te stoppen",
        "Scherm aan laten",
    ),
    "pl": (
        "Tryb niegasnącego ekranu włączony. Zalecane podłączenie do zasilania.",
        "Tryb aktywnego ekranu",
        "Łączna prędkość pobierania",
        "Aktywne pobierania: {count}",
        "Dotknij dowolnego miejsca, aby wyjść",
        "Utrzymuj ekran włączony",
    ),
    "pt-BR": (
        "Modo de tela sempre ligada ativado. Recomenda-se conectar à energia.",
        "Modo tela ligada",
        "Velocidade total de download",
        "Downloads ativos: {count}",
        "Toque em qualquer lugar para sair",
        "Manter tela ligada",
    ),
    "ro": (
        "Modul ecran aprins este activat. Se recomandă conectarea la alimentare.",
        "Mod ecran aprins",
        "Viteza totală de descărcare",
        "Descărcări active: {count}",
        "Atinge oriunde pentru a ieși",
        "Menține ecranul aprins",
    ),
    "ru": (
        "Режим «не гасить экран» включён. Рекомендуется подключить питание.",
        "Режим без сна",
        "Общая скорость загрузки",
        "Активных загрузок: {count}",
        "Нажмите в любом месте, чтобы выйти",
        "Не гасить экран",
    ),
    "th": (
        "เปิดโหมดไม่ปิดหน้าจอแล้ว แนะนำให้เชื่อมต่อพลังงาน",
        "โหมดหน้าจอเปิดอยู่",
        "ความเร็วดาวน์โหลดรวม",
        "กำลังดาวน์โหลด: {count}",
        "แตะที่ใดก็ได้เพื่อออก",
        "เปิดหน้าจอไว้",
    ),
    "tr": (
        "Ekran açık kalma modu açıldı. Güce bağlamanız önerilir.",
        "Ekran açık modu",
        "Toplam indirme hızı",
        "Etkin indirmeler: {count}",
        "Çıkmak için ekrana dokunun",
        "Ekranı açık tut",
    ),
    "uk": (
        "Режим «не гасити екран» увімкнено. Рекомендується підключити живлення.",
        "Режим без сну",
        "Загальна швидкість завантаження",
        "Активних завантажень: {count}",
        "Торкніться будь-де, щоб вийти",
        "Не гасити екран",
    ),
    "vi": (
        "Đã bật chế độ giữ màn hình sáng. Nên cắm nguồn điện.",
        "Chế độ giữ màn hình sáng",
        "Tốc độ tải xuống tổng",
        "Đang tải: {count}",
        "Chạm vào bất kỳ đâu để thoát",
        "Giữ màn hình sáng",
    ),
    "zh-CN": (
        "已开启不息屏模式，建议连接电源使用。",
        "不息屏挂机",
        "当前总下载速度",
        "正在下载 {count} 个任务",
        "点击屏幕任意位置退出挂机模式",
        "不息屏挂机",
    ),
    "zh-TW": (
        "已開啟不息屏模式，建議連接電源使用。",
        "不息屏掛機",
        "目前總下載速度",
        "正在下載 {count} 個任務",
        "點擊螢幕任意位置退出掛機模式",
        "不息屏掛機",
    ),
}

for locale, (toast, mode, speed_label, active_tasks, exit_hint, nav_label) in COPY.items():
    path = LOCALES_DIR / locale / "messages.json"
    messages = json.loads(path.read_text(encoding="utf-8"))
    messages["app"]["keep-awake-toast"] = toast
    messages["app"]["keep-awake-mode"] = mode
    messages["app"]["keep-awake-speed-label"] = speed_label
    messages["app"]["keep-awake-active-tasks"] = active_tasks
    messages["app"]["keep-awake-exit-hint"] = exit_hint
    messages["navigation"]["keep-awake"] = nav_label
    path.write_text(json.dumps(messages, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")

print(f"Updated {len(COPY)} locale files.")
