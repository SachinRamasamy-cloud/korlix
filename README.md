# Korlix

Korlix is an indentation-based frontend language. `.klx` source files compile to browser-native HTML, CSS and JavaScript without requiring React, Vue, Bootstrap, Tailwind or another client framework at runtime.

```klx
page Counter at "/counter"
  state count: int = 0

  card variant=raised
    h1 "Count: {count}"
    button "Increase" variant=primary click=increment

  fn increment
    count += 1
```

## Implemented V2 foundation

- Simplified `page Name at "/route"` and `fn` syntax, while retaining V1 syntax
- Optional trailing colons for indentation blocks
- Modern native HTML element registry, including common inline SVG primitives
- Korlix-native color vocabulary: `surface-*`, `content-*`, `outline-*`, `accent-*`, `fill-*`, `stroke-*`, `ring-color-*` and `caret-color-*`
- Semantic theme tokens and `light`, `dark`, or `auto` application modes
- Built-in `theme-toggle`
- More than 100 registered components across navigation, cards, forms, feedback, overlays, data, media and layout
- User components with typed props, defaults, required-prop validation and default slots
- Reactive state, local values, derived values, functions/actions, conditions, loops, interpolation and compound assignment
- HTTP queries and `get`, `post`, `put`, `patch`, `delete`, and `reload` statements
- Built-in pagination with explicit page counts or total-record calculation and optional URL synchronization
- Default application layouts and imported aliases
- Duplicate route/declaration validation and basic literal type checking
- Static multipage output and experimental SPA build output

## Quick start

```bash
cargo build --release
./target/release/korlix new my-site
cd my-site
../target/release/korlix dev
```

Build and check an existing project:

```bash
korlix check
korlix build --mode static
```

## Project structure

```text
my-site/
├── korlix.config.json
├── src/
│   ├── main.klx
│   ├── app.klx
│   ├── pages/
│   ├── layouts/
│   └── components/
├── public/
└── dist/
```

## Components and themes

```klx
app Store
  layout MainLayout
  theme auto

layout MainLayout
  navbar variant=glass
    strong "Store"
    theme-toggle
  main
    slot

component product-summary
  prop name: string
  prop price: number

  product-card variant=raised
    h3 name
    p "Price: {price}"
```

## API and pagination

```klx
page Products at "/products"
  state page: int = 1
  get products "/api/products"

  if products.loading
    spinner
  else
    for product in products.data
      product-card product=product

  pagination page=page total=products.total size=20 url-sync

  fn createProduct
    post "/api/products" { name: name }
    reload products
```

## Verification

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
node --check crates/korlix-compiler/runtime-bundle/korlix.runtime.js
```

See [the documentation index](docs/00-index.md), the [implementation-status matrix](docs/18-implementation-status.md), and the runnable [`examples/v2-showcase`](examples/v2-showcase).

## Current scope

The repository now contains a tested V2 language foundation. Static multipage compilation is the stable target. Advanced record-shape inference, lifecycle APIs, full JavaScript interoperability, a package manager, LSP tooling and production-complete behavior for every catalog component remain separate future phases.

## License

MIT
