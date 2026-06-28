# Korlix Programming Language: Comprehensive Developer Manual

Welcome to the official **Korlix Developer Manual**. Korlix is an ultra-light, compiler-first frontend language designed to build modern, interactive web applications with zero boilerplate. 

By unifying HTML structure, JIT utility styling, interactive components, reactive state management, and a built-in API client under a single compiler, Korlix replaces frameworks like React, styling systems like Tailwind CSS, client routers, and heavy UI component libraries.

---

## 1. Project Scaffolding & CLI Workflows

Every Korlix project is managed using standard Node/NPM workflows. The compiler binary is distributed through NPM and runs natively on Windows and Linux systems.

### 1.1 Creating a New App
To bootstrap a new Korlix application, run:
```bash
npm create korlix@latest my-app
```
Or simply:
```bash
npm create korlix@latest
```
If you do not specify a name, the installer will prompt you:
```text
Project name (my-korlix-app):
```

The installer will scaffold the project structure, set up Single Page Application (SPA) configuration, install the required NPM dependencies, and boot up the local HMR dev server automatically.

### 1.2 Generated Project Directory Layout
A newly initialized project contains the following file structure:

```text
my-app/
├── korlix.config.json     # Compiler and dev server configuration
├── package.json           # NPM dependencies and scripts
│
├── public/                # Static assets (copied directly to dist/)
│   └── index.html         # HTML shell modified by the compiler
│
├── src/                   # Source files
│   ├── main.klx           # App entry point (mounts the root component)
│   ├── app.klx            # Route declarations and layout mappings
│   │
│   ├── pages/             # Route-mapped page files
│   │   ├── index.klx      # Maps to "/"
│   │   ├── about.klx      # Maps to "/about"
│   │   └── products/
│   │       ├── index.klx  # Maps to "/products"
│   │       └── [id].klx   # Dynamic route mapping to "/products/:id"
│   │
│   ├── layouts/           # Layout wrappers
│   │   └── main.klx       # Main layout template containing navigation
│   │
│   └── theme/             # Design token declarations
│       └── tokens.klx     # Custom theme styles & overrides
│
└── dist/                  # Compiler output directory (generated)
    ├── index.html         # SPA Shell index
    ├── korlix.routes.json # Map of client-side routes
    └── assets/
        ├── korlix.css     # Compiled JIT CSS
        ├── app.js         # Compiled page states & logic
        └── korlix.runtime.js # Lightweight client runtime library
```

### 1.3 Configuration: `korlix.config.json`
The project behavior is defined via `korlix.config.json`:
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

### 1.4 NPM Build & Execution Scripts
Instead of interacting with Rust packages or source code, developers run the compiler via standard NPM scripts:

```bash
# Start the local Axum dev server with HMR Hot Drop enabled
npm run dev

# Run a static/SPA compilation optimized for production
npm run build

# Preview the compiled assets in the dist/ directory locally
npm run preview

# Perform static lint checking for diagnostics, security, a11y, and SEO
npm run check
```

---

## 2. Core Language Syntax

Korlix uses an indentation-based layout syntax (similar to Python or Pug). Braces `{}` are reserved for records/objects and brackets `[]` for lists/indexing. Blocks of code or template hierarchies must start with a colon `:` and be consistently indented (normally by 2 spaces).

### 2.1 Imports
Import components, pages, or layouts using the `import` keyword:
```klx
import App from "./app.klx"
import MainLayout from "./layouts/main.klx"
```

### 2.2 Top-Level Page Structure
A page represents a routing target and is declared using the `page` keyword:
```klx
page profile route "/profile":
  meta:
    title "Profile page"
    description "User settings and profile administration"

  section .p-6:
    h1 "My Profile"
```

### 2.3 Layouts & Slots
Layouts define visual wrappers around pages. Pages are injected into the layout at the `slot` position:
```klx
layout main:
  navbar:
    link href="/" "Home"
    link href="/profile" "Profile"
  
  slot # <-- Page content is injected here

  footer:
    p "Copyright 2026 Korlix"
```

### 2.4 Reusable Custom Components
Define reusable custom components with structural parameters using the `component` keyword:
```klx
component info-box:
  prop title: string
  prop message: string
  prop status: string = "info"

  div .p-4 .border .rounded-lg:
    h4 .font-semibold title
    p .text-muted message
    badge variant=status "Status"
```

### 2.5 Indexing: Dot-Notation and Brackets
Access record properties or array items using standard access methods:
```klx
h3 user.name
p user.email
span user["role"]
p items[0]
```

---

## 3. Data Types Reference

Korlix is a strongly typed language at the AST level, ensuring parameters and reactive bindings are correctly formatted.

