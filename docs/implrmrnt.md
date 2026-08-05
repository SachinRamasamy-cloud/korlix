Define rules for:

Shadowing
Duplicate declarations
Prop access
State access
Local values
Loop variables
Function parameters
Imported symbols
Implement a practical type checker

Initial type system:

string
int
number
bool
null
list<T>
Records
Nullable values
Functions
Components
API result types

Validate:

Assignments
Operators
Conditions
Function arguments
Function returns
Component props
Record member access
API result use
Complete control-flow syntax

Add only after semantics exist:

return
while
break
continue
try
catch
finally
async
await
Introduce HIR

A stable High-level Intermediate Representation should contain resolved:

Symbols
Types
Components
Props
Slots
Events
Styles
Routes
Runtime requirements

Code generation should not operate on unresolved syntax.

Release gate

Phase 1 is complete when:

Undefined identifiers fail compilation
Unknown functions fail compilation
Function argument counts and types are checked
Component prop expressions are type-checked
Record fields are checked
Import cycles produce useful diagnostics
Code generation receives only resolved HIR
6. Phase 2 — Component Model and Runtime Isolation

Suggested version family: 0.4.x

Objective

Turn components into isolated runtime instances with explicit contracts.

Work
Component instances

Each component instance should own:

Local state
Local actions
Derived values
Event subscriptions
Runtime resources
Child instances
Complete slots

Support:

Default slots
Named slots
Required slots
Fallback content
Slot props
Custom component events

Support:

Typed emitted events
Parent handlers
Event forwarding
Event payload validation
Upgrade generic catalogue components

Prioritize production behavior for:

Button
Link
Navbar
Sidebar
Tabs
Accordion
Card
Form controls
Alert
Toast
Modal
Drawer
Dropdown
Tooltip
Data table
Pagination

Each stable component needs:

Dedicated schema
Semantic HTML
Props
Slots
Events
Variants
Keyboard behavior
Accessibility tests
Light and dark theme tests
CSP-compatible events

Replace dynamic expression execution with generated handler tables.

Recommended output:

<button data-kx-event-id="e17">Save</button>
events.e17 = function (event, context) {
  context.actions.save();
};

Avoid:

eval
new Function
Executable inline attributes
Release gate

Phase 2 is complete when:

Two instances of the same component have isolated state
Mount and unmount clean up resources
Named slots work
Custom events work
Core components pass keyboard tests
Core components pass accessibility checks
Standard output is CSP-compatible
7. Phase 3 — Data, Forms, and Application Services

Suggested version family: 0.5.x

Objective

Support complete daily-use application workflows without custom JavaScript.

Work
Declarative API configuration

Support:

app MyApp
  api
    base=env.API_URL
    timeout=10000

    headers
      Accept="application/json"
Query model

Provide:

data
loading
error
status
reload
cancel
Retry
Timeout
Request deduplication
Cache
Dependency-based refresh
Mutation model

Provide:

Loading state
Data
Error
Success handlers
Error handlers
Finalization
Reset
Optimistic updates
Query invalidation
Form model

Provide:

Form state
Field registration
Two-way binding
Validation
Touched and dirty state
Field errors
Form errors
Async validation
Submission state
Reset
Pagination

Complete:

Offset pagination
Cursor pagination
Query integration
URL synchronization
Browser back/forward restoration
Standard library

Initial modules:

text
list
math
date
json
url
storage
clipboard
timer
Release gate

Phase 3 is complete when:

A CRUD application can be implemented entirely in Korlix
Requests cancel when their page or component unmounts
API errors are typed and recoverable
Forms expose valid, dirty, touched, and error state
Pagination restores correctly from the URL
Authentication hooks do not expose private environment values
8. Phase 4 — Routing, SPA, and Application Lifecycle

Suggested version family: 0.6.x

Objective

Promote SPA mode from experimental to supported without weakening static builds.

Work
Router

Implement:

Exact routes
Dynamic parameters
Nested routes
404 routes
Redirects
Navigation guards
Active-link state
Page lifecycle

On route change:

