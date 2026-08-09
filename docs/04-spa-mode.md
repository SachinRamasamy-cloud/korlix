# SPA Mode & Routing

Korlix includes a built-in client-side router for Single Page Applications (SPAs).

When SPA mode is enabled, Korlix generates a single `index.html` application shell. Navigation between registered routes is then handled in the browser without performing a full page reload.

---

## Enable SPA Mode

Enable SPA mode in `korlix.config.json`:

```json
{
  "mode": "spa"
}
```

You can also enable it directly from the CLI:

```bash
korlix build --mode spa
```

In SPA mode, the generated application uses:

```text
index.html
    │
    ▼
Korlix Runtime
    │
    ▼
Client-side Router
    │
    ├── /
    ├── /about
    ├── /blog
    ├── /blog/:slug
    ├── /products
    └── /products/:id
```

Route changes are resolved by the Korlix runtime without reloading the entire document.

---

## Define Routes

Application routes are declared in `src/app.klx`.

```klx
app:
  layout main

  routes:
    page "/"             from "./pages/index.klx"
    page "/about"        from "./pages/about.klx"
    page "/blog"         from "./pages/blog/index.klx"
    page "/blog/:slug"   from "./pages/blog/[slug].klx"
    page "/products"     from "./pages/products/index.klx"
    page "/products/:id" from "./pages/products/[id].klx"
```

Each route maps a URL path to a `.klx` page.

### Static routes

Static routes match an exact path.

```klx
page "/about" from "./pages/about.klx"
```

This resolves:

```text
/about
```

### Dynamic routes

Prefix a route segment with `:` to create a dynamic parameter.

```klx
page "/products/:id" from "./pages/products/[id].klx"
```

This route can match URLs such as:

```text
/products/42
/products/abc123
/products/keyboard
```

Similarly:

```klx
page "/blog/:slug" from "./pages/blog/[slug].klx"
```

can match:

```text
/blog/getting-started
/blog/korlix-routing
```

---

## Page and Route Mapping

A typical page structure can be organized as:

```text
src/
├── app.klx
└── pages/
    ├── index.klx
    ├── about.klx
    ├── blog/
    │   ├── index.klx
    │   └── [slug].klx
    └── products/
        ├── index.klx
        └── [id].klx
```

The corresponding routes are:

| Page file                      | Route           |
| ------------------------------ | --------------- |
| `src/pages/index.klx`          | `/`             |
| `src/pages/about.klx`          | `/about`        |
| `src/pages/blog/index.klx`     | `/blog`         |
| `src/pages/blog/[slug].klx`    | `/blog/:slug`   |
| `src/pages/products/index.klx` | `/products`     |
| `src/pages/products/[id].klx`  | `/products/:id` |

The route declaration in `src/app.klx` remains the authoritative mapping between a URL and its page.

---

## Navigation

Korlix supports both declarative and programmatic navigation.

### Declarative Navigation

Use `link` for normal application navigation:

```klx
link href="/about" "About"
link href="/products" "Products"
link href="/products/42" "View Product"
```

In SPA mode, navigation to registered application routes is handled by the client-side router.

For example:

```klx
link href="/products/42" "View Product"
```

navigates to:

```text
/products/42
```

which matches:

```klx
page "/products/:id" from "./pages/products/[id].klx"
```

---

## Programmatic Navigation

Use `navigate()` when navigation needs to happen as part of an interaction or application action.

```klx
btn "Go to About" on:click:
  navigate("/about")
```

You can navigate to dynamic routes in the same way:

```klx
btn "View Product" on:click:
  navigate("/products/42")
```

---

## Back Navigation

Use `goBack()` to return to the previous entry in the browser navigation history.

```klx
btn "Go Back" on:click:
  goBack()
```

This is useful for detail pages such as:

```text
/products/:id
/blog/:slug
```

where the user may need to return to the previous page.

---

## Route Manifest

During an SPA build, Korlix generates:

```text
dist/korlix.routes.json
```

The manifest describes the routes included in the compiled application.

Example:

```json
{
  "/": {
    "id": "index",
    "path": "/"
  },
  "/about": {
    "id": "about",
    "path": "/about"
  },
  "/products/:id": {
    "id": "products_id",
    "path": "/products/:id"
  }
}
```

The route manifest provides the compiled route map used by the generated application.

---

## SPA Build Output

A typical SPA build produces:

```text
dist/
├── index.html
├── korlix.routes.json
└── assets/
    ├── korlix.css
    ├── korlix.runtime.js
    └── app.js
```

### `index.html`

The single HTML application shell used to bootstrap the SPA.

### `korlix.routes.json`

The generated route manifest containing the application's registered routes.

### `assets/korlix.css`

Compiled application styles.

### `assets/korlix.runtime.js`

The Korlix browser runtime, including SPA routing behavior.

### `assets/app.js`

The compiled application code.

---

## Example

Consider the following application:

```klx
app:
  layout main

  routes:
    page "/"             from "./pages/index.klx"
    page "/about"        from "./pages/about.klx"
    page "/products"     from "./pages/products/index.klx"
    page "/products/:id" from "./pages/products/[id].klx"
```

A user can navigate from the product list:

```klx
link href="/products/42" "View Product"
```

The router resolves:

```text
/products/42
       │
       ▼
/products/:id
       │
       ▼
./pages/products/[id].klx
```

The page changes through the SPA router without requiring a full document reload.

---

## Summary

In SPA mode:

```text
Route declaration
      │
      ▼
src/app.klx
      │
      ▼
Korlix Compiler
      │
      ├── index.html
      ├── korlix.routes.json
      └── application assets
              │
              ▼
       Korlix SPA Router
              │
      ┌───────┼────────┐
      ▼       ▼        ▼
     /     /about   /products/:id
```

Use:

* `page` to register routes.
* `:parameter` for dynamic route segments.
* `link` for declarative navigation.
* `navigate()` for programmatic navigation.
* `goBack()` for browser-history navigation.
* `korlix.routes.json` as the generated route manifest.

SPA mode keeps navigation inside the running Korlix application and avoids full page reloads between registered routes.
