# ADR 0010: Closed Canonical 39-Algorithm Catalog and Status Gate

## Status
Accepted (Supersedes ADR 0004)

## Context
ADR 0004 defined a 26-label canonical catalog. To support comprehensive competitive programming problem classification up to 1800 rating and LeetCode, the taxonomy is expanded to a closed catalog of 39 canonical labels, governed by an explicit status gate and deterministic priority-based selection rules.

## Decision

### 1. Catalog (39 Canonical Labels)
Kite0 recognizes exclusively the following 39 canonical algorithm labels:
- **Techniques**: Binary Lifting, Rolling Hash, Binary Search on Answer, Top K, Linked List Reversal, Fast and Slow Pointers, Sliding Window, Two Pointers, Difference Array, Prefix Sum, Coordinate Compression.
- **Data Structures**: Segment Tree, Fenwick Tree, Sparse Table, Trie, DSU, Two Heaps, Heap / Priority Queue, Monotonic Stack, Monotonic Queue.
- **Algorithms**: Dijkstra, MST, Strongly Connected Components, Topological Sort, LCA, KMP, Z Algorithm, Sieve, Kadane's Algorithm, Cycle Detection, Binary Search, BFS, DFS, Sorting.
- **Paradigms**: Tree DP, Bitmask DP, Dynamic Programming, Backtracking, Divide and Conquer.

All other concepts (Hash Map, Hash Set, Frequency Counting, Stack, Queue, Linked List, Matrix / Grid, Greedy, Memoization, 1D DP, 2D DP, Knapsack DP, Shortest Path, Binary Search Tree, Intervals, Meet in the Middle, Sweep Line, Bit Manipulation, Recursion) are non-catalog candidates and are never displayed.

### 2. Status Gate
- The canonical catalog is declared in `engine/src/algorithms/catalog.toml` with `name`, `status` (`approved` or `candidate`), `category`, `priority`, and `suppresses`.
- A label is marked `approved` only when its detector achieves $\ge 95\%$ precision on the held-out real-solution benchmark with at least 10 positive and 10 negative real cases.
- Any label marked `candidate` is suppressed from the output, returning `Unknown` instead of an unverified label.
- Category is internal only and is never displayed in the extension UI.

### 3. Selection and Suppression Rules (Maximum 3 Displayed Labels)
1. **Most specific wins (Suppression)**:
   - Tree DP suppresses Dynamic Programming and DFS.
   - Bitmask DP suppresses Dynamic Programming.
   - Dijkstra suppresses BFS and Heap / Priority Queue.
   - Topological Sort suppresses BFS and DFS.
   - Strongly Connected Components suppresses DFS.
   - Binary Search on Answer suppresses Binary Search.
   - Sliding Window suppresses Two Pointers.
   - Top K and Two Heaps suppress Heap / Priority Queue.
   - Fenwick Tree and Segment Tree suppress Prefix Sum when prefix sums are routed through the tree.
2. **Priority Cap**: If more than 3 labels remain after suppression, retain the top 3 according to the canonical priority sequence (Segment Tree down to Sorting).
3. **Deterministic Output**: The final label list is sorted deterministically according to canonical priority.
4. **Graph / Tree Evidence**: DFS and BFS are singular canonical labels; tree vs graph traversal is internal evidence.
5. **Sorting Threshold**: Sorting is emitted only when it serves as the primary technique, never for incidental sorting.
