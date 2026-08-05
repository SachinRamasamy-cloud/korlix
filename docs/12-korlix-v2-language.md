# Korlix V2 Language

Korlix V2 is the simplified, indentation-based syntax accepted alongside V1 syntax. A trailing colon is optional for pages, layouts, components, functions, control-flow blocks and element child blocks.

## Minimal page

```klx
page Home at "/"
  h1 "Hello Korlix"
```

The V1 form remains valid:

```klx
page Home route "/":
  h1 "Hello Korlix"
```

## State, interpolation and functions

```klx
page Counter at "/counter"
  state count: int = 0

  h1 "Count: {count}"
  button "Increase" click=increment

  fn increment
    count += 1
```

Supported declarations and statements include `state`, `let`, `derived`, `fn`/`action`, `if`, `else`, `for`, assignment, `+=`, `-=`, function calls and API statements.

## Components and props

```klx
component user-card
  prop name: string
  prop role: string = "Member"

  card variant=raised
    h2 name
    p role

page Users at "/users"
  user-card name="Sachin" role="Admin"
```

Props without defaults are required. Missing required props are compilation errors.

## Layouts

```klx
app MyApp
  layout MainLayout
  theme auto

layout MainLayout
  navbar
    strong "My App"
    theme-toggle
  main
    slot
  footer "Copyright"
```

A page-specific layout overrides the application layout. Otherwise, the application layout is applied automatically.

## Current type checking

Korlix validates literal initializers for typed state, local variables and prop defaults. It also validates duplicate symbols, duplicate routes, unknown components and required user-component props. Full control-flow inference and record-shape typing remain future compiler work.
