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
- [x] S4.4 Push para `github.com:fabio891/sns-desktop`
      *Verificado: `ssh -T git@github-sns-desktop` → «Hi fabio891/sns-desktop! You've successfully authenticated»; `git push -u origin main` → `[new branch] main -> main`; `ls-remote` confirma `refs/heads/main` em `482c352`; `push --dry-run` sem erro de permissão (a deploy key tem escrita).*
      *Nota: o repo foi criado **privado** (a escolha inicial era público). Ver aviso no `SPEC.md` sobre o custo dos runners macOS.*

## [S5] Verificação final
- [x] S5.1 `npm run tauri:info` — confirma CLI 2.11.4 e reporta `rustc`/`cargo`/`webkit2gtk-4.1`/`rsvg2` ausentes (limitação documentada deste servidor)
- [x] S5.2 `SPEC.md`/`TASKS.md` actualizados com o que foi verificado e o que não foi

## [S6] Compilação no CI
- [x] S6.1 Repositório tornado público e tag `v0.1.0` empurrada → workflow disparado (run 34850672333)
- [x] S6.2 Primeira tentativa falhou **sem correr**: `The job was not started because your account is locked due to a billing issue.` — conta GitHub bloqueada por facturação (resolvido pelo Fábio)
- [x] S6.3 Versão alinhada em 0.1.1 (`package.json`, `tauri.conf.json`, `Cargo.toml`) + tag `v0.1.1` → **run 34851970602, os 4 alvos com sucesso**
      *linux=success (479s) · macos-x64=success (331s) · macos-arm64=success (381s) · windows=success (468s). Os 14/14 passos correram no Linux (nos outros, o passo de `apt` é ignorado por não ser Linux).*
- [x] S6.4 Artefactos publicados: `sns-desktop-windows` · `sns-desktop-linux` · `sns-desktop-macos-arm64` · `sns-desktop-macos-x64`
- [x] S6.5 Release automático — primeiro job `publicar` falhou (*não fazia checkout, logo o `gh` não inferia o repositório*); corrigido com `--repo` explícito + erros emitidos como anotações
      *Tag `v0.1.3`, run 34854848177: os 5 jobs com sucesso. Release **publicado** com 6 instaladores: `SNS_0.1.3_x64-setup.exe` (1.3 MB) · `SNS_0.1.3_x64_en-US.msi` (1.9 MB) · `SNS_0.1.3_aarch64.dmg` (1.8 MB) · `SNS_0.1.3_x64.dmg` (1.9 MB) · `SNS_0.1.3_amd64.deb` (2.2 MB) · `SNS_0.1.3_amd64.AppImage` (76.3 MB)*
      *Download **anónimo** confirmado: página do release HTTP 200 e os ficheiros HTTP 200 `application/octet-stream`, sem sessão GitHub — que era o requisito para partilhar com terceiros.*
- [ ] S6.6 Alguém instalar e abrir a aplicação para confirmar o comportamento (janela, fallback offline, links externos, atalhos)

**Resultado**: o Rust **compila** nos 4 alvos, os instaladores são gerados e estão publicados em <https://github.com/fabio891/sns-desktop/releases/tag/v0.1.3>. Continua por confirmar o comportamento em execução — **compilar não é o mesmo que funcionar**.
