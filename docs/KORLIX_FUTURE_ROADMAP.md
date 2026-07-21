# Korlix Future Roadmap

**Status:** Proposed  
**Applies to:** Work after the current `0.1.x` foundation  
**Planning model:** Quality-gated, not date-gated  
**Primary stable target:** Static multipage applications  
**Secondary target:** SPA mode after lifecycle and state isolation are complete

---

## 1. Purpose

This document defines the recommended engineering roadmap for moving Korlix from its current language foundation toward a stable `1.0` release.

The roadmap is dependency-oriented. A phase is not complete because implementation has started. It is complete only when its release gate has passed.

> Stabilize the language and compiler before expanding the ecosystem.

A feature should not be considered finished when the parser accepts it. It should be considered finished only when syntax, semantics, diagnostics, generated output, runtime behavior, tests, documentation, and migration effects are all defined.

---

## 2. Current Baseline

The current `0.1.x` implementation provides:

- A Rust compiler workspace
- An indentation-based `.klx` syntax
- Application, page, layout, and component declarations
- Static multipage output
- Experimental SPA output
- Native HTML and common SVG recognition
- A JIT utility and color system
- Light, dark, and automatic themes
- Registered built-in components
- User components with basic props and default slots
- Reactive page state
- Functions, events, conditions, and loops
- Basic API queries and mutations
- Pagination
- Development and preview servers
- Basic compiler diagnostics and tests

Known limitations include:

- Partial semantic analysis
- Partial type checking
- Incomplete top-level `let` and `derived` behavior
- Incomplete component-instance isolation
- Incomplete named slots and event forwarding
- Generic behavior for many registered components
- Experimental SPA page mounting
- Limited declarative API configuration
- Missing formatter and language server
- Dynamic handler evaluation that is not strict-CSP compatible
- Placeholder accessibility, security, and SEO analysis modes

---

## 3. Roadmap Principles

### Stabilize before expanding

New syntax, components, or runtime features should not be added unless they can be validated, tested, and documented consistently.

### Keep static builds authoritative

Static multipage output is currently the strongest Korlix target. It should remain regression-free while SPA support evolves.

### Separate stable and experimental features

Every feature should be marked as one of:

- Stable
- Experimental
- Generic
- Parsed only
- Planned
- Deprecated

### Preserve browser-native output

Korlix should continue generating standard HTML, CSS, JavaScript, browser routes, and assets.

### Require measurable release gates

Version progression should depend on passing quality and compatibility gates rather than arbitrary dates.

---

# 4. Phase 0 — Stabilization and Specification

**Suggested version:** `0.2.x`

## Objective

Create a trustworthy language baseline before expanding semantics or runtime behavior.

## Planned work

### Freeze the V2 language specification

Define:

- Indentation and comments
- Identifiers and contextual keywords
- Literals and operators
- Scope
- State and values
- Functions and actions
- Components
- Props and slots
- Events
- Pages and routes
- Layouts
- Imports
- API syntax
- Styles and themes
- Generated behavior

### Resolve known inconsistencies

Priorities:

- Theme bootstrap and persistence-key consistency
- Semantic color-token ordering
- Boolean property parsing
- Missing-import diagnostics
- Duplicate route behavior
- Top-level `let`
- `derived`
- Version synchronization between Rust and npm packages
- Warnings in official examples

### Create a conformance suite

Each language feature should have:

- Valid source fixture
- Invalid source fixture
- Parser recovery test
- AST snapshot
- HTML snapshot
- CSS snapshot
- JavaScript snapshot
- Diagnostic snapshot

## Release gate

- Every official example passes `korlix check`
- Official examples produce no warnings
- Duplicate routes never overwrite output silently
- Theme generation is deterministic
- CSS selectors are valid
- The CLI version has one authoritative source
- Cross-platform compiler checks pass

---

# 5. Phase 1 — Semantic Analysis and Type System

**Suggested version:** `0.3.x`

## Objective

Move Korlix from syntax validation to real program validation.

## Planned work

### Module graph and imports

Implement:

