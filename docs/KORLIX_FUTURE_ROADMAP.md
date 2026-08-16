Korlix Engineering Roadmap

Status: Proposed authoritative roadmap
Updated: August 2026
Applies to: Korlix after the current 0.1.x foundation
Planning model: Quality-gated, dependency-ordered, not date-gated
Primary stable target today: Static multipage applications
Secondary target: SPA after lifecycle, state isolation, routing, and cancellation semantics are production-ready
Long-term target: A browser-native frontend language with a stable compiler, predictable runtime, first-class tooling, and a controlled extension ecosystem

1. Purpose

This roadmap defines the engineering sequence for moving Korlix from the current 0.1.x language/compiler foundation toward a stable 1.0.

Korlix must not progress by feature count alone. A feature is not complete merely because the lexer recognizes it, the parser accepts it, or generated output happens to work in one example.

A feature is complete only when all applicable layers are defined:

Language syntax
    ↓
Parser / AST
    ↓
Semantic rules
    ↓
Types / resolved symbols
    ↓
HIR / lowering
    ↓
Code generation
    ↓
Runtime behavior
    ↓
Diagnostics
    ↓
Tests
    ↓
Documentation
    ↓
Compatibility / migration contract

The core rule for the roadmap is:

Stabilize the language, semantics, compiler, and runtime contracts before expanding the ecosystem.

2. Current Baseline

The current 0.1.x foundation already provides substantial functionality.

2.1 Stable foundation

The following are treated as the current stable or stable-foundation capabilities:

Rust-based multi-crate compiler workspace

.klx indentation-based syntax

V1-compatible syntax plus simplified V2 forms

Optional trailing colons in supported V2 blocks

Application, page, layout, and component declarations

Static multipage compilation

Native HTML element recognition

Common SVG primitive recognition

User components

Typed props, defaults, and required-prop validation

Default slots

Reactive state

Local values

Derived values at the current supported level

Conditions and loops

Functions/actions

Assignment and compound assignment

Interpolation

Basic GET/POST/PUT/PATCH/DELETE API statements

Pagination foundation

JIT utility-class generation

Native Korlix color system

Semantic theme variables

Light, dark, and automatic themes

Registered component catalog

Development server

Preview server

Whole-project korlix check

Duplicate route/declaration checks

Unknown-component checks

Basic literal type compatibility

Compiler diagnostics and automated test foundation

2.2 Experimental capabilities

The following must remain explicitly marked experimental until their release gates pass:

SPA compilation

Client-side routing

Route mounting and unmounting

Broad generic component-catalog runtime behavior

Advanced responsive / interactive style combinations

Complex named-slot composition

Advanced component nesting

API authentication conventions

Request retries

Request caching

Advanced cancellation

Rich type inference

Generic runtime component behavior

2.3 Known incomplete areas

The current engineering gaps include:

Complete semantic resolution

Flow-sensitive type checking

Record-shape-aware type checking

Full function argument and return validation

Complete let / derived semantics across scopes

Component-instance state isolation

Lifecycle ownership and cleanup

Named-slot completeness

Event forwarding

Custom typed component events

CSP-safe handler execution in every path

Full JS/TS interoperability

Production-grade API cache/retry/auth/cancellation behavior

Full accessibility analyzer

Full security analyzer

Full SEO analyzer

Formatter

LSP

Source-map quality across generated JS

Package manager

Third-party component registry

SSR

Data-driven SSG

Specialized production behavior for every catalog component

3. Roadmap Governance

3.1 Feature maturity states

Every documented feature must use exactly one of these states:

State

Meaning

Stable

Covered by specification, compiler/runtime contracts, tests, and compatibility policy

Experimental

Usable but may change; not covered by stable compatibility guarantees

Generic

Name/schema exists but behavior is intentionally generic

Parsed only

Syntax is recognized but semantics/runtime are incomplete

Planned

Design exists or is scheduled but implementation is not complete

Deprecated

Supported temporarily for migration

Removed

No longer supported

Documentation must never call a registered component or parsed language feature “fully supported” unless it satisfies the Stable definition.

3.2 One source of truth for feature status

docs/18-implementation-status.md must remain the current feature-status matrix.

This roadmap defines where Korlix is going.

Implementation status defines what Korlix can reliably do now.

The two files must be updated together when a feature changes maturity.

3.3 Quality before schedule

