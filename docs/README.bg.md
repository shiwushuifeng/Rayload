# Rayload — Android мениджър за изтегляния

<!-- language-selector:start -->
<details>
<summary>🌐 Language / 语言 / 語言 / Язык</summary>

[English + 简体中文](../README.md) · [简体中文 · Chinese Simplified](README.zh-CN.md) · [繁體中文 · Chinese Traditional](README.zh-TW.md) · [日本語 · Japanese](README.ja.md) · [한국어 · Korean](README.ko.md) · [Français · French](README.fr.md) · [Deutsch · German](README.de.md) · [Español · Spanish](README.es.md) · [Português · Portuguese (Brazil)](README.pt-BR.md) · [Русский · Russian](README.ru.md) · [Türkçe · Turkish](README.tr.md) · [العربية · Arabic](README.ar.md) · [**Български · Bulgarian**](README.bg.md) · [Català · Catalan](README.ca.md) · [Ελληνικά · Greek](README.el.md) · [فارسی · Persian](README.fa.md) · [Magyar · Hungarian](README.hu.md) · [हिन्दी · Hindi](README.hi.md) · [Bahasa Indonesia · Indonesian](README.id.md) · [Italiano · Italian](README.it.md) · [Norsk Bokmål · Norwegian](README.nb.md) · [Nederlands · Dutch](README.nl.md) · [Polski · Polish](README.pl.md) · [Română · Romanian](README.ro.md) · [ไทย · Thai](README.th.md) · [Українська · Ukrainian](README.uk.md) · [Tiếng Việt · Vietnamese](README.vi.md)

</details>
<!-- language-selector:end -->

> Създаваме най-чистия, най-бързия и най-настройваемия отворен Android мениджър за изтегляния. Без реклами, без телеметрия — от гийкове за гийкове.

Rayload е независим неофициален Android порт на Rayburst с мобилен интерфейс, системно споделяне, фонови изтегляния и локален Aria2 JSON-RPC.

## v1.0.2 Highlights

- Списъкът със задачи е преработен с безкрайно превъртане и двуредови карти в пълен и компактен режим.
- Добавени са екран за поддържане на дисплея включен и скоростна топка, закрепена към ръба, със статистика, групови действия и рестарт на двигателя.
- Премахнати са настройките само за компютър, локализирано е уведомлението и са подобрени разрешенията на Android и wake lock.
- Подобрени са иконата, диалозите, навигацията и темите, а размерът на APK е намален.

## Инсталиране на най-новия ARM64 APK

Изтеглете най-новия ARM64 APK от тази страница: [GitHub Releases](https://github.com/shiwushuifeng/Rayload/releases/latest)

### Quick start

1. Изтеглете най-новия APK от Releases и го инсталирайте на ARM64 устройство.
2. Разрешете достъп до всички файлове и уведомления, за да работят фоновите изтегляния.
3. Докоснете **+** или споделете връзка от браузър, за да започнете първото изтегляне.

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

### В уведомлението няма бутони за пауза и продължаване

Разрешете уведомления, изключете оптимизацията на батерията и отворете приложението отново. Някои персонализирани Android ROM-ове скриват действията до рестартиране на foreground услугата.

### Изтеглянията спират във фонов режим

Изключете Rayload от ограниченията за батерия и дълбок сън. При агресивни ROM-ове помага приложението да остане отворено или да се използва режим със включен екран по време на зареждане.

### Браузърът не се свързва с RPC

Chrome за Android не зарежда разширения за компютър. Използвайте Firefox за Android или Chromium с ръчно инсталиране на разширения и въведете адреса и RPC Secret от **Настройки → Разширени**.

### При смяна на билда се показва „Приложението не е инсталирано“

Release и Debug APK използват различни подписи. Деинсталирайте текущата версия, преди да инсталирате другия вариант.

## Roadmap

[View the full roadmap](../ROADMAP.md) — пътна карта за продукта и инженерството.

## License

Rayload is licensed under the [GPL-3.0 License](../LICENSE). It is based on
[Rayburst](https://github.com/AnInsomniacy/rayburst) by **AnInsomniacy**, whose
original work is licensed under the MIT License. The bundled Aria2 Next
(`aria2-next`) engine is licensed under **GPL-2.0-or-later**. See
[NOTICE](../NOTICE) for all third-party attributions and license notices.
Rayload is an independent, unofficial port.

> ⭐ Ако Rayload ви спестява време, дайте звезда, за да го открият повече Android потребители.

> ❤️ Like this project? [Sponsor Rayload](../SPONSOR.md)
