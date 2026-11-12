# Rayload — pengunduh Android

<!-- language-selector:start -->
<details>
<summary>🌐 Language / 语言 / 語言 / Язык</summary>

[English + 简体中文](../README.md) · [简体中文 · Chinese Simplified](README.zh-CN.md) · [繁體中文 · Chinese Traditional](README.zh-TW.md) · [日本語 · Japanese](README.ja.md) · [한국어 · Korean](README.ko.md) · [Français · French](README.fr.md) · [Deutsch · German](README.de.md) · [Español · Spanish](README.es.md) · [Português · Portuguese (Brazil)](README.pt-BR.md) · [Русский · Russian](README.ru.md) · [Türkçe · Turkish](README.tr.md) · [العربية · Arabic](README.ar.md) · [Български · Bulgarian](README.bg.md) · [Català · Catalan](README.ca.md) · [Ελληνικά · Greek](README.el.md) · [فارسی · Persian](README.fa.md) · [Magyar · Hungarian](README.hu.md) · [हिन्दी · Hindi](README.hi.md) · [**Bahasa Indonesia · Indonesian**](README.id.md) · [Italiano · Italian](README.it.md) · [Norsk Bokmål · Norwegian](README.nb.md) · [Nederlands · Dutch](README.nl.md) · [Polski · Polish](README.pl.md) · [Română · Romanian](README.ro.md) · [ไทย · Thai](README.th.md) · [Українська · Ukrainian](README.uk.md) · [Tiếng Việt · Vietnamese](README.vi.md)

</details>
<!-- language-selector:end -->

> Kami membangun pengunduh open source untuk Android yang paling bersih, cepat, dan dapat dikustomisasi. Tanpa iklan, tanpa telemetri — dibuat oleh geek, untuk geek.

Rayload adalah port Android independen dan tidak resmi dari Rayburst dengan antarmuka seluler, berbagi sistem, unduhan latar belakang, dan Aria2 JSON-RPC lokal.

## v1.0.2 Highlights

- Daftar tugas dibangun ulang dengan gulir tanpa akhir dan kartu dua baris dalam mode penuh dan ringkas.
- Dasbor layar tetap menyala dan bola kecepatan yang menempel di tepi ditambahkan, lengkap dengan statistik langsung, aksi massal, dan mulai ulang mesin.
- Pengaturan khusus desktop dihapus, notifikasi dilokalkan, serta izin Android dan perilaku wake lock diperbaiki.
- Ikon, dialog, navigasi, dan tema disempurnakan, dengan ukuran APK lebih kecil.

## Instal APK ARM64 terbaru

Unduh APK ARM64 terbaru dari halaman ini: [GitHub Releases](https://github.com/shiwushuifeng/Rayload/releases/latest)

### Quick start

1. Unduh APK terbaru dari Releases dan pasang di perangkat ARM64.
2. Beri akses ke semua file dan izinkan notifikasi agar unduhan berjalan di latar belakang.
3. Ketuk **+** atau bagikan tautan dari browser untuk memulai unduhan pertama.

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

### Notifikasi tidak memiliki tombol jeda/lanjutkan

Izinkan notifikasi, nonaktifkan optimasi baterai, lalu buka ulang aplikasi. Beberapa ROM Android kustom menyembunyikan aksi notifikasi hingga layanan foreground dimulai ulang.

### Unduhan berhenti di latar belakang

Kecualikan Rayload dari pembatasan baterai dan tidur nyenyak. Pada ROM agresif, biarkan aplikasi terbuka atau gunakan mode layar tetap menyala saat mengisi daya.

### Browser tidak dapat terhubung ke RPC

Chrome untuk Android tidak memuat ekstensi desktop. Gunakan Firefox untuk Android atau Chromium yang mendukung sideload ekstensi, lalu masukkan alamat dan RPC Secret dari **Setelan → Lanjutan**.

### Muncul “Aplikasi tidak terpasang” saat berganti build

APK Release dan Debug memakai tanda tangan berbeda. Copot pemasangan versi saat ini sebelum memasang varian lainnya.

## Roadmap

[View the full roadmap](../ROADMAP.md) — peta jalan produk dan teknik.

## License

Rayload is licensed under the [GPL-3.0 License](../LICENSE). It is based on
[Rayburst](https://github.com/AnInsomniacy/rayburst) by **AnInsomniacy**, whose
original work is licensed under the MIT License. The bundled Aria2 Next
(`aria2-next`) engine is licensed under **GPL-2.0-or-later**. See
[NOTICE](../NOTICE) for all third-party attributions and license notices.
Rayload is an independent, unofficial port.

> ⭐ Jika Rayload menghemat waktu Anda, beri bintang agar lebih banyak pengguna Android menemukannya.

> ❤️ Like this project? [Sponsor Rayload](../SPONSOR.md)
