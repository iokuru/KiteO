# KiteO

KiteO is a deterministic, non-generative browser extension powered by a local Rust/WebAssembly static analyzer for competitive programming solutions. It reads code directly from coding platform editors and determines time complexity, space complexity, and primary algorithm labels without using external servers or generative AI.

## Key Features

- **Strict 3-Field Output Contract**:
  1. **Time Complexity (TC)**: Exact symbolic Big-O notation derived via symbolic monomial analysis, loop bound algebra, harmonic sum recognition, and recurrence relation solving ($O(1)$, $O(\log n)$, $O(n)$, $O(n \log n)$, $O(n^2)$, $O(n \cdot m)$, $O((V + E) \log V)$, etc.).
  2. **Space Complexity (SC)**: Auxiliary memory footprint distinguishing input parameters from dynamic heap allocations, recursion stack frames, and distinguishing input-indexed buffers ($O(n)$) from value-indexed tables ($O(A)$).
  3. **Algorithm Classification**: Exactly $\le 3$ canonical labels drawn strictly from the 26 allowed catalog, or `Unknown` if unproven. Never guesses, never hallucinates.
- **100% Offline & Private**: Zero external network requests, zero telemetry, zero analytics, zero LLM calls. Fully sandboxed WebAssembly execution.
- **Dual-Engine Architecture**: Native tree-sitter C runtime on desktop and CLI paired with lightweight serialized AST in WebAssembly.
- **Contest Mode Non-Tampering**: Passive read-only DOM extraction that never interferes with platform timers, ACE editor, Monaco editor, or submission buttons.

## Supported Targets

- **Languages**: C++ (C++11 through C++23) and Java (Java 8 through 21), sharing a unified arena-allocated Intermediate Representation (IR).
- **Platforms**: Codeforces, LeetCode, CodeChef, AtCoder, HackerRank, GeeksforGeeks, HackerEarth, CSES, OnlineGDB, JDoodle, Programiz, Replit.
- **Browsers**: Chrome, Brave, Opera (Manifest V3), and Firefox (Gecko MV3).

## Monorepo Layout

```
KiteO/
├── .github/workflows/       # Automated CI (cargo fmt, clippy, test, wasm check, e2e)
├── benchmark/
│   ├── corpus/              # 152 canonical labeled snippets (76 C++ / 76 Java)
│   ├── empirical/           # Empirical scaling validator (power-law regression)
│   ├── reports/             # Empirical scaling validation reports
│   ├── runner/              # Rust benchmark accuracy evaluator (152/152 100%)
│   └── schema/              # JSON Schema for benchmark test cases
├── docs/
│   └── ADR/                 # 9 Architecture Decision Records
├── engine/                  # Core Rust static analysis engine
│   ├── src/
│   │   ├── algorithms/      # Canonical detectors for 26 allowed algorithms
│   │   ├── amortized/       # Two-pointer and sum-of-degrees graph rules
│   │   ├── callgraph/       # Recursive call graph & cycle detection
│   │   ├── cfg/             # Control Flow Graph basic blocks
│   │   ├── complexity/      # Symbolic expressions & monomial dominance simplification
│   │   ├── cost_model/      # Embedded TOML cost models for C++ STL & Java collections
│   │   ├── ir/              # Language-agnostic typed arena IR (CppNormalizer, JavaNormalizer)
│   │   ├── loops/           # Linear, geometric, halving, harmonic, lowbit loop analysis
│   │   ├── preprocessor/    # CP macro expansion and template header stripping
│   │   └── space/           # Fixed-size array classification & recursion depth
│   └── tests/               # Unit, snapshot, and proptest property-based suites
├── extension/               # Browser extension (Preact, TypeScript, Vite, WASM)
│   ├── e2e/                 # Playwright end-to-end and privacy audit tests
│   ├── public/icons/        # 16px, 48px, 128px Kite icons
│   ├── scripts/             # Icon generation and cross-browser packaging scripts
│   └── src/                 # Platform adapters, popup UI, content scripts, background worker
└── website/                 # Astro landing and documentation site
```

## Architecture Decision Records (ADRs)

