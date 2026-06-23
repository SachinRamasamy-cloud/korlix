# Compiler Architecture

## Overview

Korlix is implemented as a 12-crate Rust workspace. Each responsibility is isolated in its own crate, making it easy to add features without touching unrelated code.

## Crate Map

```
korlix/
└── crates/
    ├── korlix-cli          CLI binary (korlix new, dev, build, check)
    ├── korlix-core         Span tracking, diagnostics, config, source map
    ├── korlix-lexer        KLX tokeniser with indentation handling
    ├── korlix-parser       Token → AST parser
    ├── korlix-ast          AST node types (Program, Module, Item, Node, Expr)
    ├── korlix-resolver     File resolution, import graph, route mapping
    ├── korlix-style        Utility registry, JIT CSS generator, validators
    ├── korlix-components   Component registry, schemas, expander
    ├── korlix-runtime-plan Runtime feature analyzer and manifest
    ├── korlix-codegen      HTML, CSS, JS, route manifest generators
    ├── korlix-dev-server   Axum HTTP server + HMR WebSocket
    └── korlix-compiler     Pipeline orchestrator
```

## Compile Pipeline

```
.klx files
   │
   ▼  korlix-resolver/file_resolver.rs
find_klx_files(src_dir)
   │
   ▼  korlix-lexer/lexer.rs
lex(source, file_id) → Vec<Token>
   │
   ▼  korlix-parser/parser.rs
Parser::parse(tokens) → Module
   │
   ▼  korlix-ast/program.rs
Program { modules: Vec<Module> }
   │
   ▼  korlix-style/scanner.rs
scan_classes(module) → HashSet<String>
   │
   ▼  korlix-resolver/route_resolver.rs
build_route_map(routes) → HashMap<path, RouteEntry>
   │
   ▼  korlix-components/expander.rs
expand_component(node) → Node (compile-time)
   │
   ▼  korlix-codegen/html.rs
render_nodes(body) → String
   │
   ▼  korlix-codegen/css.rs
generate_css(used_classes) → String
   │
   ▼  korlix-codegen/js.rs
generate_app_js(module, routes) → String
   │
   ▼  korlix-codegen/document.rs
generate_document(page, layout, ...) → HTML
   │
   ▼  korlix-compiler/pipeline.rs
write_dist(output, project) → dist/
```

## Adding a New Utility Class

1. Open `crates/korlix-style/src/registry.rs`
2. Add to `build_registry()`:
```rust
add!("glass", CssRule::new("glass")
    .prop("background", "rgba(255,255,255,0.05)")
    .prop("backdrop-filter", "blur(12px)")
    .prop("border", "1px solid rgba(255,255,255,0.1)"));
```
3. No other files need changing.

## Adding a New Component

1. Add schema to `crates/korlix-components/src/registry.rs`
2. Add expansion to `crates/korlix-components/src/expander.rs`
3. Add CSS to `crates/korlix-style/src/generator.rs` COMPONENT_CSS section

Example:
```rust
// registry.rs
reg!(ComponentSchema {
    name: "color-swatch".into(),
    category: ComponentCategory::Primitive,
    html_tag: "div".into(),
    props: vec![
        prop("color", "string", true, None, "CSS color value"),
        prop("size", "string", false, Some("md"), "sm|md|lg"),
    ],
    default_classes: vec!["kx-color-swatch".into()],
    ..
});

// expander.rs
"color-swatch" => {
    let color = get_prop_str(node, "color").unwrap_or("#000");
    Node::Element(ElementNode {
        tag: "div".into(), classes,
        props: vec![Prop::new("style",
            Expr::String(format!("background:{}", color)), node.span)],
        ..
    })
}
```

## Adding a New Language Keyword

1. Add to `crates/korlix-lexer/src/keywords.rs`
2. Add variant to `TokenKind` enum in `token.rs`
3. Add AST node in `korlix-ast/src/`
4. Add parser rule in `korlix-parser/src/statements.rs`
5. Add JS codegen in `korlix-codegen/src/js.rs`

## Diagnostic System

All errors use the `Diagnostic` type from `korlix-core`:

```rust
Diagnostic::error("KX-E201", "Unknown class `.bg-blu`")
    .with_hint("Did you mean: bg-blue-500?")
    .with_span(span)
```

The `DiagnosticSet` collects diagnostics and is threaded through the pipeline.

## Runtime Architecture

The browser runtime is a single vanilla JS file (`korlix.runtime.js`):

```
KorlixRuntime
  .createState(initial)   → reactive proxy
  .bindEvent(sel, ev, fn) → attach DOM listener
  .call(name, args)       → dispatch built-in function
  .mount(selector)        → initialise all subsystems
  .Toast                  → toast stack
  .Overlay                → modal/drawer manager
  .Router                 → SPA router
  .Theme                  → dark/light switcher
  .Pagination             → server-side pagination
  .HMR                    → hot drop WebSocket client
```

The runtime is embedded directly into the compiler binary via `include_str!()`,
so no separate npm install is needed for the generated project.
