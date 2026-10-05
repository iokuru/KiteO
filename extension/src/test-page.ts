import init, { analyze_wasm } from './wasm/pkg/kiteo_engine.js';

const presets: Record<string, { lang: string; code: string }> = {
  linear: {
    lang: 'cpp',
    code: `void solve(int n) {\n    for (int i = 0; i < n; i++) {\n        sum += a[i];\n    }\n}`,
  },
  nested: {
    lang: 'cpp',
    code: `void solve(int n) {\n    for (int i = 0; i < n; i++) {\n        for (int j = 0; j < n; j++) {\n            matrix[i][j] = i * j;\n        }\n    }\n}`,
  },
  two_pointers: {
    lang: 'cpp',
    code: `bool hasPair(const vector<int>& a, int target) {\n    int l = 0, r = a.size() - 1;\n    while (l < r) {\n        int s = a[l] + a[r];\n        if (s == target) return true;\n        if (s < target) l++;\n        else r--;\n    }\n    return false;\n}`,
  },
  segtree: {
    lang: 'cpp',
    code: `struct SegTree {\n    int n;\n    vector<int> tree;\n    SegTree(int n) : n(n), tree(4 * n, 0) {}\n    void update(int node, int l, int r, int idx, int val) {\n        if (l == r) { tree[node] = val; return; }\n        int mid = (l + r) / 2;\n        if (idx <= mid) update(2 * node, l, mid, idx, val);\n        else update(2 * node + 1, mid + 1, r, idx, val);\n        tree[node] = tree[2 * node] + tree[2 * node + 1];\n    }\n};`,
  },
  dijkstra: {
    lang: 'java',
    code: `class Solution {\n    public long[] dijkstra(int start, int V, java.util.List<java.util.List<Edge>> g) {\n        long[] dist = new long[V];\n        java.util.PriorityQueue<long[]> pq = new java.util.PriorityQueue<>();\n        dist[start] = 0;\n        return dist;\n    }\n}`,
  },
  sieve: {
    lang: 'java',
    code: `class Solution {\n    public void sieve(int n) {\n        boolean[] isPrime = new boolean[n + 1];\n        for (int p = 2; p * p <= n; p++) {\n            if (isPrime[p]) for (int i = p * p; i <= n; i += p) isPrime[i] = false;\n        }\n    }\n}`,
  },
};

let wasmReady = false;

async function bootstrap() {
  const statusEl = document.getElementById('status');
  if (statusEl) statusEl.textContent = 'Loading WebAssembly engine...';
  try {
    await init();
    wasmReady = true;
    if (statusEl) statusEl.textContent = 'WebAssembly engine loaded ready.';
    runAnalysis();
  } catch (err) {
    if (statusEl) statusEl.textContent = 'Failed to load WebAssembly: ' + String(err);
  }
}

function runAnalysis() {
  if (!wasmReady) return;
  const langSelect = document.getElementById('lang-select') as HTMLSelectElement | null;
  const codeInput = document.getElementById('code-input') as HTMLTextAreaElement | null;
  const tcOut = document.getElementById('tc-output');
  const scOut = document.getElementById('sc-output');
  const algoOut = document.getElementById('algo-output');

  if (!langSelect || !codeInput || !tcOut || !scOut || !algoOut) return;

  const code = codeInput.value;
  const lang = langSelect.value;

  const t0 = performance.now();
  const raw = analyze_wasm(code, lang);
  const t1 = performance.now();

  try {
    const parsed = JSON.parse(raw);
    tcOut.textContent = parsed.tc || 'Unknown';
    scOut.textContent = parsed.sc || 'Unknown';
    algoOut.textContent = parsed.algorithms && parsed.algorithms.length > 0
      ? parsed.algorithms.join(', ')
      : 'None detected';

    const statusEl = document.getElementById('status');
    if (statusEl) {
      statusEl.textContent = `Analyzed locally in ${(t1 - t0).toFixed(2)} ms (Zero network requests)`;
    }
  } catch {
    tcOut.textContent = 'Error';
    scOut.textContent = 'Error';
    algoOut.textContent = 'Error';
  }
}

window.addEventListener('DOMContentLoaded', () => {
  const codeInput = document.getElementById('code-input') as HTMLTextAreaElement | null;
  const runBtn = document.getElementById('run-btn');
  const langSelect = document.getElementById('lang-select') as HTMLSelectElement | null;

  if (codeInput) {
    codeInput.value = presets.linear.code;
  }

  document.querySelectorAll('.preset-btn').forEach((btn) => {
    btn.addEventListener('click', () => {
      const type = btn.getAttribute('data-type');
      if (type && presets[type]) {
        if (codeInput) codeInput.value = presets[type].code;
        if (langSelect) langSelect.value = presets[type].lang;
        runAnalysis();
      }
    });
  });

  runBtn?.addEventListener('click', runAnalysis);
  codeInput?.addEventListener('input', runAnalysis);
  langSelect?.addEventListener('change', runAnalysis);

  bootstrap();
});
