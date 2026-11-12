# Rayload — Android-nedlaster

<!-- language-selector:start -->
<details>
<summary>🌐 Language / 语言 / 語言 / Язык</summary>

[English + 简体中文](../README.md) · [简体中文 · Chinese Simplified](README.zh-CN.md) · [繁體中文 · Chinese Traditional](README.zh-TW.md) · [日本語 · Japanese](README.ja.md) · [한국어 · Korean](README.ko.md) · [Français · French](README.fr.md) · [Deutsch · German](README.de.md) · [Español · Spanish](README.es.md) · [Português · Portuguese (Brazil)](README.pt-BR.md) · [Русский · Russian](README.ru.md) · [Türkçe · Turkish](README.tr.md) · [العربية · Arabic](README.ar.md) · [Български · Bulgarian](README.bg.md) · [Català · Catalan](README.ca.md) · [Ελληνικά · Greek](README.el.md) · [فارسی · Persian](README.fa.md) · [Magyar · Hungarian](README.hu.md) · [हिन्दी · Hindi](README.hi.md) · [Bahasa Indonesia · Indonesian](README.id.md) · [Italiano · Italian](README.it.md) · [**Norsk Bokmål · Norwegian**](README.nb.md) · [Nederlands · Dutch](README.nl.md) · [Polski · Polish](README.pl.md) · [Română · Romanian](README.ro.md) · [ไทย · Thai](README.th.md) · [Українська · Ukrainian](README.uk.md) · [Tiếng Việt · Vietnamese](README.vi.md)

</details>
<!-- language-selector:end -->

> Vi bygger den reneste, raskeste og mest tilpassbare åpen kildekode-nedlasteren for Android. Ingen annonser, ingen telemetri — laget av nerder, for nerder.

Rayload er en uavhengig, uoffisiell Android-port av Rayburst med mobilgrensesnitt, systemdeling, bakgrunnsnedlastinger og lokal Aria2 JSON-RPC.

## v1.0.2 Highlights

- Oppgavelisten er bygget på nytt med uendelig rulling og tolinjers kort i full og kompakt visning.
- Dashbord for skjerm på og hastighetsball festet til kanten er lagt til, med sanntidsstatistikk, masshandlinger og motoromstart.
- Innstillinger kun for skrivebord er fjernet, varsler er lokalisert, og Android-tillatelser og wake lock er forbedret.
- Ikon, dialoger, navigasjon og temaer er finpusset, og APK-størrelsen er redusert.

## Installer den nyeste ARM64-APK-en

Last ned nyeste ARM64-APK fra denne siden: [GitHub Releases](https://github.com/shiwushuifeng/Rayload/releases/latest)

### Quick start

1. Last ned nyeste APK fra Releases og installer den på en ARM64-enhet.
2. Gi tilgang til alle filer og tillat varsler slik at nedlastinger kan kjøre i bakgrunnen.
3. Trykk **+** eller del en lenke fra en nettleser for å starte den første nedlastingen.

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

### Varselet har ingen pause-/fortsett-knapper

Tillat varsler, slå av batterioptimalisering og åpne appen på nytt. Enkelte tilpassede Android-ROM-er skjuler handlinger til forgrunnstjenesten startes på nytt.

### Nedlastinger stopper i bakgrunnen

Utelukk Rayload fra batteri- og dyp søvn-begrensninger. På aggressive ROM-er hjelper det å ha appen åpen eller bruke skjerm-på-modus mens du lader.

### Nettleseren kobler ikke til RPC

Chrome for Android laster ikke skrivebordsutvidelser. Bruk Firefox for Android eller en Chromium-versjon som støtter manuell utvidelsesinstallasjon, og skriv inn adresse og RPC Secret fra **Innstillinger → Avansert**.

### «Appen er ikke installert» vises når jeg bytter build

Release- og Debug-APK-er har forskjellige signaturer. Avinstaller gjeldende versjon før du installerer den andre varianten.

## Roadmap

[View the full roadmap](../ROADMAP.md) — produkt- og utviklingsveikart.

## License

Rayload is licensed under the [GPL-3.0 License](../LICENSE). It is based on
[Rayburst](https://github.com/AnInsomniacy/rayburst) by **AnInsomniacy**, whose
original work is licensed under the MIT License. The bundled Aria2 Next
(`aria2-next`) engine is licensed under **GPL-2.0-or-later**. See
[NOTICE](../NOTICE) for all third-party attributions and license notices.
Rayload is an independent, unofficial port.

> ⭐ Hvis Rayload sparer deg tid, gi prosjektet en stjerne så flere Android-brukere finner det.

> ❤️ Like this project? [Sponsor Rayload](../SPONSOR.md)
