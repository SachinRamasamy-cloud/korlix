# Testing and Conformance

## Run the complete suite

```bash
cargo test --workspace
```

## Validation layers

1. Lexer tests cover keywords, indentation, classes, literals and events.
2. Parser tests cover API statements, interpolation and HTML registries.
3. Component tests verify registry contracts and the 100-component minimum.
4. Style tests verify the utility registry and color generation.
5. Compiler integration tests compile complete temporary projects.
6. V2 conformance tests verify simplified syntax, functions, layouts, user components, props, themes, pagination, semantic errors and duplicate routes.
7. Runtime JavaScript is syntax-checked with `node --check`.

## Release gate

A Korlix release should not be packaged unless all of the following pass:

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
node --check crates/korlix-compiler/runtime-bundle/korlix.runtime.js
```

The `examples/v2-showcase` project is the primary end-to-end smoke fixture.
