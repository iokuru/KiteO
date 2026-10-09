# Kite0

Kite0 is a deterministic, non-generative browser extension powered by a local Rust/WebAssembly static analyzer for competitive programming solutions. It reads code directly from coding platform editors and determines time complexity, space complexity, and primary algorithm labels without using external servers or generative AI.

## Key Features

- **Strict 3-Field Output Contract**:
  1. **Time Complexity (TC)**: Exact symbolic Big-O notation derived via symbolic monomial analysis, loop bound algebra, harmonic sum recognition, and recurrence relation solving ($O(1)$, $O(\log n)$, $O(n)$, $O(n \log n)$, $O(n^2)$, $O(n \cdot m)$, $O((V + E) \log V)$, etc.).
  2. **Space Complexity (SC)**: Auxiliary memory footprint distinguishing input parameters from dynamic heap allocations, recursion stack frames, and distinguishing input-indexed buffers ($O(n)$) from value-indexed tables ($O(A)$).
  3. **Algorithm Classification**: Exactly $\le 3$ canonical labels drawn strictly from the closed 68-algorithm catalog, or `Unknown` if unproven. Never guesses, never hallucinates.
- **100% Offline & Private**: Zero external network requests, zero telemetry, zero analytics, zero LLM calls. Fully sandboxed WebAssembly execution.
- **Dual-Engine Architecture**: Native tree-sitter C runtime on desktop and CLI paired with lightweight serialized AST in WebAssembly.
- **Contest Mode Non-Tampering**: Passive read-only DOM extraction that never interferes with platform timers, ACE editor, Monaco editor, or submission buttons.

## Supported Targets

- **Languages**: C++ (C++11 through C++23) and Java (Java 8 through 21), sharing a unified arena-allocated Intermediate Representation (IR).
- **Platforms**: Codeforces, LeetCode, CodeChef, AtCoder, HackerRank, GeeksforGeeks, HackerEarth, CSES, OnlineGDB, JDoodle, Programiz, Replit.
- **Browsers**: Chrome, Brave, Opera (Manifest V3), and Firefox (Gecko MV3).

## Monorepo Layout

