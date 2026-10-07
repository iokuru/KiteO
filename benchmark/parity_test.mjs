import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';
import { execFileSync } from 'child_process';
import { createRequire } from 'module';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.resolve(__dirname, '..');
const require = createRequire(import.meta.url);

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

async function main() {
  const args = process.argv.slice(2);
  const isHeldOut = args.includes('--held-out') || args.includes('-h');
  const corpusRel = isHeldOut
    ? 'benchmark/corpus/held_out.json'
    : 'benchmark/corpus/snippets.json';
  const corpusPath = path.resolve(rootDir, corpusRel);

  console.log(`[PARITY TEST] Loading corpus: ${corpusRel}`);
  if (!fs.existsSync(corpusPath)) {
    console.error(`Corpus not found at ${corpusPath}`);
    process.exit(1);
  }

  const corpus = JSON.parse(fs.readFileSync(corpusPath, 'utf8'));
  console.log(`[PARITY TEST] Total test cases in corpus: ${corpus.length}`);

  // 1. Obtain native runner results
  console.log('[PARITY TEST] Running native runner with --json...');
  const runnerArgs = ['run', '-p', 'kiteo-runner', '--', '--json'];
  if (isHeldOut) {
    runnerArgs.push('--held-out');
  }

  let nativeRaw;
  try {
    nativeRaw = execFileSync('cargo', runnerArgs, {
      cwd: rootDir,
      encoding: 'utf8',
      maxBuffer: 50 * 1024 * 1024,
    });
  } catch (err) {
    console.error('[PARITY TEST] Failed to execute native runner:', err.message);
    process.exit(1);
  }

  const nativeJsonStart = nativeRaw.indexOf('[');
  if (nativeJsonStart === -1) {
    console.error('[PARITY TEST] No JSON array found in native runner output');
    process.exit(1);
  }
  const nativeResultsList = JSON.parse(nativeRaw.slice(nativeJsonStart));
  const nativeMap = new Map();
  for (const item of nativeResultsList) {
    nativeMap.set(item.id, item);
  }

  // 2. Initialize WASM module
  console.log('[PARITY TEST] Initializing WebAssembly analyzer...');
  const wasmModulePath = findFile(['extension/src/wasm/pkg/kiteo_engine.js']);
  const wasmBinaryPath = findFile(['extension/src/wasm/pkg/kiteo_engine_bg.wasm']);
  const { default: initWasm, analyze_wasm_with_ast, preprocess_wasm } = await import(`file://${wasmModulePath}`);
  const wasmBytes = fs.readFileSync(wasmBinaryPath);
  await initWasm({ module_or_path: wasmBytes });

  // 3. Initialize web-tree-sitter
  console.log('[PARITY TEST] Initializing web-tree-sitter parsers (C++ and Java)...');
  const webTreeSitterPath = findFile([
    'extension/node_modules/web-tree-sitter/tree-sitter.js',
    'node_modules/web-tree-sitter/tree-sitter.js',
  ]);
  const ParserModule = await import(`file://${webTreeSitterPath}`);
  const Parser = ParserModule.default || ParserModule;
  await Parser.init();

  const cppWasmPath = findFile([
    'extension/node_modules/tree-sitter-wasms/out/tree-sitter-cpp.wasm',
    'node_modules/tree-sitter-wasms/out/tree-sitter-cpp.wasm',
  ]);
  const javaWasmPath = findFile([
    'extension/node_modules/tree-sitter-wasms/out/tree-sitter-java.wasm',
    'node_modules/tree-sitter-wasms/out/tree-sitter-java.wasm',
  ]);

  const pCpp = new Parser();
  const langCpp = await Parser.Language.load(cppWasmPath);
  pCpp.setLanguage(langCpp);

  const pJava = new Parser();
  const langJava = await Parser.Language.load(javaWasmPath);
  pJava.setLanguage(langJava);

  // 4. Compare all cases
  console.log('[PARITY TEST] Verifying parity across all cases...');
  let mismatches = 0;
  const failures = [];

  for (const caseItem of corpus) {
    const native = nativeMap.get(caseItem.id);
    if (!native) {
      mismatches++;
      failures.push({
        id: caseItem.id,
        reason: 'Missing from native runner output',
      });
      continue;
    }

    const preprocessed = preprocess_wasm(caseItem.code);
    const parser = caseItem.language === 'java' ? pJava : pCpp;
    const tree = parser.parse(preprocessed);
    const ast = serializeNode(tree.rootNode);
    const astJson = JSON.stringify(ast);

    const wasmRaw = analyze_wasm_with_ast(preprocessed, caseItem.language, astJson);
    const wasmRes = JSON.parse(wasmRaw);

    const tcMatch = wasmRes.tc === native.tc;
    const scMatch = wasmRes.sc === native.sc;

    const nativeAlgos = (native.algorithms || []).slice().sort();
    const wasmAlgos = (wasmRes.algorithms || []).slice().sort();
    const algoMatch = JSON.stringify(nativeAlgos) === JSON.stringify(wasmAlgos);

    if (!tcMatch || !scMatch || !algoMatch) {
      mismatches++;
      failures.push({
        id: caseItem.id,
        language: caseItem.language,
        tc: { native: native.tc, wasm: wasmRes.tc, match: tcMatch },
        sc: { native: native.sc, wasm: wasmRes.sc, match: scMatch },
        algorithms: { native: nativeAlgos, wasm: wasmAlgos, match: algoMatch },
      });
    }
  }

  if (mismatches > 0) {
    console.error(`\n[PARITY TEST FAILED] Found ${mismatches} mismatch(es) out of ${corpus.length}:`);
    for (const f of failures) {
      console.error(JSON.stringify(f, null, 2));
    }
    process.exit(1);
  } else {
    console.log(`\n[PARITY TEST PASSED] All ${corpus.length} cases produced IDENTICAL results between Native and WebAssembly!`);
    console.log(`  - Time Complexity (TC): 100% identical`);
    console.log(`  - Space Complexity (SC): 100% identical`);
    console.log(`  - Algorithm labels: 100% identical\n`);
    process.exit(0);
  }
}

main().catch((err) => {
  console.error('[PARITY TEST ERROR]:', err);
  process.exit(1);
});
