# Rayload — โปรแกรมดาวน์โหลดสำหรับ Android

<!-- language-selector:start -->
<details>
<summary>🌐 Language / 语言 / 語言 / Язык</summary>

[English + 简体中文](../README.md) · [简体中文 · Chinese Simplified](README.zh-CN.md) · [繁體中文 · Chinese Traditional](README.zh-TW.md) · [日本語 · Japanese](README.ja.md) · [한국어 · Korean](README.ko.md) · [Français · French](README.fr.md) · [Deutsch · German](README.de.md) · [Español · Spanish](README.es.md) · [Português · Portuguese (Brazil)](README.pt-BR.md) · [Русский · Russian](README.ru.md) · [Türkçe · Turkish](README.tr.md) · [العربية · Arabic](README.ar.md) · [Български · Bulgarian](README.bg.md) · [Català · Catalan](README.ca.md) · [Ελληνικά · Greek](README.el.md) · [فارسی · Persian](README.fa.md) · [Magyar · Hungarian](README.hu.md) · [हिन्दी · Hindi](README.hi.md) · [Bahasa Indonesia · Indonesian](README.id.md) · [Italiano · Italian](README.it.md) · [Norsk Bokmål · Norwegian](README.nb.md) · [Nederlands · Dutch](README.nl.md) · [Polski · Polish](README.pl.md) · [Română · Romanian](README.ro.md) · [**ไทย · Thai**](README.th.md) · [Українська · Ukrainian](README.uk.md) · [Tiếng Việt · Vietnamese](README.vi.md)

</details>
<!-- language-selector:end -->

> เรากำลังสร้างตัวดาวน์โหลดโอเพนซอร์สสำหรับ Android ที่เรียบง่ายที่สุด เร็วที่สุด และปรับแต่งได้มากที่สุด ไม่มีโฆษณา ไม่มี telemetry — สร้างโดย geeks เพื่อ geeks

Rayload เป็นพอร์ต Android แบบอิสระและไม่เป็นทางการของ Rayburst พร้อมอินเทอร์เฟซสำหรับมือถือ การแชร์ผ่านระบบ ดาวน์โหลดเบื้องหลัง และ Aria2 JSON-RPC ภายในเครื่อง

## v1.0.2 Highlights

- สร้างรายการงานใหม่ด้วยการเลื่อนแบบไม่สิ้นสุดและการ์ดสองแถวในโหมดเต็มและกะทัดรัด
- เพิ่มแดชบอร์ดคงหน้าจอเปิดและลูกบอลความเร็วที่เกาะขอบ พร้อมสถิติสด การควบคุมแบบกลุ่ม และการรีสตาร์ทเอนจิน
- ลบการตั้งค่าสำหรับเดสก์ท็อป แปลภาษา notification และปรับปรุงสิทธิ์ Android กับการทำงานของ wake lock
- ปรับไอคอน ไดอะล็อก การนำทาง และธีม พร้อมลดขนาด APK

## ติดตั้ง APK ARM64 ล่าสุด

ดาวน์โหลด APK ARM64 ล่าสุดได้จากหน้านี้: [GitHub Releases](https://github.com/shiwushuifeng/Rayload/releases/latest)

### Quick start

1. ดาวน์โหลด APK ล่าสุดจาก Releases และติดตั้งบนอุปกรณ์ ARM64
2. อนุญาตการเข้าถึงไฟล์ทั้งหมดและ notification เพื่อให้ดาวน์โหลดเบื้องหลังทำงาน
3. แตะ **+** หรือแชร์ลิงก์จากเบราว์เซอร์เพื่อเริ่มดาวน์โหลดครั้งแรก

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

### Notification ไม่มีปุ่มหยุด/ทำต่อ

อนุญาต notification ปิดการปรับแต่งแบตเตอรี่ แล้วเปิดแอปใหม่ ROM Android ที่ปรับแต่งบางรุ่นจะซ่อนปุ่มจนกว่าบริการ foreground จะเริ่มใหม่

### การดาวน์โหลดหยุดเมื่ออยู่เบื้องหลัง

ยกเว้น Rayload จากการจำกัดแบตเตอรี่และการหลับลึก บน ROM ที่จำกัดเข้มงวด การเปิดแอปค้างไว้หรือใช้โหมดคงหน้าจอเปิดขณะชาร์จจะช่วยได้

### เบราว์เซอร์เชื่อมต่อ RPC ไม่ได้

Chrome สำหรับ Android ไม่รองรับส่วนขยายเดสก์ท็อป ใช้ Firefox สำหรับ Android หรือ Chromium ที่ติดตั้งส่วนขยายด้วยตนเองได้ แล้วกรอกที่อยู่และ RPC Secret ใน **การตั้งค่า → ขั้นสูง**

### ขึ้นว่า “ไม่ได้ติดตั้งแอป” เมื่อสลับบิลด์

APK แบบ Release และ Debug มีลายเซ็นต่างกัน ถอนการติดตั้งเวอร์ชันปัจจุบันก่อนติดตั้งอีกเวอร์ชัน

## Roadmap

[View the full roadmap](../ROADMAP.md) — แผนงานผลิตภัณฑ์และวิศวกรรม.

## License

Rayload is licensed under the [GPL-3.0 License](../LICENSE). It is based on
[Rayburst](https://github.com/AnInsomniacy/rayburst) by **AnInsomniacy**, whose
original work is licensed under the MIT License. The bundled Aria2 Next
(`aria2-next`) engine is licensed under **GPL-2.0-or-later**. See
[NOTICE](../NOTICE) for all third-party attributions and license notices.
Rayload is an independent, unofficial port.

> ⭐ ถ้า Rayload ช่วยประหยัดเวลา ให้กด Star เพื่อให้ผู้ใช้ Android ค้นพบมากขึ้น

> ❤️ Like this project? [Sponsor Rayload](../SPONSOR.md)
