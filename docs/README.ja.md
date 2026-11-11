# Rayload — Android ダウンローダー

<!-- language-selector:start -->
<details>
<summary>🌐 Language / 语言 / 語言 / Язык</summary>

[English + 简体中文](../README.md) · [简体中文 · Chinese Simplified](README.zh-CN.md) · [繁體中文 · Chinese Traditional](README.zh-TW.md) · [**日本語 · Japanese**](README.ja.md) · [한국어 · Korean](README.ko.md) · [Français · French](README.fr.md) · [Deutsch · German](README.de.md) · [Español · Spanish](README.es.md) · [Português · Portuguese (Brazil)](README.pt-BR.md) · [Русский · Russian](README.ru.md) · [Türkçe · Turkish](README.tr.md) · [العربية · Arabic](README.ar.md) · [Български · Bulgarian](README.bg.md) · [Català · Catalan](README.ca.md) · [Ελληνικά · Greek](README.el.md) · [فارسی · Persian](README.fa.md) · [Magyar · Hungarian](README.hu.md) · [हिन्दी · Hindi](README.hi.md) · [Bahasa Indonesia · Indonesian](README.id.md) · [Italiano · Italian](README.it.md) · [Norsk Bokmål · Norwegian](README.nb.md) · [Nederlands · Dutch](README.nl.md) · [Polski · Polish](README.pl.md) · [Română · Romanian](README.ro.md) · [ไทย · Thai](README.th.md) · [Українська · Ukrainian](README.uk.md) · [Tiếng Việt · Vietnamese](README.vi.md)

</details>
<!-- language-selector:end -->

> Android で最も純粋で高速、自由にカスタマイズできるオープンソースダウンローダーを目指しています。広告なし、テレメトリなし。ギークがギークのために作っています。

Rayload は Rayburst を基にした独立・非公式の Android 版です。モバイル向け UI、共有、バックグラウンドダウンロード、ローカル Aria2 JSON-RPC を提供します。

## v1.0.2 Highlights

- タスク一覧を無限スクロールに変更し、標準とコンパクトの 2 種類の密度を持つ 2 行カードへ刷新しました。
- 常時点灯ダッシュボードと、リアルタイム統計・一括操作・エンジン再起動を備えた画面端のスピードボールを追加しました。
- デスクトップ専用設定を削除し、通知を日本語化。Android の権限とウェイクロック処理も改善しました。
- アイコン、ダイアログ、ナビゲーション、テーマを磨き上げ、APK サイズも削減しました。

## 最新の ARM64 APK をインストール

次のページから最新の ARM64 APK をダウンロードできます： [GitHub Releases](https://github.com/shiwushuifeng/Rayload/releases/latest)

### Quick start

1. Releases から最新 APK をダウンロードし、ARM64 端末にインストールします。
2. すべてのファイルへのアクセスと通知を許可し、バックグラウンドダウンロードを有効にします。
3. **+** をタップするか、ブラウザからリンクを共有して最初のダウンロードを開始します。

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

### 通知に一時停止・再開ボタンがない

通知を許可し、バッテリー最適化を無効にしてアプリを一度再起動してください。一部のカスタム ROM では、フォアグラウンドサービス再起動まで通知操作が隠れます。

### バックグラウンドでダウンロードが止まる

Rayload をバッテリー最適化と深いスリープの制限から除外してください。制限の厳しい ROM では、アプリを開いたままにするか、充電中に常時点灯モードを使うと安定します。

### ブラウザから RPC に接続できない

Android 版 Chrome はデスクトップ拡張機能に対応していません。Firefox for Android または拡張機能のサイドロードに対応した Chromium を使い、**設定 → 詳細** のアドレスと RPC Secret を入力してください。

### ビルドを切り替えると「アプリがインストールされていません」と表示される

Release APK と Debug APK は署名が異なります。別のビルドをインストールする前に、現在のバージョンをアンインストールしてください。

## Roadmap

[View the full roadmap](../ROADMAP.md) — 製品とエンジニアリングのロードマップ.

## License

Rayload is based on [Rayburst](https://github.com/AnInsomniacy/rayburst) by
**AnInsomniacy** and follows the [MIT License](LICENSE). It is an independent,
unofficial port.

> ⭐ Rayload が役に立ったら Star をお願いします。より多くの Android ユーザーに届きます。
