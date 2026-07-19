# Korlix Master Plan
> Complete implementation status — what is built, where every file lives, and what it does

---

## Legend
```
✅ Fully implemented & working
🔶 Partially implemented (foundation ready)
📋 Planned — not yet built
```

---

## 1. Workspace Overview

**Root:** `korlix/`

| File | Status | Purpose |
|------|--------|---------|
| `Cargo.toml` | ✅ | Workspace root — lists all 12 member crates + shared dependencies |
| `README.md` | ✅ | Project overview, quick start, syntax examples, architecture diagram |
| `CHANGELOG.md` | ✅ | Version history — Phase 1 features + v0.2/v0.3/v1.0 roadmap |
| `LICENSE` | ✅ | MIT License |
| `SETUP.md` | ✅ | Developer setup guide — build from source, crate dependency graph |

---

## 2. Crate-by-Crate Implementation Status

---

### 2.1 `crates/korlix-core/`
**Purpose:** Foundation shared by all other crates — spans, diagnostics, config, source map.

| File | Status | What it does |
|------|--------|--------------|
| `Cargo.toml` | ✅ | Dependencies: serde, serde_json, thiserror, colored, indexmap, dunce, pathdiff |
| `src/lib.rs` | ✅ | Re-exports all modules: span, source, diagnostics, config, result, paths |
| `src/span.rs` | ✅ | `Pos { line, col, offset }` and `Span { start, end, file_id }` — used for error locations everywhere |
| `src/source.rs` | ✅ | `SourceFile { id, path, content }` and `SourceMap` — registry of all loaded .klx files |
| `src/diagnostics.rs` | ✅ | `Diagnostic { severity, code, message, span, hint }` — the entire error/warning system with colored terminal output |
| `src/config.rs` | ✅ | `KorlixConfig` — parses `korlix.config.json` (name, src, dist, mode, theme, server, budget) |
| `src/result.rs` | ✅ | `KorlixError` enum and `KorlixResult<T>` type alias |
| `src/paths.rs` | ✅ | `normalize()`, `relative_to()`, `route_from_path()` — converts file paths to URL routes |

---

### 2.2 `crates/korlix-ast/`
**Purpose:** All AST (Abstract Syntax Tree) data types. Every compiler stage reads/writes these.

| File | Status | What it does |
|------|--------|--------------|
| `Cargo.toml` | ✅ | Depends on: korlix-core, serde, serde_json |
| `src/lib.rs` | ✅ | Re-exports all AST modules |
| `src/program.rs` | ✅ | Top-level: `Program { modules }`, `Module { file_id, path, imports, items }`, `Item` enum (AppDecl, Page, Layout, Component…) |
| `src/declarations.rs` | ✅ | `ImportDecl`, `PropDecl`, `StateDecl`, `LetDecl`, `DerivedDecl`, `ActionDecl`, `DataDecl`, `MetaBlock`, `RouteDecl`, `ThemeDecl` |
| `src/node.rs` | ✅ | `Node` enum — the body of any page/layout/component: Element, Component, Text, State, Let, If, For, Action, Slot, Raw, Assign, Call |
| `src/element.rs` | ✅ | `ElementNode` (HTML tags), `ComponentNode` (Korlix components), `ClassRef`, `Prop`, `EventHandler`, `SlotFill` |
| `src/expression.rs` | ✅ | `Expr` enum — String, Number, Bool, Null, Identifier, List, Object, Binary, Unary, Call, Member, Index, Ternary, Interpolated |
| `src/types.rs` | ✅ | `KType` enum — string, int, float, bool, list<T>, record, image, icon, url, email, color, component, slot, any… |

---

### 2.3 `crates/korlix-lexer/`
**Purpose:** Converts raw `.klx` source text into a flat list of tokens.