No roadmap phase has a date commitment.

A release advances only when its gate passes.

4. Target Architecture

The long-term compiler pipeline should converge on:

.klx source
    ↓
Lexer
    ↓
Parser
    ↓
Syntax AST
    ↓
Module graph / imports
    ↓
Name resolution
    ↓
Semantic analysis
    ↓
Type checking
    ↓
Resolved HIR
    ↓
Feature lowering
    ↓
Component lowering
    ↓
Style plan
    ↓
Runtime plan
    ↓
Code generation
    ├── HTML
    ├── CSS
    ├── JavaScript
    ├── route manifest
    ├── build manifest
    └── source maps
    ↓
Optimizer
    ↓
dist/

Code generation should eventually consume only resolved/lowered structures, not unresolved syntax AST nodes.

5. Phase 0 — Specification, Documentation, and Conformance

Suggested version: 0.2.x

Objective

Create one trustworthy language baseline before adding substantial syntax or runtime behavior.

5.1 Freeze the Korlix V2 core specification

Define normative rules for:

indentation

whitespace

comments

identifiers

contextual keywords

literals

operators

precedence

interpolation

block ownership

declarations

state

local values

derived values

functions/actions

conditions

loops

components

props

slots

events

pages

routes

layouts

imports

API statements

styling

themes

native HTML

generated output semantics

5.2 Resolve existing syntax/documentation conflicts

Required corrections:

V1 colon-required wording vs V2 optional-colon behavior

btn / button naming and alias rules

on:click vs click=handler syntax

route declaration forms

app-level route configuration vs page-local routes

data/query syntax variants

action vs fn

top-level vs component-local let

derived evaluation rules

boolean prop parsing

default slots vs named slots

old “100+ complete components” claims

SPA maturity labeling

5.3 Versioned language specification

Introduce a language-version declaration strategy before major syntax divergence occurs.

Example direction:

language "2"

or project-level configuration:

{
  "languageVersion": "2"
}

The exact syntax is a design decision, but compiler behavior must eventually be versionable.

5.4 Conformance test suite

Every stable language feature should have:

valid source fixture

invalid source fixture

lexer snapshot

parser recovery test

AST snapshot

semantic diagnostic snapshot

HIR snapshot once HIR exists

HTML snapshot

CSS snapshot

JS snapshot

browser behavior test when runtime behavior exists

5.5 Documentation architecture cleanup

Split public documentation from engineering notes:

docs/
├── language/
├── styling/
├── components/
├── compiler/
├── tooling/
├── status/
└── internal/

Move historical implementation notes and research documents under docs/internal/.

Release gate

Every official example passes korlix check

Official examples produce zero unexpected warnings

V1/V2 compatibility rules are documented

Documentation no longer contradicts implementation status

Duplicate routes never silently overwrite output

Theme generation is deterministic

Generated CSS selectors are valid

CLI/package versions have one authoritative source

Linux and Windows checks pass

Language conformance fixtures exist for all Stable syntax

6. Phase 1 — Resolver, Semantic Analysis, Type System, and HIR

Suggested version: 0.3.x

Objective

Move Korlix from syntax acceptance to real program validation.

6.1 Module graph

Implement a canonical project module graph with:

canonical paths

imports

exports

aliases

circular-import detection

project-root boundaries

duplicate-module diagnostics

missing-import diagnostics

unused-import diagnostics

deterministic module ordering

6.2 Scope model

Define:

Application
  → Module
    → Page / Layout / Component
      → Function / Action
        → Block
          → Nested Block

Resolve:

state

props

locals

derived values

function parameters

loop variables

imports

component names

layouts

routes

standard-library symbols

Define shadowing explicitly.

6.3 Practical type system

Initial type system:

string
int
float
number
bool
null
list<T>
record
url
email
color
date
unknown
any

Required validation:

initializer compatibility

assignment compatibility

binary/unary operators

boolean conditions

list element types

function arguments

return types

prop expressions

nullable access

record member access

API result shapes where declared

event payload types

6.4 Record-shape typing

Support explicit records before advanced inference.

Example future syntax:

type User
  id: string
  name: string
  email: string

Then:

state user: User

Structural inference may follow later.

6.5 Function semantics

Complete:

parameters

default parameters

return values

return checking

recursion policy

purity metadata where useful

async function contract

Add only after semantic rules exist:

