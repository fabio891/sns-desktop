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

### Segunda correcção (mesma sessão) — janelas novas e exportações

O `wry 0.55` (o que o `tauri 2.11.5` usa) **liga os downloads por omissão em todas as plataformas** — a preocupação inicial com o handler de download não se confirmou, e não se escreveu código para ela. O problema estava noutro sítio, e é concreto.

**A lacuna:** o `SCRIPT_LIGACOES` só interceptava cliques em `<a target="_blank">`. As exportações do SNS usam outras duas vias que **não** passam pelo `on_navigation`, porque pedem uma janela nova em vez de navegar:

| Via | Onde | Comportamento sem o script |
|---|---|---|
| `<form target="_blank">` | `relatorios/index.blade.php` (4 formulários PDF) | janela nova, pedido não classificado |
| `window.open(url, '_blank')` | `relatorios/index.blade.php:201` (`gerarCsv`) | idem |
| `<a target="_blank">` | `portal/telemedicina/show.blade.php:48` (Jitsi) | já coberto |

Sem isto, o WebView abria uma janela nua por cima da aplicação e o filtro de links do Rust nunca via o pedido — o que contrariava o desenho declarado («um único ponto de decisão»). Os downloads por navegação normal (`<a href>` sem `target`, como os anexos de exames e os CSV de finanças) sempre estiveram cobertos, porque esses passam pelo `on_navigation`.

**Correcção:** o script passa a cobrir as três vias, reencaminhando-as todas para a própria janela. A partir daí são indistinguíveis de um clique normal e quem decide é o Rust — internos ficam na app, externos vão para o navegador do sistema.

### Terceira correcção — redirect do Laravel em HTTP (repo `sns-angola`)

`GET https://saudenamao.ntcao.com/` devolvia `Location: http://saudenamao.ntcao.com/login`. O Laravel está atrás do Traefik e do Cloudflare (que terminam o TLS) mas não tinha `trustProxies` configurado, pelo que não via o `X-Forwarded-Proto` e assumia `http`. Além dos redirects, isto afectava os links de email e a reposição de password.

**Corrigido** em `bootstrap/app.php` do `sns-angola` (commit `0e88739`, branch `main`):

```php
$middleware->trustProxies(at: '*', headers: Request::HEADER_X_FORWARDED_PROTO);
```

Confia-se **só no esquema** (`X-Forwarded-Proto`). O `X-Forwarded-For` ficou de fora de propósito: a porta 8083 escuta em `0.0.0.0`, por isso confiar no IP encaminhado tornaria o endereço do cliente falsificável em logs e rate-limiting. Verificado depois da alteração: o redirect passou a `https://` em `saudenamao.ntcao.com` e em `gaph.ntcao.com`, e `/login` e `/up` continuam a responder 200. Não há cache de config (`bootstrap/cache/config.php` não existe) e o `bootstrap/app.php` não é cacheado, pelo que a alteração aplica-se sem reiniciar o php-fpm.

### `APP_URL` alinhado com o domínio de entrada

Resolvido no `sns-angola` a 2026-09-15:

- `.env` → `APP_URL=https://saudenamao.ntcao.com` (era `https://gaph.ntcao.com`).
- Removido o domínio duplicado em `app/Services/BrevoEmailService.php:25`, que usava `config('app.url', 'https://gaph.ntcao.com')` como fallback — passou a ler só do `.env`.
- `sns-angola-queue.service` corrigido: `Restart=on-failure` → `always` e `User=root` → `www-data`.

Duas correcções ao diagnóstico inicial. O desalinhamento **não** se limitava ao CLI: o `BrevoEmailService` lê `config('app.url')` directamente (não `url()`/`route()`) e é chamado em contexto web pelo `PortalController`, pelo que **todos** os emails de verificação apontavam para `gaph.ntcao.com`. E os 7 *Listeners* de notificação geram `route(...)` dentro do worker de fila, onde não há host do pedido, pelo que também dependiam do `APP_URL`.

