import { test, expect } from '@playwright/test';

test.describe('Kite0 Analyzer UI & Engine Tests', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/test-page.html');
    await page.waitForLoadState('networkidle');
    await expect(page.locator('#status')).toContainText('Zero network requests');
  });

  test('loads initial page with ready status', async ({ page }) => {
    await expect(page.locator('h1')).toHaveText('Kite0 Local Static Analyzer');
    const status = page.locator('#status');
    await expect(status).toBeVisible();
    await expect(status).toContainText('Zero network requests');
  });

  test('analyzes default linear loop correctly', async ({ page }) => {
    const runBtn = page.locator('#run-btn');
    await runBtn.click();

    await expect(page.locator('#tc-output')).toHaveText('O(n)');
    await expect(page.locator('#sc-output')).toHaveText('O(1)');
    await expect(page.locator('#algo-output')).toHaveText('None detected');
    await expect(page.locator('#status')).toContainText('Zero network requests');
  });

  test('analyzes Sieve preset with O(n log log n) TC and Sieve algorithm', async ({ page }) => {
    const sievePreset = page.locator('button[data-type="sieve"]');
    await sievePreset.click();

    const runBtn = page.locator('#run-btn');
    await runBtn.click();

    await expect(page.locator('#tc-output')).toHaveText('O(n log log n)');
    await expect(page.locator('#sc-output')).toHaveText('O(n)');
    await expect(page.locator('#algo-output')).toContainText('Sieve');
  });

  test('analyzes Dijkstra preset with O((V + E) log V) TC and Dijkstra algorithm', async ({ page }) => {
    const dijkstraPreset = page.locator('button[data-type="dijkstra"]');
    await dijkstraPreset.click();

    const runBtn = page.locator('#run-btn');
    await runBtn.click();

    await expect(page.locator('#tc-output')).toHaveText('O((V + E) log V)');
    await expect(page.locator('#sc-output')).toHaveText('O(V + E)');
    await expect(page.locator('#algo-output')).toContainText('Dijkstra');
  });

  test('analyzes Segment Tree preset with O(n + q log n) TC', async ({ page }) => {
    const segTreePreset = page.locator('button[data-type="segtree"]');
    await segTreePreset.click();

    const runBtn = page.locator('#run-btn');
    await runBtn.click();

    await expect(page.locator('#tc-output')).toHaveText('O(n + q log n)');
    await expect(page.locator('#sc-output')).toHaveText('O(n)');
    await expect(page.locator('#algo-output')).toContainText('Segment Tree');
  });

  test('analyzes custom user-typed quadratic loop code', async ({ page }) => {
    const textarea = page.locator('#code-input');
    await textarea.fill(`void solve(int n) {
      for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
          sum += i * j;
        }
      }
    }`);

    const runBtn = page.locator('#run-btn');
    await runBtn.click();

    await expect(page.locator('#tc-output')).toHaveText('O(n^2)');
    await expect(page.locator('#sc-output')).toHaveText('O(1)');
  });
});