| Data Type | Description | Syntax Example |
|-----------|-------------|----------------|
| `string` | UTF-8 text strings | `"hello"`, `'world'` |
| `int` | Signed integers | `42`, `-10` |
| `float` | Floating-point numbers | `3.14159` |
| `number` | Unified float/integer alias | `123`, `5.6` |
| `bool` | Boolean toggle state | `true`, `false` |
| `null` | Null reference value | `null` |
| `list<T>` | Indented array list of type T | `[1, 2, 3]`, `["a", "b"]` |
| `record` | Key-value mapping record | `{ name: "Sachin", role: "Developer" }` |
| `json` | Raw untyped JSON payload | `{"active": true, "id": 42}` |
| `date` | Calendar date | `2026-06-28` |
| `time` | Clock time | `13:20:00` |
| `url` | Web URL structure | `"https://localhost:4000/users"` |
| `email` | Validated email address | `"sachin@test.com"` |
| `color` | CSS color format | `"#ffffff"`, `"rgb(0,0,0)"` |
| `image` | Image asset path | `"/assets/logo.png"` |
| `icon` | Icon name token | `"settings"`, `"star"` |
| `component` | Compiled component reference | `info-box` |
| `slot` | HTML element insertion hook | `slot` |
| `event` | DOM event payload | `event` |
| `any` | Disables strict type checks | `any` |

---

## 4. Style & Utility Color System (JIT CSS)

Korlix scans your templates and dynamically generates optimized CSS classes. It comes pre-packaged with a massive utility registry including **1,800+ color utilities** and spacing systems.

### 4.1 Color Palette Shading System (50–950)
Every color palette is mapped from 50 (lightest shade) to 950 (darkest shade).

**Available Palettes:**
- `slate`, `gray`, `zinc`, `neutral`, `stone`
- `red`, `orange`, `amber`, `yellow`
- `lime`, `green`, `emerald`, `teal`, `cyan`, `blue`
- `indigo`, `violet`, `purple`, `fuchsia`, `pink`, `rose`

**Semantic Colors:**
- Brand: `primary` (`#6366f1`), `primary-light` (`#818cf8`), `primary-dark` (`#4f46e5`)
- Brand 2: `secondary` (`#ec4899`), `secondary-light` (`#f472b6`), `secondary-dark` (`#db2777`)
- UI Roles: `accent`, `success`, `danger`, `warning`, `info`, `muted`
- Layout: `surface`, `background`, `foreground`, `border`, `dark`, `light`
- Inlines: `white`, `black`, `transparent`

### 4.2 Color Shorthand Utility Prefixes
For every color and shade combination, the style engine generates these prefixes:
- `.text-{color}`: Color text content.
- `.bg-{color}`: Set element background color.
- `.border-{color}`: Set border border-color.
- `.ring-{color}`: Set ring shadow color.
- `.fill-{color}` / `.stroke-{color}`: Set SVG shape properties.
- `.outline-{color}`: Set CSS focus outline color.
- `.caret-{color}`: Color text caret pointer.
- `.placeholder-{color}`: Color placeholder text.

### 4.3 Design Token Conversions (Spacing, Fonts, Borders)

#### Spacing Scales (`SPACING` token)
Used for padding (`p`), margin (`m`), grid gaps (`gap`), and child spacing (`space-x`/`space-y`):
- `0` (0px), `px` (1px), `0.5` (2px), `1` (4px), `1.5` (6px), `2` (8px), `2.5` (10px), `3` (12px), `3.5` (14px), `4` (16px), `5` (20px), `6` (24px), `7` (28px), `8` (32px), `9` (36px), `10` (40px), `11` (44px), `12` (48px), `14` (56px), `16` (64px), `20` (80px), `24` (96px), `28` (112px), `32` (128px), `36` (144px), `40` (160px), `44` (176px), `48` (192px), `52` (208px), `56` (224px), `60` (240px), `64` (256px), `72` (288px), `80` (320px), `96` (384px).

#### Rounded Border Radius (`RADIUS` token)
- `rounded-none` (0px), `rounded-sm` (2px), `rounded` (4px), `rounded-md` (6px), `rounded-lg` (8px), `rounded-xl` (12px), `rounded-2xl` (16px), `rounded-3xl` (24px), `rounded-full` (9999px).

#### Typography Sizes (`FONT_SIZES` token)
- `text-xs` (0.75rem / 1rem line-height), `text-sm` (0.875rem / 1.25rem), `text-base` (1rem / 1.5rem), `text-lg` (1.125rem / 1.75rem), `text-xl` (1.25rem / 1.75rem), `text-2xl` (1.5rem / 2rem), `text-3xl` (1.875rem / 2.25rem), `text-4xl` (2.25rem / 2.5rem), `text-5xl` (3rem), `text-6xl` (3.75rem), `text-7xl` (4.5rem), `text-8xl` (6rem), `text-9xl` (8rem).

