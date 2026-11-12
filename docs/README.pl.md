# Rayload — menedżer pobierania dla Androida

<!-- language-selector:start -->
<details>
<summary>🌐 Language / 语言 / 語言 / Язык</summary>

[English + 简体中文](../README.md) · [简体中文 · Chinese Simplified](README.zh-CN.md) · [繁體中文 · Chinese Traditional](README.zh-TW.md) · [日本語 · Japanese](README.ja.md) · [한국어 · Korean](README.ko.md) · [Français · French](README.fr.md) · [Deutsch · German](README.de.md) · [Español · Spanish](README.es.md) · [Português · Portuguese (Brazil)](README.pt-BR.md) · [Русский · Russian](README.ru.md) · [Türkçe · Turkish](README.tr.md) · [العربية · Arabic](README.ar.md) · [Български · Bulgarian](README.bg.md) · [Català · Catalan](README.ca.md) · [Ελληνικά · Greek](README.el.md) · [فارسی · Persian](README.fa.md) · [Magyar · Hungarian](README.hu.md) · [हिन्दी · Hindi](README.hi.md) · [Bahasa Indonesia · Indonesian](README.id.md) · [Italiano · Italian](README.it.md) · [Norsk Bokmål · Norwegian](README.nb.md) · [Nederlands · Dutch](README.nl.md) · [**Polski · Polish**](README.pl.md) · [Română · Romanian](README.ro.md) · [ไทย · Thai](README.th.md) · [Українська · Ukrainian](README.uk.md) · [Tiếng Việt · Vietnamese](README.vi.md)

</details>
<!-- language-selector:end -->

> Tworzymy najczystszy, najszybszy i najbardziej konfigurowalny otwartoźródłowy menedżer pobierania na Androida. Bez reklam, bez telemetrii — od geeków dla geeków.

Rayload to niezależny, nieoficjalny port Rayburst na Androida z mobilnym interfejsem, udostępnianiem systemowym, pobieraniem w tle i lokalnym Aria2 JSON-RPC.

## v1.0.2 Highlights

- Lista zadań została przebudowana z nieskończonym przewijaniem i dwuwierszowymi kartami w trybie pełnym i kompaktowym.
- Dodano pulpit utrzymywania ekranu i przypiętą do krawędzi kulę prędkości z bieżącymi statystykami, akcjami zbiorczymi i restartem silnika.
- Usunięto ustawienia tylko dla komputerów, przetłumaczono powiadomienie i poprawiono uprawnienia Androida oraz działanie wake lock.
- Dopracowano ikonę, okna dialogowe, nawigację i motywy oraz zmniejszono rozmiar APK.

## Zainstaluj najnowszy plik APK ARM64

Pobierz najnowszy APK ARM64 z tej strony: [GitHub Releases](https://github.com/shiwushuifeng/Rayload/releases/latest)

### Quick start

1. Pobierz najnowszy APK z Releases i zainstaluj go na urządzeniu ARM64.
2. Zezwól na dostęp do wszystkich plików i powiadomienia, aby pobieranie działało w tle.
3. Dotknij **+** lub udostępnij link z przeglądarki, aby rozpocząć pierwsze pobieranie.

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

### Powiadomienie nie ma przycisków pauzy/wznowienia

Zezwól na powiadomienia, wyłącz optymalizację baterii i otwórz aplikację ponownie. Niektóre zmodyfikowane ROM-y Androida ukrywają akcje do czasu ponownego uruchomienia usługi pierwszoplanowej.

### Pobieranie zatrzymuje się w tle

Wyklucz Rayload z ograniczeń baterii i głębokiego uśpienia. W agresywnych ROM-ach pomaga pozostawienie aplikacji otwartej lub tryb stałego ekranu podczas ładowania.

### Przeglądarka nie łączy się z RPC

Chrome na Androida nie obsługuje rozszerzeń desktopowych. Użyj Firefoksa na Androida lub Chromium z ręczną instalacją rozszerzeń i wpisz adres oraz RPC Secret w **Ustawienia → Zaawansowane**.

### Przy zmianie kompilacji pojawia się „Aplikacja nie została zainstalowana”

APK Release i Debug mają różne podpisy. Przed instalacją drugiego wariantu odinstaluj bieżącą wersję.

## Roadmap

[View the full roadmap](../ROADMAP.md) — mapa drogowa produktu i inżynierii.

## License

Rayload is distributed under the [GPL-3.0 License](../LICENSE). It is based on
[Rayburst](https://github.com/AnInsomniacy/rayburst) by **AnInsomniacy**, whose
original work is licensed under the MIT License; see [NOTICE](../NOTICE) for
third-party attributions. Rayload is an independent, unofficial port.

> ⭐ Jeśli Rayload oszczędza Twój czas, zostaw gwiazdkę, aby więcej użytkowników Androida go odkryło.

> ❤️ Like this project? [Sponsor Rayload](../SPONSOR.md)