O serviço do worker estava **morto desde 2026-09-05**: com `--max-time=3600` o worker sai com status 0 ao fim de uma hora, e `Restart=on-failure` não reinicia em saída limpa. Ficou igual ao `govdoc-queue.service`, que já documentava essa razão. Verificado: `active (running)`, `NRestarts=0`, fila a 0 pendentes, sem `Permission denied`, e `/up` a 200 nos dois domínios.

### Critérios de aceitação v0.1.4

- [ ] CI compila os 4 alvos com a tag `v0.1.4`.
- [ ] No Windows, a janela abre **sem** abrir nada no navegador do sistema.
- [ ] A casca mostra "A ligar ao servidor…" e chega ao `/login` do SNS dentro da janela.
- [ ] Sem rede: aparece o ecrã "Sem conexão com a internet" e o botão **Tentar Reconectar** volta a ligar quando a rede regressa.
- [ ] Um link externo abre no navegador predefinido; os internos ficam na janela.
- [ ] `F5`/`F12`/`Ctrl+R` bloqueados; `Ctrl+P` e `Ctrl+F` a funcionar.
- [ ] Exportações testadas: PDF de relatórios (formulário), CSV de relatórios (`window.open`), CSV de finanças e anexos de exames (download por navegação).
- [ ] Uploads testados (anexar exame) e reportados.

## 7. Identidade — marca SaúdeNamao (v0.1.5)

O ícone do desktop era o do template Tauri e nunca tinha sido substituído — era o último item em "ainda por resolver". Ao investigar, apurou-se que a marca **nunca tinha sido aplicada a nada**: o sistema web não tinha uma única referência ao logótipo SaúdeNamao. Usava um ícone genérico do Bootstrap na navbar e no login, um favicon azul com um coração vermelho, ícones de PWA com esse mesmo favicon (338 bytes, iguais) e um `public/favicon.ico` **vazio, com 0 bytes**.

### O que mudou

| Onde | Antes | Agora |
|---|---|---|
| Ícone do desktop | Template Tauri (o "T" amarelo/ciano) | Marca SaúdeNamao, 16 ficheiros regenerados |
| Favicon | Genérico azul com coração, 32×32 | Marca, `favicon.png` 32×32 e `favicon.ico` 16/32/48 |
| Ícones da PWA | O mesmo genérico | Marca em 32, 64, 192 e 512 |
| Ecrã de login | `<i class="bi bi-heart-pulse-fill">` | Marca, 72×72 |
| Navbar e header do portal | Ícone Bootstrap | Marca sobre um chip branco arredondado |

A marca é gerada por `sns-angola/docs/projeto/gerar_logo.py`, que passou a ter três modos: o logótipo completo (por omissão), `--icone` (marca quadrada para a app) e `--web` (todos os assets do sistema). A geometria do azulejo e da cruz foi extraída para `desenhar_marca()`, alimentada por um único valor de lado, e o modo `--web` reduz de 1024px com LANCZOS porque o `ImageDraw` não faz antialiasing. Verificado: o logótipo dos documentos continua **byte-idêntico** depois do refactor (mesmo md5, `3d3b884f…`).

### Decisão de cor

A aplicação usa o azul padrão do Bootstrap (`#0d6efd`) e um gradiente azul→roxo no Portal do Paciente — não a paleta teal da marca. Optou-se por **não** trocar o tema: a marca aparece sobre um chip branco onde o fundo é azul, e o resto do sistema mantém-se. A identidade visual do produto continua, por isso, a não ser a da marca; é uma decisão consciente, não um esquecimento.

### Critérios de aceitação v0.1.5

- [ ] CI compila os 4 alvos com a tag `v0.1.5` e publica o release.
- [ ] No Windows, o ícone do executável, da barra de tarefas e do menu Iniciar é a marca SaúdeNamao.
- [ ] O favicon do browser e o ícone do ecrã inicial (Portal do Paciente instalado como PWA) são a marca.
- [ ] Login, navbar e header do portal mostram a marca com contraste legível.
