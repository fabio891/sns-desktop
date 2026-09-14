# TASKS — Aplicação Desktop SNS (casca Tauri v2)

Single Source of Truth. Cada fatia é vertical e verificada no terminal antes de marcar `- [x]`.

## [S1] Estrutura e scripts npm
- [x] S1.1 `package.json` com `tauri:dev`, `tauri:build`, `tauri:info` e `config`
- [x] S1.2 `scripts/config.mjs` — gera `src/config.js` a partir de `SNS_APP_URL` (com validação de URL)
- [x] S1.3 `src/config.js` (valor por omissão comitado) + `.gitignore`
- [x] S1.4 `README.md` — como compilar, onde se muda a URL, o que o CI produz
- [x] S1.5 `npm install` + verificação do CLI
      *Verificado: `npm install` ok (2 pacotes, 0 vulnerabilidades); `tauri-cli 2.11.4`; `node scripts/config.mjs` produz o valor por omissão e `SNS_APP_URL=https://exemplo.ao/app/` produz o valor normalizado sem barra final.*

## [S2] Janela, links externos e atalhos
- [x] S2.1 `tauri.conf.json` — `productName: SNS`, `identifier ao.ntcao.sns`, sem `app.windows`
- [x] S2.2 `Cargo.toml` — metadados (descrição/autoria); `tauri-plugin-opener` **mantido** (usa-se `open_url`, evitando dependência nova)
- [x] S2.3 `lib.rs` — `WebviewWindowBuilder` (título, 1280x800, redimensionável, centrada) + `on_navigation` + scripts injectados
- [x] S2.4 `capabilities/default.json` — mantido (`core:default`, `opener:default`)
- [x] S2.5 Remover artefactos de demonstração do template (`greet`, `src/assets/*`)
      *Verificado: `tauri.conf.json` válido contra o schema oficial 2.11.5 (validado com `ajv` em modo não-unicode; controlo negativo com erro de tipo falhou como esperado); `node --check` em todos os JS; `node -e JSON.parse` em todos os JSON.*

## [S3] Tela fallback offline
- [x] S3.1 `src/index.html` — casca local com estados "a ligar" e "sem ligação"
- [x] S3.2 `src/main.js` — health-check a `/up` (timeout 5s, `no-cors`), redirect, botão Tentar Reconectar
- [x] S3.3 `src/styles.css` — apresentação do fallback
      *Verificado (estrutural): os 4 selectores usados no JS existem no HTML; mensagem e botão presentes. Comportamento em execução não testado (sem display/Rust).*

## [S4] CI de instaladores + repo git
- [x] S4.1 Workflow matrix (macOS aarch64+x64, Ubuntu 22.04, Windows) com `tauri-action` e `--bundles` explícitos
      *Verificado: YAML parseado, 2 eventos, 4 alvos, 9 passos.*
- [x] S4.2 `git init` (`main`) + 36 ficheiros preparados (node_modules e target ignorados)
- [x] S4.3 Deploy key dedicada `~/.ssh/deploy_sns_desktop` + alias `github-sns-desktop` (`ssh -G` confirma hostname/identityfile)
- [ ] S4.4 Push para `github.com:fabio891/sns-desktop` *(bloqueado: depende de o Fábio criar o repo e adicionar a chave pública)*
