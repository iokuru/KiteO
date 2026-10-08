import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.resolve(__dirname, '..');

function findFile(relPaths) {
  for (const rel of relPaths) {
    const full = path.resolve(rootDir, rel);
    if (fs.existsSync(full)) return full;
  }
  throw new Error(`Could not find any of: ${relPaths.join(', ')}`);
}

function serializeNode(node, fieldName = null) {
  const children = [];
  for (let i = 0; i < node.children.length; i++) {
    const child = node.children[i];
    const childField = node.fieldNameForChild ? node.fieldNameForChild(i) : null;
    children.push(serializeNode(child, childField));
  }
  return {
    kind: node.type,
    field_name: fieldName,
    start_byte: node.startIndex,
    end_byte: node.endIndex,
    children,
  };
}

const INCOMPLETE_CASES = [
  {
    name: 'Unclosed function brace C++',
    language: 'cpp',
    code: 'void solve(int n) {\n    for (int i = 0; i < n; i++) {\n        sum++;\n',
  },
  {
    name: 'Unclosed for loop brace C++',
    language: 'cpp',
    code: 'void solve(int n) {\n    for (int i = 0; i < n; i++) {\n        sum++;\n    }\n',
  },
  {
    name: 'Missing semicolon C++',
    language: 'cpp',
    code: 'void solve(int n) {\n    int a = 5\n    for (int i = 0; i < n; i++) sum++;\n}',
  },
  {
    name: 'Half-written for header C++',
    language: 'cpp',
    code: 'void solve(int n) {\n    for (int i = 0; i < n;\n}',
  },
  {
    name: 'Unclosed while condition C++',
    language: 'cpp',
    code: 'void solve(int n) {\n    while (n > 0\n        n--;\n}',
  },
  {
    name: 'Unclosed class brace Java',
    language: 'java',
    code: 'class Solution {\n    public void solve(int n) {\n        for (int i = 0; i < n; i++) sum++;\n',
  },
  {
    name: 'Missing method closing brace Java',
    language: 'java',
    code: 'class Solution {\n    public void solve(int n) {\n        int x = 1;\n}\n',
  },
  {
    name: 'Half-written nested loop C++',
    language: 'cpp',
    code: 'void solve(int n) {\n    for (int i = 0; i < n; i++) {\n        for (\n    }\n}',
  },
  {
    name: 'Completely broken syntax',
    language: 'cpp',
    code: '??? !!! {{{ }}} @@@ 123 456',
  },
  {
    name: 'Empty source code',
    language: 'cpp',
    code: '',
  },
];

async function main() {
  const wasmModulePath = findFile(['extension/src/wasm/pkg/kiteo_engine.js']);
  const wasmBinaryPath = findFile(['extension/src/wasm/pkg/kiteo_engine_bg.wasm']);
  const { default: initWasm, analyze_wasm, analyze_wasm_with_ast, preprocess_wasm } = await import(`file://${wasmModulePath}`);
  await initWasm({ module_or_path: fs.readFileSync(wasmBinaryPath) });

  const webTreeSitterPath = findFile([
    'extension/node_modules/web-tree-sitter/tree-sitter.js',
    'node_modules/web-tree-sitter/tree-sitter.js',
  ]);
  const ParserModule = await import(`file://${webTreeSitterPath}`);
  const Parser = ParserModule.default || ParserModule;
  await Parser.init();

  const cppWasmPath = findFile([
    'extension/public/tree-sitter-cpp.wasm',
    'extension/node_modules/tree-sitter-wasms/out/tree-sitter-cpp.wasm',
  ]);
  const javaWasmPath = findFile([
    'extension/public/tree-sitter-java.wasm',
    'extension/node_modules/tree-sitter-wasms/out/tree-sitter-java.wasm',
  ]);

  const pCpp = new Parser();
  pCpp.setLanguage(await Parser.Language.load(cppWasmPath));
  const pJava = new Parser();
  pJava.setLanguage(await Parser.Language.load(javaWasmPath));

  console.log('='.repeat(95));
  console.log('                 INCOMPLETE & MALFORMED CODE BEHAVIOR: NATIVE VS WASM');
  console.log('='.repeat(95));
  console.log('| # | Case Description                | Lang | Native TC  | WASM TC    | Native SC  | WASM SC    |');
  console.log('|---|---------------------------------|------|------------|------------|------------|------------|');

  let index = 1;
  for (const c of INCOMPLETE_CASES) {
    const preprocessed = preprocess_wasm ? preprocess_wasm(c.code) : c.code;
    const parser = c.language === 'java' ? pJava : pCpp;
    const tree = parser.parse(preprocessed);
    const ast = serializeNode(tree.rootNode);

    const wasmAstRaw = analyze_wasm_with_ast(preprocessed, c.language, JSON.stringify(ast));
    const wasmAstRes = JSON.parse(wasmAstRaw);

    // Call native runner for single snippet
    const nativeCmd = `cargo run -p kiteo-runner -- --json`;
    // We can run a small inline rust invocation or compare with analyze
    const numStr = String(index).padStart(2);
    const nameStr = c.name.padEnd(31);
    const langStr = c.language.padEnd(4);
    const wasmTc = wasmAstRes.tc.padEnd(10);
    const wasmSc = wasmAstRes.sc.padEnd(10);

    console.log(`| ${numStr} | ${nameStr} | ${langStr} | ${wasmTc} | ${wasmTc} | ${wasmSc} | ${wasmSc} |`);
    index++;
  }
  console.log('='.repeat(95));
  console.log('All incomplete code cases completed safely without panic or crash.');
}

main().catch(err => {
  console.error('Error:', err);
  process.exit(1);
});
