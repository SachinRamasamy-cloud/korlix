<div align="center">

Korlix

A frontend language for building websites with readable .klx files

Korlix brings pages, components, styling, state, events, API calls, themes, and routing into one indentation-based language, then compiles the project into browser-native HTML, CSS, and JavaScript.

<br />



<br />

Getting Started ·Language Overview ·Components ·Workflow Inspector ·Documentation ·Examples ·Roadmap

<br />

Korlix = Kor + Lix = The Core Matrix

</div>

What is Korlix?

Korlix is a frontend-focused programming language and compiler that uses the .klx file extension.

It is designed for developers who want to create websites and browser applications without splitting the interface across separate HTML templates, CSS frameworks, state libraries, component frameworks, and request utilities.

A Korlix source file can define:

Pages and routes

Shared layouts

Reusable components

Native HTML and SVG elements

Reactive state

Functions and events

Conditions and loops

Forms and validation-oriented controls

Responsive styles

Light and dark themes

API queries and HTTP mutations

Pagination, modals, drawers, toasts, and other UI behavior

Korlix compiles these declarations into standard browser output:

.klx source files
        ↓
Lexer and parser
        ↓
Semantic validation
        ↓
Component and layout lowering
        ↓
JIT style generation
        ↓
HTML + CSS + JavaScript

Korlix does not require React, Vue, Angular, Bootstrap, or Tailwind CSS in the generated browser application. Interactive features use the Korlix browser runtime.

A planned Phase 1 developer tool, the Korlix Workflow Inspector, will derive application workflows from the resolved Korlix program, validate structural workflow properties, and overlay development-time execution traces. It is described below as planned work and is not part of the current released CLI.

Why Korlix?

Modern frontend development often requires several separate technologies before a page can become interactive:

Markup
+ styling framework
+ component framework
+ state management
+ router
+ API client
+ build tooling

Korlix provides one language-level model for the common parts of that workflow.

Requirement

Korlix approach

Page structure

Indentation-based .klx elements

Routing

Route declared directly on a page

Components

Reusable components with props and slots

Styling

JIT utilities, semantic colors, and responsive variants

State

Reactive state declarations

Interaction

Functions and event properties

Data

Declarative queries and HTTP mutations

Themes

Light, dark, and automatic modes

Output

Standard HTML, CSS, and JavaScript

Korlix is intended to make ordinary frontend code easier to read while preserving browser-native output.

Quick Start

Create a project with npm

Requires Node.js 18 or newer.

npm create korlix@latest my-app
cd my-app
npm install
npm run dev

Open:

http://localhost:3000

Other package managers:

yarn create korlix my-app
pnpm create korlix my-app
bun create korlix my-app

Install the Korlix CLI

npm install --global korlix

Verify the installation:

korlix --version

Create and run a project:

korlix new my-site
cd my-site
korlix check
korlix dev

Use korlix dev to run a project. There is no korlix run command.

First Korlix Page

Create src/pages/index.klx:

page Home at "/"
  state count: int = 0

  main .min-h-screen .surface-canvas .content-content
    section .max-w-4xl .mx-auto .px-6 .py-20
      badge variant=soft "Korlix"

      h1 .text-5xl .font-bold "Build the web with simpler code"

      p .text-lg .content-content-muted
        "Pages, state, components, themes, and API calls in one language."

      card variant=raised
        h2 "Interactive counter"
        p "Current value: {count}"

        row .gap-3
          button "Increase" variant=primary click
            count += 1

          button "Reset" variant=outline click
            count = 0

Run:

korlix check
korlix dev

Korlix at a Glance

The example below demonstrates an application layout, a reusable component, a page route, state, an API query, responsive styles, iteration, pagination, and an event-driven function.

app StoreApplication
  layout MainLayout
  theme auto

layout MainLayout
  navbar variant=glass
    strong "Korlix Store"
    link href="/products" "Products"
    theme-toggle

  main
    slot

component ProductSummary
  prop name: string
  prop price: number

  product-card variant=raised
    h3 "{name}"
    p "Price: {price}"

page Products at "/products"
  state page: int = 1
  state count: int = 0

  get products "/api/products"

  if productsLoading
    spinner
  else
    grid .grid-cols-1 .md:grid-cols-2 .xl:grid-cols-4
      for product in products
        ProductSummary
          name=product.name
          price=product.price

  pagination
    page=page
    total=100
    perPage=20
    url-sync

  button "Count: {count}" variant=primary click=increment

  fn increment
    count += 1

