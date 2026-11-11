# Rayload — gestor de baixades per a Android

<!-- language-selector:start -->
<details>
<summary>🌐 Language / 语言 / 語言 / Язык</summary>

[English + 简体中文](../README.md) · [简体中文 · Chinese Simplified](README.zh-CN.md) · [繁體中文 · Chinese Traditional](README.zh-TW.md) · [日本語 · Japanese](README.ja.md) · [한국어 · Korean](README.ko.md) · [Français · French](README.fr.md) · [Deutsch · German](README.de.md) · [Español · Spanish](README.es.md) · [Português · Portuguese (Brazil)](README.pt-BR.md) · [Русский · Russian](README.ru.md) · [Türkçe · Turkish](README.tr.md) · [العربية · Arabic](README.ar.md) · [Български · Bulgarian](README.bg.md) · [**Català · Catalan**](README.ca.md) · [Ελληνικά · Greek](README.el.md) · [فارسی · Persian](README.fa.md) · [Magyar · Hungarian](README.hu.md) · [हिन्दी · Hindi](README.hi.md) · [Bahasa Indonesia · Indonesian](README.id.md) · [Italiano · Italian](README.it.md) · [Norsk Bokmål · Norwegian](README.nb.md) · [Nederlands · Dutch](README.nl.md) · [Polski · Polish](README.pl.md) · [Română · Romanian](README.ro.md) · [ไทย · Thai](README.th.md) · [Українська · Ukrainian](README.uk.md) · [Tiếng Việt · Vietnamese](README.vi.md)

</details>
<!-- language-selector:end -->

> Creem el gestor de baixades de codi obert per a Android més net, ràpid i personalitzable. Sense anuncis ni telemetria: fet per geeks, per a geeks.

Rayload és un port independent i no oficial de Rayburst per a Android, amb interfície mòbil, compartició del sistema, baixades en segon pla i Aria2 JSON-RPC local.

## v1.0.2 Highlights

- Llista de tasques reconstruïda amb desplaçament infinit i targetes de dues files en modes complet i compacte.
- Nou tauler de pantalla sempre activa i indicador flotant ancorat a la vora amb estadístiques, accions massives i reinici del motor.
- Eliminats els ajustos exclusius d'escriptori, notificació localitzada i millor integració de permisos i wake lock d'Android.
- Icona, diàlegs, navegació i temes refinats, amb una mida d'APK més petita.

## Instal·lar l'APK ARM64 més recent

Baixa l'APK ARM64 més recent des d'aquesta pàgina: [GitHub Releases](https://github.com/shiwushuifeng/Rayload/releases/latest)

### Quick start

1. Baixa l'APK més recent de Releases i instal·la'l en un dispositiu ARM64.
2. Concedeix accés a tots els fitxers i permet notificacions per baixar en segon pla.
3. Toca **+** o comparteix un enllaç des de qualsevol navegador per iniciar la primera baixada.

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

### La notificació no té botons de pausa o represa

Permet les notificacions, desactiva l'optimització de bateria i torna a obrir l'aplicació. Algunes ROM personalitzades amaguen les accions fins que es reinicia el servei en primer pla.

### Les baixades s'aturen en segon pla

Exclou Rayload de les restriccions de bateria i son profund. En ROM agressives, mantenir l'aplicació oberta o usar el mode de pantalla activa mentre carregues ajuda.

### El navegador no connecta amb RPC

Chrome per a Android no admet extensions d'escriptori. Usa Firefox per a Android o un Chromium que permeti instal·lar extensions manualment i introdueix l'adreça i el secret RPC a **Configuració → Avançat**.

### Apareix «Aplicació no instal·lada» en canviar de compilació

Els APK Release i Debug fan servir signatures diferents. Desinstal·la la versió actual abans d'instal·lar l'altra variant.

## Roadmap

[View the full roadmap](../ROADMAP.md) — full de ruta de producte i enginyeria.

## License

Rayload is based on [Rayburst](https://github.com/AnInsomniacy/rayburst) by
**AnInsomniacy** and follows the [MIT License](LICENSE). It is an independent,
unofficial port.

> ⭐ Si Rayload et fa estalviar temps, dona-li una estrella perquè més usuaris d'Android el descobreixin.
