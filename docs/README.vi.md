# Rayload — trình tải xuống Android

<!-- language-selector:start -->
<details>
<summary>🌐 Language / 语言 / 語言 / Язык</summary>

[English + 简体中文](../README.md) · [简体中文 · Chinese Simplified](README.zh-CN.md) · [繁體中文 · Chinese Traditional](README.zh-TW.md) · [日本語 · Japanese](README.ja.md) · [한국어 · Korean](README.ko.md) · [Français · French](README.fr.md) · [Deutsch · German](README.de.md) · [Español · Spanish](README.es.md) · [Português · Portuguese (Brazil)](README.pt-BR.md) · [Русский · Russian](README.ru.md) · [Türkçe · Turkish](README.tr.md) · [العربية · Arabic](README.ar.md) · [Български · Bulgarian](README.bg.md) · [Català · Catalan](README.ca.md) · [Ελληνικά · Greek](README.el.md) · [فارسی · Persian](README.fa.md) · [Magyar · Hungarian](README.hu.md) · [हिन्दी · Hindi](README.hi.md) · [Bahasa Indonesia · Indonesian](README.id.md) · [Italiano · Italian](README.it.md) · [Norsk Bokmål · Norwegian](README.nb.md) · [Nederlands · Dutch](README.nl.md) · [Polski · Polish](README.pl.md) · [Română · Romanian](README.ro.md) · [ไทย · Thai](README.th.md) · [Українська · Ukrainian](README.uk.md) · [**Tiếng Việt · Vietnamese**](README.vi.md)

</details>
<!-- language-selector:end -->

> Chúng tôi đang xây dựng trình tải xuống mã nguồn mở gọn gàng, nhanh nhất và dễ tùy biến nhất cho Android. Không quảng cáo, không telemetry — do geek tạo ra, dành cho geek.

Rayload là bản chuyển độc lập, không chính thức của Rayburst cho Android với giao diện di động, chia sẻ hệ thống, tải xuống nền và Aria2 JSON-RPC cục bộ.

## v1.0.2 Highlights

- Danh sách tác vụ được xây dựng lại với cuộn vô hạn và thẻ hai hàng ở chế độ đầy đủ và thu gọn.
- Thêm bảng điều khiển giữ màn hình sáng và bong bóng tốc độ gắn cạnh, cùng thống kê trực tiếp, thao tác hàng loạt và khởi động lại engine.
- Loại bỏ cài đặt chỉ dành cho desktop, bản địa hóa thông báo và cải thiện quyền Android cùng hành vi wake lock.
- Tinh chỉnh biểu tượng, hộp thoại, điều hướng và giao diện, đồng thời giảm dung lượng APK.

## Cài đặt APK ARM64 mới nhất

Tải APK ARM64 mới nhất từ trang này: [GitHub Releases](https://github.com/shiwushuifeng/Rayload/releases/latest)

### Quick start

1. Tải APK mới nhất từ Releases và cài đặt trên thiết bị ARM64.
2. Cấp quyền truy cập tất cả tệp và cho phép thông báo để tải xuống chạy nền.
3. Nhấn **+** hoặc chia sẻ liên kết từ trình duyệt để bắt đầu tải xuống đầu tiên.

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

### Thông báo không có nút tạm dừng/tiếp tục

Hãy cho phép thông báo, tắt tối ưu pin và mở lại ứng dụng. Một số ROM Android tùy biến ẩn thao tác cho đến khi dịch vụ foreground khởi động lại.

### Tải xuống dừng khi chạy nền

Loại Rayload khỏi giới hạn pin và ngủ sâu. Trên ROM quản lý mạnh, giữ ứng dụng mở hoặc dùng chế độ giữ màn hình sáng khi sạc sẽ ổn định hơn.

### Trình duyệt không kết nối được RPC

Chrome cho Android không tải tiện ích desktop. Dùng Firefox cho Android hoặc Chromium hỗ trợ cài tiện ích thủ công, rồi nhập địa chỉ và RPC Secret trong **Cài đặt → Nâng cao**.

### Hiện “Ứng dụng chưa được cài đặt” khi đổi bản build

APK Release và Debug dùng chữ ký khác nhau. Gỡ bản hiện tại trước khi cài biến thể còn lại.

## Roadmap

[View the full roadmap](../ROADMAP.md) — lộ trình sản phẩm và kỹ thuật.

## License

Rayload is licensed under the [GPL-3.0 License](../LICENSE). It is based on
[Rayburst](https://github.com/AnInsomniacy/rayburst) by **AnInsomniacy**, whose
original work is licensed under the MIT License. The bundled Aria2 Next
(`aria2-next`) engine is licensed under **GPL-2.0-or-later**. See
[NOTICE](../NOTICE) for all third-party attributions and license notices.
Rayload is an independent, unofficial port.

> ⭐ Nếu Rayload giúp bạn tiết kiệm thời gian, hãy tặng một ngôi sao để nhiều người dùng Android biết đến hơn.

> ❤️ Like this project? [Sponsor Rayload](../SPONSOR.md)
