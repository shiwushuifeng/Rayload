# Rayload — téléchargeur Android

<!-- language-selector:start -->
<details>
<summary>🌐 Language / 语言 / 語言 / Язык</summary>

[English + 简体中文](../README.md) · [简体中文 · Chinese Simplified](README.zh-CN.md) · [繁體中文 · Chinese Traditional](README.zh-TW.md) · [日本語 · Japanese](README.ja.md) · [한국어 · Korean](README.ko.md) · [**Français · French**](README.fr.md) · [Deutsch · German](README.de.md) · [Español · Spanish](README.es.md) · [Português · Portuguese (Brazil)](README.pt-BR.md) · [Русский · Russian](README.ru.md) · [Türkçe · Turkish](README.tr.md) · [العربية · Arabic](README.ar.md) · [Български · Bulgarian](README.bg.md) · [Català · Catalan](README.ca.md) · [Ελληνικά · Greek](README.el.md) · [فارسی · Persian](README.fa.md) · [Magyar · Hungarian](README.hu.md) · [हिन्दी · Hindi](README.hi.md) · [Bahasa Indonesia · Indonesian](README.id.md) · [Italiano · Italian](README.it.md) · [Norsk Bokmål · Norwegian](README.nb.md) · [Nederlands · Dutch](README.nl.md) · [Polski · Polish](README.pl.md) · [Română · Romanian](README.ro.md) · [ไทย · Thai](README.th.md) · [Українська · Ukrainian](README.uk.md) · [Tiếng Việt · Vietnamese](README.vi.md)

</details>
<!-- language-selector:end -->

> Nous créons le téléchargeur open source pour Android le plus épuré, le plus rapide et le plus personnalisable. Sans publicité, sans télémétrie — conçu par des geeks, pour des geeks.

Rayload est un port Android indépendant et non officiel de Rayburst. Il propose une interface mobile, le partage système, les téléchargements en arrière-plan et Aria2 JSON-RPC en local.

## v1.0.2 Highlights

- Liste des tâches reconstruite avec défilement infini et cartes à deux lignes, en modes complet et compact.
- Ajout du tableau de bord anti-veille et du compteur flottant ancré au bord, avec statistiques, actions groupées et redémarrage du moteur.
- Suppression des réglages réservés au bureau, notification localisée et intégration Android plus robuste.
- Icône, boîtes de dialogue, navigation et thèmes affinés, avec une taille d'APK réduite.

## Installer le dernier APK ARM64

Téléchargez le dernier APK ARM64 depuis la page suivante : [GitHub Releases](https://github.com/shiwushuifeng/Rayload/releases/latest)

### Quick start

1. Téléchargez le dernier APK depuis Releases et installez-le sur un appareil ARM64.
2. Autorisez l'accès à tous les fichiers et les notifications pour les téléchargements en arrière-plan.
3. Touchez **+** ou partagez un lien depuis un navigateur pour lancer votre premier téléchargement.

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

### La notification n'a pas de bouton pause/reprise

Autorisez les notifications, désactivez l'optimisation de la batterie et rouvrez l'application. Certaines ROM Android masquent les actions jusqu'au redémarrage du service de premier plan.

### Les téléchargements s'arrêtent en arrière-plan

Excluez Rayload des restrictions de batterie et de veille profonde. Sur les ROM agressives, gardez l'application ouverte ou utilisez le mode anti-veille pendant la charge.

### Mon navigateur ne se connecte pas au RPC

Chrome pour Android ne charge pas les extensions de bureau. Utilisez Firefox pour Android ou un Chromium acceptant le chargement manuel d'extensions, puis saisissez l'adresse et le secret RPC dans **Paramètres → Avancé**.

### « Application non installée » lors du changement de build

Les APK Release et Debug utilisent des signatures différentes. Désinstallez la version actuelle avant d'installer l'autre variante.

## Roadmap

[View the full roadmap](../ROADMAP.md) — feuille de route produit et technique.

## License

Rayload is licensed under the [GPL-3.0 License](../LICENSE). It is based on
[Rayburst](https://github.com/AnInsomniacy/rayburst) by **AnInsomniacy**, whose
original work is licensed under the MIT License. The bundled Aria2 Next
(`aria2-next`) engine is licensed under **GPL-2.0-or-later**. See
[NOTICE](../NOTICE) for all third-party attributions and license notices.
Rayload is an independent, unofficial port.

> ⭐ Si Rayload vous fait gagner du temps, ajoutez une étoile pour aider davantage d'utilisateurs Android à le découvrir.

> ❤️ Like this project? [Sponsor Rayload](../SPONSOR.md)
