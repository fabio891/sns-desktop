# SNS — Aplicação Desktop

Casca desktop (Tauri v2) da aplicação web do **SNS — Sistema Nacional de Saúde**. O binário é um invólucro fino: a interface continua a ser a aplicação web servida pelo servidor, aberta numa janela nativa sem qualquer UI de navegador.

## O que a casca faz

- Abre a janela em **1280x800**, redimensionável e centrada, com a barra de título do sistema operativo.
- No arranque testa `GET {SERVIDOR}/up` (timeout de 5s). Se responder, navega para a aplicação; se não, mostra o ecrã de fallback com o botão **Tentar Reconectar**.
- Envia **links externos** para o navegador predefinido do sistema, nunca abrindo dentro da janela.
- Em builds de produção bloqueia atalhos de navegador (`F5`, `Ctrl/Cmd+R`, `F12`, `Ctrl+Shift+I/J/C`, `Ctrl+U`). Em desenvolvimento ficam disponíveis. `Ctrl+P` (impressão) e `Ctrl+F` continuam a funcionar de propósito.

## Requisitos

- Node 22+ e npm
- Rust (via [rustup](https://rustup.rs))
- Linux: `libwebkit2gtk-4.1-dev`, `libgtk-3-dev`, `librsvg2-dev`, `build-essential`

## Comandos

```bash
npm install          # dependências
npm run tauri:dev    # desenvolvimento (gera src/config.js e abre a app)
npm run tauri:build  # instalador para o SO onde corres o comando
npm run tauri:info   # diagnóstico do ambiente
```

O `.exe` só se gera em Windows, o `.dmg` só em macOS e o `.deb`/`.AppImage` só em Linux. Para os três, usa o CI (ver abaixo).

## Endereço do servidor

O valor por omissão está em `scripts/config.mjs` e é escrito em `src/config.js` a cada build. Para apontar a outro servidor, sem editar código:

```bash
SNS_APP_URL=https://outro.dominio.ao npm run tauri:build
```

O host de `SNS_APP_URL` entra automaticamente na lista de hosts internos. Domínios servidos pelo mesmo servidor mas com outro nome (aliases) vão em `SNS_APP_HOSTS`, separados por vírgulas — esta variável **substitui** a lista por omissão:

```bash
SNS_APP_URL=https://novo.dominio.ao SNS_APP_HOSTS=novo.dominio.ao,alias.dominio.ao npm run tauri:build
```

`scripts/config.mjs` escreve as duas fontes a partir do mesmo valor (`src/config.js` para o health-check e `src-tauri/src/hosts_gerados.rs` para o filtro de navegação). Não há lista de hosts escrita à mão no Rust — um domínio novo não precisa de ser acrescentado a lado nenhum.

No CI, define a variável `SNS_APP_URL` nas *Settings → Variables* do repositório (não é segredo).

## Instaladores (CI)

`.github/workflows/desktop-release.yml` compila em paralelo:

| Sistema | Artefactos |
|---|---|
| Windows | `.exe` (NSIS) e `.msi` |
| macOS | `.dmg` (Apple Silicon e Intel) |
| Linux | `.deb` e `.AppImage` |

Corre em tags `v*` e por execução manual (Actions → *Desktop SNS* → *Run workflow*). Os instaladores ficam como artefactos do workflow.

**Os instaladores não estão assinados.** Sem certificado de código, o Windows mostra o aviso do SmartScreen e o macOS exige abrir com Ctrl+clique. Assinar exige certificados pagos (Authenticode / Apple Developer ID) e é trabalho à parte.

## Estrutura

```
src/                      casca local (HTML/CSS/JS) — só o ecrã de ligação/offline
  config.js               gerado por scripts/config.mjs
src-tauri/
  tauri.conf.json         metadados do bundle (nome, ícones); a janela é criada em Rust
  src/lib.rs              janela, links externos, bloqueio de atalhos
  src/hosts_gerados.rs    gerado por scripts/config.mjs (hosts internos)
  capabilities/           permissões (mínimas: só core)
scripts/config.mjs        gera src/config.js e src-tauri/src/hosts_gerados.rs
```

Os ícones em `src-tauri/icons/` são ainda os do template do Tauri — a substituir pelos oficiais do SNS (`npm run tauri icon caminho/para/logo.png`).
