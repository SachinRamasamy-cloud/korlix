# Setting Up Korlix From Source

## Prerequisites

- Rust 1.75+ (`rustup install stable`)
- Node.js 18+ (for TypeScript runtime build, optional)

## Build

```bash
git clone https://github.com/korlix-lang/korlix
cd korlix

# Build the compiler
cargo build --release

# The binary is at:
./target/release/korlix

# Install globally (optional)
cargo install --path crates/korlix-cli
```

## Verify

```bash
korlix --version
# Korlix v0.1.0
```

## Create Your First Project

```bash
korlix new my-site
cd my-site
korlix dev
```

Visit `http://localhost:3000`.

## Workspace Structure

```
korlix/
├── Cargo.toml              Workspace root
├── crates/
│   ├── korlix-cli/         CLI binary (main entry point)
│   ├── korlix-core/        Diagnostics, spans, config, source map
│   ├── korlix-lexer/       KLX tokeniser
│   ├── korlix-parser/      Token → AST
│   ├── korlix-ast/         AST node types
│   ├── korlix-resolver/    File/import/route resolution
│   ├── korlix-style/       Utility registry + JIT CSS generator
│   ├── korlix-components/  Component registry + expander
│   ├── korlix-runtime-plan Runtime feature analyzer
│   ├── korlix-codegen/     HTML/CSS/JS generators
│   ├── korlix-dev-server/  Axum HTTP + WebSocket HMR server
│   └── korlix-compiler/    Pipeline orchestrator
├── runtime/                TypeScript runtime source
│   └── src/
│       ├── core/           Mount, state, events, DOM patcher
│       ├── router/         SPA router, params, pagination
│       ├── toast/          Toast stack
│       ├── overlay/        Modal, drawer, dropdown
│       ├── media/          Lazy image loader
│       ├── theme/          Dark/light switcher
│       └── hmr/            Hot drop WebSocket client
├── templates/basic/        Project scaffold template
├── examples/
│   ├── landing-page/       Static landing page demo
│   └── spa-dashboard/      SPA dashboard demo
├── docs/                   Full documentation (11 files)
└── tests/                  Test suites
```

## Running Tests

```bash
cargo test
```

## Building the Runtime (optional)

The runtime is pre-bundled at `crates/korlix-compiler/runtime-bundle/korlix.runtime.js`.
To rebuild from TypeScript source:

```bash
cd runtime
npm install
npm run build
cp dist/korlix.runtime.js ../crates/korlix-compiler/runtime-bundle/
```

## Phase 1 Crate Dependencies

```
korlix-cli
  └── korlix-compiler
        ├── korlix-codegen
        │     ├── korlix-style
        │     ├── korlix-components
        │     └── korlix-runtime-plan
        ├── korlix-parser
        │     ├── korlix-lexer
        │     └── korlix-ast
        ├── korlix-resolver
        │     └── korlix-ast
        └── korlix-core
  └── korlix-dev-server
        ├── korlix-compiler
        └── korlix-core
```