Unmount current page
Remove event listeners
Cancel requests
Destroy component instances
Release timers
Mount next page
Restore focus
Restore or reset scroll
Route-level output

Generate:

assets/runtime.js
assets/shared.js
assets/routes/home.js
assets/routes/products.js
Strengthen static mode

Support:

Base paths
Canonical URLs
Asset path configuration
Static hosting adapters
Multi-page navigation without history fallback
Complete SPA mode

Support:

One application shell
Page replacement
Route state isolation
Error boundaries
History fallback
Lazy route loading
SSG research

Do not expose stable SSG until Korlix defines:

Build-time data
Dynamic page generation
Revalidation
Hydration boundaries
Release gate

Phase 4 is complete when:

Static mode remains regression-free
SPA route changes replace content
Route changes do not leak listeners or state
Dynamic routes work
404 behavior works
Back and forward navigation restore expected state
9. Phase 5 — Developer Experience and Tooling

Suggested version family: 0.7.x

Objective

Make Korlix practical for daily development.

Work
Formatter

Add:

korlix fmt
korlix fmt --check

Requirements:

Deterministic
Idempotent
Comment-preserving
Stable indentation rules
Language server

Support:

Diagnostics
Autocomplete
Component prop suggestions
Hover documentation
Go to definition
Find references
Rename symbol
Route navigation
Color previews
Editor support

Priority:

VS Code
Neovim LSP
JetBrains through LSP where practical
Source maps

Map generated browser errors back to:

.klx file
Line
Column
Function
Component instance
Development server

Improve:

Incremental rebuild graph
CSS-only replacement
Error overlay
Automatic browser opening
Port conflict handling
Stable file watcher behavior
Inspection commands

Add:

korlix inspect
korlix doctor

Potential output:

Route table
Component tree
Runtime features
Generated bundle size
Unused utilities
Environment checks
Release gate

Phase 5 is complete when:

Formatter output is deterministic
LSP works across imported files
Runtime errors map to KLX source
Development rebuild latency is measured
The error overlay does not destroy the previous valid build
10. Phase 6 — Package and Extension Ecosystem

Suggested version family: 0.8.x

Objective

Allow safe reuse beyond local project files.

Work
Package format

Define package metadata for:

Components
Themes
Utilities
Runtime requirements
Korlix compiler compatibility
Exports
Dependency resolution

Define:

Version rules
Lock file
Checksums
Reproducible installation
Conflict diagnostics
Extension APIs

Potential extension points:

Custom components
Custom color palettes
Custom utility classes
Theme packages
Runtime modules
Compiler-safe transformations
Security

Require:

Integrity verification
Permission declarations
Restricted compiler extensions
Package audit
Path isolation
Official templates

Maintain tested templates for:

Landing page
Documentation site
Dashboard
Storefront
CRUD application
Portfolio
Release gate

Phase 6 is complete when:

Package installations are reproducible
The lock file is deterministic
Package compatibility is validated
Extensions declare required capabilities
Official packages pass compiler-version tests
11. Phase 7 — Production Hardening

Suggested version family: 0.9.x

Objective

Prepare release candidates for production use.

Work
Compiler performance

Improve:

Incremental compilation
Parsed-module caching
Parallel parsing
Dependency-based rebuilds
Memory usage
Output optimization

Implement:

Minification
Content hashing
Dead-code elimination
Runtime tree-shaking
Critical CSS
Bundle budgets
Security analysis

Validate:

Unsafe URL protocols
XSS payloads
Raw HTML
CSP
Environment-secret leakage
Import path traversal
Public asset path traversal
Quality commands

Implement actual behavior for:

korlix check --a11y
korlix check --security
korlix check --seo
Platform releases

Provide tested binaries for:

Linux
Windows
macOS

Release artifacts should include:

Checksums
Changelog
Version compatibility
Migration notes
Release gate

Phase 7 is complete when:

Release builds are reproducible
Bundle budgets are enforced
Security tests pass
Cross-platform CI passes
Core components have no critical accessibility failures
Release binaries and npm packages use synchronized versions
12. Korlix 1.0 Release Criteria

