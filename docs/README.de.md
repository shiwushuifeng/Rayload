# Rayload — Android-Downloader

<!-- language-selector:start -->
<details>
<summary>🌐 Language / 语言 / 語言 / Язык</summary>

[English + 简体中文](../README.md) · [简体中文 · Chinese Simplified](README.zh-CN.md) · [繁體中文 · Chinese Traditional](README.zh-TW.md) · [日本語 · Japanese](README.ja.md) · [한국어 · Korean](README.ko.md) · [Français · French](README.fr.md) · [**Deutsch · German**](README.de.md) · [Español · Spanish](README.es.md) · [Português · Portuguese (Brazil)](README.pt-BR.md) · [Русский · Russian](README.ru.md) · [Türkçe · Turkish](README.tr.md) · [العربية · Arabic](README.ar.md) · [Български · Bulgarian](README.bg.md) · [Català · Catalan](README.ca.md) · [Ελληνικά · Greek](README.el.md) · [فارسی · Persian](README.fa.md) · [Magyar · Hungarian](README.hu.md) · [हिन्दी · Hindi](README.hi.md) · [Bahasa Indonesia · Indonesian](README.id.md) · [Italiano · Italian](README.it.md) · [Norsk Bokmål · Norwegian](README.nb.md) · [Nederlands · Dutch](README.nl.md) · [Polski · Polish](README.pl.md) · [Română · Romanian](README.ro.md) · [ไทย · Thai](README.th.md) · [Українська · Ukrainian](README.uk.md) · [Tiếng Việt · Vietnamese](README.vi.md)

</details>
<!-- language-selector:end -->

> Wir entwickeln den klarsten, schnellsten und anpassbarsten Open-Source-Downloader für Android. Keine Werbung, keine Telemetrie — von Geeks für Geeks.

Rayload ist ein unabhängiger, inoffizieller Android-Port von Rayburst mit mobiler Oberfläche, Systemfreigabe, Hintergrunddownloads und lokalem Aria2 JSON-RPC.

## v1.0.2 Highlights

- Aufgabenliste mit Endlos-Scrollen und zweizeiligen Karten in voller und kompakter Dichte neu aufgebaut.
- Bildschirm-an-Dashboard und randseitiger Geschwindigkeitsball mit Live-Statistik, Sammelaktionen und Engine-Neustart ergänzt.
- Desktop-only-Einstellungen entfernt, Benachrichtigung lokalisiert und Android-Berechtigungen sowie Wake-Lock-Verhalten verbessert.
- Symbol, Dialoge, Navigation und Themes überarbeitet und die APK-Größe reduziert.

## Aktuelle ARM64-APK installieren

Lade die neueste ARM64-APK von dieser Seite herunter: [GitHub Releases](https://github.com/shiwushuifeng/Rayload/releases/latest)

### Quick start

1. Lade die neueste APK von Releases herunter und installiere sie auf einem ARM64-Gerät.
2. Erlaube Dateizugriff und Benachrichtigungen, damit Downloads im Hintergrund laufen.
3. Tippe auf **+** oder teile einen Link aus einem Browser, um den ersten Download zu starten.

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

### Die Benachrichtigung hat keine Pause-/Fortsetzen-Schaltfläche

Erlaube Benachrichtigungen, deaktiviere die Akku-Optimierung und öffne die App erneut. Manche Android-ROMs blenden Aktionen bis zum Neustart des Vordergrunddienstes aus.

### Downloads stoppen im Hintergrund

Nimm Rayload von Akku- und Tiefschlaf-Beschränkungen aus. Bei aggressiven ROMs hilft es, die App geöffnet zu lassen oder während des Ladens den Bildschirm-an-Modus zu verwenden.

### Mein Browser verbindet sich nicht mit RPC

Chrome für Android lädt keine Desktop-Erweiterungen. Nutze Firefox für Android oder ein Chromium mit manueller Erweiterungsinstallation und gib Adresse sowie RPC Secret unter **Einstellungen → Erweitert** ein.

### Beim Wechsel der Build-Variante erscheint „App nicht installiert“

Release- und Debug-APKs haben unterschiedliche Signaturen. Deinstalliere die aktuelle Version, bevor du die andere Variante installierst.

## Roadmap

[View the full roadmap](../ROADMAP.md) — Produkt- und Engineering-Roadmap.

## License

Rayload is distributed under the [GPL-3.0 License](../LICENSE). It is based on
[Rayburst](https://github.com/AnInsomniacy/rayburst) by **AnInsomniacy**, whose
original work is licensed under the MIT License; see [NOTICE](../NOTICE) for
third-party attributions. Rayload is an independent, unofficial port.

> ⭐ Wenn Rayload dir Zeit spart, gib dem Projekt einen Stern — so finden mehr Android-Nutzer dazu.

> ❤️ Like this project? [Sponsor Rayload](../SPONSOR.md)
