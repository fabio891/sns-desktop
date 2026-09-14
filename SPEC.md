# SPEC — Aplicação Desktop SNS (casca Tauri v2)

Data: 2026-09-14
Estado: CONCLUÍDO no que respeita a código e configuração — **compilação e comportamento por verificar** (dependem do CI)

## 1. Escopo fechado

Aplicação desktop leve que envolve a aplicação web do SNS numa janela nativa (Tauri v2), apontando para o servidor. A UI continua a ser a web — o binário é só a casca.

1. **Projecto Tauri v2** em `/opt/GAPH/sns-desktop`, repositório próprio, separado do repo Laravel `sns-angola` (que está a servir produção).
2. **Janela nativa**: 1280x800, redimensionável, centrada no ecrã, com a barra de título do sistema operativo. Sem qualquer UI de navegador (sem barra de endereço, sem separadores, sem botões de navegação).
3. **Links externos** abrem no navegador predefinido do SO, nunca dentro da janela.
4. **Ecrã de fallback offline**: sem rede ou servidor inacessível → página local com «Sem conexão com a internet. Verifique sua rede e tente novamente.» e botão **Tentar Reconectar**.
5. **Atalhos de navegador restringidos** em builds de produção (F5, Ctrl/Cmd+R, F12, Ctrl+Shift+I/J/C, Ctrl+U). Em desenvolvimento ficam todos disponíveis.
6. **Scripts npm** `tauri:dev` e `tauri:build`.
7. **CI** que gera `.exe` (NSIS+MSI), `.dmg` e `.deb`/`.AppImage`.

### FORA DE ESCOPO

- **Compilação local neste servidor**: não há Rust nem bibliotecas webkit2gtk. A compilação corre no GitHub Actions. Este servidor é Linux e não gera `.exe`/`.dmg` de qualquer forma.
- **Assinatura de código** (certificado Windows Authenticode / Apple Developer ID): sem certificados, os instaladores mostram aviso do SmartScreen/Gatekeeper. Fase própria se for necessário.
- **Auto-update** (Tauri updater): não implementado.
- **Ícones oficiais**: por agora ficam os ícones do template Tauri; a substituir pelos do SNS.
- **Modo offline funcional**: o fallback é uma página informativa com reconexão, não um cliente offline. O SNS tem sincronização offline própria noutra camada (SQLCipher), que não é tocada aqui.
- **Alterações ao backend Laravel**: nenhuma. Este projecto não toca em `/opt/GAPH/sns-angola`.

## 2. Decisões e regras

### Nome
- `productName`: **SNS**
- Título da janela: **SNS — Sistema Nacional de Saúde**
- `identifier`: `ao.ntcao.sns`

### Endereço do servidor
- Por omissão `https://saudenamao.ntcao.com`, gerado em `src/config.js` por `scripts/config.mjs`.
- Sobrescrevível sem editar código: `SNS_APP_URL=https://outro.dominio npm run tauri:build` (e variável `SNS_APP_URL` nas configurações do repositório para o CI).
- O Rust tem a sua própria lista de hosts internos (`HOSTS_INTERNOS` em `src-tauri/src/lib.rs`); um domínio novo tem de ser acrescentado lá, senão os links para ele são tratados como externos. Documentado no `README.md`.

### Janela
- A janela é criada em **Rust** (`WebviewWindowBuilder`) e não em `tauri.conf.json`, porque `on_navigation` e `initialization_script` só existem no builder. `tauri.conf.json` fica sem `app.windows`.
- A janela não expõe IPC (`withGlobalTauri` desligado), tanto para a página local como para as páginas remotas do servidor.

### Arranque e fallback
1. A janela abre `index.html` **local** (a casca).
2. A casca faz `GET {SERVIDOR}/up` com `mode: no-cors`, `cache: no-store` e timeout de 5s.
3. Responde → `location.replace(SERVIDOR)`. Falha → mostra o ecrã de fallback com o botão de reconexão (mesma lógica, repetível).
4. `no-cors` é deliberado: o endpoint `/up` é do Laravel e não devolve cabeçalhos CORS; neste modo o pedido não é bloqueado por CORS e um erro de rede continua a rejeitar a promessa, que é o sinal que queremos.

