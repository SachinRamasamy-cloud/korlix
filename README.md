# ◈ Korlix

**Ultra-light frontend language** — Korlix compiles `.klx` files into clean HTML, CSS, and JavaScript.

```klx
page index route "/":
  section .min-h-screen .center .bg-dark:
    h1 .text-6xl .font-bold .text-primary "Hello Korlix"
    p .text-muted "Build websites with clean syntax."

    btn .primary "Show Toast" on:click:
      toast success "Korlix is working!"

    state count: int = 0
    btn .ghost "Count: " on:click:
      count = count + 1
    text count
```

## What Korlix replaces

| Tool | Korlix equivalent |
|------|------------------|
| React | Page + Component + State system |
| Tailwind CSS | Built-in JIT utility engine |
| React Router | Built-in SPA router |
| React Toastify | Built-in `toast` component |
| React Modal | Built-in `modal` component |
| React Hook Form | Built-in `form` + validation |
| Framer Motion | Built-in `motion` utilities |

## Quick Start

```bash
cargo install korlix
korlix new my-site
cd my-site
korlix dev
```

Open `http://localhost:3000`.

## CLI Commands

```bash
korlix new <name>         Create a new project
korlix dev                Start dev server with hot drop
korlix build              Build for production (static)
korlix build --mode spa   Build as SPA
korlix check              Lint all .klx files
korlix check --ast        Print AST to stdout
korlix preview            Preview production build
```

## Features — Phase 1

- **KLX Language** — Clean, indentation-based syntax
- **Lexer + Parser + AST** — Full compiler pipeline in Rust
- **12 Rust crates** — Modular, extensible workspace
- **JIT CSS engine** — Only ships classes you use
- **Full color system** — 17 palettes × 11 shades + semantic tokens
- **100+ utility classes** — Layout, spacing, typography, effects, variants
- **100+ components** — Button, modal, toast, avatar, pagination, forms, and more
- **Component expander** — All components expand to HTML at compile time
- **SPA router** — Built-in client-side routing
- **Hot drop** — CSS, route, and component hot reload in dev
- **TypeScript runtime** — Modular browser runtime (~20kb)
- **Error overlay** — Browser error display during development

## Project Structure

```
my-site/
├── korlix.config.json
├── src/
│   ├── main.klx          # Entry point
│   ├── app.klx           # App config + routes
│   ├── pages/            # Route-mapped pages
│   ├── layouts/          # Layout wrappers
│   ├── components/       # Reusable components
│   └── theme/            # Design tokens
└── dist/                 # Build output
    ├── index.html
    └── assets/
        ├── korlix.css
        ├── korlix.runtime.js
        └── app.js
```

## Syntax Highlights

### Page with state and events
```klx
page counter route "/counter":
  state count: int = 0

  div .flex .flex-col .items-center .gap-4 .py-20:
    h1 .text-5xl .font-bold count
    div .flex .gap-3:
      btn .primary "+" on:click:
        count = count + 1
      btn .ghost "-" on:click:
        count = count - 1
      btn .danger "Reset" on:click:
        count = 0
```

### Component with props
```klx
component user-card:
  prop name: string
  prop role: string = "Member"
  prop avatar: string

  card .p-6 .rounded-xl:
    avatar src=avatar name=name size="lg"
    h3 .font-bold .mt-3 name
    p .text-muted role
```

### Data fetching
```klx
page products route "/products":
  data items = get "/api/products":
    loading skeleton-card count=6
    error empty-state icon="warning" title="Failed to load"
    empty empty-state icon="box" title="No products"

  div .grid .grid-cols-3 .gap-6:
    for item in items:
      product-card product=item
```

## Architecture

```
.klx files
   ↓ File Resolver
   ↓ Lexer (korlix-lexer)
   ↓ Parser (korlix-parser)
   ↓ AST (korlix-ast)
   ↓ Resolver (korlix-resolver)
   ↓ Validator (korlix-validator)
   ↓ Component Expander (korlix-components)
   ↓ Style Scanner (korlix-style)
   ↓ Runtime Analyzer (korlix-runtime-plan)
   ↓ HTML/CSS/JS Generator (korlix-codegen)
   ↓ dist/
```

## License

MIT © Korlix Contributors
# korlix
# korlix