| File | Status | What it does |
|------|--------|--------------|
| `Cargo.toml` | ✅ | Depends on: korlix-core, korlix-ast |
| `src/lib.rs` | ✅ | Public API: exports `lex(source, file_id) → (Vec<Token>, DiagnosticSet)` |
| `src/token.rs` | ✅ | `TokenKind` enum — all 50+ token types: keywords, literals, classes, operators, brackets, OnEvent, Indent, Dedent, Newline, Eof |
| `src/keywords.rs` | ✅ | `lookup_keyword(str) → Option<TokenKind>` — maps identifiers to keyword tokens (page, layout, component, state, if, for, etc.) |
| `src/indentation.rs` | ✅ | `IndentStack` — tracks indentation levels, produces INDENT/DEDENT tokens Python-style |
| `src/lexer.rs` | ✅ | Main `Lexer` struct — full character-by-character scanning: strings, numbers, identifiers, class refs (`.flex`), on:event tokens, comments, arbitrary values in `[]` |

**Key behaviours:**
- `.bg-blue-500` → `TokenKind::Class("bg-blue-500")`
- `on:click` → `TokenKind::OnEvent("click")`
- `"hello"` → `TokenKind::StringLit("hello")`
- Indentation → `INDENT` / `DEDENT` tokens
- `# comment` → skipped entirely

---

### 2.4 `crates/korlix-parser/`
**Purpose:** Converts the token stream into an AST `Module`.

| File | Status | What it does |
|------|--------|--------------|
| `Cargo.toml` | ✅ | Depends on: korlix-core, korlix-ast, korlix-lexer |
| `src/lib.rs` | ✅ | Public API: exports `Parser` |
| `src/parser.rs` | ✅ | `Parser` struct — token navigation (current, peek_ahead, advance, check, expect), top-level `parse()` entry point |
| `src/statements.rs` | ✅ | Parses: `import`, `mount`, `app:`, `page:`, `layout:`, `component:`, `state`, `let`, `derived`, `action`, `data`, `if/else`, `for` |
| `src/elements.rs` | ✅ | Parses HTML elements and Korlix components — collects classes, props (key=value), inline text, on:event handlers, child blocks |
| `src/expressions.rs` | ✅ | Full expression parser: OR → AND → equality → comparison → add → multiply → unary → postfix (member access, function call, index) → primary |
| `src/blocks.rs` | ✅ | `parse_block()` — consumes INDENT, parses nodes until DEDENT; `skip_newlines()` helper |
| `src/recovery.rs` | ✅ | `recover_to_newline()`, `recover_to_dedent()` — skip tokens to recover from parse errors |

---

### 2.5 `crates/korlix-resolver/`
**Purpose:** File discovery, import resolution, route mapping, symbol table.

| File | Status | What it does |
|------|--------|--------------|
| `Cargo.toml` | ✅ | Depends on: korlix-core, korlix-ast, walkdir, serde, serde_json, indexmap |
| `src/lib.rs` | ✅ | Re-exports all resolver modules |
| `src/file_resolver.rs` | ✅ | `find_klx_files(dir) → Vec<PathBuf>` — walks src/ recursively finding all .klx files; `resolve_relative()` — resolves import paths |
| `src/import_resolver.rs` | ✅ | `resolve_import(base, path)` — resolves relative import paths to absolute |
| `src/route_resolver.rs` | ✅ | `RouteEntry { id, path, source }`, `build_route_map(routes) → HashMap` — converts route declarations to route map used by router and codegen |
| `src/symbol_table.rs` | ✅ | `SymbolTable { components, layouts, pages }` — cross-module reference tracking |

---

### 2.6 `crates/korlix-style/`
**Purpose:** The JIT CSS engine — utility registry, scanner, validator, generator.

