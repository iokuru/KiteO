# ADR 0003: Multi-Test Cases and Array Space Semantics

## Status
Accepted

## Context
In competitive programming (especially Codeforces and CodeChef), problems frequently run across multiple test cases (e.g. `int t; cin >> t; while (t--) solve();`). Furthermore, fixed-size global buffers (e.g. `int a[200005]`) are standard practice in C++.

## Decision
1. **Per-Test Complexity**: The engine reports complexity per individual test case, where `n` denotes the input size of a single test case. The outer `t` loop multiplier is treated as a test-case driver and does not pollute the per-test algorithmic complexity.
2. **Fixed-Size Array Space Complexity**:
   - Fixed-size arrays indexed up to `n` (or sized to problem constraints proportional to input length) yield $O(n)$ space.
   - Fixed-size arrays indexed by element values (such as frequency count tables or sieve arrays) yield $O(A)$ space.
   - If array usage is ambiguous and cannot be bounded to input size or values, space complexity defaults to `Unknown` unless the buffer is statically negligible ($O(1)$).
