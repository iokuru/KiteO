# ADR 0002: Cost Semantics and Bound Representation

## Status
Accepted

## Context
Algorithms exhibit different cost characteristics depending on whether operations are worst-case, expected, or amortized. For example, hash map lookups are $O(1)$ expected but $O(n)$ worst-case under adversarial collisions. Two-pointer increments and dynamic array appends are amortized $O(1)$.

## Decision
1. Internally, the engine computes and tracks three complexity profiles:
   - Worst-case bound
   - Expected bound
   - Amortized bound
2. For end-user reporting, the engine displays the tightest bound valid over the complete execution of the solution.
3. Hash containers (e.g. `unordered_map`, `HashMap`) report their expected bound ($O(1)$ lookup and insertion) rather than pathological worst-case collision bounds.
4. When a bound cannot be soundly proven by static analysis, the engine outputs `Unknown`. It never provides speculative guesses.
