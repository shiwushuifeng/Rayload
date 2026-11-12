# Rayload — Android 다운로더

<!-- language-selector:start -->
<details>
<summary>🌐 Language / 语言 / 語言 / Язык</summary>

[English + 简体中文](../README.md) · [简体中文 · Chinese Simplified](README.zh-CN.md) · [繁體中文 · Chinese Traditional](README.zh-TW.md) · [日本語 · Japanese](README.ja.md) · [**한국어 · Korean**](README.ko.md) · [Français · French](README.fr.md) · [Deutsch · German](README.de.md) · [Español · Spanish](README.es.md) · [Português · Portuguese (Brazil)](README.pt-BR.md) · [Русский · Russian](README.ru.md) · [Türkçe · Turkish](README.tr.md) · [العربية · Arabic](README.ar.md) · [Български · Bulgarian](README.bg.md) · [Català · Catalan](README.ca.md) · [Ελληνικά · Greek](README.el.md) · [فارسی · Persian](README.fa.md) · [Magyar · Hungarian](README.hu.md) · [हिन्दी · Hindi](README.hi.md) · [Bahasa Indonesia · Indonesian](README.id.md) · [Italiano · Italian](README.it.md) · [Norsk Bokmål · Norwegian](README.nb.md) · [Nederlands · Dutch](README.nl.md) · [Polski · Polish](README.pl.md) · [Română · Romanian](README.ro.md) · [ไทย · Thai](README.th.md) · [Українська · Ukrainian](README.uk.md) · [Tiếng Việt · Vietnamese](README.vi.md)

</details>
<!-- language-selector:end -->

> Android에서 가장 순수하고 빠르며 자유롭게 꾸밀 수 있는 오픈 소스 다운로더를 만들고 있습니다. 광고 없음, 텔레메트리 없음. 기크가 기크를 위해 만듭니다.

Rayload는 Rayburst 기반의 독립적인 비공식 Android 포트입니다. 모바일 UI, 시스템 공유, 백그라운드 다운로드 및 로컬 Aria2 JSON-RPC를 제공합니다.

## v1.0.2 Highlights

- 작업 목록을 무한 스크롤과 전체/간결 두 가지 밀도의 두 줄 카드로 재구성했습니다.
- 화면 켜짐 대시보드와 실시간 통계, 일괄 제어, 엔진 재시작을 갖춘 가장자리 속도 볼을 추가했습니다.
- 데스크톱 전용 설정을 제거하고 알림을 현지화했으며 Android 권한과 웨이크록 동작을 개선했습니다.
- 아이콘, 대화상자, 탐색, 테마를 다듬고 APK 크기를 줄였습니다.

## 최신 ARM64 APK 설치

다음 페이지에서 최신 ARM64 APK를 내려받으세요: [GitHub Releases](https://github.com/shiwushuifeng/Rayload/releases/latest)

### Quick start

1. Releases에서 최신 APK를 내려받아 ARM64 기기에 설치합니다.
2. 모든 파일 접근과 알림을 허용해 백그라운드 다운로드를 활성화합니다.
3. **+**를 누르거나 브라우저에서 링크를 공유해 첫 다운로드를 시작합니다.

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

### 알림에 일시정지/재개 버튼이 없습니다

알림을 허용하고 배터리 최적화를 끈 뒤 앱을 다시 여세요. 일부 커스텀 ROM은 포그라운드 서비스가 다시 시작되기 전까지 알림 작업을 숨깁니다.

### 백그라운드 다운로드가 중단됩니다

Rayload를 배터리 및 딥 슬립 제한에서 제외하세요. 제한이 강한 ROM에서는 앱을 열어 두거나 충전 중 화면 켜짐 모드를 사용하면 안정적입니다.

### 브라우저에서 RPC에 연결할 수 없습니다

Android용 Chrome은 데스크톱 확장 프로그램을 지원하지 않습니다. Firefox for Android 또는 확장 프로그램 사이드로드를 지원하는 Chromium을 사용하고 **설정 → 고급**의 주소와 RPC Secret을 입력하세요.

### 빌드를 바꾸면 '앱이 설치되지 않음'이 표시됩니다

Release와 Debug APK는 서명이 다릅니다. 다른 빌드를 설치하기 전에 현재 버전을 제거하세요.

## Roadmap

[View the full roadmap](../ROADMAP.md) — 제품 및 엔지니어링 로드맵.

## License

Rayload is licensed under the [GPL-3.0 License](../LICENSE). It is based on
[Rayburst](https://github.com/AnInsomniacy/rayburst) by **AnInsomniacy**, whose
original work is licensed under the MIT License. The bundled Aria2 Next
(`aria2-next`) engine is licensed under **GPL-2.0-or-later**. See
[NOTICE](../NOTICE) for all third-party attributions and license notices.
Rayload is an independent, unofficial port.

> ⭐ Rayload가 도움이 되었다면 Star를 눌러 더 많은 Android 사용자가 발견할 수 있게 해 주세요.

> ❤️ Like this project? [Sponsor Rayload](../SPONSOR.md)