| File | Status | What it does |
|------|--------|--------------|
| `Cargo.toml` | ✅ | Depends on: korlix-core, korlix-ast, serde, serde_json, indexmap, once_cell, regex |
| `src/lib.rs` | ✅ | Re-exports: registry, generator, scanner, validator |
| `src/colors.rs` | ✅ | `build_palette()` — 17 color palettes × 11 shades (187 colors) + 21 semantic tokens. `build_css_vars()` — CSS `:root { --kx-* }` declarations |
| `src/tokens.rs` | ✅ | Design token arrays: `SPACING` (0–96), `RADIUS`, `SHADOWS`, `BREAKPOINTS`, `FONT_SIZES`, `FONT_WEIGHTS`, `OPACITY`, `Z_INDEX`, `TRANSITIONS`, `DURATIONS` |
| `src/registry.rs` | ✅ | `CssRule { selector, declarations, media_query }`. `build_registry()` — generates 1000+ utility rules: display, position, flex, grid, spacing, sizing, typography, colors (11 families × all colors), border, shadow, opacity, overflow, z-index, transitions, transforms, cursor, accessibility, inset. `lookup(class)`, `suggest(class)` via Levenshtein distance |
| `src/variants.rs` | ✅ | `STATE_VARIANTS` — 25 state/pseudo variants (hover, focus, dark, group-hover, peer-checked, motion-safe…). `parse_variant()` — splits `hover:bg-blue-500` into `("hover", "bg-blue-500")`. `apply_variant()` — modifies selector or wraps in @media |
| `src/scanner.rs` | ✅ | `scan_classes(module) → HashSet<String>` — walks the full AST collecting all class references used |
| `src/validator.rs` | ✅ | `validate_classes(classes, diag)` — checks every used class against registry, emits KX-E201 with "did you mean" suggestions |
| `src/generator.rs` | ✅ | `generate_css(classes) → String` — JIT: only generates CSS for used classes. Includes BASE_CSS reset, CSS vars, COMPONENT_CSS (toast, modal, drawer, skeleton, spinner, animations, pagination). `resolve_arbitrary()` — handles `w-[320px]`, `bg-[#0f1c24]`, etc. |
| `src/typography.rs` | ✅ | Reserved for future typography helpers (currently handled by registry) |
| `src/layout.rs` | ✅ | Reserved for future layout helpers |
| `src/spacing.rs` | ✅ | Reserved for future spacing helpers |

---

### 2.7 `crates/korlix-components/`
**Purpose:** Component registry, schemas, HTML expander, runtime requirement tracking.

| File | Status | What it does |
|------|--------|--------------|
| `Cargo.toml` | ✅ | Depends on: korlix-core, korlix-ast, korlix-style, serde, serde_json, indexmap, once_cell |
| `src/lib.rs` | ✅ | Re-exports: registry, schema, expander |
| `src/schema.rs` | ✅ | `ComponentSchema { name, category, props, slots, default_classes, runtime_features, html_tag, self_closing, aria_role, description }`. `PropSchema`, `SlotSchema`, `RuntimeFeature` enum, `ComponentCategory` enum |
| `src/registry.rs` | ✅ | `COMPONENT_REGISTRY` (35+ components registered): btn, button, link, icon, image, avatar, card, navbar, footer, sidebar, toast, alert, badge, spinner, skeleton, skeleton-card, empty-state, modal, drawer, tooltip, pagination, input, select, textarea, checkbox, switch, accordion, tabs, table, hero, progress, profile-card, breadcrumb, section, container. `get_component()`, `is_component()`, `all_component_names()` |
| `src/expander.rs` | ✅ | `expand_component(node) → Node` — compile-time expansion of every registered component to HTML ElementNodes. Full expansions for: btn (disabled, loading state), icon (aria), image (lazy, width/height), avatar (initials fallback), spinner, skeleton, skeleton-card, empty-state, toast trigger, modal (with header, close button), pagination (with page buttons), progress (aria), badge, alert, card, navbar, footer, container, section, link (SPA-aware vs external), hero |
| `src/runtime.rs` | ✅ | `analyze_features(module) → HashSet<RuntimeFeature>` — walks AST to determine which runtime modules are needed |
| `src/categories/mod.rs` | ✅ | Stub — future: split each category into its own file |

---

### 2.8 `crates/korlix-runtime-plan/`
**Purpose:** Analyzes what runtime modules are needed, produces feature manifest.

