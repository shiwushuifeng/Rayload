# Rayload — دانلودر Android

<!-- language-selector:start -->
<details>
<summary>🌐 Language / 语言 / 語言 / Язык</summary>

[English + 简体中文](../README.md) · [简体中文 · Chinese Simplified](README.zh-CN.md) · [繁體中文 · Chinese Traditional](README.zh-TW.md) · [日本語 · Japanese](README.ja.md) · [한국어 · Korean](README.ko.md) · [Français · French](README.fr.md) · [Deutsch · German](README.de.md) · [Español · Spanish](README.es.md) · [Português · Portuguese (Brazil)](README.pt-BR.md) · [Русский · Russian](README.ru.md) · [Türkçe · Turkish](README.tr.md) · [العربية · Arabic](README.ar.md) · [Български · Bulgarian](README.bg.md) · [Català · Catalan](README.ca.md) · [Ελληνικά · Greek](README.el.md) · [**فارسی · Persian**](README.fa.md) · [Magyar · Hungarian](README.hu.md) · [हिन्दी · Hindi](README.hi.md) · [Bahasa Indonesia · Indonesian](README.id.md) · [Italiano · Italian](README.it.md) · [Norsk Bokmål · Norwegian](README.nb.md) · [Nederlands · Dutch](README.nl.md) · [Polski · Polish](README.pl.md) · [Română · Romanian](README.ro.md) · [ไทย · Thai](README.th.md) · [Українська · Ukrainian](README.uk.md) · [Tiếng Việt · Vietnamese](README.vi.md)

</details>
<!-- language-selector:end -->

> در حال ساختن پاک‌ترین، سریع‌ترین و قابل‌شخصی‌سازی‌ترین دانلودر متن‌باز Android هستیم. بدون تبلیغ، بدون تله‌متری — ساخته‌شده توسط گیمرها برای گیمرها.

Rayload یک نسخه مستقل و غیررسمی از Rayburst برای Android است و رابط موبایل، اشتراک‌گذاری سیستمی، دانلود پس‌زمینه و Aria2 JSON-RPC محلی را ارائه می‌دهد.

## v1.0.2 Highlights

- فهرست وظایف با اسکرول بی‌نهایت و کارت‌های دوخطی در حالت کامل و فشرده بازسازی شد.
- داشبورد روشن ماندن صفحه و توپ سرعت چسبیده به لبه با آمار زنده، کنترل گروهی و راه‌اندازی مجدد موتور اضافه شد.
- تنظیمات مخصوص دسکتاپ حذف شد، اعلان بومی‌سازی شد و مجوزهای Android و رفتار wake lock بهبود یافت.
- آیکون، دیالوگ‌ها، ناوبری و تم‌ها بهینه شدند و حجم APK کاهش یافت.

## نصب جدیدترین APK ARM64

آخرین APK مخصوص ARM64 را از این صفحه دانلود کنید: [GitHub Releases](https://github.com/shiwushuifeng/Rayload/releases/latest)

### Quick start

1. آخرین APK را از Releases دانلود و روی دستگاه ARM64 نصب کنید.
2. برای دانلود در پس‌زمینه دسترسی به همه فایل‌ها و اعلان‌ها را بدهید.
3. برای شروع اولین دانلود **+** را بزنید یا پیوندی را از مرورگر به اشتراک بگذارید.

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

### اعلان دکمه توقف یا ادامه ندارد

اعلان‌ها را مجاز کنید، بهینه‌سازی باتری را خاموش کنید و برنامه را دوباره باز کنید. برخی ROMهای سفارشی تا راه‌اندازی مجدد سرویس پیش‌زمینه، اقدام‌های اعلان را پنهان می‌کنند.

### دانلودها در پس‌زمینه متوقف می‌شوند

Rayload را از محدودیت باتری و خواب عمیق خارج کنید. در ROMهای سخت‌گیر، باز نگه‌داشتن برنامه یا استفاده از حالت روشن ماندن صفحه هنگام شارژ کمک می‌کند.

### مرورگر به RPC متصل نمی‌شود

Chrome برای Android افزونه‌های دسکتاپ را بارگذاری نمی‌کند. از Firefox برای Android یا Chromium با نصب دستی افزونه استفاده کنید و نشانی و RPC Secret را در **تنظیمات → پیشرفته** وارد کنید.

### هنگام تغییر بیلد پیام «برنامه نصب نشد» نمایش داده می‌شود

APKهای Release و Debug امضای متفاوت دارند. پیش از نصب نسخه دیگر، نسخه فعلی را حذف کنید.

## Roadmap

[View the full roadmap](../ROADMAP.md) — نقشه راه محصول و مهندسی.

## License

Rayload is distributed under the [GPL-3.0 License](../LICENSE). It is based on
[Rayburst](https://github.com/AnInsomniacy/rayburst) by **AnInsomniacy**, whose
original work is licensed under the MIT License; see [NOTICE](../NOTICE) for
third-party attributions. Rayload is an independent, unofficial port.

> ⭐ اگر Rayload در وقت شما صرفه‌جویی می‌کند، یک ستاره بدهید تا کاربران بیشتری آن را پیدا کنند.

> ❤️ Like this project? [Sponsor Rayload](../SPONSOR.md)