| ADR | Title | Key Decision |
| :--- | :--- | :--- |
| [ADR 0001](docs/ADR/0001-symbolic-representation-and-variables.md) | Symbolic Representation & Variables | Canonical monomial ordering ($n < m < q < k < V < E < A$) and dominance algebra. |
| [ADR 0002](docs/ADR/0002-cost-semantics.md) | Cost Semantics for C++ STL & Java | Embedded TOML cost models accounting for hidden amortized costs and map overheads. |
| [ADR 0003](docs/ADR/0003-test-cases-and-array-space.md) | Multi-Test Cases & Array Space | Per-testcase driver loop stripping and value-indexed $O(A)$ vs input-indexed $O(n)$ array rules. |
| [ADR 0004](docs/ADR/0004-algorithm-detection-catalog.md) | Canonical Algorithm Catalog | Strict closed catalog of 26 allowed labels; max 3 tags; fallback to Unknown without guessing. |
| [ADR 0005](docs/ADR/0005-unified-ir-for-cpp-and-java.md) | Unified Arena IR for C++ & Java | Language-agnostic Typed Arena IR (`BlockId`, `StmtId`, `ExprId`) sharing identical analyzers. |
| [ADR 0006](docs/ADR/0006-leetcode-adapter-and-constraints.md) | LeetCode Adapter & Constraints | Monaco editor inspection and problem input boundary validation against time limits. |
| [ADR 0007](docs/ADR/0007-parser-wasm-architecture.md) | Dual-Engine WebAssembly Parser | Native tree-sitter C runtime on CLI/desktop paired with lightweight serialized AST in WASM. |
| [ADR 0008](docs/ADR/0008-contest-mode-and-positioning.md) | Contest Mode & DOM Isolation | Read-only inspection isolating extension logic from contest timers, ACE, and Monaco editors. |
| [ADR 0009](docs/ADR/0009-browser-execution-model.md) | Sandboxed Zero-Network Execution | 100% offline WebAssembly execution with zero external network requests and zero telemetry. |

## 26 Allowed Canonical Algorithm Catalog

KiteO strictly restricts its algorithm classifications to the following 26 labels:

1. Two Pointers
2. Sliding Window
3. Binary Search
4. Binary Search on Answer
5. Prefix Sum
6. Sorting
7. BFS
8. DFS
9. Topological Sort
10. DSU
11. Dijkstra
12. MST
13. Fenwick Tree
14. Segment Tree
15. Sparse Table
16. Monotonic Stack
17. Monotonic Queue
18. Binary Lifting
19. LCA
20. Tree DP
21. Bitmask DP
22. Sieve
23. KMP
24. Z Algorithm
25. Rolling Hash
26. Trie

## Verification & Accuracy

### Benchmark Suite (152 Cases)
- **Time Complexity Accuracy**: 152/152 (100.0%)
- **Space Complexity Accuracy**: 152/152 (100.0%)
- **Algorithm Match Accuracy**: 152/152 (100.0%)

```bash
cargo run -p kiteo-runner
```

### Empirical Power-Law Validator
Empirically confirms scaling exponents ($\Delta \ln T / \Delta \ln N$) across doubling input sizes ($N \le 8,000,000$):
- $O(1)$: $\alpha \approx 0.0$
- $O(\log n)$: $\alpha \approx 0.0$
- $O(n)$: $\alpha \approx 1.0$
- $O(n \log n)$: $\alpha \approx 1.1$
- $O(n^2)$: $\alpha \approx 2.0$
- $O(n^3)$: $\alpha \approx 3.0$

```bash
python benchmark/empirical/validator.py
```

### Playwright E2E & Privacy Audit Tests
Verifies UI cards, preset loading, Monaco/ACE DOM isolation, and audits network traffic to prove zero outbound requests during static analysis:

```bash
cd extension
npm run test:e2e
```

## Building & Packaging

### Rust Engine & Native CLI
```bash
cargo test --workspace
cargo clippy --all-targets --all-features -- -D warnings
```

### WebAssembly Engine
```bash
cd engine
wasm-pack build --target web --out-dir ../extension/src/wasm/pkg
```

### Browser Extension Packages
Builds Chrome (Manifest V3) and Firefox distributions and produces release `.zip` archives in `extension/dist-packages/`:

```bash
cd extension
npm install
npm run package
```

### Documentation Site
```bash
cd website
npm install
npm run build
npm run preview
```

## License

MIT License. Crafted for competitive programmers with zero hallucinations, deterministic static proofs, and complete privacy.
