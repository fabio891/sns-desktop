// Gera src/config.js — o endereço do servidor que a janela vai abrir.
//
//   npm run config
//   SNS_APP_URL=https://outro.dominio npm run tauri:build
//
// O valor por omissão está aqui; a variável de ambiente sobrepõe-no sem ser
// preciso editar código (é o que o CI usa, via variável do repositório).

import { writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const POR_OMISSAO = 'https://saudenamao.ntcao.com';
const servidor = (process.env.SNS_APP_URL || POR_OMISSAO).trim().replace(/\/+$/, '');

if (!/^https?:\/\/[^/\s]+/.test(servidor)) {
  console.error(`SNS_APP_URL inválido: "${servidor}". Esperado algo como https://exemplo.ao`);
  process.exit(1);
}

const destino = join(dirname(fileURLToPath(import.meta.url)), '..', 'src', 'config.js');
const conteudo = `// GERADO por scripts/config.mjs — não editar à mão.
export const SERVIDOR = '${servidor}';
`;

writeFileSync(destino, conteudo);
console.log(`src/config.js -> ${servidor}`);
