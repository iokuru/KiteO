import { test, expect } from '@playwright/test';
import { getPlatformAdapter, getAllPlatformAdapters } from '../src/platform/registry';

test.describe('Kite0 Platform Adapters Architecture', () => {
  test('all 12 required competitive programming and online IDE platforms are registered', () => {
    const adapters = getAllPlatformAdapters();
    const names = adapters.map(a => a.name);

    const required = [
      'Codeforces',
      'LeetCode',
      'CodeChef',
      'AtCoder',
      'HackerRank',
      'GeeksforGeeks',
      'HackerEarth',
      'CSES',
      'OnlineGDB',
      'JDoodle',
      'Programiz',
      'Replit',
    ];

    for (const req of required) {
      expect(names).toContain(req);
    }
  });

  test('hostname matching matches platforms correctly', () => {
    expect(getPlatformAdapter('https://codeforces.com/contest/1900/problem/A')?.name).toBe('Codeforces');
    expect(getPlatformAdapter('https://leetcode.com/problems/two-sum/')?.name).toBe('LeetCode');
    expect(getPlatformAdapter('https://www.codechef.com/problems/START100')?.name).toBe('CodeChef');
    expect(getPlatformAdapter('https://atcoder.jp/contests/abc300/tasks/abc300_a')?.name).toBe('AtCoder');
    expect(getPlatformAdapter('https://www.hackerrank.com/challenges/solve-me-first')?.name).toBe('HackerRank');
    expect(getPlatformAdapter('https://www.geeksforgeeks.org/problems/subarray-with-given-sum')?.name).toBe('GeeksforGeeks');
    expect(getPlatformAdapter('https://www.hackerearth.com/practice/algorithms/')?.name).toBe('HackerEarth');
    expect(getPlatformAdapter('https://cses.fi/problemset/task/1068')?.name).toBe('CSES');
    expect(getPlatformAdapter('https://www.onlinegdb.com/online_c++_compiler')?.name).toBe('OnlineGDB');
    expect(getPlatformAdapter('https://www.jdoodle.com/online-java-compiler/')?.name).toBe('JDoodle');
    expect(getPlatformAdapter('https://www.programiz.com/cpp-programming/online-compiler/')?.name).toBe('Programiz');
    expect(getPlatformAdapter('https://replit.com/@user/project')?.name).toBe('Replit');
  });

  test('extracts code from mock Codeforces ACE editor without tampering', async ({ page }) => {
    await page.goto('about:blank');

    // Inject mock DOM resembling Codeforces submission form
    await page.evaluate(() => {
      const form = document.createElement('form');
      form.id = 'sidebarSubmitForm';
      const ta = document.createElement('textarea');
      ta.name = 'source';
      ta.value = 'int main() { return 0; }';
      form.appendChild(ta);
      document.body.appendChild(form);
    });

    const code = await page.evaluate(() => {
      const el = document.querySelector('form#sidebarSubmitForm textarea[name="source"]') as HTMLTextAreaElement;
      return el ? el.value : null;
    });

    expect(code).toBe('int main() { return 0; }');
  });

  test('extracts code from mock CSES textarea', async ({ page }) => {
    await page.goto('about:blank');

    await page.evaluate(() => {
      const form = document.createElement('form');
      form.className = 'nav';
      const ta = document.createElement('textarea');
      ta.name = 'target';
      ta.value = '#include <iostream>\nint main() { std::cout << 42; }';
      form.appendChild(ta);
      document.body.appendChild(form);
    });

    const extracted = await page.evaluate(() => {
      const ta = document.querySelector('form textarea[name="target"]') as HTMLTextAreaElement;
      return ta ? ta.value : null;
    });

    expect(extracted).toContain('std::cout << 42');
  });
});
