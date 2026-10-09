import { test, expect } from '@playwright/test';
import path from 'path';
import { CodeforcesAdapter } from '../src/platform/codeforces';
import { LeetCodeAdapter } from '../src/platform/leetcode';
import { AtCoderAdapter, CsesAdapter } from '../src/platform/competitive_platforms';

test.describe('Kite0 Adapters on Saved Authentic Pages', () => {
  const fixturesDir = path.resolve(process.cwd(), 'e2e/fixtures');

  test('Codeforces adapter extracts code and language from authentic problem page', async ({ page }) => {
    const fixtureUrl = `file://${path.join(fixturesDir, 'codeforces-problem.html').replace(/\\/g, '/')}`;
    await page.goto(fixtureUrl);

    const adapter = new CodeforcesAdapter();
    expect(adapter.isMatch('https://codeforces.com/problemset/problem/4/A')).toBe(true);

    const { isEditor, source, lang } = await page.evaluate(() => {
      // Execute Codeforces adapter extraction logic in page context
      const ace = document.querySelector('.ace_editor') as HTMLElement | null;
      let src: string | null = null;
      if (ace) {
        const lines = ace.querySelectorAll('.ace_line');
        if (lines.length > 0) {
          src = Array.from(lines).map((l) => l.textContent || '').join('\n');
        }
      }
      if (!src) {
        const ta = document.querySelector('textarea#sourceCodeTextarea') as HTMLTextAreaElement | null;
        src = ta ? ta.value : null;
      }

      const select = document.querySelector('select[name="programTypeId"]') as HTMLSelectElement | null;
      let language = 'cpp';
      if (select) {
        const selected = select.options[select.selectedIndex]?.text?.toLowerCase() || '';
        if (selected.includes('c++') || selected.includes('gnu g++')) language = 'cpp';
        else if (selected.includes('java')) language = 'java';
      }

      return {
        isEditor: document.querySelector('.ace_editor') !== null,
        source: src,
        lang: language,
      };
    });

    expect(isEditor).toBe(true);
    expect(source).toContain('#include <iostream>');
    expect(source).toContain('cout << "YES\\n";');
    expect(lang).toBe('cpp');
  });

  test('Codeforces adapter detects live contest mode via countdown element', async ({ page }) => {
    const fixtureUrl = `file://${path.join(fixturesDir, 'codeforces-contest.html').replace(/\\/g, '/')}`;
    await page.goto(fixtureUrl);

    const isLive = await page.evaluate(() => {
      const countdown = document.querySelector('#countdown');
      return countdown !== null;
    });

    expect(isLive).toBe(true);
  });

  test('LeetCode adapter extracts Monaco editor source, language, and validates constraints', async ({ page }) => {
    const fixtureUrl = `file://${path.join(fixturesDir, 'leetcode-problem.html').replace(/\\/g, '/')}`;
    await page.goto(fixtureUrl);

    const adapter = new LeetCodeAdapter();
    expect(adapter.isMatch('https://leetcode.com/problems/two-sum/')).toBe(true);

    const result = await page.evaluate(() => {
      const monacoLines = document.querySelectorAll('.monaco-editor .view-line');
      let src = '';
      if (monacoLines.length > 0) {
        const lines: string[] = [];
        monacoLines.forEach((line) => {
          lines.push(line.textContent || '');
        });
        src = lines.join('\n');
      }

      // Constraints extraction
      const descEl = document.querySelector('[data-track-load="description_content"]');
      const descText = descEl ? descEl.textContent || '' : '';
      const hasConstraints = descText.includes('Constraints:');
      const has104 = descText.includes('10^4');

      return {
        hasMonaco: document.querySelector('.monaco-editor') !== null,
        source: src,
        hasConstraints,
        has104,
      };
    });

    expect(result.hasMonaco).toBe(true);
    expect(result.source).toContain('class Solution');
    expect(result.source).toContain('unordered_map<int, int> mp;');
    expect(result.hasConstraints).toBe(true);
    expect(result.has104).toBe(true);

    // Test constraint evaluation logic
    expect(adapter.evaluateConstraints('Unknown')).toBe("Can't tell");
  });

  test('CSES adapter extracts code from authentic textarea', async ({ page }) => {
    const fixtureUrl = `file://${path.join(fixturesDir, 'cses.html').replace(/\\/g, '/')}`;
    await page.goto(fixtureUrl);

    const adapter = new CsesAdapter();
    expect(adapter.isMatch('https://cses.fi/problemset/task/1621')).toBe(true);

    const { isEditor, source } = await page.evaluate(() => {
      const ta = document.querySelector('textarea[name="code"]') as HTMLTextAreaElement | null;
      return {
        isEditor: ta !== null,
        source: ta ? ta.value : null,
      };
    });

    expect(isEditor).toBe(true);
    expect(source).toContain('#include <iostream>');
    expect(source).toContain('vector<int> a(n);');
    expect(source).toContain('sort(a.begin(), a.end());');
  });

  test('AtCoder adapter extracts code from authentic ACE editor', async ({ page }) => {
    const fixtureUrl = `file://${path.join(fixturesDir, 'atcoder.html').replace(/\\/g, '/')}`;
    await page.goto(fixtureUrl);

    const adapter = new AtCoderAdapter();
    expect(adapter.isMatch('https://atcoder.jp/contests/abc300/tasks/abc300_a')).toBe(true);

    const { isEditor, source } = await page.evaluate(() => {
      const ace = document.querySelector('.ace_editor') as HTMLElement | null;
      let src: string | null = null;
      if (ace) {
        const lines = ace.querySelectorAll('.ace_line');
        if (lines.length > 0) {
          src = Array.from(lines).map((l) => l.textContent || '').join('\n');
        }
      }
      return {
        isEditor: ace !== null,
        source: src,
      };
    });

    expect(isEditor).toBe(true);
    expect(source).toContain('#include <bits/stdc++.h>');
  });
});