| File | Status | What it does |
|------|--------|--------------|
| `Cargo.toml` | ✅ | Depends on: korlix-core, korlix-ast, serde, serde_json, indexmap |
| `src/lib.rs` | ✅ | Re-exports all modules |
| `src/feature.rs` | ✅ | `RuntimeFeature` enum: Core, Router, State, Toast, Overlay, Media, Theme, Forms, Motion, A11y |
| `src/analyzer.rs` | ✅ | `required_features(providers, has_routes, has_state) → HashSet<RuntimeFeature>` |
| `src/manifest.rs` | ✅ | `runtime_modules(features) → Vec<&str>` — returns ordered list of runtime module names to include |

---

### 2.9 `crates/korlix-codegen/`
**Purpose:** Converts the compiled AST into output files (HTML, CSS, JS, route manifest).

| File | Status | What it does |
|------|--------|--------------|
| `Cargo.toml` | ✅ | Depends on: korlix-core, korlix-ast, korlix-style, korlix-components, korlix-runtime-plan, serde, serde_json, indexmap |
| `src/lib.rs` | ✅ | Re-exports: document module |
| `src/html.rs` | ✅ | `render_nodes(nodes) → String`, `render_node(node) → String` — recursively renders AST to HTML. Handles: elements (with class → `kx-` prefix, props → attributes, events → `data-on-*`), components (via expander), text (state bindings → `data-kx-bind`), if/else (template elements), slots, raw HTML. `html_escape()`, `html_escape_attr()` for XSS safety |
| `src/css.rs` | ✅ | `generate_css_for_module(module) → String` — scans + generates. `generate_css_for_classes(classes) → String` — direct generation |
| `src/js.rs` | ✅ | `generate_app_js(module, routes) → String` — generates: route manifest `window.__KORLIX_ROUTES__`, per-page state init, event bindings via `KorlixRuntime.bindEvent()`. `expr_to_js(expr) → String` — converts AST expressions to JS |
| `src/routes.rs` | ✅ | `generate_route_manifest(routes) → String` — outputs `korlix.routes.json` |
| `src/document.rs` | ✅ | `generate_document(page, layout, css_path, js_paths, app_name) → String` — full HTML document with doctype, meta, CSS link, JS scripts, content injection. `generate_spa_shell()` — single index.html for SPA mode. `generate_build_manifest()` — `korlix.manifest.json` |
| `src/assets.rs` | ✅ | `copy_public_assets(public_dir, dist_dir)` — recursively copies public/ to dist/ |

---

### 2.10 `crates/korlix-compiler/`
**Purpose:** Pipeline orchestrator — runs all stages in order, writes dist/.

| File | Status | What it does |
|------|--------|--------------|
| `Cargo.toml` | ✅ | Depends on all other crates |
| `src/lib.rs` | ✅ | Public API: re-exports compile, write_dist, Project, CompileOutput |
| `src/project.rs` | ✅ | `Project { root, config, src_dir, public_dir, dist_dir }` — loads korlix.config.json and resolves all paths |
| `src/context.rs` | ✅ | `CompileContext { source_map, program, symbols, diagnostics, used_classes }` — shared state threaded through pipeline |
| `src/output.rs` | ✅ | `CompileOutput { pages, css, app_js, runtime_js, route_manifest, build_manifest }`. `PageFile { route, filename, html }` |
| `src/pipeline.rs` | ✅ | `compile(project, mode) → Result<CompileOutput>` — full 10-stage pipeline: find files → lex → parse → scan classes → build routes → expand components → generate HTML → generate CSS → generate JS → build manifests. `write_dist(output, project)` — writes all files to dist/. Embeds runtime via `include_str!("../runtime-bundle/korlix.runtime.js")` |
| `src/compile.rs` | ✅ | Re-exports compile and write_dist for clean public API |
| `runtime-bundle/korlix.runtime.js` | ✅ | **508-line embedded browser runtime** — reactive state (Proxy), DOM binding (data-kx-bind), toast stack, modal/drawer system, SPA router, theme (dark/light + localStorage), lazy images (IntersectionObserver), pagination, HMR WebSocket client, error overlay. Auto-mounts on DOMContentLoaded |