Korlix should not reach 1.0 based only on feature count.

The 1.0 release requires:

Language
Versioned specification
Stable grammar
Stable scope rules
Stable type rules
Documented deprecation policy
Compiler
Resolved HIR
Reliable diagnostics
Source maps
Deterministic output
Cross-platform builds
Runtime
Isolated page and component state
CSP-compatible event handling
Clean lifecycle disposal
Stable API and form behavior
Builds
Production-ready static multipage mode
Supported SPA mode
Reproducible output
Asset hashing and optimization
Components
Documented stable core set
Accessibility contracts
Keyboard behavior
Theme coverage
Browser tests
Tooling
Formatter
LSP
Editor support
Debugging support
Migration tooling
Compatibility
SemVer policy
Migration guides
Supported browser policy
Supported compiler and package version matrix
13. Suggested Version Mapping
Version family	Main focus
0.2.x	Stabilization, specification, conformance
0.3.x	Semantic analysis, types, HIR
0.4.x	Component instances, slots, CSP-safe events
0.5.x	API, forms, pagination, standard library
0.6.x	Router and SPA lifecycle
0.7.x	Formatter, LSP, source maps, editor tooling
0.8.x	Packages and extensions
0.9.x	Production hardening and release candidates
1.0.0	Stable language and compatibility contract

This mapping is a planning recommendation, not a promise of release dates.

14. Continuous Workstreams

The following must continue through every phase.

Documentation

Maintain:

Language reference
Tutorials
Component reference
Architecture
Migration guides
Implementation status
Testing

Maintain:

Unit tests
Parser tests
Semantic tests
Snapshot tests
Browser tests
Accessibility tests
Security tests
Cross-platform tests
Release engineering

Maintain:

Changelog
Version synchronization
npm packages
Rust binaries
Checksums
Release notes
Community

Establish:

Contribution guide
Issue templates
RFC process
Code of Conduct
Governance model
Maintainer responsibilities
15. Priority Order
P0 — Must fix before expansion
Compiler correctness
Theme correctness
Route correctness
Import correctness
Version synchronization
Official example reliability
Conformance tests
P1 — Required for language credibility
Semantic analysis
Type checking
Component isolation
CSP-compatible events
Complete diagnostics
P2 — Required for application development
API configuration
Forms
Pagination integration
Standard library
Supported SPA lifecycle
P3 — Ecosystem and optimization
LSP and editor tooling
Package system
Extensions
SSR and SSG
Advanced optimization
16. Definition of Done

A roadmap item is complete only when all applicable items are satisfied:

Syntax or API is specified
Parser support exists
AST/HIR support exists
Semantic rules exist
Diagnostics exist
Code generation exists
Runtime support exists
Unit tests exist
Integration tests exist
Browser tests exist when relevant
Documentation exists
Official example exists
Migration impact is documented
No stable feature depends on undocumented behavior
17. Recommended Repository Placement

Store the files as:

docs/
  19-future-roadmap.md
  diagrams/
    korlix_future_roadmap.mmd

Add the roadmap to the documentation index and root README:

- [Future Roadmap](docs/19-future-roadmap.md)
18. Scope Control

The following should not be prioritized before compiler and runtime foundations are stable:

Large numbers of additional generic components
Server-side application frameworks
Native mobile compilation
Desktop application compilation
Visual no-code editor
AI-generated source as a core compiler feature
A custom package registry before package contracts exist
SSR or SSG before route lifecycle and data contracts are stable

The most valuable next step is not adding more surface area. It is making the current language predictable, testable, and internally consistent.
'''

out_dir = Path("/mnt/data")
mmd_path = out_dir / "korlix_future_roadmap.mmd"
doc_path = out_dir / "KORLIX_FUTURE_ROADMAP.md"

mmd_path.write_text(mmd, encoding="utf-8")
doc_path.write_text(doc, encoding="utf-8")

print(f"Created: {mmd_path}")
print(f"Mermaid lines: {len(mmd.splitlines())}")
print(f"Created: {doc_path}")
print(f"Documentation lines: {len(doc.splitlines())}")