return
while
break
continue
try
catch
finally
async
await

6.6 Resolved HIR

Introduce an HIR independent from parser syntax.

HIR should contain:

resolved symbol IDs

resolved type IDs

canonical component references

normalized props

normalized slots

normalized event handlers

resolved routes

normalized style references

runtime feature requirements

source spans for diagnostics/source maps

6.7 Stable diagnostic codes

Diagnostic codes should become contractual.

Categories:

KX-L   lexer
KX-P   parser
KX-R   resolver
KX-T   types
KX-C   components
KX-S   style
KX-A   accessibility
KX-SEC security
KX-SEO SEO
KX-B   build
KX-RUN runtime

Release gate

Undefined identifiers fail compilation

Unknown functions fail compilation

Invalid member access fails compilation

Function argument and return types are checked

Component prop expressions are checked

Required props are checked after resolution

Import cycles produce actionable diagnostics

All semantic errors preserve source spans

Codegen receives resolved HIR

Stable examples do not rely on unresolved runtime name lookup

7. Phase 2 — Component Runtime, Lifecycle, and Accessibility Contracts

Suggested version: 0.4.x

Objective

Make components isolated, deterministic runtime units instead of shared page-level behavior.

7.1 Component instances

Every runtime component instance owns:

instance ID

props

local state

local derived values

local functions/actions

DOM root

event subscriptions

child component instances

timers

network requests

observers

cleanup handlers

7.2 Lifecycle

Define stable hooks:

mount
update
unmount

Potential user-level hooks can follow after internal lifecycle semantics are stable.

Unmount must:

remove listeners

cancel owned network requests

stop timers

disconnect observers

dispose subscriptions

dispose child instances

7.3 Complete slots

Support:

default slots

named slots

required slots

fallback slots

slot props

nested slots

diagnostic errors for invalid fills

7.4 Custom events

Support:

typed emitted events

payload validation

parent handlers

event forwarding

bubbling policy

cancellation policy

Potential syntax:

component Search
  event submit(query: string)

The syntax must not be stabilized until the semantics are defined.

7.5 Stable core component tier

Separate the catalog into tiers.

Tier A — Stable core

Prioritize:

button

link

input

textarea

select

checkbox

radio

switch

form-field

card

alert

toast

modal

drawer

dropdown

tooltip

tabs

accordion

navbar

sidebar

breadcrumb

pagination

table

data-table

progress

skeleton

spinner

empty-state

Each stable component requires:

dedicated schema

semantic HTML

documented props

documented slots

events

keyboard behavior

focus behavior

ARIA contract

light/dark coverage

browser tests

accessibility tests

Tier B — Generic catalog

Registered names may continue to exist as Generic until specialized behavior is implemented.

7.6 CSP-safe event generation

Eliminate runtime eval / new Function execution from stable output.

Generate:

handler IDs

handler tables

static closures/modules

deterministic registration

7.7 Focus and keyboard system

Add shared runtime primitives for:

focus trap

roving tabindex

Escape handling

arrow-key navigation

focus restoration

inert/background management

Release gate

Component instances have isolated state

Two instances of the same component never share state accidentally

Unmount cleans owned resources

Named slots work predictably

Typed custom events work

Stable core components pass keyboard tests

Stable core components pass accessibility tests

Stable output contains no eval or new Function

Modal/drawer focus restoration is correct

Runtime leak tests pass

8. Phase 3 — Data Layer, Forms, Auth Boundaries, and Network Runtime

Suggested version: 0.5.x

Objective

Support real CRUD/product workflows without requiring custom JavaScript for routine data behavior.

8.1 App-level API configuration

Proposed direction:

app MyApp
  api
    base=env.PUBLIC_API_URL
    timeout=10000

    headers
      Accept="application/json"

Only explicitly public environment variables may reach browser output.

8.2 Query model

Queries should expose:

data
loading
fetching
error
status
updatedAt
reload
cancel

Add:

retry

retry backoff

timeout

deduplication

cache

stale time

garbage collection

dependency-based refresh

page/component ownership

abort on unmount

8.3 Mutation model

Mutations should expose:

data
loading
error
status
reset
cancel

Add:

success handlers

error handlers

finalizers

optimistic updates

rollback

cache invalidation

mutation serialization policy

8.4 Typed API responses

Allow explicit response shapes.

