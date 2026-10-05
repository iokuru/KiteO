import browser from 'webextension-polyfill';
import init, { analyze_wasm } from '../wasm/pkg/kiteo_engine.js';
import type { AnalysisResult } from '../shared/types';

let wasmInitialized = false;

async function ensureWasm() {
  if (!wasmInitialized) {
    await init();
    wasmInitialized = true;
  }
}

browser.runtime.onMessage.addListener(async (message) => {
  if (message.type === 'ANALYZE_REQUEST') {
    await ensureWasm();

    const { source, language } = message.payload;
    if (!source) {
      const emptyResult: AnalysisResult = {
        tc: 'Unknown',
        sc: 'Unknown',
        algorithms: [],
      };
      return emptyResult;
    }

    const rawJson = analyze_wasm(source, language || 'cpp');
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
