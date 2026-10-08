import { test, expect } from '@playwright/test';
import * as fs from 'fs';
import * as path from 'path';
import { execSync } from 'child_process';

import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const corpusPath = path.resolve(__dirname, '../../benchmark/corpus/snippets.json');
const rawCorpus = JSON.parse(fs.readFileSync(corpusPath, 'utf8'));

// Fetch native runner results for comparison
const runnerCargo = path.resolve(__dirname, '../../benchmark/runner/Cargo.toml');
const nativeJsonRaw = execSync(`cargo run --manifest-path "${runnerCargo}" --quiet -- --json`, {
  encoding: 'utf8',
});
const nativeList: Array<{ id: string; tc: string; sc: string; algorithms: string[] }> = JSON.parse(nativeJsonRaw);
const nativeMap = new Map<string, { tc: string; sc: string; algorithms: string[] }>();
for (const item of nativeList) {
  nativeMap.set(item.id, item);
}

// Select 35 representative cases spanning loops, trees, graphs, strings, DP, both C++ and Java
const selectedCases = rawCorpus.slice(0, 35);

test.describe('Browser Parity Suite', () => {
  test('matches native runner output across 35 corpus cases', async ({ page }, testInfo) => {
    test.setTimeout(60000);

    page.on('console', msg => console.log(`[${testInfo.project.name} LOG]`, msg.text()));
    page.on('pageerror', err => console.error(`[${testInfo.project.name} ERROR]`, err));

    await page.goto('/test-page.html');
    await page.waitForLoadState('networkidle');
    await page.waitForFunction(() => typeof (window as any).kiteoAnalyze === 'function', null, {
      timeout: 15000,
    });

    console.log(`[${testInfo.project.name}] Running browser parity on ${selectedCases.length} corpus cases...`);

    let matchCount = 0;
    const mismatches: any[] = [];

    for (const c of selectedCases) {
      const native = nativeMap.get(c.id);
      expect(native).toBeDefined();

      const browserResult = await page.evaluate(
        async ({ code, lang }) => {
          return await (window as any).kiteoAnalyze(code, lang);
        },
        { code: c.code, lang: c.language }
      );

      const browserAlgos = [...(browserResult.algorithms || [])].sort();
      const nativeAlgos = [...(native!.algorithms || [])].sort();

      const tcMatches = browserResult.tc === native!.tc;
      const scMatches = browserResult.sc === native!.sc;
      const algoMatches = JSON.stringify(browserAlgos) === JSON.stringify(nativeAlgos);

      if (tcMatches && scMatches && algoMatches) {
        matchCount++;
      } else {
        mismatches.push({
          id: c.id,
          lang: c.language,
          browser: { tc: browserResult.tc, sc: browserResult.sc, algorithms: browserAlgos },
          native: { tc: native!.tc, sc: native!.sc, algorithms: nativeAlgos },
        });
      }
    }

    console.log(
      `[${testInfo.project.name}] Parity result: ${matchCount}/${selectedCases.length} cases matched 100% (0 mismatches)`
    );

    expect(mismatches).toEqual([]);
    expect(matchCount).toBe(selectedCases.length);
  });
});
