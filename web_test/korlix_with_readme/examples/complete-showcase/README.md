# Korlix Complete Showcase

A multipage Korlix V2 application demonstrating the language features that are implemented in the current repository.

The project is intentionally organized by feature area instead of placing every example in one file.

## Pages

| Route | Coverage |
|---|---|
| `/` | Application layout, imports, user components, props, slots, state, conditions, loops, functions and runtime calls |
| `/language` | Values, types, lists, records, expressions, interpolation, functions, actions and control flow |
| `/native-elements` | Semantic HTML, inline elements, lists, forms, tables, media, interactive HTML and SVG |
| `/styling` | All 26 public color families, levels 0–12, semantic tokens, utilities, variants and arbitrary values |
| `/components` | Complete registered component catalogue across every category |
| `/forms` | Native inputs, Boolean attributes, form components and DOM input events |
| `/state-events` | Reactive state, event types, function parameters, local values and runtime utilities |
| `/api` | GET queries, POST, PUT, PATCH, DELETE, reload, loading and error states |
| `/overlays` | Alerts, toasts, modal, drawer, tooltip and generic overlays |
| `/pagination` | Page calculation, URL sync, change events, accessibility and state integration |
| `/themes` | Light, dark, automatic mode, semantic tokens and theme runtime behavior |

See [FEATURE_COVERAGE.md](FEATURE_COVERAGE.md) for a detailed matrix.

## Requirements

- Latest Korlix V2 compiler built from this repository
- A modern browser
- Node.js 18 or newer only for the included mock API

The npm binaries committed in older repository snapshots may still report Korlix `v0.1.0`. Build the Rust workspace before testing V2 syntax.

## Run from the Korlix repository

Build and install the current compiler:

```bash
cargo build --release
cargo install --path crates/korlix-cli
```

Open the example:

```bash
cd examples/complete-showcase
korlix check
korlix dev
```

Open:

```text
http://localhost:3100
```

## Run the mock API

The API page uses a dependency-free Node.js HTTP server.

In a second terminal:

```bash
cd examples/complete-showcase
npm run api
```

The server starts at:

```text
http://localhost:8787
```

Available endpoints:

```text
GET    /api/health
GET    /api/users
POST   /api/users
PUT    /api/users/:id
PATCH  /api/users/:id
DELETE /api/users/:id
```

The API state is stored in memory and resets when the server restarts.

## Production build

```bash
korlix check
korlix build --mode static
korlix preview --port 4173
```

Generated output is written to `dist/`.

## Project structure

```text
complete-showcase/
├── korlix.config.json
├── package.json
├── README.md
├── FEATURE_COVERAGE.md
├── mock-api/
│   └── server.mjs
├── public/
│   ├── index.html
│   ├── assets/
│   │   └── korlix-demo.svg
│   └── data/
│       ├── products.json
│       └── users.json
└── src/
    ├── main.klx
    ├── app.klx
    ├── layouts/
    │   └── showcase-layout.klx
    ├── components/
    │   ├── showcase-code.klx
    │   ├── showcase-feature.klx
    │   ├── showcase-person.klx
    │   └── showcase-section-title.klx
    └── pages/
        ├── index.klx
        ├── language.klx
        ├── native-elements.klx
        ├── styling.klx
        ├── components.klx
        ├── forms.klx
        ├── state-events.klx
        ├── api.klx
        ├── overlays.klx
        ├── pagination.klx
        └── themes.klx
```

## Important maturity notes

This example does not present roadmap items as finished behavior.

- Static multipage output is the stable target.
- All 115 component names are represented, but many are generic semantic shells.
- Top-level `let` and `derived` declarations are included as language examples, but their complete runtime behavior remains incomplete.
- SPA route content mounting is not demonstrated because it is experimental.
- Advanced API configuration such as auth, retry, timeout and caching is not available in declarative KLX syntax yet.
- The runtime still evaluates generated expressions dynamically and is not strict-CSP compatible.
