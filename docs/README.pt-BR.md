# Rayload — gerenciador de downloads para Android

<!-- language-selector:start -->
<details>
<summary>🌐 Language / 语言 / 語言 / Язык</summary>

[English + 简体中文](../README.md) · [简体中文 · Chinese Simplified](README.zh-CN.md) · [繁體中文 · Chinese Traditional](README.zh-TW.md) · [日本語 · Japanese](README.ja.md) · [한국어 · Korean](README.ko.md) · [Français · French](README.fr.md) · [Deutsch · German](README.de.md) · [Español · Spanish](README.es.md) · [**Português · Portuguese (Brazil)**](README.pt-BR.md) · [Русский · Russian](README.ru.md) · [Türkçe · Turkish](README.tr.md) · [العربية · Arabic](README.ar.md) · [Български · Bulgarian](README.bg.md) · [Català · Catalan](README.ca.md) · [Ελληνικά · Greek](README.el.md) · [فارسی · Persian](README.fa.md) · [Magyar · Hungarian](README.hu.md) · [हिन्दी · Hindi](README.hi.md) · [Bahasa Indonesia · Indonesian](README.id.md) · [Italiano · Italian](README.it.md) · [Norsk Bokmål · Norwegian](README.nb.md) · [Nederlands · Dutch](README.nl.md) · [Polski · Polish](README.pl.md) · [Română · Romanian](README.ro.md) · [ไทย · Thai](README.th.md) · [Українська · Ukrainian](README.uk.md) · [Tiếng Việt · Vietnamese](README.vi.md)

</details>
<!-- language-selector:end -->

> Estamos criando o baixador open source para Android mais limpo, rápido e personalizável. Sem anúncios, sem telemetria — feito por geeks, para geeks.

O Rayload é um porte independente e não oficial do Rayburst para Android, com interface móvel, compartilhamento do sistema, downloads em segundo plano e Aria2 JSON-RPC local.

## v1.0.2 Highlights

- Lista de tarefas reconstruída com rolagem infinita e cartões de duas linhas nos modos completo e compacto.
- Novo painel de tela sempre ativa e medidor flutuante fixado na borda, com estatísticas, ações em massa e reinício do motor.
- Configurações exclusivas de desktop removidas, notificação localizada e integração de permissões e wake lock aprimorada.
- Ícone, diálogos, navegação e temas refinados, com APK menor.

## Instalar o APK ARM64 mais recente

Baixe o APK ARM64 mais recente nesta página: [GitHub Releases](https://github.com/shiwushuifeng/Rayload/releases/latest)

### Quick start

1. Baixe o APK mais recente em Releases e instale em um dispositivo ARM64.
2. Conceda acesso a todos os arquivos e permita notificações para baixar em segundo plano.
3. Toque em **+** ou compartilhe um link de qualquer navegador para iniciar o primeiro download.

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

### A notificação não tem botões de pausar ou continuar

Permita notificações, desative a otimização de bateria e reabra o aplicativo. Algumas ROMs personalizadas ocultam as ações até o serviço em primeiro plano reiniciar.

### Os downloads param em segundo plano

Exclua o Rayload das restrições de bateria e suspensão profunda. Em ROMs agressivas, manter o app aberto ou usar o modo de tela sempre ativa durante o carregamento ajuda.

### Meu navegador não conecta ao RPC

O Chrome para Android não carrega extensões de desktop. Use Firefox para Android ou um Chromium com instalação manual de extensões e informe o endereço e o segredo RPC em **Configurações → Avançado**.

### Aparece “Aplicativo não instalado” ao trocar de build

APKs Release e Debug usam assinaturas diferentes. Desinstale a versão atual antes de instalar a outra variante.

## Roadmap

[View the full roadmap](../ROADMAP.md) — roteiro de produto e engenharia.

## License

Rayload is licensed under the [GPL-3.0 License](../LICENSE). It is based on
[Rayburst](https://github.com/AnInsomniacy/rayburst) by **AnInsomniacy**, whose
original work is licensed under the MIT License. The bundled Aria2 Next
(`aria2-next`) engine is licensed under **GPL-2.0-or-later**. See
[NOTICE](../NOTICE) for all third-party attributions and license notices.
Rayload is an independent, unofficial port.

> ⭐ Se o Rayload economiza seu tempo, deixe uma estrela para mais usuários Android descobrirem o projeto.

> ❤️ Like this project? [Sponsor Rayload](../SPONSOR.md)
