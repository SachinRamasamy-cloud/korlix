# Hot Drop (Hot Reload)

Korlix's hot drop system updates your browser instantly as you edit `.klx` files, without losing state when possible.

## How It Works

The dev server watches your `src/` directory and recompiles on change. It communicates with the browser via WebSocket.

### Level 1 — CSS Hot Drop

When only class names or theme tokens change:
- Recompiles only the CSS
- Swaps the `<link>` tag in place
- **No page reload** — state preserved

### Level 2 — Route Hot Drop

When a page file changes:
- Recompiles the changed route
- Reloads the current route
- Global state may be preserved

### Level 3 — Full Reload

When `app.klx`, `main.klx`, or route tables change:
- Full browser reload
- All state reset

## Error Overlay

When a compile error occurs, Korlix shows a red error overlay in the browser:

```
⚠ Korlix Compile Error
[KX-E001] Expected `:`, found `div`
  → src/pages/index.klx:12:3
  ...
```

Fix the error and the overlay disappears automatically.

## WebSocket Protocol

```json
// Server → Browser messages
{ "type": "css-update" }
{ "type": "full-reload" }
{ "type": "error", "error": "KX-E001: ..." }
{ "type": "clear-error" }
```

## Enabling Hot Drop

Hot drop is automatic in `korlix dev`. No configuration needed.

The client connects to `ws://localhost:<port>/__kx_hmr`.