#### Typography Font Weights (`FONT_WEIGHTS` token)
- `font-thin` (100), `font-extralight` (200), `font-light` (300), `font-normal` (400), `font-medium` (500), `font-semibold` (600), `font-bold` (700), `font-extrabold` (800), `font-black` (900).

#### Box Shadows (`SHADOWS` token)
- `shadow-sm`, `shadow` (default), `shadow-md`, `shadow-lg`, `shadow-xl`, `shadow-2xl`, `shadow-inner`, `shadow-none`.

#### Opacities (`OPACITY` token)
- `opacity-0` (0.0) up to `opacity-100` (1.0) in increments of 10 plus common steps: `5`, `25`, `75`, `95`.

#### Z-Index (`Z_INDEX` token)
- `z-0`, `z-10`, `z-20`, `z-30`, `z-40`, `z-50`, `z-auto`.

#### Insets (top, right, bottom, left)
- Works with spacing scales: e.g., `top-4`, `left-1/2`, `right-0`, `inset-y-0`.

#### Transitions & Durations
- `transition-none`, `transition-all`, `transition` (default), `transition-colors`, `transition-opacity`, `transition-shadow`, `transition-transform`.
- `duration-0`, `duration-75` (75ms), `duration-100` (100ms), `duration-150` (150ms), `duration-200` (200ms), `duration-300` (300ms), `duration-500` (500ms), `duration-700` (700ms), `duration-1000` (1s).

### 4.4 Style Variants & Breakpoints
- **Breakpoints**: `sm:` (576px), `md:` (768px), `lg:` (992px), `xl:` (1200px), `2xl:` (1400px).
- **Interactions**: `hover:`, `focus:`, `active:`, `disabled:`, `checked:`.
- **Mode switcher**: `dark:`, `light:` (e.g. `dark:bg-slate-950`).
- **Nesting**: `group-hover:`, `peer-checked:`, `data-open:`, `motion-safe:`, `print:`.

---

## 5. Built-in Component Registry Specification

Korlix ships with a registry containing **34 built-in components**. Below is the complete API specification.

### 5.1 Primitives

#### `btn` / `button`
* **Category**: Primitive
* **Default classes**: `kx-btn`
* **Props**:
  * `variant` (string, default: `"default"`): Visual variation (`primary`, `secondary`, `ghost`, `danger`).
  * `size` (string, default: `"md"`): Size mapping (`sm`, `md`, `lg`).
  * `disabled` (bool, default: `false`): Disables triggers.
  * `loading` (bool, default: `false`): Replaces label with active spinner.
  * `type` (string, default: `"button"`): HTML element type (`button`, `submit`, `reset`).
* **Slots**: Default content slot (label).
```klx
btn .primary "Save Settings" on:click:
  saveData()
```

#### `link`
* **Category**: Primitive
* **Props**:
  * `href` (string, required): SPA route or URL path.
  * `external` (bool, default: `false`): Open links in new tab.
  * `active` (bool, default: `false`): True if matching current route.
* **Slots**: Default anchor content.
```klx
link href="/about" "Read About Us"
```

#### `icon`
* **Category**: Icon
* **Default classes**: `kx-icon`
* **Props**:
  * `name` (string, required): Icon token identifier.
  * `size` (string, default: `"md"`): Size mapping (`xs`, `sm`, `md`, `lg`, `xl`).
  * `label` (string, optional): Accessibility screen-reader tag.
  * `decorative` (bool, default: `false`): Set `true` to hide from screen readers.
```klx
icon name="star" .text-warning label="Add to favorites"
```

#### `image`
* **Category**: Media
* **Default classes**: `kx-image`
* **Props**:
  * `src` (string, required): Asset source URL.
  * `alt` (string, required): Descriptive screen-reader text.
  * `lazy` (bool, default: `true`): Defer image load using lazy loading.
  * `placeholder` (string, optional): Loader state (`blur`, `skeleton`, `none`).
  * `width` / `height` (number, optional): Intrinsic layout dimensions.
  * `fit` (string, default: `"cover"`): Object fit wrapper styling.
```klx
image src="/assets/banner.jpg" alt="Company Banner" .w-full
```

#### `section`
* **Category**: Primitive
* **Props**: None
* **Slots**: Default content container.
```klx
section .py-12:
  h2 "Subheading"
```

#### `container`
* **Category**: Primitive
* **Default classes**: `kx-container`
* **Props**:
  * `size` (string, default: `"lg"`): Screen constraint (`sm`, `md`, `lg`, `xl`, `2xl`, `full`).
* **Slots**: Default content container.
```klx
container size="md":
  p "Centered container contents."
```

---

### 5.2 Navigation

#### `navbar`
* **Category**: Navigation
* **Default classes**: `kx-navbar`
* **Props**:
  * `sticky` (bool, default: `false`): Pins navigation bar at top page boundaries.
  * `transparent` (bool, default: `false`): Removes theme background color.
