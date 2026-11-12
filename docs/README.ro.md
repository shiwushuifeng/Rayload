# Rayload — manager de descărcări pentru Android

<!-- language-selector:start -->
<details>
<summary>🌐 Language / 语言 / 語言 / Язык</summary>

[English + 简体中文](../README.md) · [简体中文 · Chinese Simplified](README.zh-CN.md) · [繁體中文 · Chinese Traditional](README.zh-TW.md) · [日本語 · Japanese](README.ja.md) · [한국어 · Korean](README.ko.md) · [Français · French](README.fr.md) · [Deutsch · German](README.de.md) · [Español · Spanish](README.es.md) · [Português · Portuguese (Brazil)](README.pt-BR.md) · [Русский · Russian](README.ru.md) · [Türkçe · Turkish](README.tr.md) · [العربية · Arabic](README.ar.md) · [Български · Bulgarian](README.bg.md) · [Català · Catalan](README.ca.md) · [Ελληνικά · Greek](README.el.md) · [فارسی · Persian](README.fa.md) · [Magyar · Hungarian](README.hu.md) · [हिन्दी · Hindi](README.hi.md) · [Bahasa Indonesia · Indonesian](README.id.md) · [Italiano · Italian](README.it.md) · [Norsk Bokmål · Norwegian](README.nb.md) · [Nederlands · Dutch](README.nl.md) · [Polski · Polish](README.pl.md) · [**Română · Romanian**](README.ro.md) · [ไทย · Thai](README.th.md) · [Українська · Ukrainian](README.uk.md) · [Tiếng Việt · Vietnamese](README.vi.md)

</details>
<!-- language-selector:end -->

> Construim cel mai curat, rapid și personalizabil downloader open source pentru Android. Fără reclame, fără telemetrie — creat de geek-i pentru geek-i.

Rayload este un port Android independent și neoficial al Rayburst, cu interfață mobilă, partajare prin sistem, descărcări în fundal și Aria2 JSON-RPC local.

## v1.0.2 Highlights

- Lista de activități a fost reconstruită cu derulare infinită și carduri pe două rânduri în modul complet și compact.
- Am adăugat panoul cu ecranul pornit și bila de viteză atașată de margine, cu statistici live, acțiuni în masă și repornirea motorului.
- Am eliminat setările doar pentru desktop, am localizat notificarea și am îmbunătățit permisiunile Android și wake lock.
- Am rafinat pictograma, dialogurile, navigarea și temele și am redus dimensiunea APK-ului.

## Instalează cel mai recent APK ARM64

Descarcă cel mai recent APK ARM64 de pe această pagină: [GitHub Releases](https://github.com/shiwushuifeng/Rayload/releases/latest)

### Quick start

1. Descarcă cel mai recent APK din Releases și instalează-l pe un dispozitiv ARM64.
2. Permite accesul la toate fișierele și notificările pentru descărcări în fundal.
3. Apasă **+** sau partajează un link din orice browser pentru prima descărcare.

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

### Notificarea nu are butoane de pauză/reluare

Permite notificările, dezactivează optimizarea bateriei și redeschide aplicația. Unele ROM-uri Android personalizate ascund acțiunile până la repornirea serviciului din prim-plan.

### Descărcările se opresc în fundal

Exclude Rayload din restricțiile de baterie și somn profund. Pe ROM-urile agresive ajută să lași aplicația deschisă sau să folosești modul ecran pornit în timpul încărcării.

### Browserul nu se conectează la RPC

Chrome pentru Android nu încarcă extensii desktop. Folosește Firefox pentru Android sau un Chromium care permite instalarea manuală a extensiilor și introdu adresa și RPC Secret din **Setări → Avansat**.

### Apare „Aplicația nu este instalată” la schimbarea build-ului

APK-urile Release și Debug folosesc semnături diferite. Dezinstalează versiunea curentă înainte de a instala cealaltă variantă.

## Roadmap

[View the full roadmap](../ROADMAP.md) — foaie de parcurs pentru produs și inginerie.

## License

Rayload is distributed under the [GPL-3.0 License](../LICENSE). It is based on
[Rayburst](https://github.com/AnInsomniacy/rayburst) by **AnInsomniacy**, whose
original work is licensed under the MIT License; see [NOTICE](../NOTICE) for
third-party attributions. Rayload is an independent, unofficial port.

> ⭐ Dacă Rayload îți economisește timp, oferă o stea pentru ca mai mulți utilizatori Android să îl descopere.

> ❤️ Like this project? [Sponsor Rayload](../SPONSOR.md)
