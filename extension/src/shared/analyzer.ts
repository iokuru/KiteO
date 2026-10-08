import init, {
  analyze_wasm,
  analyze_wasm_with_ast,
  preprocess_wasm,
} from '../wasm/pkg/kiteo_engine.js';
import type { AnalysisResult } from './types';
import Parser from 'web-tree-sitter';

let wasmInitialized = false;
let parserInitialized = false;
let cppParser: Parser | null = null;
let javaParser: Parser | null = null;
let initPromise: Promise<void> | null = null;

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

function getAssetUrl(filename: string): string {
  const g = typeof globalThis !== 'undefined' ? (globalThis as any) : undefined;
  if (g?.chrome?.runtime?.getURL) {
    try {
      return g.chrome.runtime.getURL(filename);
    } catch {}
  }
  if (g?.browser?.runtime?.getURL) {
    try {
      return g.browser.runtime.getURL(filename);
    } catch {}
  }
  return '/' + filename;
}

export async function initAnalyzer(): Promise<void> {
  if (wasmInitialized && parserInitialized) return;
  if (initPromise) return initPromise;

  initPromise = (async () => {
    if (!wasmInitialized) {
      await init();
      wasmInitialized = true;
    }

    if (!parserInitialized) {
      try {
        await Parser.init({
          locateFile(scriptName: string) {
            return getAssetUrl(scriptName);
          },
        });
        const cppUrl = getAssetUrl('tree-sitter-cpp.wasm');
        const javaUrl = getAssetUrl('tree-sitter-java.wasm');
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
  })();

  return initPromise;
}

export async function analyzeCode(source: string, language?: string): Promise<AnalysisResult> {
  if (!source) {
    return {
      tc: 'Unknown',
      sc: 'Unknown',
      algorithms: [],
    };
  }

  await initAnalyzer();

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
    return {
      tc: parsed.tc || 'Unknown',
      sc: parsed.sc || 'Unknown',
      algorithms: parsed.algorithms || [],
    };
  } catch {
    return {
      tc: 'Unknown',
      sc: 'Unknown',
      algorithms: [],
    };
  }
}