- Canonical paths
- Imports and exports
- Alias resolution
- Circular import detection
- Project-root boundaries
- Unused import diagnostics

### Scope model

Recommended hierarchy:

```text
Application
  → Module
    → Page, layout, or component
      → Function or action
        → Block
```

Define:

- Shadowing
- Duplicate declarations
- Prop access
- State access
- Local values
- Loop variables
- Function parameters
- Imported symbols

### Practical type checker

Initial coverage:

- Primitive values
- Lists
- Records
- Nullable values
- Assignments
- Operators
- Conditions
- Function arguments
- Function returns
- Component props
- API result types

### Control-flow completion

Add after semantic rules exist:

- `return`
- `while`
- `break`
- `continue`
- `try`
- `catch`
- `finally`
- `async`
- `await`

### Stable HIR

Introduce a resolved intermediate representation containing:

- Symbols
- Types
- Components
- Props
- Slots
- Events
- Styles
- Routes
- Runtime requirements

## Release gate

- Undefined identifiers fail compilation
- Unknown functions fail compilation
- Function arguments and returns are checked
- Component prop expressions are checked
- Record members are checked
- Import cycles produce useful diagnostics
- Code generation receives only resolved HIR

---

# 6. Phase 2 — Component Model and Runtime Isolation

**Suggested version:** `0.4.x`

## Objective

Turn components into isolated runtime instances with explicit contracts.

## Planned work

### Component instances

Each component instance should own:

- Local state
- Local actions
- Derived values
- Event subscriptions
- Runtime resources
- Child instances

### Complete slots

Support:

- Default slots
- Named slots
- Required slots
- Fallback content
- Slot props

### Custom events

Support:

- Typed emitted events
- Parent handlers
- Event forwarding
- Payload validation

### Upgrade the core component set

Prioritize:

- Button
- Link
- Navbar
- Sidebar
- Tabs
- Accordion
- Card
- Form controls
- Alert
- Toast
- Modal
- Drawer
- Dropdown
- Tooltip
- Data table
- Pagination

Each stable component requires:

- Dedicated schema
- Semantic HTML
- Props
- Slots
- Events
- Variants
- Keyboard behavior
- Accessibility tests
- Light and dark tests

### CSP-compatible event generation

Replace dynamic code evaluation with generated handler tables.

## Release gate

- Two component instances have isolated state
- Mount and unmount clean up resources
- Named slots work
- Custom events work
- Core components pass keyboard tests
- Core components pass accessibility checks
- Standard output does not use `eval` or `new Function`

---

# 7. Phase 3 — Data, Forms, and Standard Library

**Suggested version:** `0.5.x`

## Objective

Support complete daily-use application workflows without custom JavaScript.

## Planned work

### Declarative API configuration

Support:

```klx
app MyApp
  api
    base=env.API_URL
    timeout=10000

    headers
      Accept="application/json"
```

### Queries

Provide:

- Data
- Loading state
- Error state
- Status
- Reload
- Cancel
- Retry
- Timeout
- Deduplication
- Cache
- Dependency-based refresh

### Mutations

Provide:

- Loading
- Data
- Error
- Success handlers
- Error handlers
- Finalization
- Reset
- Optimistic updates
- Query invalidation

### Forms

Provide:

- Form state
- Field registration
- Two-way binding
- Validation
- Touched and dirty state
- Field errors
- Form errors
- Async validation
- Submission state
- Reset

### Pagination

Complete:

- Offset pagination
- Cursor pagination
- Query integration
- URL synchronization
- Browser back/forward restoration

### Standard library

Initial modules:

- `text`
- `list`
- `math`
- `date`
- `json`
- `url`
- `storage`
- `clipboard`
- `timer`

## Release gate

- A CRUD application can be written entirely in Korlix
- Requests cancel when a page or component unmounts
- API errors are typed and recoverable
- Forms expose valid, dirty, touched, and error state
- Pagination restores correctly from the URL
- Private environment values are not emitted into browser output

---

# 8. Phase 4 — Routing, SPA, and Lifecycle

**Suggested version:** `0.6.x`

## Objective