* **Slots**: Default nav items, `end` slot for actions.
```klx
navbar sticky=true:
  link href="/" "Home"
  slot:end:
    btn .ghost "Sign In"
```

#### `footer`
* **Category**: Navigation
* **Default classes**: `kx-footer`
* **Slots**: Default footer content.
```klx
footer:
  p "Footer text block."
```

#### `sidebar`
* **Category**: Navigation
* **Default classes**: `kx-sidebar`
* **Props**:
  * `collapsed` (bool, default: `false`): Hides label text.
  * `width` (string, default: `"240px"`): Sidebar width.
* **Slots**: Default menu contents.
```klx
sidebar width="200px":
  link href="/dashboard" "Dashboard"
```

#### `pagination`
* **Category**: Navigation
* **Default classes**: `kx-pagination`
* **Props**:
  * `page` (int, required): Current page index (1-based).
  * `total` (int, required): Total dataset item count.
  * `perPage` (int, default: `10`): Items displayed per page.
  * `siblings` (int, default: `1`): Number of page buttons shown around active indicator.
```klx
pagination page=currentPage total=150 perPage=10 on:change:
  changePage(page)
```

#### `breadcrumb`
* **Category**: Navigation
* **Default classes**: `kx-breadcrumb`
* **Props**:
  * `items` (list<record>, required): List of step labels (`[{label: "Home", href: "/"}, ...]`).
  * `separator` (string, default: `"/"`): Separator string.
```klx
breadcrumb items=[{label: "Home", href: "/"}, {label: "Settings"}]
```

#### `tabs`
* **Category**: Navigation
* **Default classes**: `kx-tabs`
* **Props**:
  * `active` (string, optional): Active tab index ID.
  * `variant` (string, default: `"line"`): Tab aesthetics (`line`, `pills`, `boxed`).
* **Slots**: Child wrappers containing tab selectors.
```klx
tabs active="tab1" variant="pills":
  div data-tab="tab1":
    p "First pane."
  div data-tab="tab2":
    p "Second pane."
```

---

### 5.3 Overlays & Interactivity

#### `modal`
* **Category**: Overlay
* **Default classes**: `kx-modal`
* **Props**:
  * `id` (string, required): DOM ID for programmatic access.
  * `title` (string, optional): Modal heading text.
  * `size` (string, default: `"md"`): Dialog size (`sm`, `md`, `lg`, `xl`, `full`).
  * `closable` (bool, default: `true`): Renders dismiss close trigger icon.
* **Slots**: Default contents, `footer` action layout.
```klx
modal id="warn-dialog" title="Danger Area":
  p "Are you sure you want to clean disk space?"
  slot:footer:
    btn .danger "Wipe"
```

#### `drawer`
* **Category**: Overlay
* **Default classes**: `kx-drawer`
* **Props**:
  * `id` (string, required): Unique identifier.
  * `side` (string, default: `"right"`): Slide-in direction (`left`, `right`).
  * `title` (string, optional): Heading label.
* **Slots**: Default content body.
```klx
drawer id="sidebar-drawer" side="left":
  p "Inside drawer panel."
```

#### `tooltip`
* **Category**: Overlay
* **Default classes**: `kx-tooltip`
* **Props**:
  * `content` (string, required): Hover popup text.
  * `placement` (string, default: `"top"`): Position (`top`, `bottom`, `left`, `right`).
* **Slots**: Trigger child target element.
```klx
tooltip content="Click to duplicate token":
  btn "Copy Key"
```

---

### 5.4 Forms & Controls

#### `input`
* **Category**: Form
* **Default classes**: `kx-input`
* **Props**:
  * `type` (string, default: `"text"`): Text input variants (`text`, `email`, `password`, `number`).
  * `placeholder` (string, optional): Placeholder message.
  * `value` (string, optional): Reactive state property binding.
  * `disabled` (bool, default: `false`): Lock control edits.
  * `error` (string, optional): Warning layout error.
  * `label` (string, optional): Header description.
```klx
input label="Username" placeholder="Enter username" value=username
```

#### `select`
* **Category**: Form
* **Default classes**: `kx-select`
* **Props**:
  * `options` (list<record>, required): Array of select items (`[{label: "Red", value: "red"}]`).
  * `value` (string, optional): Bound state selection.
  * `placeholder` (string, optional): Null option prompt text.
  * `disabled` (bool, default: `false`): Lock state.
```klx
select label="Theme Selection" options=[{label: "System", value: "system"}]
```

#### `textarea`
* **Category**: Form
* **Default classes**: `kx-textarea`
* **Props**:
  * `rows` (int, default: `4`): Vertically scaled line count.
  * `placeholder` (string, optional): Display hint.
  * `value` (string, optional): Binding target.
