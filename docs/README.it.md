# Rayload — download manager per Android

<!-- language-selector:start -->
<details>
<summary>🌐 Language / 语言 / 語言 / Язык</summary>

[English + 简体中文](../README.md) · [简体中文 · Chinese Simplified](README.zh-CN.md) · [繁體中文 · Chinese Traditional](README.zh-TW.md) · [日本語 · Japanese](README.ja.md) · [한국어 · Korean](README.ko.md) · [Français · French](README.fr.md) · [Deutsch · German](README.de.md) · [Español · Spanish](README.es.md) · [Português · Portuguese (Brazil)](README.pt-BR.md) · [Русский · Russian](README.ru.md) · [Türkçe · Turkish](README.tr.md) · [العربية · Arabic](README.ar.md) · [Български · Bulgarian](README.bg.md) · [Català · Catalan](README.ca.md) · [Ελληνικά · Greek](README.el.md) · [فارسی · Persian](README.fa.md) · [Magyar · Hungarian](README.hu.md) · [हिन्दी · Hindi](README.hi.md) · [Bahasa Indonesia · Indonesian](README.id.md) · [**Italiano · Italian**](README.it.md) · [Norsk Bokmål · Norwegian](README.nb.md) · [Nederlands · Dutch](README.nl.md) · [Polski · Polish](README.pl.md) · [Română · Romanian](README.ro.md) · [ไทย · Thai](README.th.md) · [Українська · Ukrainian](README.uk.md) · [Tiếng Việt · Vietnamese](README.vi.md)

</details>
<!-- language-selector:end -->

> Stiamo creando il downloader open source per Android più pulito, veloce e personalizzabile. Senza pubblicità, senza telemetria — creato da geek per geek.

Rayload è un port Android indipendente e non ufficiale di Rayburst con interfaccia mobile, condivisione di sistema, download in background e Aria2 JSON-RPC locale.

## v1.0.2 Highlights

- Elenco attività ricostruito con scorrimento infinito e schede a due righe in modalità completa e compatta.
- Aggiunti la dashboard schermo-sempre-acceso e il misuratore flottante ancorato al bordo, con statistiche, azioni multiple e riavvio del motore.
- Rimosse le impostazioni solo desktop, notifica localizzata e migliore integrazione di permessi Android e wake lock.
- Icona, finestre di dialogo, navigazione e temi rifiniti, con APK più leggero.

## Installa l'APK ARM64 più recente

Scarica l'APK ARM64 più recente da questa pagina: [GitHub Releases](https://github.com/shiwushuifeng/Rayload/releases/latest)

### Quick start

1. Scarica l'APK più recente da Releases e installalo su un dispositivo ARM64.
2. Consenti l'accesso a tutti i file e le notifiche per i download in background.
3. Tocca **+** o condividi un link da qualsiasi browser per iniziare il primo download.

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

### La notifica non ha i pulsanti pausa/riprendi

Consenti le notifiche, disattiva l'ottimizzazione della batteria e riapri l'app. Alcune ROM Android personalizzate nascondono le azioni finché il servizio in primo piano non viene riavviato.

### I download si fermano in background

Escludi Rayload dalle restrizioni su batteria e sospensione profonda. Sulle ROM aggressive aiuta tenere l'app aperta o usare la modalità schermo acceso durante la ricarica.

### Il browser non si connette a RPC

Chrome per Android non carica estensioni desktop. Usa Firefox per Android o un Chromium che supporti l'installazione manuale delle estensioni, poi inserisci indirizzo e RPC Secret da **Impostazioni → Avanzate**.

### Appare «App non installata» cambiando build

Gli APK Release e Debug usano firme diverse. Disinstalla la versione attuale prima di installare l'altra variante.

## Roadmap

[View the full roadmap](../ROADMAP.md) — roadmap di prodotto e ingegneria.

## License

Rayload is licensed under the [GPL-3.0 License](../LICENSE). It is based on
[Rayburst](https://github.com/AnInsomniacy/rayburst) by **AnInsomniacy**, whose
original work is licensed under the MIT License. The bundled Aria2 Next
(`aria2-next`) engine is licensed under **GPL-2.0-or-later**. See
[NOTICE](../NOTICE) for all third-party attributions and license notices.
Rayload is an independent, unofficial port.

> ⭐ Se Rayload ti fa risparmiare tempo, lascia una stella: aiuterà altri utenti Android a scoprirlo.

> ❤️ Like this project? [Sponsor Rayload](../SPONSOR.md)
