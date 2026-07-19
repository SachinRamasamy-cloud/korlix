# Changelog

## v0.1.0 (Phase 1) — Initial Release

### Added

**Compiler**
- 12-crate Rust workspace with clean separation of concerns
- KLX lexer with Python-style indentation handling (INDENT/DEDENT)
- Full parser: pages, layouts, components, state, events, loops, conditions
- Complete AST model with span tracking for error reporting
- Import resolver and file system route mapper
- Symbol table for cross-module references
- Basic semantic validation with helpful error messages

**Style System**
- JIT CSS engine — generates only used classes
- 17 color palettes × 11 shades (50–950) = 187 color values
- 21 semantic color tokens (primary, success, danger, muted, surface…)
- 11 utility families per color (text-, bg-, border-, ring-, fill-, stroke-…)
- Complete utility class registry: layout, flex, grid, spacing, sizing,
  typography, border, shadow, opacity, transform, transition, animation,
  overflow, cursor, z-index, inset, accessibility
- Variant support: sm/md/lg/xl/2xl breakpoints, hover, focus, active,
  disabled, dark, group-hover, peer-checked, motion-safe, data-open
- Arbitrary value support: w-[320px], bg-[#0f1c24], grid-cols-[1fr_240px]
- Levenshtein-based "did you mean" suggestions for typos

**Component Registry**
- 80+ built-in components registered with full schema
- Component expander: all components expand to semantic HTML at compile time
- Categories: Primitives, Media, Avatar, Navigation, Feedback, Loaders,
  Placeholders, Overlay, Forms, Content, DataDisplay, Marketing, Dashboard
- Full prop schemas with types, defaults, and descriptions
- Runtime feature tracking per component
- Accessibility metadata (aria roles, required props)

**Code Generation**
- HTML generator with proper escaping
- CSS JIT generator with media query grouping
- JS generator for state and event bindings
- Route manifest generation (JSON)
- Build manifest with page list and asset sizes
- SPA shell generation for client-side routing
- Static multi-page generation

**Runtime (korlix.runtime.js)**
- Reactive state engine with Proxy-based DOM binding
- Template-based conditional rendering (if/else)
- Event binding via data-on-* attributes
- Toast system (success, error, warning, info, loading)
- Modal system with focus management and Escape key
- Drawer system with backdrop click
- SPA router with pattern matching and params
- Theme system (dark/light with localStorage)
- Lazy image loading via IntersectionObserver
- Pagination component with server-side support
- HMR WebSocket client for hot drop

**Dev Server**
- Axum-based HTTP server with static file serving
- WebSocket HMR endpoint (/__kx_hmr)
- File watcher with debounced recompile (notify crate)
- Hot drop: CSS-only update, full reload, error message
- Browser error overlay injection
- Colored terminal output

**CLI**
- `korlix new <name>` — scaffold full project with all files
- `korlix dev` — start dev server with hot drop
- `korlix build` — static build with output report
- `korlix build --mode spa` — SPA build
- `korlix check` — lint all .klx files, show diagnostics
- `korlix check --ast` — print AST as JSON
- `korlix preview` — serve production dist/

**TypeScript Runtime Source**
- core/: mount, state, events, dom patcher
- router/: SPA router, link interception, params, pagination
- toast/: toast stack with animations
- overlay/: modal focus trap, drawer, dropdown
- media/: lazy image loader
- theme/: dark/light mode with localStorage
- hmr/: WebSocket client, CSS refresh, error overlay

**Examples**
- Landing page (static mode): hero, features, pricing, navbar, footer
- SPA Dashboard: sidebar layout, metrics cards, data table, modals, settings

**Documentation**
- 01 Getting Started
- 02 Project Structure
- 03 Syntax Reference
- 04 SPA Mode & Routing
- 05 Hot Drop
- 06 Colors & Utilities
- 07 Components
- 08 Toast, Modal & UI
- 09 State, Events & Functions

### Roadmap — v0.2.0
- Full JIT CSS with SSR-safe class names
- Theme token customisation via tokens.klx
- Wider component coverage (data-table, calendar, command palette)
- LSP server (hover docs, autocomplete, go-to-definition)
- VS Code extension

### Roadmap — v0.3.0
- Forms: full validation, dirty/touched, submit state
- API data layer: fetch, cache, retry, abort, pagination
- Loading/error/empty state management
- File upload component

### Roadmap — v1.0.0
- Stable syntax
- Production-grade optimizer (minify, tree-shake, critical CSS)
- Performance budget enforcement
- A11y and security checkers
- SSG mode
- Full documentation website
- Playground (REPL in browser)
