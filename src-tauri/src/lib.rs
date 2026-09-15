use tauri::{WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_opener::open_url;

mod hosts_gerados;
use hosts_gerados::HOSTS_INTERNOS;

const TITULO: &str = "SNS — Sistema Nacional de Saúde";

/// Origens que a própria casca serve, por plataforma. O Tauri usa
/// `tauri://localhost` no macOS/Linux/iOS e `http://tauri.localhost` no
/// Windows/Android — se só se reconhecer a primeira, no Windows a página
/// local da casca é classificada como link externo e vai parar ao navegador.
///
/// A comparação é exacta, aos pares: aceitar qualquer host terminado em
/// `.localhost` deixaria passar por local um domínio remoto com esse sufixo
/// (é a falha que o `is_local_url` do Tauri corrigiu no aviso de segurança).
const ORIGENS_DA_CASCA: &[(&str, &str)] = &[
    ("tauri", "localhost"),
    ("http", "tauri.localhost"),
];

/// Tudo o que peça uma janela nova é reencaminhado para a própria janela, para
/// que o `on_navigation` (Rust) decida num único sítio quem fica dentro e quem
/// vai para o navegador do sistema.
///
/// São precisos os três casos porque uma janela nova **não** passa pelo
/// `on_navigation`: sem isto o WebView abria uma janela nua por cima da app e
/// o pedido não chegava a ser classificado. Os relatórios usam
/// `<form target="_blank">` e o export CSV usa `window.open`.
const SCRIPT_LIGACOES: &str = r#"
(function () {
  function encaminhar(url) {
    if (url) window.location.href = String(url);
  }

  document.addEventListener('click', function (evento) {
    var alvo = evento.target;
    var ligacao = alvo && alvo.closest ? alvo.closest('a[target="_blank"]') : null;
    if (!ligacao || !ligacao.href) return;
    evento.preventDefault();
    encaminhar(ligacao.href);
  }, true);

  document.addEventListener('submit', function (evento) {
    var formulario = evento.target;
    if (!formulario || formulario.target !== '_blank') return;
    evento.preventDefault();
    // submit() não volta a disparar o evento, por isso não há ciclo.
    formulario.target = '_self';
    formulario.submit();
  }, true);

  window.open = function (url) {
    encaminhar(url);
    return null;
  };
})();
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

/// Navegação dentro da aplicação: a casca local, o servidor do SNS e o
/// servidor de desenvolvimento (`localhost`).
fn navegacao_interna(url: &tauri::Url) -> bool {
    let host = match url.host_str() {
        Some(host) => host,
        None => return false,
    };

    ORIGENS_DA_CASCA
        .iter()
        .any(|&(esquema, origem)| url.scheme() == esquema && host == origem)
        || host == "localhost"
        || HOSTS_INTERNOS.contains(&host)
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
