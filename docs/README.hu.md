# Rayload — Android letöltőkezelő

<!-- language-selector:start -->
<details>
<summary>🌐 Language / 语言 / 語言 / Язык</summary>

[English + 简体中文](../README.md) · [简体中文 · Chinese Simplified](README.zh-CN.md) · [繁體中文 · Chinese Traditional](README.zh-TW.md) · [日本語 · Japanese](README.ja.md) · [한국어 · Korean](README.ko.md) · [Français · French](README.fr.md) · [Deutsch · German](README.de.md) · [Español · Spanish](README.es.md) · [Português · Portuguese (Brazil)](README.pt-BR.md) · [Русский · Russian](README.ru.md) · [Türkçe · Turkish](README.tr.md) · [العربية · Arabic](README.ar.md) · [Български · Bulgarian](README.bg.md) · [Català · Catalan](README.ca.md) · [Ελληνικά · Greek](README.el.md) · [فارسی · Persian](README.fa.md) · [**Magyar · Hungarian**](README.hu.md) · [हिन्दी · Hindi](README.hi.md) · [Bahasa Indonesia · Indonesian](README.id.md) · [Italiano · Italian](README.it.md) · [Norsk Bokmål · Norwegian](README.nb.md) · [Nederlands · Dutch](README.nl.md) · [Polski · Polish](README.pl.md) · [Română · Romanian](README.ro.md) · [ไทย · Thai](README.th.md) · [Українська · Ukrainian](README.uk.md) · [Tiếng Việt · Vietnamese](README.vi.md)

</details>
<!-- language-selector:end -->

> A legletisztultabb, leggyorsabb és legtestreszabhatóbb nyílt forráskódú Android-letöltőt építjük. Reklám nélkül, telemetria nélkül — geekektől geekeknek.

A Rayload a Rayburst független, nem hivatalos Android-portja mobil felülettel, rendszerszintű megosztással, háttérletöltésekkel és helyi Aria2 JSON-RPC-vel.

## v1.0.2 Highlights

- A feladatlista végtelen görgetéssel és kétsoros kártyákkal, teljes és kompakt nézetben készült újra.
- Új ébren tartó irányítópult és szélre rögzíthető sebességjelző élő statisztikával, tömeges műveletekkel és motor-újraindítással.
- Az asztali beállítások eltávolítva, az értesítés lokalizálva, az Android-engedélyek és a wake lock viselkedése javítva.
- Finomított ikon, párbeszédek, navigáció és témák, kisebb APK-mérettel.

## A legújabb ARM64 APK telepítése

Töltsd le a legújabb ARM64 APK-t erről az oldalról: [GitHub Releases](https://github.com/shiwushuifeng/Rayload/releases/latest)

### Quick start

1. Töltsd le a legújabb APK-t a Releases oldalról, és telepítsd ARM64-es eszközre.
2. Engedélyezd az összes fájl hozzáférését és az értesítéseket a háttérletöltésekhez.
3. Koppints a **+** gombra, vagy ossz meg egy hivatkozást böngészőből az első letöltéshez.

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

### Az értesítésen nincs szüneteltetés/folytatás gomb

Engedélyezd az értesítéseket, kapcsold ki az akkumulátor-optimalizálást, majd nyisd újra az alkalmazást. Egyes egyedi ROM-ok az előtér-szolgáltatás újraindításáig elrejtik a műveleteket.

### A letöltések leállnak a háttérben

Zárd ki a Rayloadot az akkumulátor- és mélyalvás-korlátozásokból. Agresszív ROM-okon segít, ha nyitva hagyod az appot, vagy töltés közben ébren tartó módot használsz.

### A böngésző nem csatlakozik az RPC-hez

Az Androidos Chrome nem tölt be asztali kiegészítőket. Használj Firefoxot Androidra vagy kézi kiegészítő-telepítést támogató Chromiumot, és add meg a címet és az RPC Secretet a **Beállítások → Speciális** alatt.

### Buildváltáskor „Az alkalmazás nincs telepítve” jelenik meg

A Release és Debug APK aláírása eltér. A másik változat telepítése előtt távolítsd el a jelenlegit.

## Roadmap

[View the full roadmap](../ROADMAP.md) — termék- és mérnöki ütemterv.

## License

Rayload is distributed under the [GPL-3.0 License](../LICENSE). It is based on
[Rayburst](https://github.com/AnInsomniacy/rayburst) by **AnInsomniacy**, whose
original work is licensed under the MIT License; see [NOTICE](../NOTICE) for
third-party attributions. Rayload is an independent, unofficial port.

> ⭐ Ha a Rayload időt spórol neked, adj egy csillagot, hogy több Android-felhasználó megtalálja.

> ❤️ Like this project? [Sponsor Rayload](../SPONSOR.md)
