# Rayload — загрузчик для Android

<!-- language-selector:start -->
<details>
<summary>🌐 Language / 语言 / 語言 / Язык</summary>

[English](README.md) · [简体中文 · Chinese Simplified](README.zh-CN.md) · [繁體中文 · Chinese Traditional](README.zh-TW.md) · [日本語 · Japanese](README.ja.md) · [한국어 · Korean](README.ko.md) · [Français · French](README.fr.md) · [Deutsch · German](README.de.md) · [Español · Spanish](README.es.md) · [Português · Portuguese (Brazil)](README.pt-BR.md) · [**Русский · Russian**](README.ru.md) · [Türkçe · Turkish](README.tr.md) · [العربية · Arabic](README.ar.md) · [Български · Bulgarian](README.bg.md) · [Català · Catalan](README.ca.md) · [Ελληνικά · Greek](README.el.md) · [فارسی · Persian](README.fa.md) · [Magyar · Hungarian](README.hu.md) · [हिन्दी · Hindi](README.hi.md) · [Bahasa Indonesia · Indonesian](README.id.md) · [Italiano · Italian](README.it.md) · [Norsk Bokmål · Norwegian](README.nb.md) · [Nederlands · Dutch](README.nl.md) · [Polski · Polish](README.pl.md) · [Română · Romanian](README.ro.md) · [ไทย · Thai](README.th.md) · [Українська · Ukrainian](README.uk.md) · [Tiếng Việt · Vietnamese](README.vi.md)

</details>
<!-- language-selector:end -->

Rayload — независимый неофициальный порт Rayburst для Android. Проект
ориентирован только на Android и создан с использованием Tauri, Rust, Vue 3 и
Aria2 Next. Он предлагает мобильный интерфейс, системный обмен ссылками,
фоновые загрузки, уведомления, интеграцию с общей папкой Downloads и локальный
сервис Aria2 JSON-RPC.

## Требования к платформе

| Параметр                 | Требование                  |
| ------------------------ | --------------------------- |
| Минимальная система      | Android 7.0 / API 24        |
| Целевая система          | Android 17 / API 37         |
| Рабочая архитектура      | `arm64-v8a`                 |
| Идентификатор приложения | `com.shiwushuifeng.rayload` |

Rayload не предназначен для настольных платформ. Обычный Google Chrome для
Android не поддерживает настольные расширения. Для интеграции с браузером
используйте Firefox для Android или сборку Chromium с поддержкой установки
расширений вручную.

## Установка

Скачайте последний ARM64 APK на странице
[GitHub Releases](https://github.com/shiwushuifeng/Rayload/releases):

```text
Rayload-v1.0.1-android17-arm64-release.apk
```

При первом запуске выберите язык интерфейса и завершите настройку Android:

1. Предоставьте доступ ко всем файлам, чтобы Rayload мог записывать файлы в
   общую папку Downloads.
2. Разрешите уведомления для отображения состояния фоновой загрузки.
3. При необходимости отключите оптимизацию батареи для надёжной фоновой работы.

Папка загрузок по умолчанию:

```text
/storage/emulated/0/Download/Rayload
```

Release APK подписывается отдельным сертификатом Rayload, а Debug APK использует
отладочный сертификат Android. Перед переключением между вариантами удалите
предыдущую версию.

## Сборка для Android

### Зависимости

| Инструмент  | Версия или цель                          |
| ----------- | ---------------------------------------- |
| Node.js     | 22 или новее                             |
| pnpm        | `11.5.2`, как указано в `packageManager` |
| Rust        | stable с целью `aarch64-linux-android`   |
| Java        | JDK 21                                   |
| Android SDK | Platform 37 и Build Tools 37.0.0         |
| Android NDK | `29.0.14206865`                          |
| Git         | актуальная стабильная версия             |
| GitHub CLI  | необязательно, только для публикации     |

Установите цель Rust для Android:

```bash
rustup target add aarch64-linux-android
```

Настройте инструменты Android:

```bash
export JAVA_HOME=/path/to/jdk-21
export ANDROID_HOME=/path/to/android-sdk
export ANDROID_SDK_ROOT="$ANDROID_HOME"
export NDK_HOME="$ANDROID_HOME/ndk/29.0.14206865"
```

### Сборка APK

```bash
pnpm install
pnpm android:prepare
pnpm tauri android build --apk --target aarch64
```

Сборка Debug APK:

```bash
pnpm tauri android build --apk --target aarch64 --debug
```

Настроенный проект Android Studio находится в `src-tauri/gen/android`, и его
необходимо сохранять. Не запускайте `tauri android init` в этом каталоге: команда
перезапишет обработку системного меню «Поделиться», foreground-сервис, мост
DocumentsUI и точку входа Kotlin.

Параметры подписи Release считываются из локального файла
`signing/keystore.properties`. Хранилища ключей и пароли не добавляются в Git.

### Проверки качества

```bash
pnpm lint
pnpm format:check
pnpm check:repo
pnpm test
npx vue-tsc --noEmit
cargo test --manifest-path src-tauri/Cargo.toml --workspace --all-targets
```

## Настройка RPC в браузере

На устройстве Android Rayload предоставляет защищённый Aria2 JSON-RPC:

```text
Адрес: http://127.0.0.1:6800/jsonrpc
Секрет: Настройки → Дополнительно → RPC Secret
```

Установите расширение с поддержкой Aria2 JSON-RPC в Firefox для Android или
совместимой сборке Chromium, укажите адрес и вставьте секрет RPC.

## Структура проекта

```text
src/                         интерфейс Vue
src-tauri/src/               сервер Rust и системные сервисы
src-tauri/gen/android/       настроенный проект Android Studio
src-tauri/locales/           системные переводы JSON
src/shared/locales/          переводы интерфейса JSON
scripts/android/             шаблоны исходников Kotlin для Android
```

## Лицензия и авторство

Rayload основан на [Rayburst](https://github.com/AnInsomniacy/rayburst),
созданном **AnInsomniacy**, и распространяется по оригинальной
[лицензии MIT](LICENSE). Rayload поддерживается независимо, не является
официальным выпуском Rayburst и не одобрен первоначальным автором.
