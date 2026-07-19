<div align="center">

<img src="https://raw.githubusercontent.com/korlix-lang/korlix/main/assets/korlix-logo.svg" alt="Korlix Logo" width="80" height="80" />

# korlix

**Create a new Korlix app in seconds.**

[![npm version](https://img.shields.io/npm/v/create-korlix?color=%236366f1\&label=npm\&style=flat-square)](https://www.npmjs.com/package/create-korlix)
[![npm downloads](https://img.shields.io/npm/dm/create-korlix?color=%2310b981\&style=flat-square)](https://www.npmjs.com/package/create-korlix)
[![license](https://img.shields.io/npm/l/create-korlix?color=%23f59e0b\&style=flat-square)](https://github.com/korlix-lang/korlix/blob/main/LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/korlix-lang/korlix?color=%236366f1\&style=flat-square)](https://github.com/korlix-lang/korlix)

**Korlix = Kor + Lix = The Core Matrix**

Build frontend apps with a clean `.klx` language — no React setup, no router setup, no Tailwind setup, no heavy framework boilerplate.

[Documentation](https://github.com/korlix-lang/korlix/tree/main/docs) · [Examples](https://github.com/korlix-lang/korlix/tree/main/examples) · [Report Bug](https://github.com/korlix-lang/korlix/issues) · [NPM Package](https://www.npmjs.com/package/create-korlix)

</div>

---

## What is Korlix?

**Korlix** is a frontend-first programming language that compiles `.klx` source files into optimized **HTML**, **CSS**, and **JavaScript**.

It is designed for developers who want a simpler frontend workflow without manually wiring together React, Vite, React Router, Tailwind, toast libraries, modal libraries, and state boilerplate.

```text
Korlix = syntax + compiler + styling engine + component system + frontend runtime
```

With Korlix, you write clean `.klx` files and the compiler generates the frontend output for you.

---

## Why create-korlix?

`create-korlix` is the official project scaffolder for Korlix.

It gives you a React/Vite-like project creation flow:

```bash
npm create korlix@latest my-app
```

or, without a folder name:

```bash
npm create korlix@latest
```

If no folder name is provided, Korlix asks:

```text
Project name (my-korlix-app):
```

After that, it:

```text
creates the project
sets SPA mode
```

Then it prints the commands to install dependencies and start the dev server.
This follows the modern React/Vite-style flow and avoids Windows `npm.cmd` child-process issues.

---

## Quick Start

### Create with a folder name

```bash
npm create korlix@latest my-app
```

The creator will scaffold the project and print the next commands:

```bash
cd my-app
npm install
npm run dev
```

Open:

```text
http://localhost:3000
```

---

### Create without a folder name

```bash
npm create korlix@latest
```

Korlix will ask for the project name:

```text
Project name (my-korlix-app):
```

Then it creates the project and prints the commands to install dependencies and start the dev server.

To install dependencies during creation, pass the Korlix creator option after `--`:

```bash
npm create korlix@latest my-app -- --install
```

To install and immediately start the dev server:

```bash
npm create korlix@latest my-app -- --start
```

---

## Other package managers

```bash
# npm
npm create korlix@latest my-app

# yarn
yarn create korlix my-app

# pnpm
pnpm create korlix my-app

# bun
bun create korlix my-app
```

---

## What gets created?

Running:

```bash
npm create korlix@latest my-app
```

creates a ready-to-run Korlix project:

```text
my-app/
├── korlix.config.json
├── package.json
├── public/
│   └── index.html
├── src/
│   ├── main.klx
│   ├── app.klx
│   ├── pages/
│   │   └── index.klx
│   └── theme/
│       └── tokens.klx
└── dist/
```

The project is created in **SPA mode by default**.

---

## Generated npm scripts

Inside the generated project:

```bash
npm run dev
npm run build
npm run preview
npm run check
```

These map to the Korlix CLI:

```bash
korlix dev
korlix build
korlix preview
korlix check
```

---

## Example Korlix syntax

```klx
page home route "/":

  state count: int = 0

  section .min-h-screen .center .bg-dark:

    h1 .text-6xl .font-black .text-primary count

    div .flex .gap-4:

      btn .primary "+" on:click:
        count = count + 1
        toast success "Updated"

      btn .danger "Reset" on:click:
        count = 0
```

This defines a complete interactive page with:

```text
routing
state
events
buttons
toast interaction
utility styling
```

No JSX. No `useState`. No external router setup.

---

## Korlix language features

Korlix is built around a simple frontend syntax.

### Pages and routing

```klx
page about route "/about":

  meta:
    title "About"
    description "About this app"

  section .py-20 .px-8:
    h1 .text-4xl .font-bold "About"
```

### State and events

```klx
page counter route "/counter":

  state count: int = 0

  btn .primary "Increment" on:click:
    count = count + 1

  p "Count: " count
```

### Components

```klx
component feature-card:

  prop title: string
  prop description: string

  card .p-6 .rounded-xl .bg-surface:
    h3 .text-xl .font-bold title
    p .text-muted description
```

### Layout-ready structure

```klx
layout main:

  navbar:
    link href="/" "Home"
    link href="/about" "About"

  slot

  footer:
    p "Built with Korlix"
```

---

## Built-in frontend workflow

Korlix is designed to reduce repeated frontend setup.

| Need         | Traditional setup     | Korlix approach            |
| ------------ | --------------------- | -------------------------- |
| App creation | Vite/React setup      | `npm create korlix@latest` |
| Routing      | Router package        | Korlix pages/routes        |
| Styling      | Tailwind setup/config | Built-in utility classes   |
| State        | Hooks/boilerplate     | `state count: int = 0`     |
| Toasts       | Toast library         | `toast success "Saved"`    |
| Modals       | Modal library         | Built-in modal syntax      |
| Build        | Bundler config        | `korlix build`             |
| Dev server   | Tooling setup         | `korlix dev`               |

---

## Styling system

Korlix includes a utility-first styling system.

Example:

```klx
section .min-h-screen .bg-background .text-foreground .flex .items-center .justify-center:

  div .max-w-5xl .text-center:
    h1 .text-6xl .font-black .text-primary "Hello Korlix"
    p .text-xl .text-muted "Build faster with less setup."
```

Common utility groups include:

```text
layout
spacing
typography
colors
borders
radius
shadow
responsive variants
hover/focus states
dark mode utilities
```

---

## Hot Drop development

The Korlix dev server supports fast local development.

```bash
npm run dev
```

or:

```bash
korlix dev
```

The dev server starts at:

```text
http://localhost:3000
```

During development, Korlix watches your `.klx` files and updates the browser.

---

## Platform support

The Korlix npm package currently includes binaries for:

```text
Linux x64
Windows x64
```

macOS support is planned.

---

## Requirements

For `create-korlix`:

```text
Node.js 18+
npm, yarn, pnpm, or bun
```

You do **not** need to install Rust separately when using the npm package.

The Rust compiler binary is included through the `korlix` npm package.

---

## CLI reference

After the project is created, you can use:

```bash
korlix dev
korlix build
korlix preview
korlix check
```

Or through npm scripts:

```bash
npm run dev
npm run build
npm run preview
npm run check
```

---

## Project creation flow

With name:

```bash
npm create korlix@latest my-app
```

Without name:

```bash
npm create korlix@latest
```

Korlix asks for the project name if needed, creates the app, and prints:

```bash
cd my-app
npm install
npm run dev
```

---

## Why Korlix?

Korlix is for developers who want:

```text
a frontend-first language
less boilerplate
built-in routing
built-in styling
built-in UI interactions
simple state syntax
fast app creation
clean HTML/CSS/JS output
a lightweight alternative to heavy frontend setup
```

It is not trying to be React with different syntax. It is a compiler-first frontend language.

---

## Publishing notes for maintainers

Publish the main compiler package first:

```bash
cd npm/korlix
npm publish
```

Then publish the creator package:

```bash
cd npm/create-korlix
npm publish
```

Reason:

```text
create-korlix depends on korlix
```

If a version is already published, bump the version before publishing again.

Example:

```text
0.1.0 → 0.1.1
```

---

## Contributing

Contributions are welcome.

```bash
git clone https://github.com/korlix-lang/korlix
cd korlix
cargo build --release
```

Project areas:

```text
compiler
lexer
parser
codegen
style engine
runtime
CLI
dev server
templates
documentation
```

Open issues and pull requests on GitHub:

```text
https://github.com/korlix-lang/korlix
```

---

## Roadmap

Current direction:

```text
Phase 1: Compiler core, SPA mode, starter app, dev server
Phase 2: More diagnostics, stronger validation, macOS binary support
Phase 3: VS Code extension, docs website, templates
Phase 4: Stable language spec
```

---

## License

MIT

---

<div align="center">

**Korlix = Kor + Lix = The Core Matrix**

*Build the frontend. Without the complexity.*

</div>
