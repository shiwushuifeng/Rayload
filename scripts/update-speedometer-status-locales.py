#!/usr/bin/env python3
"""Add the live status and connection quality labels to every locale."""

import json
from pathlib import Path

LOCALES_DIR = Path("src/shared/locales")

KEYS = (
    "speedometer-live-status",
    "speedometer-connection-quality",
    "speedometer-quality-idle",
    "speedometer-quality-excellent",
    "speedometer-quality-good",
    "speedometer-quality-fair",
    "speedometer-quality-weak",
    "speedometer-online",
    "speedometer-offline",
    "speedometer-wifi",
    "speedometer-cellular",
)

TRANSLATIONS = {
    "ar": ("الحالة المباشرة", "جودة الاتصال", "خامل", "ممتاز", "جيد", "متوسط", "ضعيف", "متصل", "غير متصل", "Wi-Fi", "شبكة الجوال"),
    "bg": ("Актуален статус", "Качество на връзката", "Неактивно", "Отлично", "Добро", "Средно", "Слабо", "Онлайн", "Офлайн", "Wi-Fi", "Мобилна мрежа"),
    "ca": ("Estat en directe", "Qualitat de la connexió", "Inactiu", "Excel·lent", "Bo", "Acceptable", "Feble", "En línia", "Fora de línia", "Wi-Fi", "Xarxa mòbil"),
    "de": ("Live-Status", "Verbindungsqualität", "Leerlauf", "Ausgezeichnet", "Gut", "Mittel", "Schwach", "Online", "Offline", "WLAN", "Mobilfunknetz"),
    "el": ("Ζωντανή κατάσταση", "Ποιότητα σύνδεσης", "Αδρανές", "Εξαιρετική", "Καλή", "Μέτρια", "Αδύναμη", "Σε σύνδεση", "Εκτός σύνδεσης", "Wi-Fi", "Δίκτυο κινητής"),
    "en-US": ("Live status", "Link quality", "Idle", "Excellent", "Good", "Fair", "Weak", "Online", "Offline", "Wi-Fi", "Mobile network"),
    "es": ("Estado en directo", "Calidad de conexión", "Inactivo", "Excelente", "Buena", "Regular", "Débil", "En línea", "Sin conexión", "Wi-Fi", "Red móvil"),
    "fa": ("وضعیت زنده", "کیفیت اتصال", "بیکار", "عالی", "خوب", "متوسط", "ضعیف", "آنلاین", "آفلاین", "Wi-Fi", "شبکه موبایل"),
    "fr": ("État en direct", "Qualité de connexion", "Inactif", "Excellente", "Bonne", "Moyenne", "Faible", "En ligne", "Hors ligne", "Wi-Fi", "Réseau mobile"),
    "hi": ("लाइव स्थिति", "कनेक्शन गुणवत्ता", "निष्क्रिय", "उत्कृष्ट", "अच्छा", "सामान्य", "कमजोर", "ऑनलाइन", "ऑफ़लाइन", "Wi-Fi", "मोबाइल नेटवर्क"),
    "hu": ("Élő állapot", "Kapcsolat minősége", "Tétlen", "Kiváló", "Jó", "Közepes", "Gyenge", "Online", "Offline", "Wi-Fi", "Mobilhálózat"),
    "id": ("Status langsung", "Kualitas koneksi", "Menganggur", "Sangat baik", "Baik", "Cukup", "Lemah", "Daring", "Luring", "Wi-Fi", "Jaringan seluler"),
    "it": ("Stato in tempo reale", "Qualità connessione", "Inattivo", "Eccellente", "Buona", "Discreta", "Debole", "Online", "Offline", "Wi-Fi", "Rete mobile"),
    "ja": ("ライブ状態", "接続品質", "アイドル", "最高", "良好", "普通", "弱い", "オンライン", "オフライン", "Wi-Fi", "モバイルネットワーク"),
    "ko": ("실시간 상태", "연결 품질", "유휴", "매우 좋음", "좋음", "보통", "약함", "온라인", "오프라인", "Wi-Fi", "모바일 네트워크"),
    "nb": ("Sanntidsstatus", "Tilkoblingskvalitet", "Inaktiv", "Utmerket", "God", "Middels", "Svak", "På nett", "Frakoblet", "Wi-Fi", "Mobilnett"),
    "nl": ("Livestatus", "Verbindingskwaliteit", "Inactief", "Uitstekend", "Goed", "Gemiddeld", "Zwak", "Online", "Offline", "Wi-Fi", "Mobiel netwerk"),
    "pl": ("Stan na żywo", "Jakość połączenia", "Bezczynne", "Doskonała", "Dobra", "Przeciętna", "Słaba", "Online", "Offline", "Wi-Fi", "Sieć komórkowa"),
    "pt-BR": ("Status ao vivo", "Qualidade da conexão", "Ocioso", "Excelente", "Boa", "Regular", "Fraca", "Online", "Offline", "Wi-Fi", "Rede móvel"),
    "ro": ("Stare live", "Calitatea conexiunii", "Inactiv", "Excelentă", "Bună", "Medie", "Slabă", "Online", "Offline", "Wi-Fi", "Rețea mobilă"),
    "ru": ("Текущее состояние", "Качество соединения", "Простой", "Отличное", "Хорошее", "Среднее", "Слабое", "В сети", "Не в сети", "Wi-Fi", "Мобильная сеть"),
    "th": ("สถานะสด", "คุณภาพการเชื่อมต่อ", "ว่าง", "ยอดเยี่ยม", "ดี", "ปานกลาง", "อ่อน", "ออนไลน์", "ออฟไลน์", "Wi-Fi", "เครือข่ายมือถือ"),
    "tr": ("Canlı durum", "Bağlantı kalitesi", "Boşta", "Mükemmel", "İyi", "Orta", "Zayıf", "Çevrimiçi", "Çevrimdışı", "Wi-Fi", "Mobil ağ"),
    "uk": ("Поточний стан", "Якість з'єднання", "Неактивно", "Відмінна", "Добра", "Середня", "Слабка", "Онлайн", "Офлайн", "Wi-Fi", "Мобільна мережа"),
    "vi": ("Trạng thái trực tiếp", "Chất lượng kết nối", "Nhàn rỗi", "Xuất sắc", "Tốt", "Trung bình", "Yếu", "Trực tuyến", "Ngoại tuyến", "Wi-Fi", "Mạng di động"),
    "zh-CN": ("实时状态", "连接质量", "空闲", "优秀", "良好", "一般", "较弱", "在线", "离线", "Wi-Fi", "移动网络"),
    "zh-TW": ("即時狀態", "連線品質", "閒置", "優秀", "良好", "一般", "較弱", "線上", "離線", "Wi-Fi", "行動網路"),
}

for locale, values in TRANSLATIONS.items():
    path = LOCALES_DIR / locale / "messages.json"
    messages = json.loads(path.read_text(encoding="utf-8"))
    app = messages["app"]
    for key, value in zip(KEYS, values):
        app[key] = value
    path.write_text(json.dumps(messages, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")

print(f"Updated {len(TRANSLATIONS)} locale files.")
