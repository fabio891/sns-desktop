// Gera o endereço do servidor a partir de um único valor:
//
//   src/config.js                    → o que a casca local (JS) usa para o health-check
//   src-tauri/src/hosts_gerados.rs   → os hosts que o Rust considera internos
//
//   npm run config
//   SNS_APP_URL=https://outro.dominio npm run tauri:build
//   SNS_APP_HOSTS=outro.dominio,alias.dominio npm run tauri:build
//
// O valor por omissão está aqui; a variável de ambiente sobrepõe-no sem ser
// preciso editar código (é o que o CI usa, via variável do repositório).
//
// Os dois ficheiros são gerados juntos de propósito: quando a lista de hosts
// internos vivia escrita à mão no Rust, apontar a app a um domínio novo fazia
// com que o próprio servidor fosse tratado como link externo.

import { writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const POR_OMISSAO = 'https://saudenamao.ntcao.com';

// Domínios servidos pelo mesmo servidor que não sejam o próprio `SNS_APP_URL`
// (aliases históricos). `SNS_APP_HOSTS` substitui esta lista.
const HOSTS_EXTRA_POR_OMISSAO = [
  'gaph.ntcao.com',
  'sns-angola.ntcao.com',
];

const raiz = join(dirname(fileURLToPath(import.meta.url)), '..');

function normalizarUrl(valor) {
  const servidor = (valor || POR_OMISSAO).trim().replace(/\/+$/, '');
  if (!/^https?:\/\/[^/\s]+/.test(servidor)) {
    console.error(`SNS_APP_URL inválido: "${servidor}". Esperado algo como https://exemplo.ao`);
    process.exit(1);
  }
  return servidor;
}

function validarHosts(hosts, origem) {
  const validos = hosts.map((h) => h.trim().toLowerCase()).filter(Boolean);
  for (const host of validos) {
    if (!/^[a-z0-9]([a-z0-9-]*[a-z0-9])?(\.[a-z0-9]([a-z0-9-]*[a-z0-9])?)+$/.test(host)) {
      console.error(`${origem}: "${host}" não é um domínio válido.`);
      process.exit(1);
    }
  }
  return validos;
}

const servidor = normalizarUrl(process.env.SNS_APP_URL);
const hostDoServidor = new URL(servidor).hostname;

const extra = process.env.SNS_APP_HOSTS
  ? validarHosts(process.env.SNS_APP_HOSTS.split(','), 'SNS_APP_HOSTS')
  : HOSTS_EXTRA_POR_OMISSAO;

// O host do servidor entra sempre, mesmo que não conste dos extras.
const hosts = [...new Set([hostDoServidor, ...extra])];

writeFileSync(
  join(raiz, 'src', 'config.js'),
  `// GERADO por scripts/config.mjs — não editar à mão.
export const SERVIDOR = '${servidor}';
`,
);

writeFileSync(
  join(raiz, 'src-tauri', 'src', 'hosts_gerados.rs'),
  `// GERADO por scripts/config.mjs — não editar à mão.
//
// Hosts que a janela abre dentro da própria app: o de SNS_APP_URL mais os
// aliases de SNS_APP_HOSTS.

pub const HOSTS_INTERNOS: &[&str] = &[
${hosts.map((h) => `    "${h}",`).join('\n')}
];
`,
);

console.log(`src/config.js -> ${servidor}`);
console.log(`src-tauri/src/hosts_gerados.rs -> ${hosts.join(', ')}`);
