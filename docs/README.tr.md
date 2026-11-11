# Rayload — Android indirme yöneticisi

<!-- language-selector:start -->
<details>
<summary>🌐 Language / 语言 / 語言 / Язык</summary>

[English + 简体中文](../README.md) · [简体中文 · Chinese Simplified](README.zh-CN.md) · [繁體中文 · Chinese Traditional](README.zh-TW.md) · [日本語 · Japanese](README.ja.md) · [한국어 · Korean](README.ko.md) · [Français · French](README.fr.md) · [Deutsch · German](README.de.md) · [Español · Spanish](README.es.md) · [Português · Portuguese (Brazil)](README.pt-BR.md) · [Русский · Russian](README.ru.md) · [**Türkçe · Turkish**](README.tr.md) · [العربية · Arabic](README.ar.md) · [Български · Bulgarian](README.bg.md) · [Català · Catalan](README.ca.md) · [Ελληνικά · Greek](README.el.md) · [فارسی · Persian](README.fa.md) · [Magyar · Hungarian](README.hu.md) · [हिन्दी · Hindi](README.hi.md) · [Bahasa Indonesia · Indonesian](README.id.md) · [Italiano · Italian](README.it.md) · [Norsk Bokmål · Norwegian](README.nb.md) · [Nederlands · Dutch](README.nl.md) · [Polski · Polish](README.pl.md) · [Română · Romanian](README.ro.md) · [ไทย · Thai](README.th.md) · [Українська · Ukrainian](README.uk.md) · [Tiếng Việt · Vietnamese](README.vi.md)

</details>
<!-- language-selector:end -->

> Android için en sade, en hızlı ve en kişiselleştirilebilir açık kaynak indirme yöneticisini geliştiriyoruz. Reklam yok, telemetri yok — geekler tarafından geekler için.

Rayload, Rayburst tabanlı bağımsız ve resmî olmayan bir Android portudur. Mobil arayüz, sistem paylaşımı, arka plan indirmeleri ve yerel Aria2 JSON-RPC sunar.

## v1.0.2 Highlights

- Görev listesi sonsuz kaydırma ve tam/kompakt yoğunlukta iki satırlı kartlarla yeniden oluşturuldu.
- Ekranı açık tutan gösterge paneli ve canlı istatistik, toplu işlemler ve motor yeniden başlatma içeren kenara yapışık hız balonu eklendi.
- Masaüstüne özel ayarlar kaldırıldı, bildirim yerelleştirildi ve Android izinleri ile wake lock davranışı iyileştirildi.
- Simge, iletişim kutuları, gezinme ve temalar geliştirildi; APK boyutu küçültüldü.

## En son ARM64 APK kurulumu

En son ARM64 APK'yı şu sayfadan indirin: [GitHub Releases](https://github.com/shiwushuifeng/Rayload/releases/latest)

### Quick start

1. Releases sayfasından en son APK'yı indirip ARM64 bir cihaza kurun.
2. Arka plan indirmeleri için tüm dosyalara erişim ve bildirim izinlerini verin.
3. İlk indirmeyi başlatmak için **+** simgesine dokunun veya bir tarayıcıdan bağlantı paylaşın.

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

### Bildirimde duraklat/devam et düğmesi yok

Bildirimlere izin verin, pil optimizasyonunu kapatın ve uygulamayı yeniden açın. Bazı özel Android ROM'ları, ön plan hizmeti yeniden başlayana kadar bildirim eylemlerini gizler.

### İndirmeler arka planda duruyor

Rayload'u pil ve derin uyku kısıtlamalarından hariç tutun. Agresif ROM'larda uygulamayı açık tutmak veya şarj sırasında ekran açık modunu kullanmak yardımcı olur.

### Tarayıcım RPC'ye bağlanamıyor

Android için Chrome masaüstü uzantılarını yüklemez. Android için Firefox veya uzantıları elle yükleyebilen bir Chromium kullanın; adresi ve RPC Secret değerini **Ayarlar → Gelişmiş** bölümüne girin.

### Derleme değiştirirken “Uygulama yüklenmedi” hatası çıkıyor

Release ve Debug APK'ları farklı imzalara sahiptir. Diğer sürümü yüklemeden önce mevcut sürümü kaldırın.

## Roadmap

[View the full roadmap](../ROADMAP.md) — ürün ve mühendislik yol haritası.

## License

Rayload is based on [Rayburst](https://github.com/AnInsomniacy/rayburst) by
**AnInsomniacy** and follows the [MIT License](LICENSE). It is an independent,
unofficial port.

> ⭐ Rayload size zaman kazandırıyorsa bir yıldız verin; daha fazla Android kullanıcısı projeyi keşfetsin.
