# Rayload — Android-downloader

<!-- language-selector:start -->
<details>
<summary>🌐 Language / 语言 / 語言 / Язык</summary>

[English + 简体中文](../README.md) · [简体中文 · Chinese Simplified](README.zh-CN.md) · [繁體中文 · Chinese Traditional](README.zh-TW.md) · [日本語 · Japanese](README.ja.md) · [한국어 · Korean](README.ko.md) · [Français · French](README.fr.md) · [Deutsch · German](README.de.md) · [Español · Spanish](README.es.md) · [Português · Portuguese (Brazil)](README.pt-BR.md) · [Русский · Russian](README.ru.md) · [Türkçe · Turkish](README.tr.md) · [العربية · Arabic](README.ar.md) · [Български · Bulgarian](README.bg.md) · [Català · Catalan](README.ca.md) · [Ελληνικά · Greek](README.el.md) · [فارسی · Persian](README.fa.md) · [Magyar · Hungarian](README.hu.md) · [हिन्दी · Hindi](README.hi.md) · [Bahasa Indonesia · Indonesian](README.id.md) · [Italiano · Italian](README.it.md) · [Norsk Bokmål · Norwegian](README.nb.md) · [**Nederlands · Dutch**](README.nl.md) · [Polski · Polish](README.pl.md) · [Română · Romanian](README.ro.md) · [ไทย · Thai](README.th.md) · [Українська · Ukrainian](README.uk.md) · [Tiếng Việt · Vietnamese](README.vi.md)

</details>
<!-- language-selector:end -->

> We bouwen de schoonste, snelste en meest aanpasbare open-source downloader voor Android. Geen advertenties, geen telemetrie — gemaakt door geeks, voor geeks.

Rayload is een onafhankelijke, onofficiële Android-port van Rayburst met een mobiele interface, systeem delen, achtergronddownloads en lokale Aria2 JSON-RPC.

## v1.0.2 Highlights

- Takenlijst opnieuw opgebouwd met oneindig scrollen en tweeregelige kaarten in volledige en compacte modus.
- Scherm-aan-dashboard en aan de rand vastklikkende snelheidsball toegevoegd, met live statistieken, bulkacties en engine-herstart.
- Alleen-desktopinstellingen verwijderd, melding gelokaliseerd en Android-machtigingen en wake lock verbeterd.
- Pictogram, dialogen, navigatie en thema's verfijnd, met een kleinere APK.

## Nieuwste ARM64-APK installeren

Download de nieuwste ARM64-APK via deze pagina: [GitHub Releases](https://github.com/shiwushuifeng/Rayload/releases/latest)

### Quick start

1. Download de nieuwste APK via Releases en installeer deze op een ARM64-apparaat.
2. Geef toegang tot alle bestanden en sta meldingen toe voor downloads op de achtergrond.
3. Tik op **+** of deel een link vanuit een browser om je eerste download te starten.

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

### De melding heeft geen pauze-/hervatknoppen

Sta meldingen toe, schakel batterijoptimalisatie uit en open de app opnieuw. Sommige aangepaste Android-ROM's verbergen acties tot de voorgrondservice opnieuw start.

### Downloads stoppen op de achtergrond

Sluit Rayload uit van batterij- en diepslaapbeperkingen. Op agressieve ROM's helpt het om de app open te houden of scherm-aan-modus te gebruiken tijdens het opladen.

### Mijn browser kan geen verbinding maken met RPC

Chrome voor Android laadt geen desktopplug-ins. Gebruik Firefox voor Android of een Chromium-build die handmatige plug-ininstallatie ondersteunt, en vul het adres en RPC Secret in via **Instellingen → Geavanceerd**.

### Bij het wisselen van build verschijnt ‘App niet geïnstalleerd’

Release- en Debug-APK's hebben verschillende handtekeningen. Verwijder de huidige versie voordat je de andere variant installeert.

## Roadmap

[View the full roadmap](../ROADMAP.md) — product- en engineeringroadmap.

## License

Rayload is distributed under the [GPL-3.0 License](../LICENSE). It is based on
[Rayburst](https://github.com/AnInsomniacy/rayburst) by **AnInsomniacy**, whose
original work is licensed under the MIT License; see [NOTICE](../NOTICE) for
third-party attributions. Rayload is an independent, unofficial port.

> ⭐ Als Rayload je tijd bespaart, geef het een ster zodat meer Android-gebruikers het ontdekken.

> ❤️ Like this project? [Sponsor Rayload](../SPONSOR.md)
