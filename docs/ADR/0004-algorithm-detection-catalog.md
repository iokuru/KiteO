# ADR 0004: Algorithm Detection Catalog and Precision Policy

## Status
Accepted

## Context
AI code assistants frequently speculate and produce incorrect, noisy algorithmic labels (such as categorizing any loop with an `if` statement as "Greedy"). KiteO requires strict accuracy and zero speculative hallucinations.

## Decision
1. **Catalog**: KiteO recognizes only the following 26 canonical algorithm labels:
   - Binary Search
   - Binary Search on Answer
   - Two Pointers
   - Sliding Window
   - Prefix Sum
   - Sorting (only when it is the primary algorithmic technique)
   - BFS
   - DFS
   - Topological Sort
   - DSU
   - Dijkstra
   - MST
   - Fenwick Tree
   - Segment Tree
   - Sparse Table
   - Monotonic Stack
   - Monotonic Queue
   - Binary Lifting
   - LCA
   - Tree DP
   - Bitmask DP
   - Sieve
   - KMP
   - Z Algorithm
   - Rolling Hash
   - Trie
2. **Exclusions**: No `Greedy` label is permitted.
3. **Cardinality**: KiteO outputs at most 3 detected labels for any given solution.
4. **Precision Bias**: Prefer an omitted label over a false positive. Every detector pattern requires negative look-alike test cases.
