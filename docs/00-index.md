# Korlix Documentation

Welcome to the Korlix documentation. Korlix is an ultra-light frontend language that compiles `.klx` files into optimized HTML, CSS, and JavaScript.

## Contents

| # | Document | Description |
|---|----------|-------------|
| 01 | [Getting Started](./01-getting-started.md) | Install Korlix, create your first project |
| 02 | [Project Structure](./02-project-structure.md) | File layout, config, React mapping |
| 03 | [KLX Syntax](./03-syntax.md) | Pages, layouts, components, events, loops |
| 04 | [SPA Mode & Routing](./04-spa-mode.md) | Client-side routing, route manifest |
| 05 | [Hot Drop](./05-hot-drop.md) | CSS/route/component hot reload in dev |
| 06 | [Colors & Utilities](./06-colors-and-utilities.md) | Full utility class reference |
| 07 | [Components](./07-components.md) | 100+ built-in components with examples |
| 08 | [Toast, Modal & UI](./08-toast-and-ui.md) | Toast stack, modal, drawer, overlay |
| 09 | [State, Events & Functions](./09-state-events-functions.md) | Reactive state, event handlers, built-ins |
| 10 | [Error Codes](./10-error-codes.md) | All KX-E, KX-S, KX-A, KX-SEO, KX-P codes |
| 11 | [Compiler Architecture](./11-compiler-architecture.md) | 12-crate system, pipeline, extension points |

## Quick Reference

### Minimal Page

```klx
page index route "/":
  h1 .text-4xl .font-bold .text-primary "Hello World"
```

### With State & Toast

```klx
page counter:
  state count: int = 0

  btn .primary "Click" on:click:
    count = count + 1
    toast success "Clicked!"

  p count
```

### With Component

```klx
component greeting:
  prop name: string

  div .flex .gap-2:
    avatar name=name size="md"
    h3 .font-semibold name
```

### With Layout

```klx
layout main:
  navbar .sticky .top-0:
    link href="/" "Home"
  slot
  footer:
    p "Korlix"
```

### With Data

```klx
page products:
  data items = get "/api/products":
    loading skeleton-card count=6
    error empty-state icon="warning" title="Failed"
    empty empty-state icon="box" title="No items"

  div .grid .grid-cols-3 .gap-6:
    for item in items:
      card .p-4:
        p item.name
```

## Key Naming

| Term | Value |
|------|-------|
| Language | Korlix |
| File extension | `.klx` |
| Binary | `korlix` |
| NPM package | `korlix` |
| CSS prefix | `kx-` |
| Root element | `#korlix-root` |
| Route registry | `window.__KORLIX_ROUTES__` |
| Build manifest | `korlix.manifest.json` |
| Route manifest | `korlix.routes.json` |