Example future direction:

get users: list<User> "/api/users"

Compiler should validate access such as:

user.email

against the declared result type.

8.5 Authentication integration

Korlix should define browser-safe authentication integration without pretending to be an identity provider.

Support patterns for:

bearer tokens

cookie sessions

CSRF-protected mutations

token refresh hooks

unauthorized response hooks

Do not expose secrets or server-only credentials in generated browser output.

8.6 Forms

Implement:

form state

field registration

value binding

touched

dirty

validity

field errors

form errors

synchronous validation

asynchronous validation

submission state

reset

server-error mapping

8.7 Validation schema

Introduce a native validation model before adding many validation helper functions.

Potential concepts:

required
min
max
minLength
maxLength
email
url
pattern
custom

8.8 Pagination

Complete:

offset pagination

page-number pagination

cursor pagination

query integration

URL synchronization

browser back/forward restoration

loading preservation

accessible pagination semantics

8.9 Realtime transport

Experimental after normal HTTP semantics stabilize:

Server-Sent Events

WebSocket client

reconnect policy

subscription lifecycle

page/component ownership

8.10 Standard library

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
regex
number
object

Browser capabilities requiring permissions must have explicit contracts.

Release gate

A CRUD application can be built entirely in Korlix

Requests cancel when owning page/component unmounts

Query deduplication is tested

Retries cannot create mutation duplication by default

API errors are typed/recoverable

Forms expose valid/dirty/touched/error state

Async validation is cancellation-safe

Pagination restores from URL/history

No private environment values appear in generated assets

Cookie and bearer-token examples have security guidance

9. Phase 4 — Router, SPA, Navigation, and Route Lifecycle

Suggested version: 0.6.x

Objective

Promote SPA mode from Experimental to Supported while preserving Static as a first-class target.

9.1 Router

Complete:

static routes

dynamic parameters

nested routes

index routes

catch-all routes

404 routes

redirects

route metadata

navigation guards

active-link state

query-string API

hash API

9.2 Route lifecycle

On navigation:

guard
  ↓
cancel outgoing work
  ↓
unmount old page
  ↓
dispose old component tree
  ↓
load route module
  ↓
mount next page
  ↓
restore focus
  ↓
restore/reset scroll

9.3 Route state policy

Explicitly define:

page-local state

application state

preserved state

URL state

navigation state

State survival must never depend on accidental runtime object reuse.

9.4 Route-level code splitting

Generate:

shared runtime

shared vendor/interop code

per-route JS

per-route CSS when beneficial

preload metadata

9.5 Navigation UX

Add:

pending navigation state

route loading UI

route error boundary

focus announcement

scroll restoration

optional View Transitions API integration

9.6 Strengthen static mode

Static mode must support:

base paths

canonical URLs

asset base configuration

trailing-slash policy

clean URLs

static-host adapters

deterministic route output

9.7 Deployment adapters

Provide configuration/guides for browser/static hosting targets such as:

Cloudflare Pages

GitHub Pages

Netlify

Vercel static deployment

Nginx

S3/R2-compatible object hosting

Adapters should configure routing/output, not turn Korlix into a platform-specific runtime.

Release gate

Static mode has zero regressions

SPA navigation never leaks page state/listeners

Dynamic routes pass browser tests

Nested routes pass browser tests

404 behavior is deterministic

Back/forward restores URL and route state correctly

Pending navigation is accessible

Route-level chunks load correctly with base paths

Direct refresh of registered SPA routes has documented hosting behavior

10. Phase 5 — JavaScript Interop and Web Platform Escape Hatches

Suggested version: 0.6.x–0.7.x experimental first

Objective

Allow Korlix projects to use browser and npm ecosystems without turning KLX into disguised JavaScript.

10.1 JavaScript module interoperability

Define an explicit boundary for:

importing ESM modules

calling exported functions

using browser libraries

typed bindings

async return values

Avoid implicit global-variable access.

10.2 Generated binding layer

Interop should lower to generated modules rather than arbitrary string evaluation.

10.3 Browser API access

Provide safe native access or wrappers for high-value APIs:

fetch

URL / URLSearchParams

history

localStorage/sessionStorage

clipboard

IntersectionObserver

ResizeObserver

matchMedia

WebSocket

EventSource

10.4 Escape hatch

A raw JavaScript escape hatch, if provided, must be explicitly unsafe/advanced and excluded from some static guarantees.