### Links externos
- `on_navigation` (Rust): host interno ou esquema `tauri` → navegação permitida; qualquer outro → `open::that(url)` (navegador do SO) e a navegação é cancelada.
- Como links com `target="_blank"` não passam pelo `on_navigation`, um script injectado em todas as páginas intercepta o clique e navega na própria janela, deixando o `on_navigation` decidir. Assim há **um único** ponto de decisão (o Rust).

### Atalhos
- Script injectado **apenas em builds de release** (`#[cfg(not(debug_assertions))]`).
- Bloqueados: `F5`, `Ctrl/Cmd+R`, `F12`, `Ctrl+Shift+I`, `Ctrl+Shift+J`, `Ctrl+Shift+C`, `Ctrl+U`.
- **Não** bloqueados de propósito: `Ctrl+P` (impressão — o sistema tem relatórios/faturas em PDF que os utilizadores imprimem) e `Ctrl+F` (procura na página).

## 3. Restrições técnicas

- Tauri 2, Rust edition 2021, Node 22 + npm.
- Dependência extra: crate `open` (abrir URL no navegador do SO). O plugin `tauri-plugin-opener` do template foi **removido** por não ser usado.
- `tauri.conf.json` com `csp: null` (o template não define CSP e a aplicação remota traz os seus próprios cabeçalhos; impor CSP aqui arriscava partir a app).
- Não alterar nada em `/opt/GAPH/sns-angola`.

## 4. Critérios de aceitação (Definition of Done)

- [x] Projecto em `/opt/GAPH/sns-desktop` com repo git próprio e `node_modules` ignorado.
- [x] `npm install` completa e `npx tauri info` reconhece o CLI v2 (reportando Rust/webkit como ausentes **neste** servidor, o que é esperado).
- [x] `tauri.conf.json` válido contra o schema `https://schema.tauri.app/config/2`, com `productName: SNS` e sem `app.windows`.
- [x] `package.json` expõe `npm run tauri:dev` e `npm run tauri:build`, ambos a gerar `src/config.js` antes.
- [x] `src/config.js` gerado com `SNS_APP_URL` quando a variável existe, e com o valor por omissão quando não existe.
- [x] `src-tauri/src/lib.rs` cria a janela com título, 1280x800, redimensionável, centrada, `on_navigation` e `initialization_script`.
- [x] Página local: mostra o ecrã de fallback com a mensagem pedida e o botão **Tentar Reconectar**; os selectores usados pelo JS existem todos no HTML *(presença verificada estaticamente — o comportamento de redireccionar/falhar não foi executado).*
- [x] Workflow `.github/workflows/desktop-release.yml` com matrix macOS (2 alvos), Ubuntu e Windows.
- [x] `git init` + commit inicial + deploy key dedicada (`~/.ssh/deploy_sns_desktop`) com alias `github-sns-desktop`.
- [x] **Verificação honesta**: nada disto é compilado neste servidor. A validação local é estrutural (JSON, npm, scripts Node); a validação de compilação e de comportamento é o CI, depois do Fábio criar o repo e adicionar a deploy key.

## 5. O que NÃO foi verificado

- Compilação do Rust e arranque real da aplicação. `npm run tauri:info` confirma o motivo: `rustc`, `cargo`, `webkit2gtk-4.1` e `rsvg2` **não instalados** neste servidor, que também é headless (sem display).
- Comportamento da janela, dos links externos, dos atalhos e do fallback offline em execução — só no CI e no PC do Fábio.
- O workflow ainda **não correu**. O repositório foi criado como **privado** (a decisão inicial era público): em repo privado os minutos de runners macOS são pagos a 10x, pelo que o `v*` só deve ser criado depois de decidir a visibilidade ou limitar os alvos.
