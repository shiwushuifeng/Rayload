# Rayload — Android 下載器

<!-- language-selector:start -->
<details>
<summary>🌐 Language / 语言 / 語言 / Язык</summary>

[English + 简体中文](../README.md) · [简体中文 · Chinese Simplified](README.zh-CN.md) · [**繁體中文 · Chinese Traditional**](README.zh-TW.md) · [日本語 · Japanese](README.ja.md) · [한국어 · Korean](README.ko.md) · [Français · French](README.fr.md) · [Deutsch · German](README.de.md) · [Español · Spanish](README.es.md) · [Português · Portuguese (Brazil)](README.pt-BR.md) · [Русский · Russian](README.ru.md) · [Türkçe · Turkish](README.tr.md) · [العربية · Arabic](README.ar.md) · [Български · Bulgarian](README.bg.md) · [Català · Catalan](README.ca.md) · [Ελληνικά · Greek](README.el.md) · [فارسی · Persian](README.fa.md) · [Magyar · Hungarian](README.hu.md) · [हिन्दी · Hindi](README.hi.md) · [Bahasa Indonesia · Indonesian](README.id.md) · [Italiano · Italian](README.it.md) · [Norsk Bokmål · Norwegian](README.nb.md) · [Nederlands · Dutch](README.nl.md) · [Polski · Polish](README.pl.md) · [Română · Romanian](README.ro.md) · [ไทย · Thai](README.th.md) · [Українська · Ukrainian](README.uk.md) · [Tiếng Việt · Vietnamese](README.vi.md)

</details>
<!-- language-selector:end -->

> 致力打造 Android 平台上最純粹、極速、可自訂的開源下載器。無廣告、無遙測，由極客打造，為極客服務。

Rayload 是 Rayburst 的獨立非官方 Android 移植版，專注於 Android 平台，提供行動介面、系統分享、背景下載與本機 Aria2 JSON-RPC。

## v1.0.2 Highlights

- 任務列表改為無限捲動，並重構為完整與精簡兩種密度的兩行卡片。
- 新增不息屏掛機頁，以及具即時統計、批量控制與引擎重啟的貼邊懸浮球。
- 移除桌面端無效設定、完成通知中文化，並改善 Android 權限與喚醒鎖。
- 最佳化圖示、對話框、導覽與主題系統，同時縮小 APK 體積。

## 安裝最新的 ARM64 APK

從下列頁面下載最新的 ARM64 APK： [GitHub Releases](https://github.com/shiwushuifeng/Rayload/releases/latest)

### Quick start

1. 從 Releases 下載最新 APK 並安裝到 ARM64 裝置。
2. 授予所有檔案存取權限並允許通知，讓背景下載正常運作。
3. 點擊 **+**，或從瀏覽器分享連結，開始第一個下載。

- Minimum: Android 7.0 / API 24
- Target: Android 17 / API 37
- ABI: `arm64-v8a`

## Software requirements

| Software | Required version |
| -------- | ---------------- |
| Node.js | 22 or newer |
| pnpm | `11.5.2` |
| Rust | stable with `aarch64-linux-android` |
| Java | JDK 21 |
| Android SDK | Platform 37 |
| Android Build Tools | 37.0.0 |
| Android NDK | `29.0.14206865` |
| Git | current stable |
| GitHub CLI | optional, only for releases |

Full build and browser integration instructions are available in the
[English README](../README.md).

## Troubleshooting and FAQ

### 通知列沒有暫停或繼續按鈕

請允許通知、關閉電池最佳化並重新開啟應用程式。部分客製化 Android 系統會在前景服務重新啟動前隱藏通知操作。

### 背景下載會中斷

請將 Rayload 排除在電池與深度睡眠限制之外。在限制較嚴格的系統上，保持應用程式開啟或充電時使用不息屏模式會更穩定。

### 瀏覽器無法連線到 RPC

Android 版 Chrome 不支援桌面擴充功能。請使用 Firefox for Android 或支援側載擴充功能的 Chromium，並在**設定 → 進階**中填入位址與 RPC Secret。

### 切換安裝包時顯示「應用程式未安裝」

Release 與 Debug APK 使用不同的簽章。安裝另一個版本前，請先解除安裝目前版本。

## Roadmap

[View the full roadmap](../ROADMAP.md) — 產品與工程開發藍圖.

## License

Rayload is distributed under the [GPL-3.0 License](../LICENSE). It is based on
[Rayburst](https://github.com/AnInsomniacy/rayburst) by **AnInsomniacy**, whose
original work is licensed under the MIT License; see [NOTICE](../NOTICE) for
third-party attributions. Rayload is an independent, unofficial port.

> ⭐ 如果 Rayload 節省了你的時間，請給我們一個 Star，讓更多 Android 使用者發現它。

> ❤️ 喜歡這個項目？歡迎 [贊助與支持 (Sponsor)](../SPONSOR.md)