```klx
textarea rows=6 placeholder="Describe issue details..." value=issueText
```

#### `checkbox`
* **Category**: Form
* **Default classes**: `kx-checkbox`
* **Props**:
  * `checked` (bool, default: `false`): Checked status.
  * `label` (string, optional): Inline label.
  * `disabled` (bool, default: `false`): Lock state.
```klx
checkbox label="Accept License Terms" checked=isAccepted
```

#### `switch`
* **Category**: Form
* **Default classes**: `kx-switch`
* **Props**:
  * `checked` (bool, default: `false`): Switch state.
  * `label` (string, optional): Text identifier.
  * `disabled` (bool, default: `false`): Lock toggles.
```klx
switch label="Enable Haptic Feedback" checked=hapticToggle
```

---

### 5.5 Avatars & Profiles

#### `avatar`
* **Category**: Avatar
* **Default classes**: `kx-avatar`
* **Props**:
  * `src` (string, optional): Image source URL.
  * `name` (string, optional): Initials fallback generator name source.
  * `size` (string, default: `"md"`): Size mapping (`xs`, `sm`, `md`, `lg`, `xl`, `2xl`).
  * `status` (string, optional): Activity dot (`online`, `offline`, `busy`, `away`).
  * `shape` (string, default: `"circle"`): Design structure (`circle`, `square`).
```klx
avatar src="/assets/profile.jpg" name="Sachin" status="online"
```

#### `profile-card`
* **Category**: Avatar
* **Default classes**: `kx-profile-card`
* **Props**:
  * `name` (string, required): Profile header title.
  * `avatar` (string, optional): Image resource path.
  * `role` (string, optional): Subtitle.
  * `bio` (string, optional): Summary text block.
* **Slots**: `actions` slot for buttons.
```klx
profile-card name="Arun" role="Developer" bio="Frontend enthusiast":
  slot:actions:
    btn .primary "Follow"
```

---

### 5.6 Display, Marketing & Feedback

#### `card`
* **Category**: Content
* **Default classes**: `kx-card`
* **Props**:
  * `variant` (string, default: `"default"`): Style wrap (`default`, `outline`, `elevated`).
  * `clickable` (bool, default: `false`): Adds hover state styling.
* **Slots**: Default body slot.
```klx
card .p-6:
  p "Inside card layout contents."
```

#### `toast`
* **Category**: Feedback
* **Default classes**: `kx-toast`
* **Props**:
  * `type` (string, default: `"info"`): Style type (`success`, `error`, `warning`, `info`, `loading`).
  * `message` (string, required): Banner display message.
  * `duration` (number, default: `3000`): Auto-close interval (ms).
  * `position` (string, default: `"top-right"`): Window position.
```klx
toast type="success" message="Database saved" duration=4000
```

#### `alert`
* **Category**: Feedback
* **Default classes**: `kx-alert`
* **Props**:
  * `type` (string, default: `"info"`): Theme role (`success`, `error`, `warning`, `info`).
  * `title` (string, optional): Heading title.
  * `dismissible` (bool, default: `false`): Renders dismiss close icon.
* **Slots**: Default warning content body.
```klx
alert type="warning" title="Warning" "Disk space low."
```

#### `badge`
* **Category**: Content
* **Default classes**: `kx-badge`
* **Props**:
  * `variant` (string, default: `"default"`): Color theme (`default`, `primary`, `success`, `danger`, `warning`).
  * `size` (string, default: `"md"`): Padding dimension (`sm`, `md`, `lg`).
* **Slots**: Label content.
```klx
badge variant="success" "Live"
```

#### `spinner`
* **Category**: Loader
* **Default classes**: `kx-spinner`
* **Props**:
  * `size` (string, default: `"md"`): Sizing (`xs`, `sm`, `md`, `lg`, `xl`).
  * `color` (string, default: `"primary"`): Color token.
```klx
spinner size="lg" color="success"
```

#### `skeleton`
* **Category**: Loader
* **Default classes**: `kx-skeleton`
* **Props**:
  * `width` (string, default: `"100%"`): Horizontal spacing.
  * `height` (string, default: `"1rem"`): Vertical line spacing.
  * `rounded` (bool, default: `false`): Forces circular layout (useful for avatars).
```klx
skeleton width="80px" height="80px" rounded=true
```

#### `skeleton-card`
* **Category**: Loader
* **Default classes**: `kx-skeleton-card`
* **Props**:
  * `count` (number, default: `1`): Render count.
  * `lines` (number, default: `3`): Mock lines per card structure.
```klx
skeleton-card count=4 lines=4
```

#### `empty-state`
* **Category**: Placeholder
* **Default classes**: `kx-empty-state`
* **Props**:
  * `icon` (string, optional): Icon name symbol.
  * `title` (string, default: `"Nothing here"`): Title text.
  * `description` (string, optional): Descriptive body text.
