# SPEC — Aplicação Desktop SNS (casca Tauri v2)

Data: 2026-09-14 — actualizado a 2026-09-15 com a correcção v0.1.4 (ver §6)
Estado: a v0.1.3 **não arrancava no Windows** (janela preta, sem chegar ao servidor). Correcção escrita na v0.1.4; **compilação e comportamento por verificar** (dependem do CI e de uma instalação real)

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
- [x] Workflow `.github/workflows/desktop-release.yml` com matrix macOS (2 alvos), Ubuntu e Windows, e job `publicar` que cria um release na tag com os instaladores.
      *Verificado: release `v0.1.3` publicado com 6 ficheiros e download anónimo a funcionar (HTTP 200 sem sessão).*
- [x] `git init` + commit inicial + deploy key dedicada (`~/.ssh/deploy_sns_desktop`) com alias `github-sns-desktop`.
- [x] **Verificação honesta**: nada disto é compilado neste servidor. A validação local é estrutural (JSON, npm, scripts Node); a validação de compilação e de comportamento é o CI, depois do Fábio criar o repo e adicionar a deploy key.

## 5. O que NÃO foi verificado

- Compilação do Rust e arranque real da aplicação. `npm run tauri:info` confirma o motivo: `rustc`, `cargo`, `webkit2gtk-4.1` e `rsvg2` **não instalados** neste servidor, que também é headless (sem display).
- Comportamento da janela, dos links externos, dos atalhos e do fallback offline em execução — só no CI e no PC do Fábio.
- **Comportamento em execução.** Ninguém instalou nem abriu a aplicação. A compilação passou, mas isso **não** prova que a janela abre a 1280x800, que o fallback offline aparece sem rede, que os links externos vão para o navegador do SO ou que os atalhos ficam bloqueados em release. Só se confirma instalando um artefacto.

  *(Histórico: a primeira tentativa, tag `v0.1.0`, falhou sem correr um único passo — a conta GitHub estava bloqueada por facturação (`The job was not started because your account is locked due to a billing issue.`). Resolvido pelo Fábio; a tag `v0.1.1` já correu normalmente. As runs em `main` com `startup_failure`/`path: BuildFailed` tinham a mesma origem — a visibilidade do repositório era irrelevante.)*

## 6. Correcção v0.1.4 — a app não arrancava no Windows

### Sintoma relatado (instalação real, 2026-09-15)

Fábio instalou o `.exe`/`.msi` no **Windows**. Ao abrir: a janela ficou **preta/vazia** e um **navegador do sistema abriu um link externo** apontado a `tauri localhost`. A app nunca chegou ao `/login` do SNS. Este é o primeiro teste em execução de sempre — e o veredicto é que a v0.1.3 **não funcionava**.

### Causa raiz

`navegacao_interna()` (em `lib.rs`) só reconhecia duas coisas: o esquema `tauri` e o host `localhost`. Mas o Tauri v2 serve as páginas locais em origens **diferentes por plataforma**:

| Plataforma | Origem da casca local |
|---|---|
| macOS · Linux · iOS | `tauri://localhost` |
| **Windows · Android** | **`http://tauri.localhost`** |

No Windows, a página local chega com esquema `http` e host `tauri.localhost` — não bate certo com nenhuma das duas condições. Consequência em cadeia:

1. `on_navigation` é chamado logo para a **primeira** navegação, a da própria `index.html`.
2. Como `navegacao_interna` devolve `false`, a casca classifica a sua própria página como link externo e chama `open_url()`.
3. O navegador do sistema abre `http://tauri.localhost/index.html`, que fora da app não existe (nada escuta nesse host).
4. A navegação é cancelada dentro da janela → **janela preta**. Como a `index.html` nunca corre, o `main.js` nunca faz o health-check a `/up` nem o `location.replace` → **nunca chega ao servidor**.

Ou seja: o erro não estava nos pedidos ao servidor (que nunca aconteceram) mas no filtro que decide o que é interno. Passou despercebido porque no macOS/Linux o esquema `tauri://` faz o código entrar pelo caminho certo.

### O que foi corrigido

1. **Deteção de origem por pares `(esquema, host)` exactos.** `ORIGENS_DA_CASCA` lista `("tauri", "localhost")` e `("http", "tauri.localhost")`. A comparação é exacta de propósito: aceitar qualquer host terminado em `.localhost` reabriria a falha que a Tauri corrigiu no `is_local_url` (aviso de segurança em que URLs remotas passavam por locais no Windows/Android).
2. **Fim da duplicação da lista de hosts.** `HOSTS_INTERNOS` deixou de estar escrita à mão no Rust e passou a ser **gerada**: `scripts/config.mjs` escreve `src/config.js` *e* `src-tauri/src/hosts_gerados.rs` a partir do mesmo valor, com `SNS_APP_URL` (o host do servidor entra sempre) e `SNS_APP_HOSTS` (aliases). Antes, apontar a app a um domínio novo fazia com que o **próprio servidor** fosse tratado como externo — bomba-relógio documentada no README que agora desaparece.

### Riscos identificados mas NÃO corrigidos (precisam de teste)

- **Downloads.** A app faz downloads reais: `FileDownloadController::download` → `Storage::download()` (`routes/web.php:152`), relatórios PDF (`routes/web.php:247`) e exportação de dados do portal (`routes/web.php:354`). O `wry` não liga o handler de download por omissão; é frequente o clique não fazer nada. **Por testar** — não se alterou código sem primeiro confirmar o comportamento.
- **Redirect do Laravel em HTTP.** `GET https://saudenamao.ntcao.com/` devolve `Location: http://saudenamao.ntcao.com/login` (o Laravel está atrás do Cloudflare e não reconhece o `X-Forwarded-Proto`). Salva-se pelo HSTS, mas deve ser corrigido no `APP_URL`/TrustProxies **do projecto `sns-angola`**, fora deste repositório.
- **Ícones** continuam a ser os do template.

### Critérios de aceitação v0.1.4

- [ ] CI compila os 4 alvos com a tag `v0.1.4`.
- [ ] No Windows, a janela abre **sem** abrir nada no navegador do sistema.
- [ ] A casca mostra "A ligar ao servidor…" e chega ao `/login` do SNS dentro da janela.
- [ ] Sem rede: aparece o ecrã "Sem conexão com a internet" e o botão **Tentar Reconectar** volta a ligar quando a rede regressa.
- [ ] Um link externo abre no navegador predefinido; os internos ficam na janela.
- [ ] `F5`/`F12`/`Ctrl+R` bloqueados; `Ctrl+P` e `Ctrl+F` a funcionar.
- [ ] Downloads e uploads testados e reportados (funcionam ou viram fatia própria).
