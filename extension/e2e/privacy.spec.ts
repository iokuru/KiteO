import { test, expect } from '@playwright/test';

test.describe('KiteO Privacy & Zero-Network-Request Audit', () => {
  test('strictly zero external network requests are made during page load and analysis', async ({ page }) => {
    const interceptedRequests: string[] = [];

    page.on('request', (req) => {
      interceptedRequests.push(req.url());
    });

    await page.goto('/test-page.html');
    await page.waitForLoadState('networkidle');
    await expect(page.locator('#status')).toContainText('Zero network requests');

    // Verify all initial requests are local assets only
    for (const url of interceptedRequests) {
      const parsed = new URL(url);
      expect(parsed.hostname).toBe('127.0.0.1');
      // Assert no external analytics, telemetry, or remote LLM calls
      expect(url).not.toContain('api.openai.com');
      expect(url).not.toContain('anthropic');
      expect(url).not.toContain('googleapis');
      expect(url).not.toContain('google-analytics');
      expect(url).not.toContain('telemetry');
    }

    // Now record requests specifically during the static analysis phase
    const analysisRequests: string[] = [];
    page.on('request', (req) => {
      analysisRequests.push(req.url());
    });

    const runBtn = page.locator('#run-btn');
    await runBtn.click();

    // Wait for analysis to complete and update DOM
    await expect(page.locator('#tc-output')).toHaveText('O(n)');
    await expect(page.locator('#status')).toContainText('Zero network requests');

    // Assert that analysis was completely offline in WebAssembly: zero HTTP requests dispatched
    expect(analysisRequests).toEqual([]);
  });

  test('multiple consecutive analyses perform zero network activity', async ({ page }) => {
    await page.goto('/test-page.html');
    await page.waitForLoadState('networkidle');
    await expect(page.locator('#status')).toContainText('Zero network requests');

    const presets = [
      'button[data-type="nested"]',
      'button[data-type="two_pointers"]',
      'button[data-type="segtree"]',
      'button[data-type="dijkstra"]',
      'button[data-type="sieve"]',
    ];

    const requestsDuringPresets: string[] = [];
    page.on('request', (req) => {
      requestsDuringPresets.push(req.url());
    });

    for (const selector of presets) {
      await page.locator(selector).click();
      await page.locator('#run-btn').click();
      await expect(page.locator('#status')).toContainText('Zero network requests');
    }

    // Must be completely zero network activity during all preset analyses
    expect(requestsDuringPresets).toEqual([]);
  });
});
