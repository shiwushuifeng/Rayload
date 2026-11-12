# Rayload — λήψεις για Android

<!-- language-selector:start -->
<details>
<summary>🌐 Language / 语言 / 語言 / Язык</summary>

[English + 简体中文](../README.md) · [简体中文 · Chinese Simplified](README.zh-CN.md) · [繁體中文 · Chinese Traditional](README.zh-TW.md) · [日本語 · Japanese](README.ja.md) · [한국어 · Korean](README.ko.md) · [Français · French](README.fr.md) · [Deutsch · German](README.de.md) · [Español · Spanish](README.es.md) · [Português · Portuguese (Brazil)](README.pt-BR.md) · [Русский · Russian](README.ru.md) · [Türkçe · Turkish](README.tr.md) · [العربية · Arabic](README.ar.md) · [Български · Bulgarian](README.bg.md) · [Català · Catalan](README.ca.md) · [**Ελληνικά · Greek**](README.el.md) · [فارسی · Persian](README.fa.md) · [Magyar · Hungarian](README.hu.md) · [हिन्दी · Hindi](README.hi.md) · [Bahasa Indonesia · Indonesian](README.id.md) · [Italiano · Italian](README.it.md) · [Norsk Bokmål · Norwegian](README.nb.md) · [Nederlands · Dutch](README.nl.md) · [Polski · Polish](README.pl.md) · [Română · Romanian](README.ro.md) · [ไทย · Thai](README.th.md) · [Українська · Ukrainian](README.uk.md) · [Tiếng Việt · Vietnamese](README.vi.md)

</details>
<!-- language-selector:end -->

> Δημιουργούμε τον πιο καθαρό, γρήγορο και προσαρμόσιμο open-source downloader για Android. Χωρίς διαφημίσεις, χωρίς telemetry — από geeks για geeks.

Το Rayload είναι μια ανεξάρτητη ανεπίσημη μεταφορά του Rayburst για Android με κινητή διεπαφή, κοινή χρήση συστήματος, λήψεις στο παρασκήνιο και τοπικό Aria2 JSON-RPC.

## v1.0.2 Highlights

- Η λίστα εργασιών ξαναχτίστηκε με ατελείωτο κύλισμα και διγραμμικές κάρτες σε πλήρη και συμπαγή προβολή.
- Προστέθηκαν πίνακας διατήρησης οθόνης και αιωρούμενη μπάλα ταχύτητας στην άκρη με ζωντανά στατιστικά, μαζικές ενέργειες και επανεκκίνηση μηχανής.
- Αφαιρέθηκαν ρυθμίσεις μόνο για υπολογιστή, μεταφράστηκε η ειδοποίηση και βελτιώθηκαν τα δικαιώματα Android και το wake lock.
- Βελτιώθηκαν εικονίδιο, διάλογοι, πλοήγηση και θέματα, με μικρότερο μέγεθος APK.

## Εγκατάσταση του πιο πρόσφατου APK ARM64

Κατέβασε το τελευταίο ARM64 APK από αυτή τη σελίδα: [GitHub Releases](https://github.com/shiwushuifeng/Rayload/releases/latest)

### Quick start

1. Κατέβασε το τελευταίο APK από τα Releases και εγκατέστησέ το σε συσκευή ARM64.
2. Δώσε πρόσβαση σε όλα τα αρχεία και επίτρεψε ειδοποιήσεις για λήψεις στο παρασκήνιο.
3. Πάτησε **+** ή μοιράσου έναν σύνδεσμο από οποιονδήποτε browser για την πρώτη λήψη.

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

### Η ειδοποίηση δεν έχει κουμπιά παύσης/συνέχισης

Επίτρεψε τις ειδοποιήσεις, απενεργοποίησε τη βελτιστοποίηση μπαταρίας και άνοιξε ξανά την εφαρμογή. Ορισμένες προσαρμοσμένες ROM κρύβουν τις ενέργειες μέχρι να επανεκκινηθεί η υπηρεσία foreground.

### Οι λήψεις σταματούν στο παρασκήνιο

Εξαίρεσε το Rayload από περιορισμούς μπαταρίας και βαθύ ύπνου. Σε αυστηρές ROM βοηθά να μένει η εφαρμογή ανοιχτή ή να χρησιμοποιείς τη λειτουργία διατήρησης οθόνης κατά τη φόρτιση.

### Ο browser δεν συνδέεται με το RPC

Το Chrome για Android δεν φορτώνει επεκτάσεις υπολογιστή. Χρησιμοποίησε Firefox για Android ή Chromium με χειροκίνητη εγκατάσταση επεκτάσεων και βάλε διεύθυνση και RPC Secret από **Ρυθμίσεις → Για προχωρημένους**.

### Εμφανίζεται «Η εφαρμογή δεν εγκαταστάθηκε» κατά την αλλαγή build

Τα Release και Debug APK έχουν διαφορετικές υπογραφές. Απεγκατέστησε την τρέχουσα έκδοση πριν εγκαταστήσεις την άλλη.

## Roadmap

[View the full roadmap](../ROADMAP.md) — οδικός χάρτης προϊόντος και μηχανικής.

## License

Rayload is distributed under the [GPL-3.0 License](../LICENSE). It is based on
[Rayburst](https://github.com/AnInsomniacy/rayburst) by **AnInsomniacy**, whose
original work is licensed under the MIT License; see [NOTICE](../NOTICE) for
third-party attributions. Rayload is an independent, unofficial port.

> ⭐ Αν το Rayload σου εξοικονομεί χρόνο, δώσε ένα αστέρι ώστε να το ανακαλύψουν περισσότεροι χρήστες Android.

> ❤️ Like this project? [Sponsor Rayload](../SPONSOR.md)
