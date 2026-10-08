# ADR 0011: Authoritative Closed Canonical Algorithm Catalog (68 Labels) and Status Gate

## Status
Accepted (Supersedes ADR 0004 and ADR 0010)

## Context
Previous iterations specified 26-label (ADR 0004) and 39-label (ADR 0010) taxonomies. To provide comprehensive, authoritative coverage across both competitive programming (e.g. Codeforces, CSES, AtCoder) and high-frequency technical interview patterns (e.g. LeetCode) while preserving determinism and zero hallucination, the vocabulary is formalized into a closed, authoritative catalog of exactly 68 canonical labels.

## Decision

### 1. Authoritative 68 Canonical Labels
The catalog is partitioned internally across canonical algorithmic groups:
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

No label outside this closed catalog may ever reach the normal UI.

### 2. Single Source of Truth (`catalog.toml`)
- Defined in `engine/src/algorithms/catalog.toml` with `name`, `status`, `category`, `priority`, and `suppresses`.
- No secondary or conflicting priority table may exist in code.
- Category metadata (`algorithm`, `technique`, `data_structure`, `paradigm`) is internal only and never exposed in the UI.

### 3. Status Gate
- A label is marked `approved` only when its detector mechanically satisfies:
  1. $\text{Precision} \ge 95\%$
  2. $\text{Positive count} \ge 10$ on locked real solutions
  3. $\text{Negative count} \ge 10$ on locked real solutions
- Any label marked `candidate` is suppressed from the output, returning `Unknown`.
- All labels initialize to `status = "candidate"`.

### 4. Selection and Suppression Pipeline (Maximum 3 Displayed Labels)
Execution follows this strict deterministic sequence:
1. Run applicable detectors collecting structured internal evidence.
2. Filter by status gate (candidate labels suppressed).
3. Apply suppression rules:
   - Tree DP suppresses Dynamic Programming and DFS.
   - Bitmask DP, Interval DP, Knapsack DP, 2D DP, 1D DP suppress Dynamic Programming.
   - Dijkstra suppresses BFS and Heap / Priority Queue.
   - Topological Sort suppresses BFS and DFS.
   - Strongly Connected Components, Bridges and Articulation Points, Eulerian Path suppress DFS.
   - Binary Search on Answer suppresses Binary Search.
   - Sliding Window suppresses Two Pointers.
   - Top K and Two Heaps suppress Heap / Priority Queue.
   - Monotonic Stack suppresses Stack; Monotonic Queue suppresses Queue.
   - Lazy Segment Tree suppresses Segment Tree.
   - Fenwick Tree and Segment Tree suppress Prefix Sum when prefix calculations are routed through the tree structure.
4. Priority Ordering: Lower priority number in `catalog.toml` takes precedence.
5. Three-Label Cap: Retain at most 3 highest-priority labels.
6. Deterministic Output: Same input always produces the same labels in the same order. If empty, return `Unknown`.
