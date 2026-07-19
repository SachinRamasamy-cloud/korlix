# Korlix V2 Implementation Status

This document separates implemented language behavior from experimental or planned work. It should be updated whenever a feature changes maturity.

## Stable foundation

The following behavior is implemented, compiled by the workspace, and covered by automated tests or runnable examples:

- V1-compatible syntax plus simplified V2 `page Name at "/route"` and `fn` declarations
- Indentation blocks with optional trailing colons
- Modern native HTML tag recognition, common SVG primitives, and void-element metadata
- Application layouts, imported aliases, default slots, user components, typed props, prop defaults, and required-prop checks
- More than 100 registered UI component names with shared schema-driven lowering
- Korlix-native color properties and 13-level color scales
- Semantic theme variables, `light`, `dark`, and `auto` modes, and `theme-toggle`
- State, local values, derived values, interpolation, conditions, loops, functions/actions, and compound assignment
- Basic HTTP query and mutation statements for GET, POST, PUT, PATCH, DELETE, and reload
- Pagination rendering, page calculations, and optional URL synchronization
- Duplicate route and declaration checks, unknown-component checks, and basic literal type compatibility
- Static multipage compilation
- Whole-project semantic checks through `korlix check`

## Experimental

These features are available but should be treated as evolving:

- SPA output and client-side routing
- Runtime behavior for the broad generic component catalog; core components have richer behavior than catalog-only components
- API authentication conventions, retries, caching, and advanced cancellation policies
- Rich type inference beyond literal compatibility
- Complex named-slot and nested component composition patterns
- Advanced responsive and interactive style combinations

## Not yet production-complete

The following remain future engineering phases and are not claimed as complete in this repository:

- A flow-sensitive or record-shape-aware type system
- Full JavaScript/TypeScript module interoperability
- A CSP-strict runtime without dynamic function evaluation in every execution path
- Component lifecycle hooks and complete instance isolation for every advanced component
- A package manager and third-party component registry
- Language Server Protocol tooling, autocomplete, rename, and go-to-definition
- A formatter migration engine for every V1 syntax form
- Server-side rendering and data-driven static-site generation
- Production-grade specialized behavior and accessibility certification for every catalog component

## Release verification

Run the following before publishing a revision:

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
node --check crates/korlix-compiler/runtime-bundle/korlix.runtime.js
```

The V2 showcase is located at `examples/v2-showcase`.
