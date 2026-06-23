# Project Structure

Every Korlix project follows this structure:

```text
my-korlix-site/
├── korlix.config.json     # Project configuration
├── package.json           # NPM scripts
│
├── public/                # Static assets (copied to dist/)
│   ├── index.html         # HTML shell (modified by compiler)
│   └── assets/
│       ├── logo.png
│       └── hero.jpg
│
├── src/                   # All .klx source files
│   ├── main.klx           # Entry point — mounts the app
│   ├── app.klx            # App config — routes, layouts, providers
│   │
│   ├── pages/             # Route-mapped pages
│   │   ├── index.klx      # Maps to /
│   │   ├── about.klx      # Maps to /about
│   │   └── products/
│   │       ├── index.klx  # Maps to /products
│   │       └── [id].klx   # Maps to /products/:id
│   │
│   ├── layouts/           # Page layout wrappers
│   │   └── main.klx       # Main site layout
│   │
│   ├── components/        # Reusable components
│   │   ├── hero.klx
│   │   └── navbar.klx
│   │
│   └── theme/             # Design tokens
│       └── tokens.klx
│
└── dist/                  # Build output (generated)
    ├── index.html
    ├── about/index.html
    └── assets/
        ├── korlix.css
        ├── korlix.runtime.js
        └── app.js
```

## korlix.config.json

```json
{
  "name": "my-site",
  "version": "0.1.0",
  "src": "src",
  "public": "public",
  "dist": "dist",
  "mode": "spa",
  "theme": {
    "default": "dark",
    "dark": true
  },
  "server": {
    "port": 3000
  },
  "budget": {
    "runtime": "20kb",
    "css": "80kb",
    "page": "150kb"
  }
}
```

## React Mapping

| React | Korlix |
|-------|--------|
| `src/main.jsx` | `src/main.klx` |
| `src/App.jsx` | `src/app.klx` |
| `src/pages/` | `src/pages/` |
| `src/components/` | `src/components/` |
| `tailwind.config.js` | `src/theme/tokens.klx` |
| `styles.css` | Generated `korlix.css` |
