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
- [x] S6.6 Fábio instalou a v0.1.3 no **Windows** e abriu-a
      *Resultado: **falhou**. Janela preta/vazia, um navegador do sistema abriu um link externo `tauri localhost`, e a app nunca chegou ao `/login`. Diagnóstico completo em `SPEC.md` §6. É a primeira verificação em execução do projecto — e confirma exactamente porque é que ela era necessária.*

**Resultado (v0.1.3)**: o Rust **compila** nos 4 alvos, os instaladores são gerados e estão publicados em <https://github.com/fabio891/sns-desktop/releases/tag/v0.1.3>. Em execução **não funcionava no Windows** — **compilar não é o mesmo que funcionar**.

## [S7] Correcção v0.1.4 — arranque no Windows
- [x] S7.1 `navegacao_interna` passa a reconhecer origens por pares `(esquema, host)` exactos — `("tauri","localhost")` e `("http","tauri.localhost")`
      *Verificado (estrutural): revisão do `lib.rs`; a comparação é exacta, sem sufixos `.localhost` (evita reabrir a falha do `is_local_url` do Tauri). Compilação por confirmar no CI.*
- [x] S7.2 Fim da lista de hosts duplicada: `scripts/config.mjs` gera `src-tauri/src/hosts_gerados.rs` além de `src/config.js`
      *Verificado: (A) omissão → `saudenamao.ntcao.com, gaph.ntcao.com, sns-angola.ntcao.com`; (B) `SNS_APP_URL=https://novo.exemplo.ao/` → host novo entra, barra final normalizada; (C) `SNS_APP_HOSTS="a.exemplo.ao, b.exemplo.ao"` → substitui os extras e o host do servidor mantém-se; (D) `SNS_APP_HOSTS='nao valido!'` → exit 1 com mensagem; (E) `SNS_APP_URL=exemplo.ao` → exit 1. Valor por omissão reposto no fim.*
- [x] S7.3 `README.md` — secção do endereço reescrita (documenta `SNS_APP_HOSTS` e o ficheiro gerado); aviso do `HOSTS_INTERNOS` à mão removido
- [x] S7.4 Versão 0.1.4 alinhada em `package.json`, `tauri.conf.json` e `Cargo.toml`
- [x] S7.5 `SCRIPT_LIGACOES` passa a cobrir as três vias que pedem janela nova — `<a target="_blank">` (já existia), `<form target="_blank">` e `window.open()`
      *Verificado: os 2 scripts injectados extraídos do `lib.rs` e validados com `new vm.Script(...)` — sintaxe OK. O `submit()` do JS não volta a disparar o evento de submit, logo não há ciclo. Comportamento em execução por confirmar no Windows.*
      *Origem: `wry 0.55` liga downloads por omissão (a hipótese inicial estava errada); o problema real eram os `<form target="_blank">` dos PDFs (`relatorios/index.blade.php`) e o `window.open` do CSV (`:201`), que não passavam pelo `on_navigation`.*
- [x] S7.6 Tag `v0.1.4` → CI compila os 4 alvos e publica o release
      *Verificado: run #5 concluído com sucesso (19:30→19:39 UTC) e release v0.1.4 publicado com 6 instaladores — `.exe`, `.msi`, dois `.dmg`, `.deb` e `.AppImage`.*
- [ ] S7.7 Fábio instala a v0.1.4 no Windows e confirma: arranque sem link externo, chegada ao `/login`, fallback offline, links externos, atalhos
- [ ] S7.8 Testar as exportações (PDF/CSV de relatórios, CSV de finanças, anexos de exames) e uploads conforme os critérios em `SPEC.md` §6
- [x] S7.9 *(projecto `sns-angola`)* `trustProxies` com `Request::HEADER_X_FORWARDED_PROTO` em `bootstrap/app.php` — commit `0e88739`, push para `main`
      *Verificado: `php -l` sem erros; `curl -sI https://saudenamao.ntcao.com/` → `location: https://saudenamao.ntcao.com/login` (antes `http://`) e igual em `gaph.ntcao.com`; `/login` e `/up` a 200. Sem cache de config, logo sem necessidade de reiniciar o php-fpm.*
      *Resolvido a 2026-09-15: `APP_URL` passou a `https://saudenamao.ntcao.com` e o fallback duplicado em `BrevoEmailService.php:25` foi removido. O `sns-angola-queue.service` foi corrigido (`Restart=always`, `User=www-data`) por estar morto desde 05/09 — ver `SPEC.md` §6.*

## [S8] Identidade — marca SaúdeNamao na app e no sistema web (v0.1.5)
- [x] S8.1 *(projecto `sns-angola`)* `gerar_logo.py` — geometria da marca extraída para `desenhar_marca()` e modos `--icone` e `--web`
      *Verificado: logótipo dos documentos regenerado para `/tmp` com md5 igual ao publicado (`3d3b884f087b3497fa5d2589615cbb5a`) — o refactor não alterou o desenho. `--icone` → 1024×1024 RGBA; `--web` → `favicon.png` 32, `marca.png` 256, `icon-{32,64,192,512}.png` e `favicon.ico` com 16/32/48 (tamanhos confirmados relendo o ICO com PIL).*
- [x] S8.2 Ícones do desktop regenerados com `npx tauri icon` a partir da marca
      *Verificado: 16 ficheiros em `src-tauri/icons/` com md5 novo (o `icon.png` passou de `d07d64dd…` para `e3642540…`). As pastas `android/`, `ios/` e o `64x64.png` criados pelo CLI foram removidos — nada os referenciava e a app é só de desktop (`targets: all`).*
- [x] S8.3 *(projecto `sns-angola`)* Marca aplicada no sistema web — favicon, ícones da PWA, login, navbar e header do portal
      *Verificado em produção: `/login` a 200 com a marca no cartão (72×72) e na navbar; `marca.png`, `favicon.png`, `icon-512.png` e `favicon.ico` a 200 com os bytes novos (7919, 1215, 14969 e 3764 — o `favicon.ico` estava vazio). Sem cache de config, aplica-se sem reiniciar o php-fpm. A conferência visual no browser fica para o Fábio.*
- [x] S8.4 Versão 0.1.5 alinhada em `package.json`, `tauri.conf.json` e `Cargo.toml`
- [ ] S8.5 Tag `v0.1.5` → CI compila os 4 alvos e publica o release
- [ ] S8.6 Fábio confirma no Windows: ícone do executável, da barra de tarefas e do menu Iniciar
