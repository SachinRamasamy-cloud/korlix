# KLX Language Syntax

## Core Concepts

Korlix uses indentation-based syntax, similar to Python. Each block is started with a colon `:` and indented.

## Import

```klx
import App from "./app.klx"
import MainLayout from "./layouts/main.klx"
```

## Pages

```klx
page index route "/":
  h1 "Hello World"
```

```klx
page about route "/about":
  meta:
    title "About Us"
    description "Learn about our company"

  section .py-20 .px-8:
    h1 .text-4xl .font-bold "About"
```

## Layouts

```klx
layout main:
  navbar:
    link href="/" "Home"
    link href="/about" "About"

  slot          # ← page content goes here

  footer:
    p "© 2024 My Site"
```

## Components

```klx
component feature-card:
  prop title: string
  prop description: string
  prop icon: string = "star"

  card .feature-card .p-6 .rounded-xl:
    icon name=icon .size-8 .text-primary .mb-4
    h3 .text-xl .font-semibold .mb-2 title
    p .text-muted description
```

## State & Events

```klx
page counter:
  state count: int = 0
  state open: bool = false

  btn .primary "Increment" on:click:
    count = count + 1

  btn .ghost "Toggle" on:click:
    open = !open

  p count
```

## Conditions

```klx
if count > 10:
  alert type="success" "You've clicked 10+ times!"
else:
  p .text-muted "Keep clicking..."
```

## Loops

```klx
for item in products:
  product-card product=item
```

## Data Fetching

```klx
page products:
  data products = get "/api/products":
    loading skeleton-card count=6
    error empty-state icon="warning" title="Failed to load"
    empty empty-state icon="box" title="No products"

  grid .grid-cols-3 .gap-6:
    for product in products:
      product-card product=product
```

## Built-in Elements

All standard HTML elements work directly:

```klx
div .container:
  h1 "Title"
  p "Paragraph"
  a href="/about" "Link"
  img src="/logo.png" alt="Logo"
  button "Click"
  input type="email" placeholder="Email"
```

## Events Available

```text
on:click        on:dblclick     on:mouseenter   on:mouseleave
on:input        on:change       on:focus        on:blur
on:submit       on:keydown      on:keyup        on:keypress
on:scroll       on:resize       on:mount        on:unmount
```