Promote SPA mode from experimental to supported without weakening static builds.

## Planned work

### Router

Implement:

- Dynamic parameters
- Nested routes
- 404 routes
- Redirects
- Navigation guards
- Active-link state

### Page lifecycle

On route change:

- Unmount the current page
- Remove listeners
- Cancel requests
- Destroy component instances
- Release timers
- Mount the next page
- Restore focus
- Restore or reset scroll

### Route-level output

Generate per-route JavaScript and shared runtime chunks.

### Strengthen static mode

Support:

- Base paths
- Canonical URLs
- Asset-path configuration
- Static-hosting adapters

### Complete SPA mode

Support:

- One shell
- Page replacement
- Route-state isolation
- Error boundaries
- History fallback
- Lazy route loading

### SSG research

Do not expose stable SSG until build-time data, dynamic pages, revalidation, and hydration boundaries are specified.

## Release gate

- Static mode remains regression-free
- SPA route changes replace page content
- Route changes do not leak state or listeners
- Dynamic routes work
- 404 behavior works
- Back and forward navigation restore expected state

---

# 9. Phase 5 — Developer Experience

**Suggested version:** `0.7.x`

## Objective

Make Korlix practical for daily development.

## Planned work

### Formatter

Add:

```bash
korlix fmt
korlix fmt --check
```

The formatter must be deterministic, idempotent, and comment-preserving.

### Language server

Support:

- Diagnostics
- Autocomplete
- Component prop suggestions
- Hover documentation
- Go to definition
- Find references
- Rename symbol
- Route navigation
- Color previews

### Editor integrations

Priority:

1. VS Code
2. Neovim through LSP
3. JetBrains through LSP where practical

### Source maps

Map browser errors back to `.klx` file, line, column, function, and component instance.

### Development server

Improve:

- Incremental rebuilds
- CSS-only updates
- Error overlay
- Automatic browser opening
- Port-conflict handling
- File-watcher reliability

### Inspection tools

Add:

```bash
korlix inspect
korlix doctor
```

## Release gate

- Formatter output is deterministic
- LSP works across imported files
- Runtime errors map to KLX source
- Development rebuild latency is measured
- Error overlays preserve the previous valid build

---

# 10. Phase 6 — Package and Extension Ecosystem

**Suggested version:** `0.8.x`

## Objective

Allow safe reuse beyond local project files.

## Planned work

### Package format

Define metadata for:

- Components
- Themes
- Utilities
- Runtime requirements
- Compiler compatibility
- Exports

### Dependency resolution

Define:

- Version rules
- Lock file
- Checksums
- Reproducible installs
- Conflict diagnostics

### Extension APIs

Potential extension points:

- Custom components
- Custom color palettes
- Custom utilities
- Theme packages
- Runtime modules
- Compiler-safe transformations

### Security

Require:

- Integrity verification
- Permission declarations
- Restricted compiler extensions
- Package audit
- Path isolation

### Official templates

Maintain tested templates for:

- Landing page
- Documentation site
- Dashboard
- Storefront
- CRUD application
- Portfolio

## Release gate

- Package installations are reproducible
- Lock files are deterministic
- Compatibility is validated
- Extensions declare capabilities
- Official packages pass compiler-version tests

---

# 11. Phase 7 — Production Hardening

**Suggested version:** `0.9.x`

## Objective

Prepare release candidates for production use.

## Planned work

### Compiler and runtime performance

Improve:

- Incremental compilation
- Caching
- Parallel parsing
- Dependency-based rebuilds
- Runtime size
- State-update performance

### Output optimization

Implement:

- Minification
- Content hashing
- Dead-code elimination
- Runtime tree-shaking
- Critical CSS
- Bundle budgets

### Security analysis

Validate:

- Unsafe URL protocols
- XSS payloads
- Raw HTML
- CSP
- Environment-secret leakage
- Import traversal
- Public-asset traversal

### Quality commands

Implement real behavior for:

```bash
korlix check --a11y
korlix check --security
korlix check --seo
```

### Cross-platform releases

Publish tested binaries for Linux, Windows, and macOS, including checksums and release notes.

