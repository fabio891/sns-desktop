import { SERVIDOR } from './config.js';

const ENDPOINT_SAUDE = `${SERVIDOR}/up`;
const TIMEOUT_MS = 5000;

const ecraLigacao = document.querySelector('#estado-ligacao');
const ecraOffline = document.querySelector('#estado-offline');
const botaoReconectar = document.querySelector('#btn-reconectar');

document.querySelector('#servidor').textContent = SERVIDOR;

/**
 * Testa se o servidor responde. Usa `mode: no-cors` de propósito: o /up do
 * Laravel não devolve cabeçalhos CORS e em no-cors o pedido não é bloqueado.
 * Um erro de rede (sem internet, DNS, servidor em baixo) rejeita a promessa,
 * que é exactamente o sinal de que precisamos.
 */
async function servidorResponde() {
  const controlador = new AbortController();
  const temporizador = setTimeout(() => controlador.abort(), TIMEOUT_MS);

  try {
    await fetch(ENDPOINT_SAUDE, {
      mode: 'no-cors',
      cache: 'no-store',
      signal: controlador.signal,
    });
    return true;
  } catch {
    return false;
  } finally {
    clearTimeout(temporizador);
  }
}

function mostrarOffline() {
  ecraLigacao.hidden = true;
  ecraOffline.hidden = false;
  botaoReconectar.disabled = false;
  botaoReconectar.textContent = 'Tentar Reconectar';
  botaoReconectar.focus();
}

async function ligar() {
  if (await servidorResponde()) {
    window.location.replace(SERVIDOR);
    return;
  }
  mostrarOffline();
}

botaoReconectar.addEventListener('click', async () => {
  botaoReconectar.disabled = true;
  botaoReconectar.textContent = 'A tentar…';
  await ligar();
});

ligar();
