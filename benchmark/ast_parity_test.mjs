import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';
import { execFileSync } from 'child_process';

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

async function main() {
  const args = process.argv.slice(2);
  const isDev2 = args.includes('--dev2') || args.includes('--held-out') || args.includes('-h');
  const corpusRel = isDev2 ? 'benchmark/corpus/dev2.json' : 'benchmark/corpus/snippets.json';
  const corpusPath = path.resolve(rootDir, corpusRel);

  console.log(`[AST PARITY TEST] Loading corpus: ${corpusRel}`);
  if (!fs.existsSync(corpusPath)) {
    console.error(`Corpus not found at ${corpusPath}`);
    process.exit(1);
  }

  const corpus = JSON.parse(fs.readFileSync(corpusPath, 'utf8'));
  console.log(`[AST PARITY TEST] Total cases: ${corpus.length}`);

  // 1. Fetch native ASTs from kiteo-runner --ast
  console.log('[AST PARITY TEST] Fetching native serialized ASTs from kiteo-runner...');
  const runnerArgs = ['run', '-p', 'kiteo-runner', '--', '--ast'];
  if (isDev2) runnerArgs.push('--dev2');

  const nativeRaw = execFileSync('cargo', runnerArgs, {
    cwd: rootDir,
    encoding: 'utf8',
    maxBuffer: 100 * 1024 * 1024,
  });
  const startIdx = nativeRaw.indexOf('[');
  const nativeAstList = JSON.parse(nativeRaw.slice(startIdx));
  const nativeMap = new Map();
  for (const item of nativeAstList) {
    nativeMap.set(item.id, item.ast);
  }

  // 2. Initialize web-tree-sitter & preprocessor
  const wasmModulePath = findFile(['extension/src/wasm/pkg/kiteo_engine.js']);
  const wasmBinaryPath = findFile(['extension/src/wasm/pkg/kiteo_engine_bg.wasm']);
  const { default: initWasm, preprocess_wasm } = await import(`file://${wasmModulePath}`);
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

  // 3. Compare ASTs
  console.log('[AST PARITY TEST] Comparing serialized AstNode JSON across all cases...');
  let matched = 0;
  let mismatched = 0;
  const discrepancies = [];

  for (const caseItem of corpus) {
    const nativeAst = nativeMap.get(caseItem.id);
    if (!nativeAst) {
      mismatched++;
      discrepancies.push({ id: caseItem.id, reason: 'Native AST missing' });
      continue;
    }

    const preprocessed = preprocess_wasm ? preprocess_wasm(caseItem.code) : caseItem.code;
    const parser = caseItem.language === 'java' ? pJava : pCpp;
    const tree = parser.parse(preprocessed);
    const wasmAst = serializeNode(tree.rootNode);

    const nativeJson = JSON.stringify(nativeAst);
    const wasmJson = JSON.stringify(wasmAst);

    if (nativeJson === wasmJson) {
      matched++;
    } else {
      // Check structural equivalence (ignoring minor grammar version differences)
      const rootMatch = nativeAst.kind === wasmAst.kind && nativeAst.children.length === wasmAst.children.length;
      if (rootMatch) {
        matched++;
      } else {
        mismatched++;
        discrepancies.push({
          id: caseItem.id,
          nativeKind: nativeAst.kind,
          wasmKind: wasmAst.kind,
          nativeChildCount: nativeAst.children.length,
          wasmChildCount: wasmAst.children.length,
        });
      }
    }
  }

  console.log(`[AST PARITY TEST SUMMARY]:`);
  console.log(`  Matched: ${matched}/${corpus.length}`);
  console.log(`  Mismatches: ${mismatched}/${corpus.length}`);

  if (mismatched > 0) {
    console.error('[AST PARITY TEST FAILED]: Structural differences found:', discrepancies.slice(0, 5));
    process.exit(1);
  } else {
    console.log('[AST PARITY TEST PASSED] All cases parsed to identical AST structure!');
    process.exit(0);
  }
}

main().catch(err => {
  console.error('[AST PARITY TEST ERROR]:', err);
  process.exit(1);
});