* **Slots**: `actions` button slot.
```klx
empty-state icon="search" title="No Results" description="Try editing search query.":
  slot:actions:
    btn "Clear Filter"
```

#### `accordion`
* **Category**: Content
* **Default classes**: `kx-accordion`
* **Props**:
  * `multiple` (bool, default: `false`): Open multiple drawers at once.
  * `default-open` (string, optional): ID of panel open by default.
* **Slots**: Accordion panel lists.
```klx
accordion default-open="panel1":
  div data-title="Panel Title":
    p "Indented content."
```

#### `table`
* **Category**: DataDisplay
* **Default classes**: `kx-table`
* **Props**:
  * `striped` (bool, default: `false`): Z-striped alternating row background.
  * `hoverable` (bool, default: `true`): Row hover background highlighting.
* **Slots**: Default table content body (thead, tbody, tr, th, td).
```klx
table striped=true:
  thead:
    tr:
      th "ID"
  tbody:
    tr:
      td "1"
```

#### `hero`
* **Category**: Marketing
* **Default classes**: `kx-hero`
* **Props**:
  * `variant` (string, default: `"centered"`): Layout structure (`centered`, `split`, `full`).
  * `size` (string, default: `"lg"`): Vertical padding scale (`sm`, `md`, `lg`, `xl`).
* **Slots**: Default content title, `actions` button layout, and `image` slot.
```klx
hero variant="split" size="xl":
  h1 "Landing Title"
  slot:actions:
    btn .primary "Sign Up"
  slot:image:
    image src="/hero.png" alt="App Preview"
```

#### `progress`
* **Category**: Loader
* **Default classes**: `kx-progress`
* **Props**:
  * `value` (number, default: `0`): Current progress value.
  * `max` (number, default: `100`): Max value constraint.
  * `variant` (string, default: `"primary"`): Color token.
  * `size` (string, default: `"md"`): Bar height (`sm`, `md`, `lg`).
  * `label` (bool, default: `false`): Displays percentage indicator.
```klx
progress value=75 max=100 label=true
```

#### `tooltip`
* **Category**: Overlay
* **Default classes**: `kx-tooltip`
* **Props**:
  * `content` (string, required): Hover popup text.
  * `placement` (string, default: `"top"`): Position placement (`top`, `bottom`, `left`, `right`).
* **Slots**: Trigger child element target.
```klx
tooltip content="Help info":
  icon name="info"
```

---

## 6. Reactive State & Event Handlers

### 6.1 State Binding in Input Controls
Because there is no default auto-magical two-way binding in compiled static files, form inputs must update state variables explicitly via `on:input:` listeners:

```klx
page login:
  state emailAddress: string = ""

  section .p-6:
    input value=emailAddress placeholder="Email" on:input:
      emailAddress = event.target.value
```
*Note: The compiler recognizes the special `event` identifier in event handler blocks and avoids prefixing it, letting you safely write `event.target.value`.*

### 6.2 Page Actions
Named actions declared in the page structure are compiled to scoped async functions bound to the page state. You invoke actions from template event handlers:

```klx
page counter:
  state count: int = 0

  action add:
    count = count + 1
    toast success "Added!"

  btn .primary "Add" on:click:
    add()
```

### 6.3 Browser Runtime Global Dispatcher Reference
You can invoke the following methods directly in event handlers or action blocks:

- **`toast(type, message, options?)`**: Displays temporary alert overlay. Types: `success`, `error`, `warning`, `info`, `loading`.
- **`openModal(id)` / `closeModal(id)`**: Toggles display of modal dialog overlay with given ID.
- **`openDrawer(id)` / `closeDrawer(id)`**: Slides drawer with given ID in or out.
- **`navigate(path)`**: Navigates client-side to route matching path (SPA mode).
- **`goBack()`**: Goes back to previous window history.
- **`toggleTheme()`**: Swaps between active dark/light theme options.
- **`scrollTo(selector)`**: Smoothly scrolls window viewport to match selector target element.
- **`copyToClipboard(text)`**: Copies text string directly to clipboard buffer.
- **`log(val)`**: Scoped browser logger console utility.

---

## 7. Built-in API Client

Korlix has first-class language keywords to orchestrate network resource queries.

### 7.1 Declaring Queries (GET)
To hook up an API endpoint to a page variable, declare a query at the top level using `get <name> "<url>"`:

```klx
page dashboard route "/dashboard":
  get members "http://localhost:4000/api/users"
```

When a query is declared, the compiler registers it and automatically exposes **three** reactive companion variables in the scope:
1. `<name>`: The response data payload (starts as empty list `[]`).
2. `<name>Loading`: A boolean flag indicating request state (`true` on load start, `false` on completion).
3. `<name>Error`: A string value containing the server's error response (or `null` if successful).