---

### 2.11 `crates/korlix-dev-server/`
**Purpose:** Development HTTP server with WebSocket HMR (Hot Module Replacement).

| File | Status | What it does |
|------|--------|--------------|
| `Cargo.toml` | ✅ | Depends on: korlix-core, korlix-compiler, tokio, axum, tower, tower-http, tokio-tungstenite, futures, notify, serde, serde_json, colored, anyhow |
| `src/lib.rs` | ✅ | Re-exports server module |
| `src/server.rs` | ✅ | `DevServer { project, port }` — Axum HTTP server serving dist/ as static files + WebSocket endpoint at `/__kx_hmr`. On startup: initial build → spawn file watcher thread → start Axum server. Watcher: debounced recompile on .klx change → send HMR message → CSS update or full reload |
| `src/watcher.rs` | ✅ | `watch_project(src_dir, handler)` — uses `notify` crate with 200ms poll interval. Detects `Modify` and `Create` events. Classifies as StyleChange (theme files) or ContentChange |
| `src/websocket.rs` | ✅ | `HmrMessage { type, error? }` — typed messages: css-update, full-reload, error, clear-error. `create_hmr_channel()` — tokio broadcast channel for fan-out to all connected browsers |
| `src/hot_drop.rs` | ✅ | `send_css_update()`, `send_full_reload()`, `send_error()`, `send_clear_error()` — helpers to send HMR messages |
| `src/error_overlay.rs` | ✅ | `format_error_html(error)` — generates the browser error overlay HTML |
| `src/static_files.rs` | ✅ | `guess_mime(path)` — MIME type detection for static file serving |

---

### 2.12 `crates/korlix-cli/`
**Purpose:** The `korlix` binary — command-line interface.

| File | Status | What it does |
|------|--------|--------------|
| `Cargo.toml` | ✅ | Binary target. Depends on: korlix-core, korlix-compiler, korlix-dev-server, clap, tokio, colored, indicatif, anyhow, walkdir |
| `src/main.rs` | ✅ | `Cli` struct (clap derive), `Commands` enum: New, Dev, Build, Check, Preview. Routes to command modules. Tokio async runtime |
| `src/output.rs` | ✅ | `print_banner()`, `print_success()`, `print_error()`, `print_info()`, `print_step()` — colored terminal output helpers |
| `src/commands/mod.rs` | ✅ | Declares: new, dev, build, check, preview |
| `src/commands/new.rs` | ✅ | `run(name)` — creates full project scaffold: all directories, korlix.config.json, package.json, .gitignore, public/index.html, src/main.klx, src/app.klx, src/pages/index.klx, src/pages/about.klx, src/layouts/main.klx, src/components/hero.klx, src/theme/tokens.klx |
| `src/commands/dev.rs` | ✅ | `run()` — loads project, starts DevServer |
| `src/commands/build.rs` | ✅ | `run(mode)` — loads project, runs pipeline, writes dist/, prints size report |
| `src/commands/check.rs` | ✅ | `run(a11y, security, seo, ast)` — lexes + parses all .klx files, validates classes, prints diagnostics per file, prints summary |
| `src/commands/preview.rs` | ✅ | `run(port)` — serves dist/ with Axum, requires build to have run first |

---

## 3. Runtime — TypeScript Source

**Location:** `runtime/src/`