10.5 Type declaration bridge

Long-term research:

consume selected .d.ts information

generate Korlix binding metadata

validate imported function signatures

Do not make this a 1.0 blocker unless interop becomes required by stable core workflows.

Release gate for supported interop

ESM imports are deterministic

Interop failures have source-located diagnostics

No arbitrary global lookup is required

Async calls are lifecycle-aware where applicable

Bundling behavior is documented

CSP guarantees remain explicit

11. Phase 6 — Developer Experience, Formatter, LSP, and Debugging

Suggested version: 0.7.x

Objective

Make Korlix practical for daily development rather than only compiler-driven experimentation.

11.1 Formatter

Add:

korlix fmt
korlix fmt --check

Requirements:

deterministic

idempotent

comment-preserving

stable indentation

V1/V2 migration awareness

no semantic changes

11.2 Language server

Support:

parse diagnostics

semantic diagnostics

autocomplete

HTML element suggestions

component suggestions

prop suggestions

event suggestions

utility-class suggestions

color previews

hover docs

type hover

go-to-definition

find references

rename

route navigation

import navigation

11.3 Editor integrations

Priority:

VS Code

Neovim via LSP

JetBrains via LSP where practical

11.4 Source maps

Map browser errors back to:

.klx file
line
column
page/component
function/action
generated module

11.5 Dev server

Improve:

incremental rebuilds

dependency-aware rebuilds

CSS-only Hot Drop

component/page scoped updates when safe

error overlay

automatic browser opening

port-conflict handling

resilient file watching

recovery to previous valid build

11.6 Inspection commands

Add:

korlix inspect
korlix doctor
korlix explain <diagnostic-code>

Potential inspect views:

AST
HIR
route graph
component graph
runtime features
used utilities
generated chunks
bundle sizes

11.7 Migration command

Future:

korlix migrate

Used for controlled syntax/schema migrations between language versions.

Release gate

Formatter is deterministic/idempotent

LSP resolves symbols across files

Rename respects scopes

Browser errors map back to KLX

Dev rebuild latency is measured

Error overlay keeps previous valid output active

doctor detects common configuration/version problems

Windows/Linux editor workflows pass integration checks

12. Phase 7 — Styling System 2.0, Design Tokens, and Motion

Suggested version: 0.7.x–0.8.x

Objective

Turn the existing JIT utility system into a stable design-system substrate without losing deterministic CSS generation.

12.1 Theme tokens

Support user-defined tokens:

colors
spacing
radius
shadow
font
breakpoint
motion
z-index

12.2 Semantic token contracts

Stable semantic groups:

background
surface
surface-elevated
foreground
muted
border
primary
secondary
accent
success
warning
danger
info
focus

12.3 Modern CSS

Add after compatibility tests:

container queries

CSS logical properties

clamp() helpers

aspect-ratio utilities

subgrid

modern viewport units

color-mix()

relative color syntax where supported

@supports

cascade layers where useful

12.4 Motion

Native primitives:

transition

enter/leave

reduced-motion behavior

View Transitions integration

spring-like presets only if implemented without excessive runtime

12.5 CSS correctness

The JIT engine must:

produce valid escaped selectors

preserve deterministic ordering

deduplicate rules

avoid invalid variant combinations

produce stable hashes when hashing is enabled

Release gate

Theme overrides work without editing compiler source

Generated CSS is deterministic

Container queries have browser tests

Reduced-motion behavior is respected

Variant parser handles arbitrary values reliably

CSS output passes parser validation

13. Phase 8 — Asset, Image, Metadata, SEO, and PWA Pipeline

Suggested version: 0.8.x

Objective

Support production web delivery requirements that sit outside language semantics.

13.1 Asset pipeline

Implement:

asset references

fingerprinting

copy rules

deterministic manifest

cache-control metadata guidance

13.2 Image pipeline

Optional compiler tooling:

width/height metadata

responsive srcset

lazy loading

modern format generation where tooling is available

aspect-ratio preservation

Avoid making heavyweight image processing mandatory for a basic build.

13.3 Metadata

Stable page metadata:

title

description

canonical

robots

Open Graph

Twitter cards

structured metadata hooks

13.4 SEO analyzer

korlix check --seo should perform real checks, including:

missing title

missing description

invalid canonical

heading hierarchy warnings

