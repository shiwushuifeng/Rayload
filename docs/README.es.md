# Rayload — descargador para Android

<!-- language-selector:start -->
<details>
<summary>🌐 Language / 语言 / 語言 / Язык</summary>

[English + 简体中文](../README.md) · [简体中文 · Chinese Simplified](README.zh-CN.md) · [繁體中文 · Chinese Traditional](README.zh-TW.md) · [日本語 · Japanese](README.ja.md) · [한국어 · Korean](README.ko.md) · [Français · French](README.fr.md) · [Deutsch · German](README.de.md) · [**Español · Spanish**](README.es.md) · [Português · Portuguese (Brazil)](README.pt-BR.md) · [Русский · Russian](README.ru.md) · [Türkçe · Turkish](README.tr.md) · [العربية · Arabic](README.ar.md) · [Български · Bulgarian](README.bg.md) · [Català · Catalan](README.ca.md) · [Ελληνικά · Greek](README.el.md) · [فارسی · Persian](README.fa.md) · [Magyar · Hungarian](README.hu.md) · [हिन्दी · Hindi](README.hi.md) · [Bahasa Indonesia · Indonesian](README.id.md) · [Italiano · Italian](README.it.md) · [Norsk Bokmål · Norwegian](README.nb.md) · [Nederlands · Dutch](README.nl.md) · [Polski · Polish](README.pl.md) · [Română · Romanian](README.ro.md) · [ไทย · Thai](README.th.md) · [Українська · Ukrainian](README.uk.md) · [Tiếng Việt · Vietnamese](README.vi.md)

</details>
<!-- language-selector:end -->

> Creamos el descargador de código abierto para Android más limpio, rápido y personalizable. Sin anuncios, sin telemetría: hecho por geeks para geeks.

Rayload es un port independiente y no oficial de Rayburst para Android, con interfaz móvil, uso compartido del sistema, descargas en segundo plano y Aria2 JSON-RPC local.

## v1.0.2 Highlights

- Lista de tareas reconstruida con desplazamiento infinito y tarjetas de dos filas en modos completo y compacto.
- Nuevo panel de pantalla siempre activa y medidor flotante anclado al borde con estadísticas, acciones masivas y reinicio del motor.
- Eliminados los ajustes exclusivos de escritorio, notificación localizada y mejor integración de permisos y wake lock de Android.
- Icono, diálogos, navegación y temas refinados, con menor tamaño del APK.

## Instalar el APK ARM64 más reciente

Descarga el APK ARM64 más reciente desde esta página: [GitHub Releases](https://github.com/shiwushuifeng/Rayload/releases/latest)

### Quick start

1. Descarga el APK más reciente desde Releases e instálalo en un dispositivo ARM64.
2. Concede acceso a todos los archivos y permite notificaciones para descargar en segundo plano.
3. Toca **+** o comparte un enlace desde cualquier navegador para iniciar la primera descarga.

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

### La notificación no tiene botones de pausa o reanudación

Permite las notificaciones, desactiva la optimización de batería y vuelve a abrir la aplicación. Algunas ROM personalizadas ocultan las acciones hasta reiniciar el servicio en primer plano.

### Las descargas se detienen en segundo plano

Excluye Rayload de las restricciones de batería y sueño profundo. En ROM agresivas, mantener la app abierta o usar el modo de pantalla activa mientras cargas ayuda.

### Mi navegador no conecta con RPC

Chrome para Android no admite extensiones de escritorio. Usa Firefox para Android o un Chromium que permita instalar extensiones manualmente e introduce la dirección y el secreto RPC en **Ajustes → Avanzado**.

### Aparece «Aplicación no instalada» al cambiar de compilación

Los APK Release y Debug usan firmas distintas. Desinstala la versión actual antes de instalar la otra variante.

## Roadmap

[View the full roadmap](../ROADMAP.md) — hoja de ruta de producto e ingeniería.

## License

Rayload is distributed under the [GPL-3.0 License](../LICENSE). It is based on
[Rayburst](https://github.com/AnInsomniacy/rayburst) by **AnInsomniacy**, whose
original work is licensed under the MIT License; see [NOTICE](../NOTICE) for
third-party attributions. Rayload is an independent, unofficial port.

> ⭐ Si Rayload te ahorra tiempo, dale una estrella para que más usuarios de Android lo descubran.

> ❤️ Like this project? [Sponsor Rayload](../SPONSOR.md)