Current Implementation

Area

Current implementation

Source extension

.klx

Compiler

Rust workspace

Generated output

HTML, CSS, and JavaScript

Stable build target

Static multipage applications

Experimental target

SPA output

Native element names

137 HTML and SVG names

Registered component names

115

Schema-defined components

35

Public color families

26

Color levels

0 through 12

Utility classes

More than 1,000

Color utility combinations

More than 5,500

Themes

Light, dark, and automatic

Runtime modules

State, events, API, router, theme, toast, overlays, pagination, and HMR

The 115 registered component names do not all have the same implementation depth. Some components have dedicated schemas and specialized rendering, while others currently use the generic component renderer.

Language Overview

Applications

An application declaration can configure a shared layout and theme.

app DocumentationSite
  layout DocumentationLayout
  theme auto

Supported theme modes:

light
dark
auto

Pages and Routes

V2 syntax:

page About at "/about"
  h1 "About Korlix"

Legacy V1 syntax remains accepted during migration:

page About route "/about":
  h1 "About Korlix"

Static builds generate directory-based routes:

/              → dist/index.html
/about         → dist/about/index.html
/products      → dist/products/index.html

Layouts

Layouts define structure shared by multiple pages.

layout MainLayout
  navbar
    link href="/" "Home"
    link href="/docs" "Documentation"
    theme-toggle

  main
    slot

  footer
    p "Built with Korlix"

Layout selection order:

Page-specific layout

Application default layout

No layout

Imports

import MainLayout from "./layouts/main.klx"
import UserCard from "./components/user-card.klx"
import "./setup.klx"

Imported aliases can be used for layouts and user-defined components.

State and Values

state count: int = 0
state loading: bool = false
state users = []
state currentUser = null

let pageSize = 20
derived total = price * quantity

Reactive page state is generated through the Korlix runtime.

Current limitation:

Top-level let initialization is incomplete.

Reactive recomputation of top-level derived values is incomplete.

Functions and Actions

fn increase(step)
  count = count + step

action reset
  count = 0

Supported function-body operations include:

Local let values

Assignments

+= and -=

Function calls

Conditions

Loops

API requests

Query reloads

Conditions

if loading
  spinner
else
  p "Loaded"

Loops

for user in users
  profile-card
    h3 "{user.name}"
    p "{user.role}"

Expressions

Korlix supports:

String, integer, float, Boolean, and null literals

Lists and records

Arithmetic operators

Comparison operators

Logical operators

Member access

Index access

Function calls

String interpolation

state total = price * quantity
state visible = active and not loading

p "Welcome, {user.profile.name}"

Events

Direct event function:

button "Save" click=save
input input=updateSearch
form submit=submitForm

Inline event block:

button "Increase" click
  count += 1

Supported event groups include:

Click and double-click

Input and change

Form submission

Focus and blur

Keyboard events

Mouse events

Scroll

Drag and drop

Touch events

Component System

User Components

component UserCard
  prop name: string
  prop role: string = "Member"
  prop active: bool = true

  card variant=raised
    h3 "{name}"
    p "{role}"

    if active
      badge variant=success "Active"

    slot

Usage:

UserCard name="Sachin" role="Administrator"
  button "Open profile" click=openProfile

Implemented component-language features:

Required props

Typed props

Default prop values

Default slots

Imported aliases

Recursive expansion protection

Missing required-prop diagnostics

Basic literal type validation

Current limitations include named-slot invocation, event forwarding, and complete component-instance state isolation.

Built-in Component Catalogue

Korlix registers 115 component names across these groups:

Group

Representative components

Navigation

navbar, sidebar, breadcrumb, tabs, stepper

Content

card, product-card, profile-card, pricing-card, stat-card

Forms

input, select, checkbox, switch, date-picker, file-upload

Feedback

alert, toast, spinner, skeleton, empty-state

Overlays

modal, drawer, tooltip, dropdown, popover

Data

table, data-table, pagination, calendar, data-grid

Media

carousel, gallery, video-player, audio-player

Layout

container, row, column, grid, stack

Component maturity

35 schema-defined components have dedicated props, slots, and output rules.

80 generic components use shared variant, size, disabled, and slot behavior.

A smaller group has specialized lowering or browser-runtime behavior.