missing image alt

duplicate page metadata where detectable

crawlability configuration

13.5 PWA support

Optional:

web app manifest generation

service-worker generation

install metadata

offline asset policy

PWA must remain opt-in.

Release gate

Asset hashes are deterministic

Metadata output is valid

SEO analyzer uses documented rules

Image dimensions prevent avoidable layout shift in official examples

PWA mode is opt-in and removable without affecting core runtime

14. Phase 9 — Package and Extension Ecosystem

Suggested version: 0.8.x

Objective

Allow safe reuse outside a single repository without compromising compiler determinism.

14.1 Package format

Package metadata must describe:

name

version

compiler compatibility

language compatibility

exports

components

themes

utilities

runtime modules

required capabilities

14.2 Dependency resolver

Define:

semantic version rules

lock file

checksums

deterministic resolution

offline/cache behavior

conflict diagnostics

reproducible installs

14.3 Extension types

Initially permit declarative extensions:

components

themes

color palettes

utility packs

icon packs

Compiler/plugin execution should be a later and more restricted capability.

14.4 Security model

Require:

integrity verification

package checksums

path isolation

declared capabilities

compatibility checks

audit metadata

14.5 Official packages/templates

Maintain tested first-party packages or templates for:

landing page

documentation

dashboard

CRUD application

portfolio

storefront shell

admin application

Release gate

Package installs are reproducible

Lock files are deterministic

Package compatibility is validated before compilation

Package integrity is verified

Extensions cannot silently execute arbitrary compiler-side code

Official packages test across supported compiler versions

15. Phase 10 — Testing Framework and Browser Conformance

Suggested version: 0.8.x–0.9.x

Objective

Make browser behavior testable from Korlix projects and make the compiler itself conformance-driven.

15.1 Compiler conformance

Maintain:

lexer tests

parser tests

AST snapshots

resolver tests

semantic tests

type tests

HIR snapshots

codegen snapshots

CSS snapshots

diagnostic snapshots

15.2 Runtime conformance

Test:

state updates

derived updates

component isolation

lifecycle cleanup

forms

query cancellation

routing

focus management

keyboard behavior

Hot Drop recovery

15.3 Browser matrix

Define supported browser versions before 1.0.

At minimum evaluate:

Chromium

Firefox

WebKit/Safari

15.4 Korlix test command

Research an official command:

korlix test

Initial responsibility may be compiler/unit fixtures rather than inventing a complete browser-test framework.

15.5 Visual regression

Optional official tooling for core components and themes.

Release gate

Stable runtime behavior has browser tests

Supported browser policy is documented

Core components have visual + behavioral regression coverage

Cross-platform CI validates the compiler

Release builds run conformance suite before publication

16. Phase 11 — Production Hardening, Security, Accessibility, and Optimization

Suggested version: 0.9.x

Objective

Prepare release candidates for real production applications.

16.1 Compiler performance

Measure and optimize:

cold build time

incremental build time

parser throughput

semantic-analysis time

CSS generation time

codegen time

peak memory

Use profiling before introducing complex caching.

16.2 Incremental compilation

Implement dependency-aware invalidation:

file changed
   ↓
affected module graph
   ↓
affected semantic units
   ↓
affected routes/components/styles
   ↓
minimal rebuild

16.3 Output optimization

Implement:

JS minification

CSS minification

HTML minification where safe

content hashing

dead-code elimination

runtime feature tree-shaking

route chunking

critical CSS research

asset preload hints

bundle budgets

16.4 Security analyzer

korlix check --security should check:

raw HTML

unsafe URL schemes

DOM injection sinks

environment-secret leakage

import traversal

public asset traversal

insecure target-blank links

unsafe inline scripts where generated

CSP compatibility

16.5 Accessibility analyzer

korlix check --a11y should include:

required alt text

label/control association

interactive-role validation

tabindex misuse

heading-order warnings

ARIA attribute validity

duplicate IDs where detectable

inaccessible click-only controls

modal/dialog contracts

16.6 CSP

Stable default builds should target strict CSP compatibility.

Document any feature that weakens that guarantee.

16.7 Reproducibility

Release build should be deterministic for identical:

source
config
compiler version
lock file
platform-independent inputs

16.8 Cross-platform binaries

Publish tested artifacts for:

Windows x64

Linux x64

macOS arm64

