# ADR 0001: Symbolic Representation and Variable Binding

## Status
Accepted

## Context
Competitive programming problems use standard asymptotic variables to express problem dimensions. Generic algebra systems tend to aggressively reduce or conflate distinct problem dimensions, which destroys the semantic meaning required by competitive programmers (e.g. distinguishing query count from array size).

## Decision
1. KiteO tracks standard competitive programming size dimensions:
   - `n`, `m`: primary input sequence or matrix dimensions
   - `q`: number of queries
   - `k`: window size, subset size, or block parameter
   - `V`: number of vertices in a graph
   - `E`: number of edges in a graph
   - `A`: value range or coordinate maximum (for example, binary search on the answer: `O(n log A)`)
2. Unrelated variables are never merged or collapsed into a single dominant variable: `n + q` remains `n + q` unless explicitly constrained.
3. Asymptotic expressions are normalized with standard Big-O dominance rules within matching variable terms (e.g., `n^2 + n -> n^2`, `n log n + n -> n log n`).
