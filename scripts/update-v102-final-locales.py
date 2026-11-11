#!/usr/bin/env python3
"""Apply the v1.0.2 empty-state and torrent-picker copy to every locale."""

import json
from pathlib import Path

LOCALES_DIR = Path("src/shared/locales")

EMPTY_HINT = {
    "ar": "اضغط على زر + في أسفل اليمين لإنشاء مهمة، أو شارك رابطًا من المتصفح إلى Rayload.",
    "bg": "Натиснете бутона + долу вдясно, за да създадете задача, или споделете връзка от браузъра към Rayload.",
    "ca": "Toca el botó + a la part inferior dreta per crear una tasca, o comparteix un enllaç del navegador amb Rayload.",
    "de": "Tippe unten rechts auf +, um eine Aufgabe zu erstellen, oder teile einen Link aus dem Browser mit Rayload.",
    "el": "Πατήστε το κουμπί + κάτω δεξιά για νέα εργασία ή μοιραστείτε έναν σύνδεσμο από το πρόγραμμα περιήγησης στο Rayload.",
    "en-US": "Tap the + button at the bottom right to create a task, or share a link from your browser to Rayload.",
    "es": "Toca el botón + en la esquina inferior derecha para crear una tarea, o comparte un enlace desde el navegador a Rayload.",
    "fa": "برای ایجاد وظیفه، دکمه + در پایین سمت راست را بزنید یا پیوندی را از مرورگر با Rayload به اشتراک بگذارید.",
    "fr": "Touchez le bouton + en bas à droite pour créer une tâche, ou partagez un lien depuis votre navigateur vers Rayload.",
    "hi": "कार्य बनाने के लिए नीचे दाईं ओर + बटन दबाएँ, या ब्राउज़र से लिंक Rayload के साथ साझा करें।",
    "hu": "Új feladat létrehozásához koppintson a jobb alsó + gombra, vagy osszon meg egy hivatkozást a böngészőjéből a Rayloaddal.",
    "id": "Ketuk tombol + di kanan bawah untuk membuat tugas, atau bagikan tautan dari browser ke Rayload.",
    "it": "Tocca il pulsante + in basso a destra per creare un'attività, oppure condividi un link dal browser a Rayload.",
    "ja": "右下の + ボタンでタスクを作成するか、ブラウザからリンクを Rayload に共有してください。",
    "ko": "오른쪽 아래 + 버튼을 눌러 작업을 만들거나 브라우저에서 링크를 Rayload로 공유하세요.",
    "nb": "Trykk på + nederst til høyre for å opprette en oppgave, eller del en lenke fra nettleseren til Rayload.",
    "nl": "Tik op de +-knop rechtsonder om een taak te maken, of deel een link vanuit je browser met Rayload.",
    "pl": "Dotknij przycisku + w prawym dolnym rogu, aby utworzyć zadanie, lub udostępnij link z przeglądarki do Rayload.",
    "pt-BR": "Toque no botão + no canto inferior direito para criar uma tarefa, ou compartilhe um link do navegador com o Rayload.",
    "ro": "Atinge butonul + din dreapta jos pentru a crea o sarcină sau partajează un link din browser către Rayload.",
    "ru": "Нажмите кнопку + в правом нижнем углу, чтобы создать задачу, или поделитесь ссылкой из браузера в Rayload.",
    "th": "แตะปุ่ม + ที่มุมล่างขวาเพื่อสร้างงาน หรือแชร์ลิงก์จากเบราว์เซอร์ไปยัง Rayload",
    "tr": "Görev oluşturmak için sağ alttaki + düğmesine dokunun veya tarayıcınızdan Rayload'a bağlantı paylaşın.",
    "uk": "Натисніть кнопку + у правому нижньому куті, щоб створити завдання, або поділіться посиланням із браузера в Rayload.",
    "vi": "Nhấn nút + ở góc dưới bên phải để tạo tác vụ, hoặc chia sẻ liên kết từ trình duyệt tới Rayload.",
    "zh-CN": "点击右下角 + 号新建任务，或从浏览器分享链接至 Rayload",
    "zh-TW": "點擊右下角 + 號新建任務，或從瀏覽器分享連結至 Rayload",
}

SELECT_TORRENT = {
    "ar": "انقر لاختيار ملف .torrent محلي",
    "bg": "Щракнете, за да изберете локален .torrent файл",
    "ca": "Fes clic per triar un fitxer .torrent local",
    "de": "Klicken, um eine lokale .torrent-Datei auszuwählen",
    "el": "Πατήστε για να επιλέξετε ένα τοπικό αρχείο .torrent",
    "en-US": "Click to choose a local .torrent file",
    "es": "Haz clic para elegir un archivo .torrent local",
    "fa": "برای انتخاب یک فایل .torrent محلی کلیک کنید",
    "fr": "Cliquez pour choisir un fichier .torrent local",
    "hi": "स्थानीय .torrent फ़ाइल चुनने के लिए क्लिक करें",
    "hu": "Kattintson egy helyi .torrent fájl kiválasztásához",
    "id": "Klik untuk memilih file .torrent lokal",
    "it": "Fai clic per scegliere un file .torrent locale",
    "ja": "クリックしてローカルの .torrent ファイルを選択",
    "ko": "클릭하여 로컬 .torrent 파일 선택",
    "nb": "Klikk for å velge en lokal .torrent-fil",
    "nl": "Klik om een lokaal .torrent-bestand te kiezen",
    "pl": "Kliknij, aby wybrać lokalny plik .torrent",
    "pt-BR": "Clique para escolher um arquivo .torrent local",
    "ro": "Faceți clic pentru a alege un fișier .torrent local",
    "ru": "Нажмите, чтобы выбрать локальный файл .torrent",
    "th": "คลิกเพื่อเลือกไฟล์ .torrent ในเครื่อง",
    "tr": "Yerel bir .torrent dosyası seçmek için tıklayın",
    "uk": "Натисніть, щоб вибрати локальний файл .torrent",
    "vi": "Nhấp để chọn tệp .torrent cục bộ",
    "zh-CN": "点击选择本地 .torrent 文件",
    "zh-TW": "點擊選擇本機 .torrent 檔案",
}

for locale, empty_hint in EMPTY_HINT.items():
    path = LOCALES_DIR / locale / "messages.json"
    messages = json.loads(path.read_text(encoding="utf-8"))
    messages["task"]["empty-hint"] = empty_hint
    messages["task"]["select-torrent"] = SELECT_TORRENT[locale]
    path.write_text(json.dumps(messages, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")

print(f"Updated {len(EMPTY_HINT)} locale files.")