macOS x64 where supported

with checksums.

Release gate

Security analyzer has real rule coverage

Accessibility analyzer has real rule coverage

No dynamic handler evaluation in stable output

Release output is reproducible

Bundle budgets are enforceable

Cross-platform CI passes

Core components have no critical accessibility defects

Compiler/runtime versions are synchronized

Benchmark regressions have thresholds

17. Phase 12 — SSG, SSR Research, and Deployment Evolution

Target: Post-foundation / potentially 1.x unless readiness is proven earlier

Objective

Expand Korlix beyond client/static compilation only after language/runtime semantics are stable.

17.1 Static-site generation

SSG requires definitions for:

build-time data

dynamic route enumeration

pagination

cache/revalidation policy

environment separation

error handling

deterministic builds

17.2 SSR

SSR requires much stronger contracts:

server runtime

request scope

server/client module boundaries

hydration

serialization

security

deployment target

streaming

errors

caching

Do not add SSR merely to match other frameworks.

17.3 Islands / partial hydration research

Because Korlix compiles browser-native output, research whether isolated interactive islands can reduce runtime cost without creating a second component model.

17.4 Server actions

Not a pre-1.0 requirement.

Only research after server/client boundaries are formalized.

Release gate for any stable server rendering

Server/client boundaries are explicit

Secrets never cross to client output

Hydration is deterministic

Route errors are recoverable

Deployment model is documented

Browser-native static output remains available

18. Internationalization and Localization

Target: 0.8.x+ or 1.x, depending on demand

Objective

Provide a compiler-aware internationalization model without embedding one vendor.

Potential capabilities:

message catalogs

locale detection

locale routing

number formatting

date formatting

pluralization

RTL metadata

missing-translation diagnostics

compile-time key validation

Do not invent a custom translation file format until requirements are validated.

19. Korlix 1.0 Release Criteria

Korlix reaches 1.0 only when the compatibility contract is credible.

19.1 Language

versioned specification

stable grammar

stable indentation rules

stable scope rules

stable type rules

stable component declaration rules

stable page/layout/route rules

documented deprecation policy

19.2 Compiler

canonical module graph

semantic resolver

practical type checker

resolved HIR

deterministic lowering

reliable diagnostics

source maps

deterministic builds

cross-platform binaries

19.3 Runtime

component instance isolation

page lifecycle isolation

CSP-safe events

resource cleanup

stable state behavior

stable query behavior

stable form behavior

stable focus behavior

19.4 Build targets

production-ready static multipage

supported SPA

deterministic asset output

base-path support

hashed production assets

optimization pipeline

documented deployment patterns

SSR is not required for 1.0.

19.5 Components

clearly defined Stable core component tier

accessibility contracts

keyboard contracts

light/dark coverage

browser tests

no claim that Generic catalog components are specialized

19.6 Styling

deterministic JIT generation

stable semantic tokens

user theme overrides

documented variant rules

arbitrary values

responsive behavior

modern CSS support with compatibility policy

19.7 Data/forms

lifecycle-aware HTTP

cancellation

retry rules

cache rules

forms

validation

pagination

public/private environment contract

19.8 Tooling

formatter

LSP

VS Code integration

source maps

doctor/inspect tools

migration support for breaking syntax changes

19.9 Quality

conformance suite

browser matrix

accessibility suite

security suite

performance benchmarks

reproducible releases

19.10 Compatibility

SemVer policy

language-version policy

migration guides

supported-browser policy

compiler/package compatibility matrix

20. Suggested Version Mapping

Version

Main engineering focus

0.2.x

Specification, documentation cleanup, conformance

0.3.x

Resolver, semantic analysis, type system, HIR

0.4.x

Component isolation, lifecycle, slots, events, CSP

0.5.x

Data layer, forms, network runtime, pagination

0.6.x

SPA lifecycle, router, route splitting

0.7.x

JS interop, formatter, LSP, debugging, styling evolution

0.8.x

Assets/SEO/PWA, packages, testing ecosystem

0.9.x

Security, accessibility, optimization, production hardening

1.0.0

Stable language/compiler/runtime compatibility contract

1.x

SSG/SSR research maturation, advanced ecosystem features

This mapping is directional, not a date promise.

21. Continuous Workstreams

These do not wait for a single phase.

21.1 Documentation

Maintain:

language specification

tutorials

component catalog

