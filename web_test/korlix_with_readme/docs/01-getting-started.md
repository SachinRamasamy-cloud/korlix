# Getting Started with Korlix

## What is Korlix?

Korlix is a frontend-first language that compiles `.klx` files into clean, optimized HTML, CSS, and JavaScript. It replaces most of the need for React, Tailwind, React Router, and toast/modal/form libraries — with a single clean syntax.

```text
Korlix = syntax + compiler + styling engine + component system + frontend runtime
```

## Installation

```bash
cargo install korlix
```

Or build from source:

```bash
git clone https://github.com/korlix-lang/korlix
cd korlix
cargo build --release
cp target/release/korlix /usr/local/bin/
```

## Your First Project

```bash
korlix new my-site
cd my-site
korlix dev
```

Open `http://localhost:3000` — you'll see your Korlix app running with hot drop.

## CLI Commands

| Command | Description |
|---------|-------------|
| `korlix new <name>` | Create a new project |
| `korlix dev` | Start dev server with hot drop |
| `korlix build` | Production build (static) |
| `korlix build --mode spa` | SPA production build |
| `korlix check` | Lint all .klx files |
| `korlix check --ast` | Print AST to stdout |
| `korlix preview` | Preview the production build |

## Hello World

Create `src/pages/index.klx`:

```klx
page index route "/":
  section .min-h-screen .center .bg-dark:
    h1 .text-5xl .font-bold .text-primary "Hello, Korlix!"
    p .text-lg .text-muted "Build faster with less code."
```

Run `korlix build` and inspect `dist/index.html`.