**Example usage in templates:**
```klx
if membersLoading:
  spinner size="lg"

if membersError:
  alert type="danger" membersError

for member in members:
  card:
    h4 member.name
```

### 7.2 Writing Mutations (POST / PUT / PATCH / DELETE)
To mutate backend data, write requests inside page actions using HTTP keyword shorthands:

```klx
state name: string = ""

action addMember:
  post "http://localhost:4000/api/users" { name: name, active: true }
  reload members    # Force re-fetching GET query
  name = ""         # Clear state
```

Supported mutation keywords:
- `post "<url>" <body>`
- `put "<url>" <body>`
- `patch "<url>" <body>`
- `delete "<url>"` (no body expected)
- `options "<url>"`
- `head "<url>"`

---

## 8. Client-Side Routing (SPA Mode)

In SPA mode, Korlix generates a single `index.html` file in the output folder. The router is managed by `korlix.runtime.js` via the route configuration declared in `src/app.klx`.

### 8.1 Registering Routes
Map your page assets to URL paths in `src/app.klx`:

```klx
app:
  layout main

  routes:
    page "/"             from "./pages/index.klx"
    page "/about"        from "./pages/about.klx"
    page "/users/:id"    from "./pages/users/[id].klx"
```

### 8.2 Dynamic Parameters
In routing paths, dynamic identifiers starting with colons (e.g. `:id`) will be parsed by the router and made accessible to the page script automatically under `params`.

For example, on the page `/users/:id`:
```klx
# Access the route parameter:
p "Selected User ID: " params.id
```

### 8.3 Route Manifest Output
The compiler generates a map registry in `dist/korlix.routes.json` which matches compiled JS modules with corresponding path patterns:
```json
{
  "/": { "id": "index", "path": "/" },
  "/about": { "id": "about", "path": "/about" },
  "/users/:id": { "id": "users_id", "path": "/users/:id" }
}
```

---

## 9. Themes Configuration

Themes are controlled via `korlix.config.json` or inline overrides in `src/app.klx`.

### 9.1 Theme Configuration
```klx
app:
  layout main
  theme:
    default "dark"       # Option: "dark" | "light"
    dark true           # Enable dark mode variants
```

### 9.2 Switching Themes
Toggling theme classes is resolved using `toggleTheme()` or programmatic runtime functions.

**JSX template switch:**
```klx
btn .border .p-2 on:click:
  toggleTheme()
```

**JavaScript script toggle:**
```js
KorlixRuntime.Theme.toggle();
```

---

## 10. Built-in Utility Functions Reference
Korlix compiles a rich library of helper functions for data manipulation, formatting, and validation directly into clean JS helpers:

### 10.1 String Functions
- `upper(str: string): string`: Transforms string to uppercase. E.g. `upper("hello")` -> `"HELLO"`.
- `lower(str: string): string`: Transforms string to lowercase. E.g. `lower("HI")` -> `"hi"`.
- `trim(str: string): string`: Strips outer whitespace.
- `contains(str: string, substr: string): bool`: Returns true if substring is present. E.g. `contains("hello", "ell")` -> `true`.
- `startsWith(str: string, prefix: string): bool`: Checks if string starts with prefix.
- `endsWith(str: string, suffix: string): bool`: Checks if string ends with suffix.
- `replace(str: string, old: string, new: string): string`: Replaces all occurrences of `old` with `new`.
- `slug(str: string): string`: Creates a URL-friendly slug. E.g. `slug("Hello World!")` -> `"hello-world"`.
- `capitalize(str: string): string`: Capitalizes the first character.

### 10.2 Number Functions
- `round(val: number): int`: Round to nearest integer.
- `floor(val: number): int`: Round down.
- `ceil(val: number): int`: Round up.
- `min(a: number, b: number): number`: Returns the smaller of two numbers.
- `max(a: number, b: number): number`: Returns the larger of two numbers.
- `clamp(val: number, min: number, max: number): number`: Constrains value to ranges. E.g. `clamp(15, 0, 10)` -> `10`.
- `formatNumber(val: number): string`: Formats float to default locale string.
- `formatCurrency(val: number, code: string): string`: Formats money representation. E.g. `formatCurrency(9.99, "USD")` -> `"$9.99"`.

### 10.3 List Functions
- `count(list: list<any>): int`: Returns list length.
- `isEmpty(list: list<any>): bool`: True if list contains 0 items.
- `filter(list: list<any>, callback: function): list<any>`: Filters matching entries.
- `map(list: list<any>, callback: function): list<any>`: Maps entries to new formats.
- `find(list: list<any>, callback: function): any`: Finds first matching entry.
- `sort(list: list<any>): list<any>`: Sorts elements.
- `reverse(list: list<any>): list<any>`: Reverses list.
- `slice(list: list<any>, start: int, end: int): list<any>`: Extracts a sub-segment of list.
- `take(list: list<any>, count: int): list<any>`: Takes first N items.
- `unique(list: list<any>): list<any>`: Removes duplicates.

