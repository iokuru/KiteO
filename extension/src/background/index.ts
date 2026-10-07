import browser from 'webextension-polyfill';
import init, {
  analyze_wasm,
  analyze_wasm_with_ast,
  preprocess_wasm,
} from '../wasm/pkg/kiteo_engine.js';
import type { AnalysisResult } from '../shared/types';
import Parser from 'web-tree-sitter';

let wasmInitialized = false;
let parserInitialized = false;
let cppParser: Parser | null = null;
let javaParser: Parser | null = null;

function serializeNode(node: Parser.SyntaxNode, fieldName: string | null = null): any {
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

async function ensureWasm() {
  if (!wasmInitialized) {
    await init();
    wasmInitialized = true;
  }
}

async function ensureParsers() {
  if (parserInitialized) return;
  try {
    await Parser.init();
    const cppUrl = browser.runtime.getURL('tree-sitter-cpp.wasm');
    const javaUrl = browser.runtime.getURL('tree-sitter-java.wasm');
    const [langCpp, langJava] = await Promise.all([
      Parser.Language.load(cppUrl),
      Parser.Language.load(javaUrl),
    ]);
    cppParser = new Parser();
    cppParser.setLanguage(langCpp);
    javaParser = new Parser();
    javaParser.setLanguage(langJava);
    parserInitialized = true;
  } catch (e) {
    console.warn('Parser initialization fallback to heuristic analyzer:', e);
  }
}

browser.runtime.onMessage.addListener(async (message) => {
  if (message.type === 'ANALYZE_REQUEST') {
    await ensureWasm();
    await ensureParsers();

    const { source, language } = message.payload;
    if (!source) {
      const emptyResult: AnalysisResult = {
        tc: 'Unknown',
        sc: 'Unknown',
        algorithms: [],
      };
      return emptyResult;
    }

    const lang = language || 'cpp';
    let rawJson: string;

    if (parserInitialized && ((lang === 'cpp' && cppParser) || (lang === 'java' && javaParser))) {
      try {
        const preprocessed = preprocess_wasm ? preprocess_wasm(source) : source;
        const parser = lang === 'java' ? javaParser! : cppParser!;
        const tree = parser.parse(preprocessed);
        const ast = serializeNode(tree.rootNode);
        rawJson = analyze_wasm_with_ast(preprocessed, lang, JSON.stringify(ast));
      } catch {
        rawJson = analyze_wasm(source, lang);
      }
    } else {
      rawJson = analyze_wasm(source, lang);
    }

    try {
      const parsed: AnalysisResult = JSON.parse(rawJson);
      return parsed;
    } catch {
      return {
        tc: 'Unknown',
        sc: 'Unknown',
        algorithms: [],
      };
    }
  }
});
