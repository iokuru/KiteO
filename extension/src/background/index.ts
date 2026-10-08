import browser from 'webextension-polyfill';
import { analyzeCode } from '../shared/analyzer';
import type { AnalysisResult } from '../shared/types';

browser.runtime.onMessage.addListener(async (message) => {
  if (message.type === 'ANALYZE_REQUEST') {
    const { source, language } = message.payload || {};
    const result: AnalysisResult = await analyzeCode(source, language);
    return result;
  }
});
