import browser from 'webextension-polyfill';
import type { AnalysisResult } from '../shared/types';

browser.runtime.onMessage.addListener(async (message) => {
  if (message.type === 'ANALYZE_REQUEST') {
    const result: AnalysisResult = {
      tc: 'Unknown',
      sc: 'Unknown',
      algorithms: [],
    };
    return result;
  }
});
