# SPA Mode & Routing

## Overview

Korlix has a built-in SPA router. In SPA mode, the compiler generates a single `index.html` with a JS-powered router that handles navigation without page reloads.

## Enable SPA Mode

```json
// korlix.config.json
{
  "mode": "spa"
}
```

Or at build time:

```bash
korlix build --mode spa
```

## Defining Routes

In `src/app.klx`:

```klx
app:
  layout main

  routes:
    page "/"         from "./pages/index.klx"
    page "/about"    from "./pages/about.klx"
    page "/blog"     from "./pages/blog/index.klx"
    page "/blog/:slug" from "./pages/blog/[slug].klx"
    page "/products"    from "./pages/products/index.klx"
    page "/products/:id" from "./pages/products/[id].klx"
```

## File-System Route Mapping

| File | Route |
|------|-------|
| `src/pages/index.klx` | `/` |
| `src/pages/about.klx` | `/about` |
| `src/pages/blog/index.klx` | `/blog` |
| `src/pages/blog/[slug].klx` | `/blog/:slug` |
| `src/pages/products/[id].klx` | `/products/:id` |

## Navigation

### Declarative (links)

```klx
link href="/about" "About"
link href="/products/42" "View Product"
```

### Programmatic

```klx
btn "Go to About" on:click:
  navigate("/about")

btn "Go Back" on:click:
  goBack()
```

## Route Manifest

The compiler generates `dist/korlix.routes.json`:

```json
{
  "/": { "id": "index", "path": "/" },
  "/about": { "id": "about", "path": "/about" },
  "/products/:id": { "id": "products_id", "path": "/products/:id" }
}
```

## Generated Output (SPA)

```text
dist/
├── index.html          ← single shell HTML
├── korlix.routes.json
└── assets/
    ├── korlix.css
    ├── korlix.runtime.js
    └── app.js
```
