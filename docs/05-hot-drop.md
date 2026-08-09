# Hot Drop (Hot Reload)

Korlix includes a built-in Hot Drop system for development.

When running the development server, changes to `.klx` files are detected automatically and reflected in the browser with the smallest reload scope possible.

Where supported, Korlix preserves the current application state instead of performing a full page reload.

---

## Enable Hot Drop

Hot Drop is enabled automatically when running:

```bash
korlix dev
```

No additional configuration is required.

The development server:

```text
Watches source files
        │
        ▼
Detects changes
        │
        ▼
Recompiles affected output
        │
        ▼
Sends update through WebSocket
        │
        ▼
Browser applies the update
```

The browser connects to the development server through:

```text
ws://localhost:<port>/__kx_hmr
```

---

## How Hot Drop Works

Korlix watches the application's `src/` directory for source changes.

When a file changes, the development server determines the minimum update required and sends an appropriate message to the browser.

Korlix uses three update levels:

```text
Source change
     │
     ▼
Change classification
     │
     ├── CSS-only change
     │      └── Level 1: CSS Hot Drop
     │
     ├── Page change
     │      └── Level 2: Route Hot Drop
     │
     └── Application structure change
            └── Level 3: Full Reload
```

---

## Level 1 — CSS Hot Drop

CSS Hot Drop is used when a change only affects styling, such as class names or theme tokens.

Korlix:

1. Recompiles the affected CSS.
2. Notifies the browser through WebSocket.
3. Replaces the active stylesheet.
4. Keeps the current page running.

```text
Theme / class change
        │
        ▼
CSS recompilation
        │
        ▼
css-update
        │
        ▼
Stylesheet replaced
        │
        ▼
Application state preserved
```

### Result

* No full page reload.
* Current route remains active.
* Application state is preserved.
* Updated styles appear immediately.

---

## Level 2 — Route Hot Drop

Route Hot Drop is used when the currently active page or another route page changes.

For example:

```text
src/pages/about.klx
src/pages/products/[id].klx
src/pages/blog/index.klx
```

Korlix recompiles the affected route and refreshes the route in the browser.

```text
Page file changes
       │
       ▼
Affected route recompiles
       │
       ▼
Current route refreshes
       │
       ▼
Updated page rendered
```

### Result

* The entire browser document does not need to be reloaded.
* The affected route is refreshed.
* Global application state may remain available.
* Route-local state may be recreated depending on the update.

This provides a broader update than CSS Hot Drop while avoiding a complete application restart where possible.

---

## Level 3 — Full Reload

A full browser reload is required when the application's structure changes.

Examples include modifications to:

```text
app.klx
main.klx
route declarations
route tables
```

These files can affect application initialization or routing behavior, so a partial update is not sufficient.

```text
Application structure changes
           │
           ▼
Full recompilation
           │
           ▼
full-reload
           │
           ▼
Browser reloads
           │
           ▼
Application starts again
```

### Result

* The browser performs a full reload.
* The application is initialized again.
* Runtime state is reset.

---

## Hot Drop Levels

| Level   | Change type                  | Browser behavior        | State            |
| ------- | ---------------------------- | ----------------------- | ---------------- |
| Level 1 | CSS, classes, theme tokens   | Stylesheet updated      | Preserved        |
| Level 2 | Page / route file            | Current route refreshed | May be preserved |
| Level 3 | App structure or route table | Full browser reload     | Reset            |

Korlix automatically selects the appropriate update level based on the changed source.

---

## Compile Error Overlay

If compilation fails during development, Korlix displays an error overlay directly in the browser.

Example:

```text
⚠ Korlix Compile Error

[KX-E001] Expected `:`, found `div`

→ src/pages/index.klx:12:3

...
```

The overlay identifies:

* The Korlix error code.
* The compiler error message.
* The affected source file.
* The line and column where the error was detected.

This allows compile failures to be corrected without switching away from the running application.

---

## Error Recovery

A compile error does not require restarting the development server.

The development flow is:

```text
Source change
     │
     ▼
Compilation fails
     │
     ▼
error
     │
     ▼
Browser shows error overlay
     │
     ▼
Developer fixes source
     │
     ▼
Compilation succeeds
     │
     ▼
clear-error
     │
     ▼
Overlay removed
```

Once the source is valid again, Korlix automatically removes the error overlay and applies the successful update.

---

## WebSocket Connection

Hot Drop communication between the Korlix development server and browser uses WebSocket.

The client connects to:

```text
ws://localhost:<port>/__kx_hmr
```

The connection remains active while the development server is running.

Conceptually:

```text
Korlix Dev Server
      │
      │ WebSocket
      ▼
Browser Runtime
      │
      ├── Apply CSS update
      ├── Refresh route
      ├── Reload application
      ├── Show error
      └── Clear error
```

---

## WebSocket Protocol

The development server sends update messages to the browser.

### CSS Update

```json
{
  "type": "css-update"
}
```

The browser refreshes the generated stylesheet without performing a full page reload.

---

### Full Reload

```json
{
  "type": "full-reload"
}
```

The browser reloads the application.

---

### Compile Error

```json
{
  "type": "error",
  "error": "KX-E001: ..."
}
```

The browser displays the compiler error overlay.

---

### Clear Compile Error

```json
{
  "type": "clear-error"
}
```

The browser removes the active error overlay after compilation succeeds.

---

## Development Flow

A normal Hot Drop development cycle looks like:

```text
korlix dev
    │
    ▼
Dev server starts
    │
    ├── Watches src/
    │
    └── Opens Hot Drop WebSocket
                │
                ▼
         Browser connects
                │
                ▼
        Developer edits .klx
                │
                ▼
         Change detected
                │
                ▼
            Recompile
                │
         ┌──────┼────────┐
         ▼      ▼        ▼
       CSS     Route    App
       only    change   structure
         │       │        │
         ▼       ▼        ▼
     CSS Drop  Route    Full
               Drop     Reload
```

If compilation fails, the normal update is temporarily replaced by the compile error overlay until the error is fixed.

---

## Example

Suppose the application contains:

```text
src/
├── app.klx
├── main.klx
└── pages/
    ├── index.klx
    ├── about.klx
    └── products/
        └── [id].klx
```

### Editing a theme token

```text
theme token changed
       │
       ▼
Level 1
       │
       ▼
CSS Hot Drop
       │
       ▼
No page reload
```

### Editing `products/[id].klx`

```text
products/[id].klx changed
          │
          ▼
        Level 2
          │
          ▼
   Route Hot Drop
          │
          ▼
Current route refreshed
```

### Editing `app.klx`

```text
app.klx changed
       │
       ▼
     Level 3
       │
       ▼
   Full Reload
       │
       ▼
Runtime state reset
```

---

## Summary

Korlix Hot Drop provides three update strategies:

```text
                     Hot Drop
                        │
         ┌──────────────┼──────────────┐
         ▼              ▼              ▼
     Level 1         Level 2         Level 3
     CSS Drop       Route Drop      Full Reload
         │              │              │
    Style-only       Page change    App structure
      changes
         │              │              │
         ▼              ▼              ▼
   Preserve state   May preserve     Reset state
                       state
```

Use `korlix dev` to start development with Hot Drop enabled automatically.

Korlix then watches `src/`, recompiles changed sources, and uses the WebSocket connection at:

```text
/__kx_hmr
```

to apply browser updates, report compilation errors, and recover automatically after those errors are fixed.