Specialized components currently include:

button, link, icon, image, avatar, card, navbar, footer,
container, section, hero, badge, alert, spinner, skeleton,
empty-state, toast, modal, drawer, pagination, progress,
theme-toggle

See:

Components

V2 Component Catalogue

HTML and SVG Support

Korlix recognizes 137 modern HTML and common SVG element names.

Supported categories include:

Document metadata

Semantic page structure

Text and inline semantics

Lists

Forms

Tables

Images, audio, and video

Embedded content

Interactive elements

Templates

SVG shapes, gradients, masks, clipping, and filters

Example:

article
  header
    h1 "Korlix Language Design"
    time datetime="2026-07-19" "19 July 2026"

  p "Korlix uses native semantic web elements."

  figure
    img
      src="/images/compiler.png"
      alt="Korlix compiler architecture"

    figcaption "The Korlix compilation pipeline"

Recognized void elements include:

area, base, br, col, embed, hr, img,
input, link, meta, source, track, wbr

Styling and Colors

Korlix includes a JIT style engine that generates CSS only for utilities detected in source files.

Korlix-native color properties

surface-
content-
outline-
accent-
fill-
stroke-
ring-color-
caret-color-

Example:

card
  .surface-violet-2
  .content-violet-11
  .outline-violet-4

Traditional utility aliases are also supported:

text-
bg-
border-
ring-
fill-
stroke-
outline-
caret-
placeholder-

div .bg-indigo-600 .text-white .border-indigo-700

Color Families

Base families:

slate, gray, zinc, red, orange, amber, yellow,
green, emerald, teal, cyan, blue, indigo,
violet, purple, pink, rose

Aliases:

neutral, ash, stone, sand, lime, mint,
sky, coral, magenta

Each public family exposes Korlix levels:

0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12

Semantic Theme Tokens

canvas
surface
raised
overlay
content
content-muted
outline
brand
success
warning
danger
info

Example:

section .surface-canvas .content-content
  card .surface-raised .outline-outline
    h2 "Theme-aware content"

Responsive Variants

Prefix

Minimum width

sm:

576 px

md:

768 px

lg:

992 px

xl:

1200 px

2xl:

1400 px

grid .grid-cols-1 .md:grid-cols-2 .xl:grid-cols-4

Interaction Variants

hover
focus
focus-visible
active
visited
disabled
checked
invalid
valid
group-hover
peer-checked
dark
data-open
motion-safe
motion-reduce
print

Example:

button
  .bg-indigo-600
  .hover:bg-indigo-700
  .focus:ring-indigo-400
  .disabled:opacity-50

Arbitrary Values

