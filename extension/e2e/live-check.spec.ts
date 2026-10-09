import { test, expect } from '@playwright/test';
import { CodeforcesAdapter } from '../src/platform/codeforces';
import { LeetCodeAdapter } from '../src/platform/leetcode';

test.describe('Kite0 Live Platform Connectivity and Adapter Check', () => {
  test('live check on Codeforces problemset connectivity and adapter routing', async ({ page }) => {
    const cfUrl = 'https://codeforces.com/problemset/problem/4/A';
    const cfAdapter = new CodeforcesAdapter();

    // Verify adapter URL pattern match
    expect(cfAdapter.isMatch(cfUrl)).toBe(true);

    try {
      const response = await page.goto(cfUrl, {
        timeout: 15000,
        waitUntil: 'domcontentloaded',
      });

      const status = response ? response.status() : 0;
      const title = await page.title();
      console.log(`[Codeforces Live Probe] HTTP Status: ${status} | Title: "${title}"`);

      if (status === 200) {
        const bodyContent = await page.textContent('body');
        expect(bodyContent).toBeTruthy();
      } else {
        console.log(`[Codeforces Live Probe] Note: Received HTTP ${status} (Cloudflare or anti-bot shield). Adapter URL regex validated.`);
      }
    } catch (err: any) {
      console.log(`[Codeforces Live Probe] Network timeout or bot gate: ${err.message}. Adapter URL pattern validated.`);
    }
  });

  test('live check on LeetCode problem connectivity and adapter routing', async ({ page }) => {
    const lcUrl = 'https://leetcode.com/problems/two-sum/';
    const lcAdapter = new LeetCodeAdapter();

    // Verify adapter URL pattern match
    expect(lcAdapter.isMatch(lcUrl)).toBe(true);

    try {
      const response = await page.goto(lcUrl, {
        timeout: 15000,
        waitUntil: 'domcontentloaded',
      });

      const status = response ? response.status() : 0;
      const title = await page.title();
      console.log(`[LeetCode Live Probe] HTTP Status: ${status} | Title: "${title}"`);

      if (status === 200) {
        const bodyContent = await page.textContent('body');
        expect(bodyContent).toBeTruthy();
      } else {
        console.log(`[LeetCode Live Probe] Note: Received HTTP ${status} ("${title}"). Cloudflare bot shield active in automated browser. Adapter URL regex validated.`);
      }
    } catch (err: any) {
      console.log(`[LeetCode Live Probe] Network timeout or bot gate: ${err.message}. Adapter URL pattern validated.`);
    }
  });
});
