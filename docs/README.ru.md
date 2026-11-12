# Rayload — загрузчик для Android

<!-- language-selector:start -->
<details>
<summary>🌐 Language / 语言 / 語言 / Язык</summary>

[English + 简体中文](../README.md) · [简体中文 · Chinese Simplified](README.zh-CN.md) · [繁體中文 · Chinese Traditional](README.zh-TW.md) · [日本語 · Japanese](README.ja.md) · [한국어 · Korean](README.ko.md) · [Français · French](README.fr.md) · [Deutsch · German](README.de.md) · [Español · Spanish](README.es.md) · [Português · Portuguese (Brazil)](README.pt-BR.md) · [**Русский · Russian**](README.ru.md) · [Türkçe · Turkish](README.tr.md) · [العربية · Arabic](README.ar.md) · [Български · Bulgarian](README.bg.md) · [Català · Catalan](README.ca.md) · [Ελληνικά · Greek](README.el.md) · [فارسی · Persian](README.fa.md) · [Magyar · Hungarian](README.hu.md) · [हिन्दी · Hindi](README.hi.md) · [Bahasa Indonesia · Indonesian](README.id.md) · [Italiano · Italian](README.it.md) · [Norsk Bokmål · Norwegian](README.nb.md) · [Nederlands · Dutch](README.nl.md) · [Polski · Polish](README.pl.md) · [Română · Romanian](README.ro.md) · [ไทย · Thai](README.th.md) · [Українська · Ukrainian](README.uk.md) · [Tiếng Việt · Vietnamese](README.vi.md)

</details>
<!-- language-selector:end -->

Rayload — независимый неофициальный порт Rayburst для Android. Проект
ориентирован только на Android и создан с использованием Tauri, Rust, Vue 3 и
Aria2 Next. Он предлагает мобильный интерфейс, системный обмен ссылками,
фоновые загрузки, уведомления, интеграцию с общей папкой Downloads и локальный
сервис Aria2 JSON-RPC.

## Видение проекта

> 🚀 Создаём самый чистый, быстрый и настраиваемый открытый загрузчик для Android. Без рекламы, без телеметрии — создан гиками для гиков.

Приоритет — приватность, единый движок для всех протоколов, нативный мобильный
опыт, а также развитие через темы и плагины. Будущие планы описаны в
[дорожной карте](../ROADMAP.md).

## Что нового в v1.0.2

- Список задач переведён на бесконечную прокрутку и двухстрочные карточки с
  полным и компактным режимами.
- Добавлены экран бодрствования и прикреплённый к краю индикатор скорости со
  статистикой, массовыми действиями и перезапуском движка.
- Удалены настройки для ПК, уведомление локализовано, доработаны разрешения
  Android и поведение wake lock.
- Улучшены иконка, диалоги, навигация и темы, а также уменьшен размер APK.

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

### Быстрый старт

1. Скачайте последний APK на странице Releases и установите его на устройство
   ARM64.
2. Предоставьте доступ ко всем файлам и разрешите уведомления для фоновой
   работы.
3. Нажмите **+** или поделитесь ссылкой из любого браузера, чтобы начать первую
   загрузку.

Для длительных загрузок также отключите оптимизацию батареи.

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
docs/screenshots/            скриншоты для README
ROADMAP.md                   план развития продукта и инженерии
```

## Частые вопросы

### В уведомлении нет кнопок паузы и продолжения

Разрешите уведомления, отключите оптимизацию батареи и перезапустите приложение.
Некоторые прошивки Android скрывают действия до перезапуска foreground-сервиса.

### Загрузки останавливаются в фоне

Исключите Rayload из ограничений батареи и глубокого сна. На агрессивных
прошивках помогает открытое приложение или режим бодрствования при зарядке.

### Браузер не подключается к RPC

Chrome для Android не поддерживает настольные расширения. Используйте Firefox
для Android или Chromium с ручной установкой расширений; адрес и секрет RPC
находятся в разделе **Настройки → Дополнительно**.

### При смене сборки появляется «Приложение не установлено»

Release и Debug APK подписаны разными сертификатами. Перед установкой другого
варианта удалите текущую версию.

## Лицензия и авторство

Rayload распространяется по [лицензии GPL-3.0](../LICENSE) и основан на
[Rayburst](https://github.com/AnInsomniacy/rayburst), созданном
**AnInsomniacy** (лицензия MIT). Сведения о сторонних компонентах и текст
лицензии MIT приведены в [NOTICE](../NOTICE). Rayload поддерживается
независимо, не является официальным выпуском Rayburst и не одобрен
первоначальным автором.

---

<div align="center">

**⭐ Если Rayload экономит ваше время, поставьте звезду — так больше пользователей Android узнают о нём.**

👉 [Полная дорожная карта проекта](../ROADMAP.md)

</div>

> ❤️ Like this project? [Sponsor Rayload](../SPONSOR.md)
