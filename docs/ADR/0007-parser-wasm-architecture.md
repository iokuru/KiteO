# ADR 0007: WebAssembly Parser Strategy

## Status
Accepted

## Context
During initial compilation spikes on `wasm32-unknown-unknown`, compiling native C Tree-sitter parsers directly through `clang` failed due to missing standard C library headers (`stdio.h`). The system must run smoothly in standard browser environments (Chrome, Brave, Opera, Firefox) without bloated emulation runtimes.

## Decision
1. **Browser Runtime (WASM)**: The browser extension utilizes `web-tree-sitter` in JavaScript with precompiled `.wasm` grammars (`tree-sitter-cpp.wasm`, `tree-sitter-java.wasm`) to parse source code into a structured syntax tree. The serialized AST is handed directly to the Rust WebAssembly module for normalization and analysis.
2. **Native Runtime (Tests & Benchmarks)**: The Rust engine builds native `tree-sitter`, `tree-sitter-cpp`, and `tree-sitter-java` bindings when compiled for non-wasm targets (e.g. `cargo test`, empirical runner, benchmark evaluators).
3. **AST Contract**: The IR lowerer consumes an AST abstraction that accepts both native Tree-sitter nodes and serialized JSON ASTs, guaranteeing identical behavior across both environments.