compiler architecture

runtime architecture

implementation status

roadmap

migration guides

error-code reference

21.2 Testing

Maintain:

unit tests

parser recovery tests

semantic tests

snapshot tests

browser tests

accessibility tests

security tests

cross-platform tests

21.3 Release engineering

Maintain:

changelog

compiler version

npm wrapper version

checksums

binary publishing

lock-file compatibility

release notes

migration notes

21.4 Performance

Track benchmark history for:

compiler startup

cold build

incremental build

generated CSS size

generated JS/runtime size

page startup

state update

route navigation

21.5 Community and governance

Introduce when external contribution volume justifies it:

contribution guide

issue templates

RFC process

Code of Conduct

security reporting

maintainer roles

compatibility policy

22. Priority Order

P0 — Correctness before expansion

specification consistency

parser correctness

route correctness

import correctness

theme correctness

version synchronization

official examples

conformance tests

deterministic output

P1 — Language credibility

resolver

semantic analysis

type checking

HIR

component isolation

lifecycle cleanup

CSP-safe handlers

diagnostics

P2 — Real application development

stable data/query layer

forms

validation

pagination

router lifecycle

stable SPA

core accessible components

P3 — Daily developer usability

formatter

LSP

source maps

dev-server performance

inspection tools

JS interop

P4 — Ecosystem

package format

lock file

themes

utility packs

component packages

templates

P5 — Advanced platform capabilities

PWA

i18n

SSG

SSR

islands/partial hydration

server-side capabilities

23. Features That Should NOT Be Prioritized Yet

To protect the language foundation, do not prioritize these ahead of semantic/runtime correctness:

large numbers of new component names

arbitrary compiler plugins

SSR

server actions

custom backend framework

visual page builder

proprietary package registry

complex animation DSL

AI-generated application syntax baked into the language

native mobile target

desktop target

These can be revisited after 1.0 contracts are stable.

24. Definition of Done

A feature is complete only when every applicable requirement is satisfied:

syntax/API is specified

lexer/parser support exists

AST support exists

resolver support exists

HIR/lowering support exists

type rules exist

semantic diagnostics exist

codegen exists

runtime support exists

lifecycle behavior exists

security implications are documented

accessibility implications are documented

unit tests exist

integration tests exist

browser tests exist when relevant

documentation exists

implementation status is updated

official example exists when useful

migration impact is documented

stable behavior does not depend on undocumented implementation details

25. Immediate Execution Plan

The next implementation sequence should be:

1. Documentation/spec reconciliation
2. Conformance fixtures
3. Resolver/module graph audit
4. Semantic symbol model
5. Practical type checker
6. Resolved HIR
7. Codegen migration to HIR
8. Component-instance isolation
9. CSP-safe event handlers
10. Lifecycle ownership/cancellation
11. Stable core component tier
12. Data/query layer
13. Forms/validation
14. SPA lifecycle
15. Formatter/LSP
16. Production hardening

Immediate milestone A — Documentation truth

Before new language syntax:

make 18-implementation-status.md authoritative

mark SPA experimental everywhere

distinguish Stable vs Generic components

reconcile V1 vs V2 syntax docs

remove duplicate/outdated claims

move internal research notes out of public-doc navigation

Immediate milestone B — Compiler truth

Audit every currently documented feature against:

lexer
parser
AST
resolver
semantic checks
codegen
runtime
tests

Produce a feature matrix before promoting any capability to Stable.

Immediate milestone C — 0.2 release gate

Do not call 0.2 complete until:

documentation and code agree

official examples are clean

conformance fixtures are in CI

duplicate routes/import errors are deterministic

theme generation is deterministic

cross-platform compiler checks pass

26. Long-Term Product Position

Korlix should remain:

A frontend-first, browser-native language that compiles concise .klx source into predictable HTML, CSS, and JavaScript.

It should not become a clone of React, Next.js, Tailwind, or a backend framework.

The long-term differentiators should be:

concise language syntax

compile-time validation

browser-native output

integrated styling and themes

integrated accessible components

small feature-driven runtime

deterministic production builds

first-class diagnostics and tooling

explicit lifecycle and data semantics

controlled interoperability instead of framework lock-in

That direction keeps Korlix technically coherent while still leaving room for SPA, packages, SSG, SSR, interop, and broader tooling after the language foundation is trustworthy.