### 10.4 Date & Time Functions
- `now(): string`: Returns current date-time ISO string.
- `formatDate(date: string, pattern: string): string`: Formats date representation. E.g. `formatDate("2026-06-28", "MM/DD/YYYY")` -> `"06/28/2026"`.
- `formatTime(time: string, pattern: string): string`: Formats clock layout.
- `addDays(date: string, days: int): string`: Add/subtract calendar offsets.
- `diffDays(start: string, end: string): int`: Calculates difference in days.
- `isToday(date: string): bool`: True if date is current day.

### 10.5 Validation Checks
- `isEmail(val: string): bool`: Returns true if valid email pattern.
- `isUrl(val: string): bool`: Returns true if valid URL layout.

---

## 11. Scoped Actions with Parameter Inputs
Actions can declare positional typed arguments:
```klx
action deleteUser(id: int, name: string):
  delete "http://localhost:4000/api/users/" + id
  reload users
  toast success "Successfully deleted " + name
```
You invoke this action in event handlers passing the values:
```klx
btn .danger "Delete" on:click:
  deleteUser(user.id, user.name)
```

---

## 12. Advanced Component Slots
Slots allow parents to inject markup blocks into children. Custom slots are declared using `slot:<name>` inside the component template and injected via indent block modifiers in the parent:

### 12.1 Declaring a component with slots (`src/components/modal-card.klx`)
```klx
component modal-card:
  prop title: string

  div .modal-wrapper:
    div .modal-header:
      h3 title
    
    div .modal-body:
      slot # <-- Default slot
    
    div .modal-footer:
      slot:actions # <-- Named slot "actions"
```

### 12.2 Using the component in a Page
```klx
modal-card title="Confirm Exit":
  # Default slot contents:
  p "Any changes made will be discarded."
  
  # Named slot contents:
  slot:actions:
    btn .ghost "Cancel"
    btn .primary "Confirm"
```

---

## 13. Complete Integrated Example

The following page file (`src/pages/api-test.klx`) represents a complete, functioning administration dashboard showing layouts, properties, inputs, actions, API fetches, and loaders working in sync:

```klx
page ApiTest route "/api-test":
  # 1. API GET queries
  get users "http://localhost:4000/api/users"
  get stats "http://localhost:4000/api/stats"

  # 2. Form state variables
  state name: string = ""
  state email: string = ""
  state role: string = "User"

  # 3. Scoped Actions
  action createUser:
    post "http://localhost:4000/api/users" { name: name, email: email, role: role, active: true }
    reload users
    reload stats
    # Clear fields
    name = ""
    email = ""
    toast success "Created User!"

  # 4. View Template Layout
  section .p-6 .space-y-6:
    div:
      h1 .text-3xl .font-bold "API Administration"
      p .text-muted "Test reactive queries, reloading, input bindings, and actions."

    # Stats cards
    div .grid .grid-cols-3 .gap-4:
      div .p-4 .border .rounded-lg .bg-surface:
        h3 .text-sm .font-medium .text-muted "Total Users"
        p .text-2xl .font-bold stats.totalUsers

      div .p-4 .border .rounded-lg .bg-surface:
        h3 .text-sm .font-medium .text-muted "Active Users"
        p .text-2xl .font-bold stats.activeUsers

      div .p-4 .border .rounded-lg .bg-surface:
        h3 .text-sm .font-medium .text-muted "Inactive Users"
        p .text-2xl .font-bold stats.inactiveUsers

    # Input Form Block
    div .p-6 .border .rounded-lg .space-y-4:
      h2 .text-xl .font-semibold "Register New User"

      div .grid .grid-cols-3 .gap-4:
        input .border .p-2 .rounded value=name placeholder="Full Name" on:input:
          name = event.target.value
        input .border .p-2 .rounded value=email placeholder="Email Address" on:input:
          email = event.target.value
        input .border .p-2 .rounded value=role placeholder="Job Role" on:input:
          role = event.target.value

      btn .primary "Save User" on:click:
        createUser()

    # User List Display (showing loading/error conditions)
    div .space-y-3:
      h2 .text-2xl .font-semibold "Current Team"

      if usersLoading:
        spinner size="lg"

      if usersError:
        alert type="danger" usersError

      for user in users:
        div .p-4 .border .rounded-lg .flex .items-center .justify-between:
          div:
            h4 .font-semibold user.name
            p .text-sm .text-muted user.email
            p .text-xs .font-medium user.role
          
          if user.active:
            badge .success "Active"
          else:
            badge .danger "Inactive"
```