| File | Status | What it does |
|------|--------|--------------|
| `package.json` | ✅ | Runtime package — esbuild + typescript dev deps |
| `tsconfig.json` | ✅ | TypeScript config — ES2017 target, strict, DOM lib |
| `src/core/index.ts` | ✅ | Exports: mount, createState, bindEvents, updateDOM |
| `src/core/mount.ts` | ✅ | `mount(selector)` — initialises Theme, Media, Pagination; wires data-on-* events |
| `src/core/state.ts` | ✅ | `createState(initial)` — Proxy-based reactive state. Updates `data-kx-bind` elements on change |
| `src/core/events.ts` | ✅ | `bindEvents()`, `delegateEvent()` — event binding helpers |
| `src/core/dom.ts` | ✅ | `updateDOM(el, html)` — minimal DOM differ/patcher |
| `src/router/index.ts` | ✅ | Exports router public API |
| `src/router/router.ts` | ✅ | `initRouter()` — intercepts `data-kx-link` clicks, history.pushState, popstate. `navigate(path)`, `matchRoute(path)`, `matchPattern()` |
| `src/router/link.ts` | ✅ | `initLinks()` — SPA link activation |
| `src/router/params.ts` | ✅ | `extractParams()`, `initPagination()` — renders pagination component with page buttons |
| `src/toast/index.ts` | ✅ | Exports showToast, dismissToast |
| `src/toast/toast.ts` | ✅ | `showToast(type, message, opts)` — creates toast DOM, auto-dismiss, close button, slide animation |
| `src/overlay/modal.ts` | ✅ | `openModal(id)` — focus trap, backdrop, Escape key. `closeModal(id)` — restores focus |
| `src/overlay/drawer.ts` | ✅ | `openDrawer(id)`, `closeDrawer(id)` — slide-in drawer with backdrop |
| `src/overlay/dropdown.ts` | ✅ | `initDropdowns()` — click-outside to close |
| `src/media/image.ts` | ✅ | `initMedia()` — IntersectionObserver lazy loader for `img[data-src]` |
| `src/media/lazy.ts` | ✅ | `lazyLoad(img, src)` — assigns data-src for deferred loading |
| `src/theme/index.ts` | ✅ | `initTheme()`, `applyTheme()`, `toggleTheme()`, `getCurrentTheme()` — dark/light + localStorage |
| `src/hmr/client.ts` | ✅ | `connectHMR(port)` — WebSocket to `/__kx_hmr`, handles css-update, full-reload, error overlay, clear-error, auto-reconnect |
| `src/hmr/apply.ts` | ✅ | `applyUpdate()` — stub for future granular patch application |

---

## 4. Templates

**Location:** `templates/basic/`

| File | Status | Purpose |
|------|--------|---------|
| `korlix.config.json` | ✅ | Template config (name replaced by `korlix new`) |
| `package.json` | ✅ | npm scripts: dev, build, preview |
| `.gitignore` | ✅ | Ignores dist/, node_modules/, logs |
| `public/index.html` | ✅ | HTML shell with `#korlix-root` div |
| `src/main.klx` | ✅ | Entry point: `mount App to "#korlix-root"` |
| `src/app.klx` | ✅ | App with 2 routes, toast/modal/theme providers |
| `src/pages/index.klx` | ✅ | Welcome page with CTA buttons and toast demo |
| `src/pages/about.klx` | ✅ | Counter demo page with state and events |
| `src/layouts/main.klx` | ✅ | Navbar + slot + footer layout |
| `src/theme/tokens.klx` | ✅ | Theme config |

---

## 5. Examples

### Landing Page — `examples/landing-page/`

| File | Status | Route | Description |
|------|--------|-------|-------------|
| `korlix.config.json` | ✅ | — | Static mode, port 3000 |
| `src/main.klx` | ✅ | — | Entry point |
| `src/app.klx` | ✅ | — | 3 routes, dark theme |
| `src/pages/index.klx` | ✅ | `/` | Hero, 3 feature cards, stats, CTA |
| `src/pages/about.klx` | ✅ | `/about` | Philosophy, 4-item grid |
| `src/pages/pricing.klx` | ✅ | `/pricing` | 2-tier pricing cards |
| `src/layouts/main.klx` | ✅ | — | Sticky navbar + footer |
| `public/index.html` | ✅ | — | HTML shell |

