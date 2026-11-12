# Rayload — مدير تنزيلات Android

<!-- language-selector:start -->
<details>
<summary>🌐 Language / 语言 / 語言 / Язык</summary>

[English + 简体中文](../README.md) · [简体中文 · Chinese Simplified](README.zh-CN.md) · [繁體中文 · Chinese Traditional](README.zh-TW.md) · [日本語 · Japanese](README.ja.md) · [한국어 · Korean](README.ko.md) · [Français · French](README.fr.md) · [Deutsch · German](README.de.md) · [Español · Spanish](README.es.md) · [Português · Portuguese (Brazil)](README.pt-BR.md) · [Русский · Russian](README.ru.md) · [Türkçe · Turkish](README.tr.md) · [**العربية · Arabic**](README.ar.md) · [Български · Bulgarian](README.bg.md) · [Català · Catalan](README.ca.md) · [Ελληνικά · Greek](README.el.md) · [فارسی · Persian](README.fa.md) · [Magyar · Hungarian](README.hu.md) · [हिन्दी · Hindi](README.hi.md) · [Bahasa Indonesia · Indonesian](README.id.md) · [Italiano · Italian](README.it.md) · [Norsk Bokmål · Norwegian](README.nb.md) · [Nederlands · Dutch](README.nl.md) · [Polski · Polish](README.pl.md) · [Română · Romanian](README.ro.md) · [ไทย · Thai](README.th.md) · [Українська · Ukrainian](README.uk.md) · [Tiếng Việt · Vietnamese](README.vi.md)

</details>
<!-- language-selector:end -->

> نصنع مدير التنزيل مفتوح المصدر الأنقى والأسرع والأكثر قابلية للتخصيص على Android. بلا إعلانات، بلا تتبّع — من المبرمجين للمبرمجين.

Rayload نسخة Android مستقلة وغير رسمية مبنية على Rayburst، وتوفر واجهة للهاتف والمشاركة من النظام والتنزيل في الخلفية وخدمة Aria2 JSON-RPC محلية.

## v1.0.2 Highlights

- أُعيد بناء قائمة المهام بتمرير لا نهائي وبطاقات من سطرين بوضعي العرض الكامل والمضغوط.
- أُضيفت لوحة إبقاء الشاشة مضاءة وكرّة سرعة ملتصقة بالحافة مع إحصاءات حية وتحكم جماعي وإعادة تشغيل المحرك.
- أُزيلت إعدادات سطح المكتب، وعُرّبت الإشعارات، وحُسّنت أذونات Android وسلوك Wake Lock.
- حُسّنت الأيقونة والنوافذ والتنقل والسمات، مع تقليل حجم APK.

## تثبيت أحدث ملف APK لمعمارية ARM64

نزّل أحدث APK لمعمارية ARM64 من هذه الصفحة: [GitHub Releases](https://github.com/shiwushuifeng/Rayload/releases/latest)

### Quick start

1. نزّل أحدث APK من Releases وثبّته على جهاز ARM64.
2. امنح الوصول إلى كل الملفات واسمح بالإشعارات لتعمل التنزيلات في الخلفية.
3. اضغط **+** أو شارك رابطًا من أي متصفح لبدء أول تنزيل.

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

### لا تظهر أزرار الإيقاف المؤقت أو المتابعة في الإشعار

اسمح بالإشعارات، وأوقف تحسين البطارية، ثم أعد فتح التطبيق. بعض إصدارات Android المخصصة تخفي إجراءات الإشعار حتى تُعاد خدمة الواجهة الأمامية.

### تتوقف التنزيلات في الخلفية

استثنِ Rayload من قيود البطارية والنوم العميق. على الأنظمة الصارمة، يساعد إبقاء التطبيق مفتوحًا أو استخدام وضع إبقاء الشاشة مضاءة أثناء الشحن.

### لا يتصل المتصفح بخدمة RPC

لا يدعم Chrome لنظام Android إضافات سطح المكتب. استخدم Firefox لنظام Android أو نسخة Chromium تدعم تثبيت الإضافات يدويًا، وأدخل العنوان وRPC Secret من **الإعدادات ← متقدم**.

### تظهر رسالة «لم يتم تثبيت التطبيق» عند تبديل الإصدار

تستخدم نسختا Release وDebug توقيعين مختلفين. أزل الإصدار الحالي قبل تثبيت النسخة الأخرى.

## Roadmap

[View the full roadmap](../ROADMAP.md) — خارطة طريق المنتج والهندسة.

## License

Rayload is licensed under the [GPL-3.0 License](../LICENSE). It is based on
[Rayburst](https://github.com/AnInsomniacy/rayburst) by **AnInsomniacy**, whose
original work is licensed under the MIT License. The bundled Aria2 Next
(`aria2-next`) engine is licensed under **GPL-2.0-or-later**. See
[NOTICE](../NOTICE) for all third-party attributions and license notices.
Rayload is an independent, unofficial port.

> ⭐ إذا وفّر Rayload وقتك، أضف نجمة ليساعده المزيد من مستخدمي Android على اكتشافه.

> ❤️ Like this project? [Sponsor Rayload](../SPONSOR.md)
