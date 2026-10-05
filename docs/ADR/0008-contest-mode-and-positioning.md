# ADR 0008: Contest Mode and Non-Generative Product Positioning

## Status
Accepted

## Context
Online competitive programming platforms (Codeforces, AtCoder, CodeChef, LeetCode) enforce strict integrity policies during live rated contests. Tools that assist competitors during contests can jeopardize user accounts and platform trust.

## Decision
1. **Contest Mode Default**: When a platform adapter detects that an active URL is within a live, rated contest, the analyzer is disabled by default and presents the platform's active rule.
2. **No Evasion**: KiteO will never implement bypasses or spoofing techniques to evade contest detection.
3. **Non-Generative Scope**: KiteO does not generate code, offer hints, provide natural language explanations, or suggest refactorings. It serves exclusively as a transparent, deterministic static calculator for asymptotic bounds and algorithmic structure.