### SPA Dashboard — `examples/spa-dashboard/`

| File | Status | Route | Description |
|------|--------|-------|-------------|
| `korlix.config.json` | ✅ | — | SPA mode, port 3001 |
| `src/main.klx` | ✅ | — | Entry point |
| `src/app.klx` | ✅ | — | 3 routes, dashboard layout |
| `src/pages/dashboard.klx` | ✅ | `/` | Metrics grid, activity feed, quick actions, modals |
| `src/pages/users.klx` | ✅ | `/users` | Search bar, data table, add-user modal |
| `src/pages/settings.klx` | ✅ | `/settings` | Profile form, notification switches, danger zone |
| `src/layouts/dashboard.klx` | ✅ | — | Sidebar nav + user avatar + main slot |
| `public/index.html` | ✅ | — | HTML shell |

---

## 6. Documentation

**Location:** `docs/`

| File | Status | Contents |
|------|--------|---------|
| `docs/00-index.md` | ✅ | Table of contents + quick reference snippets |
| `docs/01-getting-started.md` | ✅ | Install, `korlix new`, first project, CLI commands |
| `docs/02-project-structure.md` | ✅ | Full directory layout, korlix.config.json, React mapping |
| `docs/03-syntax.md` | ✅ | Pages, layouts, components, events, loops, data types |
| `docs/04-spa-mode.md` | ✅ | SPA routing, route manifest, navigation, file-system mapping |
| `docs/05-hot-drop.md` | ✅ | 4 hot drop levels, WebSocket protocol, error overlay |
| `docs/06-colors-and-utilities.md` | ✅ | Semantic colors, palettes 50-950, utility families, variants, arbitrary values |
| `docs/07-components.md` | ✅ | All 35+ components with full syntax examples |
| `docs/08-toast-and-ui.md` | ✅ | Toast props, modal, drawer, UI function reference |
| `docs/09-state-events-functions.md` | ✅ | State types, derived, events, actions, all built-in functions |
| `docs/10-error-codes.md` | ✅ | KX-E, KX-S, KX-A, KX-SEO, KX-P — all error/warning codes |
| `docs/11-compiler-architecture.md` | ✅ | Crate map, pipeline flow, how to add utilities/components/keywords |

---

## 7. Tests

**Location:** `tests/`

| File | Status | What it tests |
|------|--------|--------------|
| `tests/lexer/basic.rs` | ✅ | Keywords, classes, strings, on:event, indentation, booleans, numbers, comments, arbitrary classes |
| `tests/style/registry.rs` | ✅ | Lookup flex/bg-primary/text-blue-500, spacing, typography, border-radius, color palette, suggest(), grid cols, sr-only, total count > 1000 |
| `tests/components/registry.rs` | ✅ | btn/image/avatar registered, toast runtime feature, modal overlay feature, pagination props, minimum count, is_component(), forms |
| `tests/integration/basic_build.rs` | 🔶 | Hello world compile, CSS generation, no-error on valid KLX |

---

## 8. What Is NOT Yet Built (Roadmap)

### Phase 1 — Gaps (small)

| Item | Location needed | Priority |
|------|----------------|----------|
| Full type checker | `crates/korlix-validator/` (to create) | Medium |
| Full a11y checker | `crates/korlix-a11y/` (to create) | Medium |
| Full security checker | `crates/korlix-security/` (to create) | Medium |
| SEO checker | `crates/korlix-seo/` (to create) | Low |
| CSS minifier | `crates/korlix-optimizer/` (to create) | Low |
| Runtime TS build script | `runtime/build.js` | Low |

### Phase 2 Roadmap