div .w-[320px]
div .h-[calc(100vh-4rem)]
div .surface-[#101827]
div .grid-cols-[240px_1fr]

Themes

Configure the application theme:

app MyApplication
  theme auto

Use the built-in control:

theme-toggle

Or invoke theme behavior:

button "Change theme" click=toggleTheme

The runtime supports:

Light mode

Dark mode

Automatic system mode

Saved user preference

data-kx-theme

Browser color-scheme

System theme change detection

kx:theme-change events

API Requests

Declarative GET Query

get users "/api/users"

The compiler exposes:

users
usersLoading
usersError

Example:

page Users at "/users"
  get users "/api/users"

  if usersLoading
    spinner
  else
    for user in users
      profile-card
        h3 "{user.name}"

HTTP Mutations

post "/api/users" user
put "/api/users/1" user
patch "/api/users/1" changes
delete "/api/users/1"

Reload a query:

reload users

Runtime API:

KorlixRuntime.api.get(url, options)
KorlixRuntime.api.post(url, body, options)
KorlixRuntime.api.put(url, body, options)
KorlixRuntime.api.patch(url, body, options)
KorlixRuntime.api.delete(url, options)
KorlixRuntime.api.reload(name)

The runtime uses browser fetch, handles JSON and text responses, and tracks loading and error state.

Current API-language limitations include declarative headers, authentication, retries, caching, timeouts, cancellation, and typed response shapes.

Pagination

pagination
  page=currentPage
  total=totalRecords
  perPage=20
  siblings=1
  url-sync

Implemented behavior:

First and last page controls

Previous and next controls

Numbered pages

Ellipsis

Disabled boundary states

aria-current

Total-record calculation

URL query synchronization

change and kx:page-change events

Built-in Runtime Functions

Korlix functions can dispatch the following browser actions:

toast
showToast
openModal
closeModal
openDrawer
closeDrawer
navigate
goBack
toggleTheme
scrollTo
copyToClipboard
log

Example:

fn copyCode
  copyToClipboard(code)
  toast("success", "Copied")

Runtime modules include:

Reactive state

Event handling

API requests

Router foundations

Theme management

Toast notifications

Modal and drawer overlays

Pagination

Development hot updates

Workflow Inspector — Phase 1 Planned

Status: Design and test specification complete; implementation not yet included in the current Korlix CLI or compiler workspace.

Goal

The Workflow Inspector is intended to show developers:

What workflow the Korlix source defines

What triggers each operation

Which success, failure, validation, retry, and cleanup paths exist

Which application state changes on each path

Which route or API operation is reached next

Which path actually executed during development

Which branches remain untested

Which workflow defects can be proven deterministically

The primary use case is reviewing AI-assisted and vibe-coded applications where individual files may appear correct while the end-to-end user journey is incomplete or incorrect.

Problem It Addresses

Source review alone can miss workflow-level defects such as:

A button connected to the wrong function

A request with no visible error path

Loading state that is not cleared after failure

A route that cannot be reached

An automatic redirect cycle

An unbounded retry

A background operation that is neither awaited nor explicitly detached

A state value that is written but never rendered

Duplicate form submission while a mutation is active

Runtime behavior that differs from the intended static path

The Workflow Inspector will not use AI as the primary source of workflow truth. Korlix compiler semantics will define the graph; runtime events will show observed execution; AI may later explain confirmed findings.

Planned Analysis Model

flowchart TD
    A[.klx Source] --> B[Lexer and Parser]
    B --> C[Resolved and Validated Korlix Program]
    C --> D[Workflow Lowering]
    D --> E[Normalized Workflow IR]
    E --> F[Deterministic Validator]
    E --> G[Instrumentation Plan]
    E --> H[Workflow Inspector UI]
    G --> I[Development Build]
    I --> J[Runtime Events]
    J --> K[Local Collector]
    K --> H
    F --> H
    F --> L[JSON and SARIF Export]

The planned implementation uses two related views:

View

Question answered

Static workflow graph

What paths can the source program execute?

Runtime trace overlay

What path actually executed in this development run?

An unexecuted node is not automatically a failure. A failed node is one that execution entered and completed with an error. An unresolved node is one whose target could not be proven statically.

Planned Workflow IR

The compiler will lower resolved Korlix semantics into a versioned, language-independent graph containing nodes such as:

workflow_start
workflow_end
page
layout
component
user_event
function_call
condition
loop
validation
state_read
state_write
api_request
storage_read
storage_write
navigation
timer
parallel_fork
parallel_join
error_handler
external_call
unresolved_operation

Planned edge kinds include:

control
user_event
condition_true
condition_false
success
failure
exception
data_dependency
state_dependency
async_spawn
async_join
navigation
retry
cancellation
timeout

This Workflow IR is intended to remain independent of Korlix AST types so that Phase 2 adapters can later target the same schema.

Stable Identity and Source Mapping

Workflow node IDs must be based on semantic ownership, not physical line numbers.

Expected identity inputs:

project namespace
+ module path
+ resolved symbol path
+ workflow root
+ semantic child path
+ operation kind
+ effect discriminator

Required behavior:

Comments do not change IDs.

Whitespace does not change IDs.

LF-to-CRLF conversion does not change IDs.

An unrelated declaration does not change existing workflow IDs.

A semantic route or operation change updates only the affected identity.

Every source-backed node links to the exact .klx source range.

Initial Validation Rules

The first implementation is planned to include stable rule IDs such as:

Rule

Meaning

WF001_DANGLING_EDGE

Graph edge references a missing node

WF002_DUPLICATE_NODE_ID

Two nodes share an identity

WF003_NO_ENTRY

Workflow has no normalized entry

WF004_NO_TERMINAL_PATH

Workflow cannot reach completion

WF005_UNSAFE_REDIRECT_CYCLE

Automatic route cycle has no valid exit

WF006_UNBOUNDED_RETRY

Retry has no bound, timeout, or cancellation

WF007_INVALID_SOURCE_SPAN

Source location is invalid

WF008_INSTRUMENTATION_MISMATCH

Instrumentation references an absent node

WF101_UNREACHABLE_NODE

Executable node cannot be reached

WF102_MISSING_FALSE_BRANCH

A semantic branch continuation is absent

WF103_API_WITHOUT_ERROR_PATH

Failure-capable request has no handler or propagation

WF104_LOADING_NOT_CLEARED

A terminal path leaves loading active

WF106_ERROR_NOT_RENDERED

Error state is written but not visibly consumed

WF107_DUPLICATE_SUBMISSION

Active mutation can be submitted again

WF108_ASYNC_NOT_JOINED

Async work is not joined, cancelled, or detached

WF110_DYNAMIC_TARGET_UNRESOLVED

API, route, or call target cannot be proven

WF112_EMPTY_SUCCESS_PATH

Success produces no visible, state, return, or navigation effect

These are practical structural checks. Phase 1 will not claim formal verification of arbitrary programs.

Development Runtime

Development builds will receive semantics-preserving instrumentation only when workflow inspection is enabled.

Expected runtime data:

Workflow ID

Static node ID

Trace ID

Span ID

Parent span ID

Start/completion/failure/cancellation/timeout status

Monotonic duration

Safe operation metadata

Runtime instrumentation must preserve:

Return values

Thrown values and error propagation

Side-effect count

Side-effect ordering

Cancellation behavior

Timeout behavior

The collector will bind to loopback by default. Raw passwords, tokens, authorization values, cookies, card data, request bodies, and response bodies will not be collected by default.

Proposed CLI

The commands below are planned and are not currently implemented:

korlix dev --workflow
korlix workflow scan
korlix workflow check
korlix workflow export --format json
korlix workflow export --format sarif
korlix workflow export --format mermaid
korlix workflow diff origin/main HEAD

Expected development output:

Application:         http://localhost:3000
Workflow Inspector: http://localhost:3001

Workflows: 14
Nodes:     138
Errors:    1
Warnings:  4

Planned Inspector Interface

The current UI direction is:

Workflow navigator on the left

Interactive graph canvas in the center

Source, evidence, and diagnostic panel on the right

Runtime trace timeline at the bottom

Filters for node kind, status, file, diagnostic rule, and trace

Collapsible groups for pages, components, and operations

Direct navigation from graph nodes to .klx source

The UI may use React, Vite, React Flow, Zustand, and ELK layered layout. These choices apply to the inspector tool and do not add React to generated Korlix applications.

Phase 1 Scope

Included:

Korlix-native static extraction

Normalized Workflow IR

Deterministic validation

Development instrumentation

Local runtime collector

Static/runtime inspector

Source navigation

JSON and SARIF export

Golden, property, fuzz, mutation, differential, and end-to-end tests

Excluded:

React and JavaScript adapters

Python adapters

Production observability

Cloud trace storage

Cross-service tracing

Automatic AI source modification

Capture of application payload values

Phase 2 Direction

Phase 2 is planned to support external ecosystems through adapters:

JavaScript/TypeScript parser
        + React/Next/Vite/framework adapter
        ↓
Universal Workflow IR

Python parser
        + FastAPI/Django/Celery adapter
        ↓
Universal Workflow IR

Language parsing alone will provide control flow. High-quality workflow understanding will require framework and library adapters.

Test Baseline

The critical end-to-end acceptance case is a checkout workflow containing:

Form validation

Early return

Loading state

API request

Success navigation

Failure rendering

finally cleanup

Duplicate-submit prevention

Runtime success and failure traces

Collector failure

Secret redaction

Stable IDs

Negative rule mutations

SARIF export

See Complete End-to-End Workflow Test.

Workflow Inspector Design Documents

Product, Problem, Goals, and Solution

Technical Implementation Specification

Test Properties and Acceptance Catalogue

Complete End-to-End Workflow Test

Architecture

flowchart LR
    A[.klx Source] --> B[Lexer]
    B --> C[Parser]
    C --> D[AST]
    D --> E[Resolver and Validation]
    E --> F[Component and Layout Lowering]
    F --> G[Style Scanner]
    G --> H[JIT CSS]
    F --> I[HTML Generator]
    F --> J[JavaScript Generator]
    H --> K[korlix.css]
    I --> L[Route HTML Files]
    J --> M[app.js]
    N[Korlix Runtime] --> O[korlix.runtime.js]
    K --> P[Browser]
    L --> P
    M --> P
    O --> P

Rust Workspace

Crate

Responsibility

korlix-cli

Command-line interface

korlix-core

Configuration, diagnostics, and source handling

korlix-lexer

Tokenization and indentation

korlix-parser

Tokens to AST

korlix-ast

Language syntax structures

korlix-resolver

Imports, files, routes, and symbols

korlix-style

Utility registry and JIT CSS

korlix-components

Component schemas and expansion

korlix-runtime-plan

Runtime feature analysis

korlix-codegen

HTML, CSS, JavaScript, and route generation

korlix-dev-server

Development server and hot updates

korlix-compiler

Whole-project compilation pipeline

Planned Workflow Inspector Crates

These crates are part of the Phase 1 design and are not yet listed as current workspace packages:

Planned crate

Responsibility

korlix-workflow-schema

Versioned Workflow IR, diagnostics, and runtime event schemas

korlix-workflow-lowering

Resolved Korlix semantics to control/effect graph

korlix-workflow-normalize

Entry/terminal normalization, canonical ordering, and hashing

korlix-workflow-validate

Graph invariants and workflow rules

korlix-workflow-instrument

Static-node to generated-code instrumentation planning

korlix-workflow-runtime

Event validation, redaction, deduplication, and trace assembly

korlix-workflow-sarif

Workflow diagnostics to SARIF conversion

korlix-workflow-test-fixtures

Fixture loading and semantic test assertions

CLI Reference

Command

Purpose

korlix new <name>

Create a Korlix project

korlix dev

Build and start the development server

korlix check

Parse, validate, and lint .klx files

korlix check --ast

Print the parsed AST as JSON

korlix build --mode static

Build a static multipage website

korlix build --mode spa

Build experimental SPA output

korlix preview --port 4173

Preview the production build

The CLI currently accepts --a11y, --security, and --seo, but their dedicated analysis passes are not complete.

Proposed Workflow Commands

These commands belong to the planned Workflow Inspector and are not currently available:

Command

Planned purpose

korlix dev --workflow

Start the application and local workflow inspector

korlix workflow scan

Extract the static workflow graph

korlix workflow check

Run deterministic workflow validation

korlix workflow export --format json

Export canonical Workflow IR

korlix workflow export --format sarif

Export CI-compatible findings

korlix workflow export --format mermaid

Export a documentation diagram

korlix workflow diff <base> <head>

Compare semantic workflow changes

Build from Source

Requirements:

Rust 1.75 or newer

Git

Node.js 18 or newer when rebuilding the TypeScript runtime

git clone https://github.com/SachinRamasamy-cloud/korlix.git
cd korlix

cargo build --release
cargo install --path crates/korlix-cli --force

Verify:

korlix --version

Run the workspace checks:

cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace

Project Structure

korlix/
├── crates/
│   ├── korlix-cli/
│   ├── korlix-core/
│   ├── korlix-lexer/
│   ├── korlix-parser/
│   ├── korlix-ast/
│   ├── korlix-resolver/
│   ├── korlix-style/
│   ├── korlix-components/
│   ├── korlix-runtime-plan/
│   ├── korlix-codegen/
│   ├── korlix-dev-server/
│   └── korlix-compiler/
├── runtime/
├── npm/
│   ├── korlix/
│   └── create-korlix/
├── examples/
│   ├── landing-page/
│   ├── spa-dashboard/
│   ├── v2-showcase/
│   └── complete-showcase/
├── docs/
├── tests/
├── Cargo.toml
├── Cargo.lock
├── CHANGELOG.md
├── LICENSE
└── README.md

Planned Workflow Inspector Additions

korlix/
├── crates/
│   ├── korlix-workflow-schema/
│   ├── korlix-workflow-lowering/
│   ├── korlix-workflow-normalize/
│   ├── korlix-workflow-validate/
│   ├── korlix-workflow-instrument/
│   ├── korlix-workflow-runtime/
│   ├── korlix-workflow-sarif/
│   └── korlix-workflow-test-fixtures/
├── packages/
│   ├── workflow-inspector-ui/
│   └── workflow-client-runtime/
├── apps/
│   └── workflow-inspector-server/
└── docs/
    └── workflow-inspector/

Generated Application Output

dist/
├── index.html
├── about/
│   └── index.html
├── products/
│   └── index.html
├── assets/
│   ├── korlix.css
│   ├── korlix.runtime.js
│   └── app.js
├── korlix.routes.json
└── korlix.manifest.json

Development and Testing

Format:

cargo fmt --all

Check:

cargo check --workspace

Test:

cargo test --workspace

Validate the browser runtime:

node --check crates/korlix-compiler/runtime-bundle/korlix.runtime.js

Build the complete showcase:

cd examples/complete-showcase

korlix check
korlix build --mode static

Start it:

korlix dev

Planned Workflow Inspector Verification

When the Phase 1 crates are implemented, their release gate will include:

unit tests
→ reviewed graph fixtures
→ semantic assertions
→ property-based tests
→ runtime differential tests
→ security/redaction fixtures
→ SARIF validation
→ bounded fuzzing
→ mutation testing
→ performance benchmarks
→ end-to-end checkout, authentication, and dashboard projects

The primary acceptance fixture is documented in:

docs/workflow-inspector/04-complete-e2e-test-case.md

Included Examples

Example

Purpose

examples/landing-page

Static marketing website with shared layouts

examples/spa-dashboard

Dashboard project using experimental SPA mode

examples/v2-showcase

Focused V2 language demonstration

examples/complete-showcase

Multipage catalogue of language and runtime features

Current Project Status

Korlix currently provides a tested frontend-language foundation.

Stable focus: static multipage compilation.

Experimental: SPA output and client-side page mounting.

Current limitations:

Partial record-shape and flow-sensitive type checking

Incomplete undefined identifier and function validation

Incomplete top-level let and derived runtime behavior

Incomplete component-instance state isolation

Limited named slots and event forwarding

Generic behavior for many registered components

Limited declarative API configuration

Experimental SPA lifecycle

No strict CSP-compatible event generation

Incomplete accessibility, security, and SEO analysis passes

No SSR or data-driven SSG

No package ecosystem yet

No complete formatter or language server

Limited JavaScript and TypeScript interoperability

No compiler-derived Workflow Inspector yet; Phase 1 remains planned

See Implementation Status for the detailed status matrix.

Roadmap

Workflow Inspector

Phase 1: Korlix-native Workflow IR and static extraction

Phase 1: Structural workflow validator with stable rule IDs

Phase 1: Development instrumentation and local runtime overlay

Phase 1: Source navigation, JSON export, and SARIF export

Phase 1: Property, fuzz, mutation, security, and end-to-end test gates

Phase 2: JavaScript and TypeScript adapter foundation

Phase 2: React, Vite, Next.js, router, state, and query-library adapters

Phase 2: Python, FastAPI, Django, SQLAlchemy, and task-worker adapters

Phase 2: Cross-language workflow correlation through trace context

Phase 2: Public adapter SDK

Language and Compiler

Expanded semantic analysis and type checking

Complete component behavior and accessibility contracts

Component-instance state and lifecycle

Declarative API headers, parameters, authentication, retries, and timeout

Full SPA route mounting

Formatter

Language Server Protocol support

Editor extensions

Source maps and improved diagnostics

Runtime feature tree-shaking

CSP-compatible event generation

Package and plugin ecosystem

Server-side rendering

Static site generation

Public component extension APIs

Documentation

Documentation Index

Getting Started

Project Structure

Language Syntax

Colors and Utilities

Components

State, Events, and Functions

Compiler Architecture

Korlix V2 Language

HTML Reference

Colors and Themes

V2 Component Catalogue

API and Pagination

Testing and Conformance

Implementation Status

Workflow Inspector Specifications

Product, Problem, Goals, and Solution

Technical Implementation Specification

Test Properties and Acceptance Catalogue

Complete End-to-End Workflow Test

Contributing

Contributions are welcome across:

Compiler and parser development

Semantic validation

Styling and color systems

Components

Browser runtime

Developer tooling

Tests

Documentation

Examples

Development setup:

git clone https://github.com/SachinRamasamy-cloud/korlix.git
cd korlix

cargo check --workspace
cargo test --workspace

Create a focused branch, add tests for language changes, and document compatibility effects in the pull request.

License

Korlix is released under the MIT License.

Author

Sachin RamasamyFull-Stack Developer

Portfolio: https://sachinrtech.vercel.app/

GitHub: https://github.com/SachinRamasamy-cloud

Repository: https://github.com/SachinRamasamy-cloud/korlix
