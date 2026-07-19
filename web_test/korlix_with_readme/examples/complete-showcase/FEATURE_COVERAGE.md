# Complete Showcase Feature Coverage

## Project and module system

- `.klx` source files
- `import Name from "path"`
- side-effect import support in the language
- `mount App to "#korlix-root"`
- application declaration
- default application layout
- route declarations
- light/dark/auto application theme
- provider declarations
- static multipage configuration
- public asset copying

## Declarations

- page
- layout
- component
- prop
- state
- let
- derived
- fn
- action
- meta
- slot
- GET query

## Values and expressions

- string
- integer
- floating-point number
- Boolean
- null
- list
- record
- identifier
- member access
- index access
- function call
- interpolation
- arithmetic operators
- comparison operators
- logical operators
- unary not
- normal assignment
- `+=`
- `-=`

## Control flow

- `if`
- `else`
- `for item in list`
- condition and loop rendering in page content
- condition and loop generation inside functions

## Components

- required typed props
- optional props with defaults
- default slots
- imported component aliases
- specialized component schemas
- generic schema-driven components
- navigation catalogue
- content catalogue
- forms catalogue
- feedback catalogue
- overlays catalogue
- data display catalogue
- media catalogue
- layout catalogue

## Native elements

The project includes examples from every main native category:

- semantic document structure
- headings and text content
- inline semantics
- lists
- disclosure
- forms
- tables
- measurements
- images and picture sources
- dialog
- SVG gradients and shapes

The compiler registry recognizes 137 HTML and SVG names even when every individual name is not rendered on one page.

## Styling

- more than 1,000 utility classes
- 26 public color families
- levels 0 through 12
- traditional `bg-`, `text-`, `border-`, `ring-`, `fill-` and `stroke-` prefixes
- Korlix `surface-`, `content-`, `outline-`, `accent-`, `ring-color-` and `caret-color-` prefixes
- semantic theme tokens
- spacing
- flexbox
- grid
- sizing
- typography
- borders
- radius
- shadows
- opacity
- overflow
- responsive variants
- interaction variants
- dark variants
- arbitrary values

## Events

- click
- double-click
- input
- change
- focus
- blur
- keydown
- keyup
- mouseenter
- mouseleave
- scroll
- load
- error
- drag
- drop
- touch-start
- touch-end
- V1 `on:event:` block compatibility

## Runtime functions

- toast
- showToast
- openModal
- closeModal
- openDrawer
- closeDrawer
- navigate
- goBack
- toggleTheme
- scrollTo
- copyToClipboard
- log

## API

- named GET query
- query data value
- query loading value
- query error value
- POST
- PUT
- PATCH
- DELETE
- query reload
- direct `api.get` call
- JSON and text response handling
- local mock API with CORS

## Pagination

- current page
- total records
- page size
- siblings
- first button
- previous button
- numbered pages
- ellipsis
- next button
- last button
- disabled boundaries
- `aria-current`
- URL synchronization
- `change` event
- `kx:page-change` runtime event
- state update through `event.detail.page`

## Themes

- light
- dark
- automatic system mode
- semantic tokens
- explicit dark variants
- theme toggle component
- toggleTheme runtime call
- local preference
- system-theme listener
- `kx:theme-change`

## Features intentionally not claimed as complete

- full static type inference
- undefined identifier validation
- component lifecycle
- isolated component state
- named slot invocation
- complete SPA page mounting
- SSR or SSG
- package manager
- LSP and formatter
- advanced declarative API options
- strict CSP output
