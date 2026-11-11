# Rayload — завантажувач для Android

<!-- language-selector:start -->
<details>
<summary>🌐 Language / 语言 / 語言 / Язык</summary>

[English + 简体中文](../README.md) · [简体中文 · Chinese Simplified](README.zh-CN.md) · [繁體中文 · Chinese Traditional](README.zh-TW.md) · [日本語 · Japanese](README.ja.md) · [한국어 · Korean](README.ko.md) · [Français · French](README.fr.md) · [Deutsch · German](README.de.md) · [Español · Spanish](README.es.md) · [Português · Portuguese (Brazil)](README.pt-BR.md) · [Русский · Russian](README.ru.md) · [Türkçe · Turkish](README.tr.md) · [العربية · Arabic](README.ar.md) · [Български · Bulgarian](README.bg.md) · [Català · Catalan](README.ca.md) · [Ελληνικά · Greek](README.el.md) · [فارسی · Persian](README.fa.md) · [Magyar · Hungarian](README.hu.md) · [हिन्दी · Hindi](README.hi.md) · [Bahasa Indonesia · Indonesian](README.id.md) · [Italiano · Italian](README.it.md) · [Norsk Bokmål · Norwegian](README.nb.md) · [Nederlands · Dutch](README.nl.md) · [Polski · Polish](README.pl.md) · [Română · Romanian](README.ro.md) · [ไทย · Thai](README.th.md) · [**Українська · Ukrainian**](README.uk.md) · [Tiếng Việt · Vietnamese](README.vi.md)

</details>
<!-- language-selector:end -->

> Створюємо найчистіший, найшвидший і найгнучкіший open-source завантажувач для Android. Без реклами, без телеметрії — від гіків для гіків.

Rayload — незалежний неофіційний Android-порт Rayburst із мобільним інтерфейсом, системним поширенням, фоновими завантаженнями та локальним Aria2 JSON-RPC.

## v1.0.2 Highlights

- Список завдань перебудовано з нескінченним прокручуванням і дворядковими картками в повному та компактному режимах.
- Додано панель підтримання екрана увімкненим і швидкісну кулю, прикріплену до краю, зі статистикою, груповими діями та перезапуском рушія.
- Вилучено налаштування лише для комп'ютера, локалізовано сповіщення та покращено дозволи Android і wake lock.
- Удосконалено іконку, діалоги, навігацію й теми та зменшено розмір APK.

## Встановити найновіший ARM64 APK

Завантажте найновіший ARM64 APK із цієї сторінки: [GitHub Releases](https://github.com/shiwushuifeng/Rayload/releases/latest)

### Quick start

1. Завантажте найновіший APK зі сторінки Releases і встановіть його на пристрій ARM64.
2. Надайте доступ до всіх файлів і дозвольте сповіщення для фонових завантажень.
3. Натисніть **+** або поділіться посиланням із браузера, щоб почати перше завантаження.

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

### У сповіщенні немає кнопок паузи/продовження

Дозвольте сповіщення, вимкніть оптимізацію батареї та перезапустіть застосунок. Деякі кастомні Android ROM приховують дії до перезапуску foreground-сервісу.

### Завантаження зупиняються у фоні

Виключіть Rayload з обмежень батареї та глибокого сну. На агресивних ROM допомагає залишити застосунок відкритим або використовувати режим постійного екрана під час заряджання.

### Браузер не підключається до RPC

Chrome для Android не підтримує настільні розширення. Використовуйте Firefox для Android або Chromium з ручним встановленням розширень і введіть адресу та RPC Secret у **Налаштування → Додатково**.

### Під час зміни збірки з'являється «Застосунок не встановлено»

APK Release і Debug мають різні підписи. Видаліть поточну версію перед встановленням іншого варіанта.

## Roadmap

[View the full roadmap](../ROADMAP.md) — дорожня карта продукту та інженерії.

## License

Rayload is based on [Rayburst](https://github.com/AnInsomniacy/rayburst) by
**AnInsomniacy** and follows the [MIT License](LICENSE). It is an independent,
unofficial port.

> ⭐ Якщо Rayload економить ваш час, поставте зірку, щоб більше користувачів Android дізналися про нього.
