# KiteO

KiteO is a browser extension powered by a local Rust/WebAssembly static analyzer for competitive programming solutions. It reads code directly from coding platform editors and determines time complexity, space complexity, and primary algorithm labels without using external servers or generative AI.

## Supported Targets
- **Languages**: C++ and Java
- **Platforms**: Codeforces, LeetCode, CodeChef, AtCoder, HackerRank, GeeksforGeeks, HackerEarth, CSES, OnlineGDB, JDoodle, Programiz, Replit
- **Browsers**: Chrome, Brave, Opera, Firefox

## Architecture
- `engine/`: Core static analysis engine written in Rust (parser, macro preprocessor, IR, CFG, symbolic complexity, cost models, algorithms).
- `extension/`: Manifest V3 browser extension built with TypeScript, Vite, and Preact.
- `benchmark/`: Empirical and symbolic benchmark suite with competitive programming solution corpus.
- `website/`: Astro documentation site.
- `docs/`: Architecture Decision Records (ADRs) and design specifications.