## Release gate

- Release builds are reproducible
- Bundle budgets are enforced
- Security tests pass
- Cross-platform CI passes
- Core components have no critical accessibility failures
- Rust and npm package versions are synchronized

---

# 12. Korlix 1.0 Release Criteria

Korlix should not reach `1.0` based on feature count.

## Language

- Versioned specification
- Stable grammar
- Stable scope rules
- Stable type rules
- Documented deprecation policy

## Compiler

- Resolved HIR
- Reliable diagnostics
- Source maps
- Deterministic output
- Cross-platform builds

## Runtime

- Isolated page and component state
- CSP-compatible events
- Clean lifecycle disposal
- Stable API and form behavior

## Builds

- Production-ready static mode
- Supported SPA mode
- Reproducible output
- Asset hashing and optimization

## Components

- Documented stable core set
- Accessibility contracts
- Keyboard behavior
- Theme coverage
- Browser tests

## Tooling

- Formatter
- LSP
- Editor support
- Debugging support
- Migration tooling

## Compatibility

- SemVer policy
- Migration guides
- Supported browser policy
- Compiler/package compatibility matrix

---

# 13. Suggested Version Mapping

| Version | Main focus |
|---|---|
| `0.2.x` | Stabilization, specification, conformance |
| `0.3.x` | Semantic analysis, types, HIR |
| `0.4.x` | Component instances, slots, CSP-safe events |
| `0.5.x` | API, forms, pagination, standard library |
| `0.6.x` | Router and SPA lifecycle |
| `0.7.x` | Formatter, LSP, source maps, editor tooling |
| `0.8.x` | Packages and extensions |
| `0.9.x` | Production hardening and release candidates |
| `1.0.0` | Stable language and compatibility contract |

This is a planning recommendation, not a promise of release dates.

---

# 14. Continuous Workstreams

## Documentation

Maintain language reference, tutorials, component docs, architecture, migration guides, and implementation status.

## Testing

Maintain unit, parser, semantic, snapshot, browser, accessibility, security, and cross-platform tests.

## Release engineering

Maintain changelog, version synchronization, npm packages, Rust binaries, checksums, and migration notes.

## Community

Establish a contribution guide, issue templates, RFC process, Code of Conduct, governance, and maintainer responsibilities.

---

# 15. Priority Order

## P0 — Must fix before expansion

- Compiler correctness
- Theme correctness
- Route correctness
- Import correctness
- Version synchronization
- Official example reliability
- Conformance tests

## P1 — Required for language credibility

- Semantic analysis
- Type checking
- Component isolation
- CSP-compatible events
- Complete diagnostics

## P2 — Required for application development

- API configuration
- Forms
- Pagination integration
- Standard library
- Supported SPA lifecycle

## P3 — Ecosystem and optimization

- LSP and editor tooling
- Package system
- Extensions
- SSR and SSG
- Advanced optimization

---

# 16. Definition of Done

A roadmap item is complete only when all applicable items are satisfied:

- Syntax or API is specified
- Parser support exists
- AST/HIR support exists
- Semantic rules exist
- Diagnostics exist
- Code generation exists
- Runtime support exists
- Unit tests exist
- Integration tests exist
- Browser tests exist when relevant
- Documentation exists
- An official example exists
- Migration impact is documented
- No stable feature depends on undocumented behavior

---

# 17. Recommended Repository Placement

```text
docs/
  19-future-roadmap.md
  diagrams/
    korlix_future_roadmap.mmd
```

Add to the documentation index and root README:

```markdown
- [Future Roadmap](docs/19-future-roadmap.md)
```

---

# 18. Scope Control

The following should not be prioritized before the compiler and runtime foundations are stable:

- More generic components without dedicated behavior
- A server-side application framework
- Native mobile compilation
- Desktop application compilation
- A visual no-code editor
- AI-generated source as a core compiler feature
- A custom registry before package contracts exist
- SSR or SSG before routing and data contracts are stable

The highest-value next step is not adding more surface area. It is making the existing language predictable, testable, and internally consistent.
