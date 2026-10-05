# ADR 0009: Browser Extension Execution Model and Sandboxing

## Status
Accepted

## Context
Running intensive static code analysis inside page content scripts can degrade editor responsiveness, risk DOM conflicts, and introduce security vulnerabilities.

## Decision
1. **Isolated Execution**: The static analysis engine runs exclusively inside the browser extension's service worker (Chromium) or background event page (Firefox), or inside an offscreen sandbox when necessary.
2. **Zero Injections**: Content scripts operate in a strictly read-only capacity. They extract editor buffer contents through standard DOM/Monaco APIs and never mutate the document or inject UI components into the host platform.
3. **No External Network Requests**: The analyzer performs zero network calls during analysis, ensuring zero telemetry and complete user privacy.
4. **Cross-Browser Consistency**: Analysis logic and WebAssembly binaries remain 100% identical across Chrome, Brave, Opera, and Firefox.