```
Kite0/
├── .github/workflows/       # Automated CI (cargo fmt, clippy, test, wasm check, e2e)
├── benchmark/
│   ├── corpus/              # 152 canonical labeled snippets (76 C++ / 76 Java) & dev2 (70 synthetic)
│   ├── empirical/           # Empirical scaling validator (power-law regression on synthetic & CP patterns)
│   ├── labels/              # Benchmark ground-truth & review registries (needs_human_review.json)
│   ├── real/                # Real accepted solution schemas, cases, and sources
│   ├── reports/             # Evaluator reports, detector status, relabeling, and empirical scaling reports
│   ├── runner/              # Rust benchmark accuracy evaluator & status gate calculator
│   └── schema/              # JSON Schema for benchmark test cases
├── docs/
│   └── ADR/                 # 11 Architecture Decision Records
├── engine/                  # Core Rust static analysis engine
│   ├── src/
│   │   ├── algorithms/      # Closed 68-canonical catalog and structural detectors
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
│   ├── e2e/                 # Playwright end-to-end, browser parity, and privacy audit tests
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
| [ADR 0004](docs/ADR/0004-algorithm-detection-catalog.md) | Canonical Algorithm Catalog (Legacy) | Superseded by ADR 0010 and ADR 0011. |
| [ADR 0005](docs/ADR/0005-unified-ir-for-cpp-and-java.md) | Unified Arena IR for C++ & Java | Language-agnostic Typed Arena IR (`BlockId`, `StmtId`, `ExprId`) sharing identical analyzers. |
| [ADR 0006](docs/ADR/0006-leetcode-adapter-and-constraints.md) | LeetCode Adapter & Constraints | Monaco editor inspection and problem input boundary validation against time limits. |
| [ADR 0007](docs/ADR/0007-parser-wasm-architecture.md) | Dual-Engine WebAssembly Parser | Native tree-sitter C runtime on CLI/desktop paired with lightweight serialized AST in WASM. |
| [ADR 0008](docs/ADR/0008-contest-mode-and-positioning.md) | Contest Mode & DOM Isolation | Read-only inspection isolating extension logic from contest timers, ACE, and Monaco editors. |
| [ADR 0009](docs/ADR/0009-browser-execution-model.md) | Sandboxed Zero-Network Execution | 100% offline WebAssembly execution with zero external network requests and zero telemetry. |
| [ADR 0010](docs/ADR/0010-algorithm-detection-catalog-39.md) | Closed 39-Algorithm Catalog (Legacy) | Superseded by ADR 0011. |
| [ADR 0011](docs/ADR/0011-authoritative-canonical-algorithm-catalog-68.md) | Closed 68-Algorithm Catalog & Gate | Closed 68-label canonical catalog, single source of truth (`catalog.toml`), status gate, and selection pipeline. |

## Authoritative Closed Canonical Algorithm Catalog (68 Labels)

Kite0 strictly restricts its algorithm classification vocabulary to the following closed 68-label catalog declared in [`engine/src/algorithms/catalog.toml`](file:///d:/Hiring%20Projects/Kite0/engine/src/algorithms/catalog.toml):

### Categorical Groups
- **Arrays, Hashing and Searching**: Two Pointers, Fast and Slow Pointers, Sliding Window, Prefix Sum, Difference Array, Binary Search, Binary Search on Answer, Sorting, Hash Map, Hash Set, Frequency Counting, Kadane's Algorithm, Intervals, Matrix / Grid, Coordinate Compression.
- **Stack, Queue and Heap**: Stack, Monotonic Stack, Queue, Monotonic Queue, Heap / Priority Queue, Top K, Two Heaps.
- **Linked Lists**: Linked List, Linked List Reversal, Cycle Detection, Merge Linked Lists.
- **Trees**: BFS, DFS, Binary Search Tree, Trie, LCA, Binary Lifting, Tree DP.
- **Graphs**: DSU, Dijkstra, MST, Topological Sort, Shortest Path, Strongly Connected Components, Bridges and Articulation Points, Eulerian Path.
- **Dynamic Programming**: Dynamic Programming, Memoization, 1D DP, 2D DP, Knapsack DP, Bitmask DP, Interval DP.
- **String Algorithms**: KMP, Z Algorithm, Rolling Hash.
- **General Algorithms and Techniques**: Greedy, Backtracking, Divide and Conquer, Bit Manipulation, Meet in the Middle, Sweep Line.
- **Advanced Data Structures**: Fenwick Tree, Segment Tree, Sparse Table, Lazy Segment Tree.
- **Mathematics and Number Theory**: Sieve, GCD, LCM, Fast Exponentiation, Modular Arithmetic, Prime Factorization, Combinatorics.

### Priority Order (Highest Precedence to Lowest)
1. Segment Tree
2. Fenwick Tree
3. Sparse Table
4. Lazy Segment Tree
5. Dijkstra
6. MST
7. Strongly Connected Components
8. Bridges and Articulation Points
9. Eulerian Path
10. Topological Sort
11. LCA
12. Binary Lifting
13. Trie
14. DSU
15. KMP
16. Z Algorithm
17. Rolling Hash
18. Sieve
19. Kadane's Algorithm
20. Tree DP
21. Bitmask DP
22. Interval DP
23. Knapsack DP
24. 2D DP
25. 1D DP
26. Dynamic Programming
27. Memoization
28. Backtracking
29. Divide and Conquer
30. Binary Search on Answer
31. Two Heaps
32. Top K
33. Heap / Priority Queue
34. Monotonic Stack
35. Monotonic Queue
36. Cycle Detection
37. Linked List Reversal
38. Merge Linked Lists
39. Fast and Slow Pointers
40. Sliding Window
41. Two Pointers
42. Binary Search
43. Difference Array
44. Prefix Sum
45. Coordinate Compression
46. Intervals
47. Matrix / Grid
48. Hash Map
49. Hash Set
50. Frequency Counting
51. Stack
52. Queue
53. Linked List
54. Binary Search Tree
55. Shortest Path
56. Greedy
57. Bit Manipulation
58. Meet in the Middle
59. Sweep Line
60. GCD
61. LCM
62. Fast Exponentiation
63. Modular Arithmetic
64. Prime Factorization
65. Combinatorics
66. Sorting
67. BFS
68. DFS

### Status Gate & Selection Rules
- **Status Gate**: A label is only displayed if its status in `catalog.toml` is `"approved"`. To be approved, its detector must achieve $\ge 95\%$ precision with at least 10 positive and 10 negative cases on locked real accepted solutions. All labels initialize to `"candidate"` and return `Unknown` in normal UI until proven.
- **Selection Pipeline**: Detectors collect structured evidence $\rightarrow$ Candidate labels filtered $\rightarrow$ Evidence-based suppression applied (e.g. Tree DP suppresses Dynamic Programming and DFS; Sliding Window suppresses Two Pointers) $\rightarrow$ Priority ordering $\rightarrow$ Capped at maximum 3 labels.

## Verification & Accuracy

### Benchmark Suite
Run canonical dev corpus (152 cases) and synthetic dev2 suite (70 cases):
```bash
cargo run -p kiteo-runner
cargo run -p kiteo-runner -- --dev2
```

### Real Benchmark Evaluator & Status Gate Calculator
Runs real/placeholder benchmarks and generates `benchmark/reports/detector_status.json`:
```bash
cargo run -p kiteo-runner -- --real
cargo run -p kiteo-runner -- --real-verbose
```
*Note: Held-out real solutions live outside the repository and are evaluated exclusively by providing the `KITEO_HELD_OUT_DIR` environment variable in release builds.*

### Empirical Scaling Validators
Empirically confirms scaling exponents ($\Delta \ln T / \Delta \ln N$) across doubling input sizes ($N \le 8,000,000$) on synthetic loops and real CP patterns:
```bash
python benchmark/empirical/validator.py
python benchmark/empirical/synthetic_pattern_validator.py
```

### Playwright E2E & Parity Tests
Verifies 100% parity across Native Rust CLI, Node WASM, and browser engines (Chromium & Firefox):
```bash
cd extension
npm run test:parity          # 152/152 identical between Native and WebAssembly
npm run test:ast-parity      # 152/152 identical AST JSON between Native and web-tree-sitter
npm run test:browser-parity  # 35/35 Playwright Chromium & Firefox parity
```

## Building & Packaging

### Rust Engine & Native CLI
```bash
cargo test --workspace
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
```

### WebAssembly Engine
```bash
cd engine
wasm-pack build --target web --out-dir ../extension/src/wasm/pkg
```

### Browser Extension Packages
Builds Chrome (Manifest V3) and Firefox distributions in `extension/dist-packages/`:
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