| Feature | Status | Notes |
|---------|--------|-------|
| Theme token customisation via `tokens.klx` | 📋 | Color overrides from user file |
| `data-table` component | 📋 | Sort, filter, pagination built-in |
| `calendar` component | 📋 | Date picker + monthly view |
| `command-palette` component | 📋 | ⌘K search interface |
| `chart-container` component | 📋 | Wraps chart library |
| `kanban` component | 📋 | Drag-and-drop board |
| Container queries (`@container`) | 📋 | CSS utility classes |
| Scroll-driven animations | 📋 | `scroll-timeline` utilities |
| CSS logical properties | 📋 | `margin-inline`, `padding-block` etc. |
| LSP server | `crates/korlix-lsp/` (to create) | Hover docs, autocomplete, go-to-definition |
| VS Code extension | Separate repo | `.klx` syntax highlighting + LSP |
| Formatter (`korlix fmt`) | `crates/korlix-formatter/` (to create) | Auto-format .klx files |

### Phase 3 Roadmap

| Feature | Status | Notes |
|---------|--------|-------|
| Full forms layer | 📋 | Validation, dirty/touched, field errors |
| API data layer | 📋 | fetch, cache, retry, abort, optimistic updates, infinite scroll |
| SSG mode | 📋 | Pre-render routes at build time |
| Performance optimizer | 📋 | Minify, critical CSS, asset hashing, tree-shake |
| Plugin system | 📋 | Custom components/utilities/runtime modules |

### v1.0 Milestones

| Item | Status |
|------|--------|
| Stable syntax spec (no breaking changes) | 📋 |
| All 100+ components fully implemented | 🔶 35/100+ done |
| Full utility class coverage | 🔶 1000+ done, container queries pending |
| Production-grade optimizer | 📋 |
| Full documentation website | 📋 |
| Browser playground (REPL) | 📋 |

---

## 9. Quick File Finder

| What you want to change | File |
|------------------------|------|
| Add a new utility class | `crates/korlix-style/src/registry.rs` → `build_registry()` |
| Add a new component | `crates/korlix-components/src/registry.rs` → `build_registry()`, then `crates/korlix-components/src/expander.rs` |
| Add a new keyword | `crates/korlix-lexer/src/keywords.rs` → `lookup_keyword()`, then `token.rs`, `parser/statements.rs` |
| Change color palette | `crates/korlix-style/src/colors.rs` → `build_palette()` |
| Change base CSS reset | `crates/korlix-style/src/generator.rs` → `BASE_CSS` const |
| Change component CSS | `crates/korlix-style/src/generator.rs` → `COMPONENT_CSS` const |
| Change the browser runtime | `crates/korlix-compiler/runtime-bundle/korlix.runtime.js` |
| Add CLI command | `crates/korlix-cli/src/commands/` → new file, add to `mod.rs` and `main.rs` |
| Change build pipeline | `crates/korlix-compiler/src/pipeline.rs` → `compile()` function |
| Add HTML codegen for new node | `crates/korlix-codegen/src/html.rs` → `render_node()` match arm |
| Add JS codegen for new node | `crates/korlix-codegen/src/js.rs` → `gen_handler_body()` |
| Change project scaffold | `crates/korlix-cli/src/commands/new.rs` → template string consts |
| Add design token | `crates/korlix-style/src/tokens.rs` → appropriate const array |

---

## 10. Dependency Graph

```
korlix-cli
├── korlix-compiler
│   ├── korlix-codegen
│   │   ├── korlix-style
│   │   │   ├── korlix-ast
│   │   │   └── korlix-core
│   │   ├── korlix-components
│   │   │   ├── korlix-style
│   │   │   ├── korlix-ast
│   │   │   └── korlix-core
│   │   └── korlix-runtime-plan
│   │       ├── korlix-ast
│   │       └── korlix-core
│   ├── korlix-parser
│   │   ├── korlix-lexer
│   │   │   ├── korlix-ast
│   │   │   └── korlix-core
│   │   ├── korlix-ast
│   │   └── korlix-core
│   └── korlix-resolver
│       ├── korlix-ast
│       └── korlix-core
└── korlix-dev-server
    ├── korlix-compiler (full tree above)
    └── korlix-core
```

---

*Korlix Master Plan — Phase 1 Complete · Updated v0.1.0*
