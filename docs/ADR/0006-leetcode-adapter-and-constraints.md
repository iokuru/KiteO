# ADR 0006: LeetCode Adapter and Constraint Checker

## Status
Accepted

## Context
LeetCode solutions are structured as member functions inside a `Solution` class rather than competitive programming `main()` programs. Readers also need auxiliary space complexity rather than total memory, and need to know whether their asymptotic complexity will pass the given problem limits.

## Decision
1. **Entry Point**: The analyzer targets the solution method of `class Solution`.
2. **Dimension Binding**: Dimension variables (`n`, `m`, `k`) are derived directly from input parameters:
   - `vector.size()` / array length -> `n`
   - matrix dimensions `grid.size()`, `grid[0].size()` -> `n`, `m`
   - string length `s.length()` -> `n`
   - tree/graph node counts -> `V`
3. **Space Semantics**: Space complexity evaluates auxiliary space and strictly excludes input containers and returned answer data structures.
4. **Constraint Verification**:
   - Optional, disabled by default.
   - Parses the LeetCode Constraints section and binds upper bounds to `n`, `m`, `k` and value bounds to `A`.
   - Substitutes maximum values into the computed Time Complexity formula.
   - Evaluates to one of three statuses:
     - `Within limits`: $\le 10^8$ operations
     - `Likely too slow`: $> 10^9$ operations
     - `Can't tell`: Unknown complexity or unparsed constraints
