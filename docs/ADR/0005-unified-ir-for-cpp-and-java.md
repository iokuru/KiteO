# ADR 0005: Unified Intermediate Representation for C++ and Java

## Status
Accepted

## Context
KiteO treats C++ and Java as first-class languages. Maintaining two independent static analysis pipelines risks divergent complexity calculations, duplicated logic, and maintenance overhead.

## Decision
1. C++ and Java parse into their respective Tree-sitter concrete syntax trees and are immediately lowered into a shared, language-agnostic Intermediate Representation (IR).
2. The IR is arena-allocated and uses typed index references (`StmtId`, `ExprId`, `BlockId`, `VarId`) to ensure high cache locality and zero reference cycles.
3. Equivalent C++ and Java implementations of the same algorithm must lower into isomorphic IR graphs and produce identical complexity and algorithm classifications.
