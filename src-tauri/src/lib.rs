use tauri::{WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_opener::open_url;

const TITULO: &str = "SNS — Sistema Nacional de Saúde";

/// Hosts servidos pela aplicação. Tudo o que não esteja aqui (nem seja a
/// própria casca local) é link externo e abre no navegador do sistema.
/// Ao apontar a app a um domínio novo, acrescentar aqui — ver README.
const HOSTS_INTERNOS: &[&str] = &[
    "saudenamao.ntcao.com",
    "gaph.ntcao.com",
    "sns-angola.ntcao.com",
];

/// Links com `target="_blank"` não passam pelo `on_navigation`, por isso o
/// clique é reencaminhado para a própria janela. Assim a decisão de abrir
/// dentro ou fora fica num único sítio (o Rust).
const SCRIPT_LIGACOES: &str = r#"
document.addEventListener('click', function (evento) {
  var alvo = evento.target;
  var ligacao = alvo && alvo.closest ? alvo.closest('a[target="_blank"]') : null;
  if (!ligacao || !ligacao.href) return;
  evento.preventDefault();
  window.location.href = ligacao.href;
}, true);
"#;

/// Bloqueia atalhos de navegador nos builds de produção. Em desenvolvimento
/// não é injectado. Ctrl+P (impressão de relatórios) e Ctrl+F ficam livres.
#[cfg(not(debug_assertions))]
const SCRIPT_ATALHOS: &str = r#"
document.addEventListener('keydown', function (evento) {
  var tecla = evento.key;
  var ctrl = evento.ctrlKey || evento.metaKey;
  var bloqueado =
    tecla === 'F5' ||
    tecla === 'F12' ||
    (ctrl && (tecla === 'r' || tecla === 'R' || tecla === 'u' || tecla === 'U')) ||
    (ctrl && evento.shiftKey && ['i', 'j', 'c'].indexOf(tecla.toLowerCase()) !== -1);
  if (bloqueado) {
    evento.preventDefault();
    evento.stopPropagation();
  }
}, true);
"#;

/// Navegação dentro da aplicação: a casca local (`tauri://`) e o servidor.
fn navegacao_interna(url: &tauri::Url) -> bool {
    if url.scheme() == "tauri" {
        return true;
    }

    match url.host_str() {
        Some(host) => host == "localhost" || HOSTS_INTERNOS.contains(&host),
        None => false,
    }
}

fn abrir_no_navegador(url: &tauri::Url) {
    if let Err(erro) = open_url(url.as_str(), None::<&str>) {
        eprintln!("Não foi possível abrir {url} no navegador do sistema: {erro}");
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // A janela é criada aqui e não no tauri.conf.json porque
            // on_navigation e initialization_script só existem no builder.
            let mut janela = WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::App("index.html".into()),
            )
            .title(TITULO)
            .inner_size(1280.0, 800.0)
            .resizable(true)
            .center()
            .on_navigation(|url| {
                if navegacao_interna(url) {
                    return true;
                }

                abrir_no_navegador(url);
                false
            })
            .initialization_script(SCRIPT_LIGACOES);

            #[cfg(not(debug_assertions))]
            {
                janela = janela.initialization_script(SCRIPT_ATALHOS);
            }

            janela.build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("erro ao iniciar a aplicação SNS");
}
