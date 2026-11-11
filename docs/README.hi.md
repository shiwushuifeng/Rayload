# Rayload — Android डाउनलोडर

<!-- language-selector:start -->
<details>
<summary>🌐 Language / 语言 / 語言 / Язык</summary>

[English + 简体中文](../README.md) · [简体中文 · Chinese Simplified](README.zh-CN.md) · [繁體中文 · Chinese Traditional](README.zh-TW.md) · [日本語 · Japanese](README.ja.md) · [한국어 · Korean](README.ko.md) · [Français · French](README.fr.md) · [Deutsch · German](README.de.md) · [Español · Spanish](README.es.md) · [Português · Portuguese (Brazil)](README.pt-BR.md) · [Русский · Russian](README.ru.md) · [Türkçe · Turkish](README.tr.md) · [العربية · Arabic](README.ar.md) · [Български · Bulgarian](README.bg.md) · [Català · Catalan](README.ca.md) · [Ελληνικά · Greek](README.el.md) · [فارسی · Persian](README.fa.md) · [Magyar · Hungarian](README.hu.md) · [**हिन्दी · Hindi**](README.hi.md) · [Bahasa Indonesia · Indonesian](README.id.md) · [Italiano · Italian](README.it.md) · [Norsk Bokmål · Norwegian](README.nb.md) · [Nederlands · Dutch](README.nl.md) · [Polski · Polish](README.pl.md) · [Română · Romanian](README.ro.md) · [ไทย · Thai](README.th.md) · [Українська · Ukrainian](README.uk.md) · [Tiếng Việt · Vietnamese](README.vi.md)

</details>
<!-- language-selector:end -->

> हम Android के लिए सबसे साफ़, सबसे तेज़ और सबसे अनुकूलन योग्य ओपन-सोर्स डाउनलोडर बना रहे हैं। कोई विज्ञापन नहीं, कोई टेलीमेट्री नहीं — गीक्स द्वारा, गीक्स के लिए।

Rayload, Rayburst का एक स्वतंत्र और अनौपचारिक Android पोर्ट है जिसमें मोबाइल इंटरफ़ेस, सिस्टम शेयर, बैकग्राउंड डाउनलोड और स्थानीय Aria2 JSON-RPC शामिल हैं।

## v1.0.2 Highlights

- टास्क सूची को अनंत स्क्रॉल और पूर्ण व कॉम्पैक्ट घनत्व वाले दो-पंक्ति कार्ड के साथ फिर से बनाया गया।
- स्क्रीन-ऑन डैशबोर्ड और किनारे पर टिकने वाला स्पीड बॉल जोड़ा गया, जिसमें लाइव आँकड़े, बल्क नियंत्रण और इंजन रीस्टार्ट शामिल हैं।
- डेस्कटॉप-केवल सेटिंग्स हटाई गईं, सूचना का स्थानीयकरण किया गया और Android अनुमतियाँ व wake lock व्यवहार बेहतर किया गया।
- आइकन, डायलॉग, नेविगेशन और थीम को निखारा गया और APK का आकार घटाया गया।

## नवीनतम ARM64 APK इंस्टॉल करें

इस पेज से नवीनतम ARM64 APK डाउनलोड करें: [GitHub Releases](https://github.com/shiwushuifeng/Rayload/releases/latest)

### Quick start

1. Releases से नवीनतम APK डाउनलोड करें और ARM64 डिवाइस पर इंस्टॉल करें।
2. बैकग्राउंड डाउनलोड के लिए सभी फ़ाइलों तक पहुँच और सूचनाओं की अनुमति दें।
3. पहला डाउनलोड शुरू करने के लिए **+** दबाएँ या किसी ब्राउज़र से लिंक साझा करें।

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

### सूचना में पॉज़/रिज़्यूम बटन नहीं है

सूचनाओं की अनुमति दें, बैटरी ऑप्टिमाइज़ेशन बंद करें और ऐप को फिर से खोलें। कुछ कस्टम Android ROM फोरग्राउंड सेवा फिर शुरू होने तक सूचना क्रियाएँ छिपा देते हैं।

### बैकग्राउंड में डाउनलोड रुक जाते हैं

Rayload को बैटरी और डीप-स्लीप प्रतिबंधों से बाहर रखें। आक्रामक ROM पर ऐप खुला रखना या चार्जिंग के दौरान स्क्रीन-ऑन मोड उपयोग करना मदद करता है।

### मेरा ब्राउज़र RPC से कनेक्ट नहीं होता

Android के लिए Chrome डेस्कटॉप एक्सटेंशन नहीं लोड करता। Firefox for Android या एक्सटेंशन साइडलोड करने वाला Chromium उपयोग करें और **सेटिंग्स → उन्नत** में पता व RPC Secret डालें।

### बिल्ड बदलते समय “ऐप इंस्टॉल नहीं हुआ” दिखता है

Release और Debug APK के हस्ताक्षर अलग होते हैं। दूसरा संस्करण इंस्टॉल करने से पहले मौजूदा संस्करण को अनइंस्टॉल करें।

## Roadmap

[View the full roadmap](../ROADMAP.md) — उत्पाद और इंजीनियरिंग रोडमैप.

## License

Rayload is based on [Rayburst](https://github.com/AnInsomniacy/rayburst) by
**AnInsomniacy** and follows the [MIT License](LICENSE). It is an independent,
unofficial port.

> ⭐ अगर Rayload आपका समय बचाता है, तो स्टार दें ताकि और Android उपयोगकर्ता इसे खोज सकें।
