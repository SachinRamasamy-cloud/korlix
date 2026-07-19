# Korlix User Manual
> Complete reference for building frontend apps with the `.klx` language

---

## Table of Contents

1. [Installation & Setup](#1-installation--setup)
2. [Project Structure](#2-project-structure)
3. [Core Syntax](#3-core-syntax)
4. [Pages & Routing](#4-pages--routing)
5. [Layouts](#5-layouts)
6. [Components — Creating Your Own](#6-components--creating-your-own)
7. [State & Reactivity](#7-state--reactivity)
8. [Events & Actions](#8-events--actions)
9. [Built-in UI Components](#9-built-in-ui-components)
   - 9.1 Primitives
   - 9.2 Avatar & Profile
   - 9.3 Navigation
   - 9.4 Loaders & Skeletons
   - 9.5 Feedback & Toast
   - 9.6 Overlay — Modal, Drawer, Tooltip
   - 9.7 Forms
   - 9.8 Content Cards
   - 9.9 Data Display
   - 9.10 Marketing Sections
10. [Color System](#10-color-system)
11. [Utility Classes — Full Reference](#11-utility-classes--full-reference)
12. [Variants & Responsive Design](#12-variants--responsive-design)
13. [Dark Mode](#13-dark-mode)
14. [Arbitrary Values](#14-arbitrary-values)
15. [Data Fetching](#15-data-fetching)
16. [Built-in Functions](#16-built-in-functions)
17. [Connecting to a Backend](#17-connecting-to-a-backend)
18. [SPA vs Static Mode](#18-spa-vs-static-mode)
19. [Hot Drop (Dev Server)](#19-hot-drop-dev-server)
20. [Configuration Reference](#20-configuration-reference)
21. [CLI Reference](#21-cli-reference)
22. [Error Codes](#22-error-codes)

---

## 1. Installation & Setup

### Install Rust (required)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Build Korlix from source
```bash
git clone https://github.com/korlix-lang/korlix
cd korlix
cargo build --release
# Move binary to PATH
cp target/release/korlix /usr/local/bin/
```

### Verify installation
```bash
korlix --version
# ◈  Korlix v0.1.0
```

### Create your first project
```bash
korlix new my-website
cd my-website
korlix dev
# Open http://localhost:3000
```

---

## 2. Project Structure

```
my-website/
│
├── korlix.config.json          ← Project config (name, mode, port, theme)
├── package.json                ← NPM scripts (dev, build, preview)
├── .gitignore
│
├── public/                     ← Static assets copied directly to dist/
│   ├── index.html              ← HTML shell (compiler injects CSS + JS)
│   └── assets/
│       ├── logo.png
│       ├── hero.jpg
│       └── favicon.ico
│
├── src/                        ← All your .klx source files live here
│   ├── main.klx                ← Entry point — mounts the app
│   ├── app.klx                 ← Routes, layout, providers, theme
│   │
│   ├── pages/                  ← Each file = one route
│   │   ├── index.klx           → /
│   │   ├── about.klx           → /about
│   │   ├── contact.klx         → /contact
│   │   └── products/
│   │       ├── index.klx       → /products
│   │       └── [id].klx        → /products/:id  (dynamic route)
│   │
│   ├── layouts/                ← Wrappers that wrap page content
│   │   ├── main.klx            ← Default layout (navbar + slot + footer)
│   │   ├── auth.klx            ← Auth pages layout
│   │   └── dashboard.klx       ← App/dashboard layout
│   │
│   ├── components/             ← Your reusable custom components
│   │   ├── hero.klx
│   │   ├── feature-card.klx
│   │   └── pricing-card.klx
│   │
│   ├── state/                  ← Shared state stores (optional)
│   │   ├── user-state.klx
│   │   └── cart-state.klx
│   │
│   └── theme/
│       └── tokens.klx          ← Design tokens (colors, spacing overrides)
│
└── dist/                       ← Build output (auto-generated, do not edit)
    ├── index.html
    ├── about/index.html
    ├── korlix.routes.json
    ├── korlix.manifest.json
    └── assets/
        ├── korlix.css          ← JIT-generated CSS (only used classes)
        ├── korlix.runtime.js   ← Browser runtime (~20kb)
        └── app.js              ← Your compiled app logic
```

---

## 3. Core Syntax

### Rules
- `.klx` files use **indentation** to define blocks (like Python)
- Blocks are opened with a **colon `:`**
- Classes start with a **dot `.`** (`.flex`, `.bg-primary`)
- Props are written as **key=value** (`src="/logo.png"`)
- Comments use **`#`**

### Basic example
```klx
# This is a comment
page index route "/":
  div .flex .flex-col .items-center .py-20:
    h1 .text-5xl .font-bold .text-primary "Hello, Korlix!"
    p .text-muted "Build websites with clean syntax."
    btn .primary "Click me"
```

### HTML elements available directly
```klx
div span p h1 h2 h3 h4 h5 h6
a button input textarea select label
img video audio iframe
section article header footer main nav aside
ul ol li table thead tbody tr td th
code pre blockquote br hr strong em
form figure figcaption details summary
```

### Inline text
```klx
h1 "Static text"
h1 myVariable
h1 "Hello " + name + "!"
```

---

## 4. Pages & Routing

### Defining a page
```klx
page index route "/":
  h1 "Home page"
```

```klx
page about route "/about":
  h1 "About page"
```

### Dynamic routes
```klx
# File: src/pages/products/[id].klx
page product-detail route "/products/:id":
  h1 "Product detail"
```

### Page with meta (SEO)
```klx
page about route "/about":
  meta:
    title "About Us — My Site"
    description "Learn about our team and mission."

  h1 "About"
```

### Registering routes in app.klx
```klx
app:
  layout main

  routes:
    page "/"              from "./pages/index.klx"
    page "/about"         from "./pages/about.klx"
    page "/blog"          from "./pages/blog/index.klx"
    page "/blog/:slug"    from "./pages/blog/[slug].klx"
    page "/products"      from "./pages/products/index.klx"
    page "/products/:id"  from "./pages/products/[id].klx"

  providers:
    toast
    modal
    theme
```

### Navigating between pages
```klx
# Declarative link (SPA-aware)
link href="/about" "About"

# Programmatic navigation
btn "Go to About" on:click:
  navigate("/about")

btn "Go Back" on:click:
  goBack()
```

---

## 5. Layouts

A layout wraps every page that uses it. The **`slot`** keyword marks where page content is inserted.

### Defining a layout
```klx
# src/layouts/main.klx
layout main:
  navbar .bg-surface .border-b .border-border .sticky .top-0 .z-50:
    div .max-w-7xl .mx-auto .px-6 .py-4 .flex .items-center .justify-between:
      link href="/" .font-bold .text-primary "My Site"
      div .flex .gap-6:
        link href="/"       .text-muted "Home"
        link href="/about"  .text-muted "About"
        link href="/contact" .text-muted "Contact"

  main .min-h-screen:
    slot            ← page content goes here

  footer .bg-surface .border-t .border-border .py-8 .text-center:
    p .text-muted "© 2024 My Site"
```

### Using a layout in a page
```klx
page index route "/":
  layout main          ← or set globally in app.klx

  section .py-20:
    h1 "Content here"
```

### Multiple layouts
```klx
# dashboard layout
layout dashboard:
  div .flex .min-h-screen:
    aside .w-64 .bg-surface .border-r:
      nav .p-4:
        link href="/" "Dashboard"
        link href="/users" "Users"
    div .flex-1:
      slot            ← page content
```

---

## 6. Components — Creating Your Own

### Basic component
```klx
# src/components/feature-card.klx
component feature-card:
  prop title: string
  prop description: string
  prop icon: string = "star"

  card .p-6 .rounded-xl .border .border-border:
    icon name=icon .size-8 .text-primary .mb-4
    h3 .text-lg .font-bold .mb-2 title
    p .text-muted description
```

### Using your component
```klx
feature-card
  title="Compile-time first"
  description="All validation happens before output."
  icon="zap"
```

### Component with slot (children)
```klx
component section-wrapper:
  prop heading: string

  section .py-20 .px-6:
    h2 .text-4xl .font-bold .mb-10 heading
    slot             ← children go here
```

```klx
section-wrapper heading="Our Features":
  feature-card title="Fast" description="Blazing speed"
  feature-card title="Safe" description="Compile-time checks"
```

### Component with multiple slots
```klx
component page-header:
  slot:title
  slot:actions

  header .flex .justify-between .py-6:
    slot title
    div .flex .gap-3:
      slot actions
```

---

## 7. State & Reactivity

### Declaring state
```klx
state count: int = 0
state name: string = "World"
state open: bool = false
state items: list<string> = []
state user: record = { name: "Sachin", role: "admin" }
```

### State types
| Type | Example |
|------|---------|
| `string` | `"hello"` |
| `int` | `42` |
| `float` | `3.14` |
| `bool` | `true` / `false` |
| `null` | `null` |
| `list<string>` | `["a", "b"]` |
| `record` | `{ key: value }` |

### Binding state to the UI
State automatically updates DOM when changed:
```klx
state count: int = 0

p count                    # shows current value
p "Count: " + count        # inline binding
h1 .text-primary count     # bound with classes
```

### Computed values (derived)
```klx
state price: float = 9.99
state quantity: int = 3

derived total = price * quantity
derived displayTotal = "Total: $" + round(total, 2)
derived isValid = name.length > 0 && email.length > 0
```

### Let (local constants)
```klx
let title = "Dashboard"
let apiBase = "https://api.example.com"
```

---

## 8. Events & Actions

### Inline event handlers
```klx
btn "Click" on:click:
  count = count + 1
  toast success "Clicked!"
```

### Multiple actions in one handler
```klx
btn .primary "Save & Close" on:click:
  saveData()
  closeModal("form")
  toast success "Saved"
  navigate("/dashboard")
```

### All available events
```
on:click          on:dblclick       on:mouseenter     on:mouseleave
on:input          on:change         on:focus          on:blur
on:submit         on:keydown        on:keyup          on:keypress
on:scroll         on:resize         on:mount          on:unmount
```

### Named actions (reusable)
```klx
action saveUser:
  post "/api/users" { name: name, email: email }
  toast success "User saved"
  navigate("/users")

action validateForm:
  if name.length < 2:
    toast error "Name too short"
  else:
    saveUser()

btn "Save" on:click:
  validateForm()
```

### Conditional logic
```klx
if count > 10:
  alert type="success" "Great job!"
else:
  p .text-muted "Keep going..."
```

### Loops
```klx
for item in products:
  product-card
    name=item.name
    price=item.price
```

---

## 9. Built-in UI Components

All components are expanded at **compile time** to clean HTML. No runtime JS overhead.

---

### 9.1 Primitives

#### btn / button
```klx
btn .primary "Save"
btn .secondary "Export"
btn .ghost "Cancel"
btn .danger "Delete"
btn .primary disabled=true "Not available"
btn .primary loading=true "Saving..."
btn .primary type="submit" "Submit"
btn .primary on:click:
  doSomething()
  "Click me"
```

#### link
```klx
link href="/about" "About"                         # SPA link
link href="https://example.com" external=true "External"  # new tab + rel=noopener
link href="/" active=true .font-bold "Home"        # active state
```

#### icon
```klx
icon name="user" label="User profile"              # accessible
icon name="star" .size-4 .text-primary             # sized + colored
icon name="close" decorative=true                  # hidden from screen readers
icon name="check" .size-6 .text-success label="Done"
```

#### image
```klx
image src="/hero.jpg" alt="Hero image"
image src="/thumb.jpg" alt="Thumbnail" .rounded-xl .w-full
image src="/pic.jpg" alt="Photo" lazy=true width=800 height=400
image src="/portrait.jpg" alt="Portrait" fit="contain"
```

#### section / container / box
```klx
section .py-20 .px-6 .bg-surface:
  h1 "Section"

container size="lg":                               # max-w + mx-auto
  p "Centered content"

container size="xl":
  div .grid .grid-cols-3 .gap-8:
    ...
```

---

### 9.2 Avatar & Profile

#### avatar
```klx
avatar src="/profile.jpg" name="Sachin"           # image with fallback initials
avatar name="Sachin Dev" size="md"                 # initials only
avatar src="/pic.jpg" name="Alice" size="lg" status="online"

# Sizes: xs | sm | md | lg | xl | 2xl
# Status: online | offline | busy | away
```

#### avatar-group
```klx
avatar-group:
  avatar src="/a.jpg" name="Sachin"
  avatar src="/b.jpg" name="Arun"
  avatar name="Meera"
```

#### profile-card
```klx
profile-card name="Sachin" avatar="/profile.jpg" role="Full-Stack Developer":
  slot:actions:
    btn .primary "Follow"
    btn .ghost "Message"

profile-card name="Arun" role="Backend Engineer" bio="Building SaveTrax":
  slot:actions:
    btn .secondary "Connect"
```

---

### 9.3 Navigation

#### navbar
```klx
navbar .bg-surface .border-b .sticky .top-0 .z-50:
  div .max-w-7xl .mx-auto .px-6 .py-4 .flex .items-center .justify-between:
    link href="/" .font-bold .text-xl "Brand"
    div .flex .gap-6:
      link href="/" "Home"
      link href="/about" "About"
    btn .primary "Sign Up"
```

#### sidebar
```klx
sidebar collapsed=false width="240px":
  nav .p-4:
    link href="/" "Dashboard"
    link href="/users" "Users"
    link href="/settings" "Settings"
```

#### pagination
```klx
pagination page=1 total=100 perPage=10
pagination page=currentPage total=itemCount perPage=10 on:change:
  loadPage(page)
```

#### breadcrumb
```klx
breadcrumb items=[
  { label: "Home", href: "/" },
  { label: "Products", href: "/products" },
  { label: "MacBook Pro" }
]
```

#### tabs
```klx
tabs active="overview":
  div data-tab="overview":
    p "Overview content"
  div data-tab="specs":
    p "Specs content"
  div data-tab="reviews":
    p "Reviews content"
```

---

### 9.4 Loaders & Skeletons

#### spinner
```klx
spinner                              # default md
spinner size="sm"
spinner size="lg" color="primary"
```

#### skeleton
```klx
skeleton width="100%" height="1.5rem"
skeleton width="60%" height="1rem"
skeleton width="2.5rem" height="2.5rem" rounded=true    # circular
```

#### skeleton-card
```klx
skeleton-card count=3               # shows 3 card skeletons
skeleton-card count=6 lines=4       # 6 cards, 4 lines each
```

#### progress
```klx
progress value=65                   # 0-100 percent
progress value=40 max=100 label=true  # show % label
progress value=75 variant="success"
```

#### page-loader
```klx
page-loader                         # full-screen loading overlay
```

---

### 9.5 Feedback & Toast

#### toast (triggered via events)
```klx
btn "Save" on:click:
  toast success "Saved successfully"

btn "Delete" on:click:
  toast error "Failed to delete"

btn "Submit" on:click:
  toast warning "Please check your input"

btn "Info" on:click:
  toast info "Processing your request..."

btn "Upload" on:click:
  toast loading "Uploading..."

# With options
toast success "Done!" position="bottom-right" duration=5000
toast error "Failed" duration=0     # manual dismiss only
```

#### alert (inline)
```klx
alert type="success" "Your changes have been saved."
alert type="danger" "Something went wrong. Try again."
alert type="warning" "Your session expires in 5 minutes."
alert type="info" title="Note" "Read-only mode is active."
alert type="danger" dismissible=true "Error occurred"
```

#### banner
```klx
banner type="warning" "Scheduled maintenance on Sunday 2AM UTC."
```

---

### 9.6 Overlay — Modal, Drawer, Tooltip

#### modal
```klx
# Trigger
btn "Open" on:click:
  openModal("my-dialog")

# Modal definition
modal id="my-dialog" title="Edit Profile" size="md":
  div .space-y-4:
    form-field:
      label "Name"
      input type="text" placeholder="Your name"
  slot:footer:
    btn .primary "Save" on:click:
      closeModal("my-dialog")
      toast success "Saved"
    btn .ghost "Cancel" on:click:
      closeModal("my-dialog")

# Sizes: sm | md | lg | xl | full
```

#### drawer
```klx
btn "Open Settings" on:click:
  openDrawer("settings")

drawer id="settings" title="Settings" side="right":
  div .p-6 .space-y-6:
    h3 .font-semibold "Preferences"
    form-field:
      switch label="Dark mode" checked=true

# side: right | left
```

#### tooltip
```klx
tooltip content="Saves your work" placement="top":
  btn .ghost "Save"

tooltip content="Delete permanently" placement="right":
  icon name="trash" .size-4
```

#### popover
```klx
popover trigger="click":
  slot:trigger:
    btn .ghost "Options"
  slot:content:
    div .p-4 .space-y-2:
      button .w-full .text-left "Edit"
      button .w-full .text-left "Duplicate"
      button .w-full .text-left .text-danger "Delete"
```

---

### 9.7 Forms

```klx
form on:submit:
  handleSubmit()

  form-field:
    label "Full name"
    input type="text" placeholder="Sachin Dev"
    form-help "This will be displayed publicly."

  form-field:
    label "Email"
    input type="email" placeholder="you@example.com"
    form-error error="Please enter a valid email"

  form-field:
    label "Password"
    input type="password" placeholder="••••••••"

  form-field:
    label "Bio"
    textarea rows=4 placeholder="Tell us about yourself..."

  form-field:
    label "Country"
    select options=[
      { label: "India", value: "IN" },
      { label: "USA",   value: "US" },
      { label: "UK",    value: "GB" }
    ]

  form-field:
    checkbox label="I agree to the terms" checked=false

  form-field:
    switch label="Enable notifications" checked=true

  form-field:
    label "Rating"
    range min=0 max=10 value=5

  form-field:
    label "Attachment"
    file-upload accept=".pdf,.doc" label="Upload document"

  div .flex .gap-3:
    btn .primary type="submit" "Submit"
    btn .ghost type="reset" "Reset"
```

#### Specialized inputs
```klx
input type="email" placeholder="email@example.com"
input type="password" placeholder="Password"
input type="number" min=0 max=100 step=1
input type="search" placeholder="Search..."
input type="url" placeholder="https://example.com"
input type="tel" placeholder="+91 9876543210"
```

#### OTP input
```klx
otp-input length=6 on:complete:
  verifyCode(otp)
```

---

### 9.8 Content Cards

#### card
```klx
card .p-6 .rounded-xl:
  h3 "Card title"
  p "Card content"

card clickable=true .hover:border-primary on:click:
  navigate("/details")
```

#### badge / tag / chip
```klx
badge "New"
badge .primary "Active"
badge .success "Verified"
badge .danger "Urgent"
badge .warning "Pending"

tag "Design"
tag .primary "React"
tag .removable on:remove:
  removeTag(tag)
```

#### accordion
```klx
accordion:
  div data-title="What is Korlix?":
    p "Korlix is a frontend language..."
  div data-title="How does routing work?":
    p "Add routes in app.klx..."
  div data-title="Is it free?":
    p "Yes, MIT licensed."

accordion multiple=true:             # allow multiple open panels
```

#### timeline
```klx
timeline:
  div data-date="2024-01" data-title="Started Zoriqa":
    p "Built the first prototype."
  div data-date="2024-06" data-title="Rebuilt as Korlix":
    p "Complete rewrite in Rust."
  div data-date="2024-12" data-title="Phase 1 launched":
    p "12-crate compiler released."
```

#### code-block
```klx
code-block language="klx":
  "page index:\n  h1 \"Hello World\""
```

---

### 9.9 Data Display

#### table
```klx
table striped=true hoverable=true:
  thead:
    tr:
      th "Name"
      th "Email"
      th "Status"
      th "Actions"
  tbody:
    for user in users:
      tr:
        td user.name
        td user.email
        td:
          badge user.status
        td:
          btn .ghost .sm "Edit" on:click:
            editUser(user.id)
          btn .danger .sm "Delete" on:click:
            deleteUser(user.id)
```

#### stat-card
```klx
stat-card value="$48,295" label="Total Revenue" change="+12.5%" trend="up"
stat-card value="2,847" label="Active Users" change="+8.1%" trend="up"
stat-card value="3.6%" label="Conversion" change="-0.3%" trend="down"
```

---

### 9.10 Marketing Sections

#### hero
```klx
hero variant="centered" size="xl":
  h1 .text-7xl .font-black "Build the Future"
  p .text-xl .text-muted "Your tagline here."
  slot:actions:
    btn .primary "Get Started"
    btn .ghost "Learn More"

# variant: centered | split | full
# size: sm | md | lg | xl
```

#### empty-state
```klx
empty-state icon="inbox" title="No messages" description="Your inbox is empty."
empty-state icon="search" title="No results" description="Try a different search."
empty-state icon="box" title="No products":
  slot:actions:
    btn .primary "Add Product"
```

---

## 10. Color System

### Semantic Colors
Use these for consistent theming across your app:

```klx
.text-primary          # Main brand color (violet by default)
.text-primary-light    # Lighter variant
.text-primary-dark     # Darker variant
.text-secondary        # Secondary brand color
.text-accent           # Accent/highlight color
.text-success          # Green — success states
.text-danger           # Red — errors and warnings
.text-warning          # Amber — cautions
.text-info             # Blue — informational
.text-muted            # Gray — secondary text
.text-foreground       # Default text color
.text-background       # Page background color
.text-surface          # Card/panel background
.text-border           # Border color
```

Same prefixes work for backgrounds, borders, rings, fill, stroke:
```klx
.bg-primary  .bg-surface  .bg-background
.border-primary  .border-border
.ring-primary  .ring-danger
.fill-success  .stroke-primary
```

### Color Palettes (50–950)

Every shade from lightest (50) to darkest (950):

```klx
# Slate
.bg-slate-50  .bg-slate-100  .bg-slate-200  .bg-slate-300  .bg-slate-400
.bg-slate-500  .bg-slate-600  .bg-slate-700  .bg-slate-800  .bg-slate-900  .bg-slate-950

# Blue
.bg-blue-50  .bg-blue-500  .bg-blue-900

# Violet (great for dark UIs)
.bg-violet-400  .bg-violet-500  .bg-violet-600  .bg-violet-700

# Emerald
.bg-emerald-400  .bg-emerald-500

# Rose / Red / Orange / Amber / Yellow / Green / Teal / Cyan
# Sky / Indigo / Purple / Fuchsia / Pink
```

**Available palettes:** `slate`, `gray`, `zinc`, `neutral`, `stone`, `red`, `orange`, `amber`, `yellow`, `lime`, `green`, `emerald`, `teal`, `cyan`, `sky`, `blue`, `indigo`, `violet`, `purple`, `fuchsia`, `pink`, `rose`

### Color Utility Families
For EVERY color token and palette shade:

```
.text-{color}          Color the text
.bg-{color}            Set background
.border-{color}        Set border color
.ring-{color}          Focus ring color
.fill-{color}          SVG fill
.stroke-{color}        SVG stroke
.outline-{color}       Outline color
.caret-{color}         Input caret color
.placeholder-{color}   Placeholder text
```

---

## 11. Utility Classes — Full Reference

### Layout & Display
```klx
.block  .inline-block  .inline  .flex  .inline-flex
.grid  .inline-grid  .hidden  .contents  .table  .table-cell
```

### Position
```klx
.static  .relative  .absolute  .fixed  .sticky
.top-0  .right-0  .bottom-0  .left-0
.inset-0  .inset-x-0  .inset-y-0
```

### Flexbox
```klx
.flex-row  .flex-col  .flex-row-reverse  .flex-col-reverse
.flex-wrap  .flex-nowrap
.flex-1  .flex-auto  .flex-none
.flex-grow  .flex-grow-0  .flex-shrink  .flex-shrink-0
.items-start  .items-center  .items-end  .items-stretch  .items-baseline
.justify-start  .justify-center  .justify-end
.justify-between  .justify-around  .justify-evenly
.self-start  .self-center  .self-end  .self-stretch
.center                 # shortcut: place-items: center
```

### Grid
```klx
.grid-cols-1  .grid-cols-2  .grid-cols-3  .grid-cols-4
.grid-cols-5  .grid-cols-6  .grid-cols-12
.grid-rows-1  .grid-rows-2  .grid-rows-3
.col-span-1  .col-span-2  .col-span-3  .col-span-full
.gap-1  .gap-2  .gap-4  .gap-6  .gap-8
.gap-x-4  .gap-y-6
```

### Spacing (0–96 scale)
```
Scale: 0 | px | 0.5 | 1 | 1.5 | 2 | 2.5 | 3 | 3.5 | 4 | 5 | 6 | 7 | 8 | 9
       10 | 11 | 12 | 14 | 16 | 20 | 24 | 28 | 32 | 36 | 40 | 44 | 48 | 52
       56 | 60 | 64 | 72 | 80 | 96
```

```klx
.p-4   .px-6   .py-3   .pt-4  .pr-4  .pb-4  .pl-4
.m-4   .mx-auto .my-8  .mt-4  .mr-2  .mb-4  .ml-2
.space-x-4   .space-y-2
```

### Sizing
```klx
.w-full  .w-screen  .w-auto  .w-1/2  .w-1/3  .w-2/3  .w-1/4  .w-3/4
.h-full  .h-screen  .h-auto
.min-h-screen  .min-h-full  .min-w-0
.max-w-sm  .max-w-md  .max-w-lg  .max-w-xl
.max-w-2xl  .max-w-3xl  .max-w-4xl  .max-w-5xl  .max-w-6xl  .max-w-7xl
.size-4  .size-6  .size-8  .size-12         # width + height together
```

### Typography
```klx
# Font size + line-height
.text-xs   .text-sm   .text-base  .text-lg   .text-xl
.text-2xl  .text-3xl  .text-4xl   .text-5xl  .text-6xl
.text-7xl  .text-8xl  .text-9xl

# Font weight
.font-thin  .font-light  .font-normal  .font-medium
.font-semibold  .font-bold  .font-extrabold  .font-black

# Alignment
.text-left  .text-center  .text-right  .text-justify

# Style
.italic  .not-italic
.uppercase  .lowercase  .capitalize  .normal-case

# Decoration
.underline  .line-through  .no-underline

# Line height
.leading-none  .leading-tight  .leading-snug  .leading-normal
.leading-relaxed  .leading-loose

# Letter spacing
.tracking-tight  .tracking-normal  .tracking-wide  .tracking-wider  .tracking-widest

# Overflow
.truncate  .whitespace-nowrap  .whitespace-pre
.break-words  .break-all
```

### Border
```klx
.border    .border-0  .border-2  .border-4  .border-8
.border-t  .border-r  .border-b  .border-l
.border-solid  .border-dashed  .border-dotted  .border-none

# Radius
.rounded-none  .rounded-sm  .rounded  .rounded-md  .rounded-lg
.rounded-xl  .rounded-2xl  .rounded-3xl  .rounded-full
```

### Shadow
```klx
.shadow-sm  .shadow  .shadow-md  .shadow-lg  .shadow-xl  .shadow-2xl
.shadow-inner  .shadow-none
```

### Opacity
```klx
.opacity-0  .opacity-5  .opacity-10  .opacity-20  .opacity-25
.opacity-30  .opacity-40  .opacity-50  .opacity-60
.opacity-70  .opacity-75  .opacity-80  .opacity-90  .opacity-95  .opacity-100
```

### Effects & Filters
```klx
.blur-none  .blur-sm  .blur  .blur-md  .blur-lg  .blur-xl  .blur-2xl  .blur-3xl
.brightness-50  .brightness-75  .brightness-90  .brightness-100
.brightness-110  .brightness-125
.backdrop-blur  .backdrop-blur-sm  .backdrop-blur-md  .backdrop-blur-xl
```

### Transform
```klx
.scale-0   .scale-50  .scale-75  .scale-90  .scale-95
.scale-100  .scale-105  .scale-110  .scale-125  .scale-150
.rotate-0  .rotate-1   .rotate-2  .rotate-3   .rotate-6
.rotate-12  .rotate-45  .rotate-90  .rotate-180
.-rotate-1  .-rotate-2  .-rotate-6  .-rotate-12
```

### Transition & Animation
```klx
.transition          # all properties
.transition-colors   # color only
.transition-transform
.transition-opacity
.transition-shadow
.transition-none
.duration-75   .duration-100  .duration-150  .duration-200
.duration-300  .duration-500  .duration-700  .duration-1000
```

### Z-index
```klx
.z-0  .z-10  .z-20  .z-30  .z-40  .z-50  .z-auto
```

### Overflow
```klx
.overflow-auto  .overflow-hidden  .overflow-visible  .overflow-scroll
.overflow-x-auto  .overflow-y-auto
.overflow-x-hidden  .overflow-y-hidden
```

### Cursor
```klx
.cursor-auto  .cursor-default  .cursor-pointer  .cursor-wait
.cursor-text  .cursor-not-allowed  .cursor-grab  .cursor-grabbing
```

### Accessibility
```klx
.sr-only           # visually hidden but screen-reader accessible
.not-sr-only       # undo sr-only
.outline-none      # remove focus outline (use with custom ring)
.ring              # focus ring (uses --kx-ring-color)
.ring-2
```

---

## 12. Variants & Responsive Design

### Responsive breakpoints
```klx
sm:    # min-width: 576px
md:    # min-width: 768px
lg:    # min-width: 992px
xl:    # min-width: 1200px
2xl:   # min-width: 1400px
```

```klx
div .hidden .md:block .lg:grid .lg:grid-cols-3:
  "Responsive layout"

h1 .text-3xl .md:text-5xl .lg:text-7xl "Big heading"
```

### State variants
```klx
btn .bg-primary .hover:bg-primary-dark .active:scale-95 .focus:ring "Button"
input .border .focus:border-primary .focus:ring "Input"
li .hover:bg-surface .cursor-pointer "List item"
btn .disabled:opacity-50 .disabled:cursor-not-allowed "Disabled"
input .checked:border-primary .checked:bg-primary type="checkbox"
```

### Dark / Light variants
```klx
div .bg-white .dark:bg-slate-900:
  p .text-black .dark:text-white "Adapts to theme"
```

### Group variants
```klx
div .group:
  icon .group-hover:text-primary name="arrow-right"
  p .group-hover:translate-x-1 "Hover the parent"
```

### Peer variants
```klx
input .peer type="checkbox"
span .peer-checked:text-primary "Checked state label"
```

### Reduced motion
```klx
div .transition .motion-reduce:transition-none "Respects OS setting"
div .motion-safe:animate-bounce "Only animates if user allows"
```

---

## 13. Dark Mode

### Setup
```klx
# app.klx
app:
  theme:
    default "dark"    # or "light"
    dark true         # enable dark mode support

  providers:
    theme             # include theme provider
```

### Toggle dark mode
```klx
btn "Toggle Theme" on:click:
  toggleTheme()
```

### Write dark-mode styles
```klx
div .bg-white .dark:bg-slate-950:
  p .text-gray-900 .dark:text-gray-100 "Content"
  p .text-gray-500 .dark:text-gray-400 "Muted"
```

---

## 14. Arbitrary Values

Use any valid CSS value when the built-in scale doesn't fit:

```klx
div .w-[432px] .h-[calc(100vh-4rem)]
div .bg-[#0f1c24] .text-[#a78bfa]
div .grid-cols-[240px_1fr_300px]
div .mt-[clamp(1rem,5vw,3rem)]
div .border-[rgba(124,58,237,0.3)]
div .top-[var(--header-height)]
div .bg-[url(/hero.jpg)]
```

**Blocked for security:** `javascript:` and `expression()` inside arbitrary values.

---

## 15. Data Fetching

```klx
page products route "/products":
  # GET request with loading, error, empty states
  data products = get "/api/products":
    loading skeleton-card count=6
    error empty-state icon="warning" title="Failed to load"
    empty empty-state icon="box" title="No products found"

  # Use the data
  div .grid .grid-cols-3 .gap-6:
    for p in products:
      product-card product=p
```

### All HTTP methods
```klx
data users    = get    "/api/users"
data newUser  = post   "/api/users"   { name: name, email: email }
data updated  = put    "/api/users/1" { name: newName }
data patched  = patch  "/api/items/1" { active: false }
data deleted  = delete "/api/items/42"
data uploaded = upload "/api/files"   file
```

### Programmatic API calls in events
```klx
btn "Save" on:click:
  post "/api/contact" { name: name, email: email, message: msg }
  toast success "Message sent!"
  navigate("/")

btn "Delete" on:click:
  delete "/api/items/" + itemId
  toast success "Deleted"
  navigate("/items")
```

---

## 16. Built-in Functions

### String
```klx
upper("hello")                # "HELLO"
lower("HELLO")                # "hello"
trim("  hello  ")             # "hello"
replace("foo bar", "foo", "baz")   # "baz bar"
contains("hello world", "world")   # true
startsWith("https://x.com", "https")  # true
endsWith("file.pdf", ".pdf")       # true
slug("My Blog Post")               # "my-blog-post"
capitalize("hello")                # "Hello"
```

### Number
```klx
round(3.7)          # 4
floor(3.9)          # 3
ceil(3.1)           # 4
min(5, 3)           # 3
max(5, 3)           # 5
clamp(150, 0, 100)  # 100
formatNumber(1000000)          # "1,000,000"
formatCurrency(9.99, "USD")    # "$9.99"
formatCurrency(499.00, "INR")  # "₹499.00"
```

### List
```klx
count(items)                         # length
isEmpty(items)                       # true/false
filter(items, fn)                    # filtered list
map(items, fn)                       # transformed list
find(items, fn)                      # first match
sort(items)                          # sorted copy
reverse(items)                       # reversed copy
slice(items, 0, 5)                   # sub-list
take(items, 3)                       # first N items
unique(items)                        # deduplicated
```

### Date
```klx
now()                                # current datetime
formatDate(date, "YYYY-MM-DD")
formatTime(date, "HH:mm")
addDays(date, 7)
diffDays(start, end)                 # number of days between
isToday(date)                        # true/false
```

### Validation
```klx
isEmpty(value)                       # null, "", [], {}
isEmail("a@b.com")                   # true
isUrl("https://x.com")              # true
```

### UI Functions
```klx
toast(type, message, opts?)          # show toast
openModal(id)                        # open modal
closeModal(id)                       # close modal
openDrawer(id)                       # open drawer
closeDrawer(id)                      # close drawer
navigate(path)                       # SPA navigate
goBack()                             # browser history back
toggleTheme()                        # dark / light
scrollTo("#section")                 # smooth scroll
copyToClipboard("some text")         # clipboard
log(value)                           # console.log (debug)
```

---

## 17. Connecting to a Backend

Korlix is **frontend-only**. It sends HTTP requests to your backend API from the browser.

### Any backend works
```
Korlix frontend (.klx)
        ↓  HTTP requests
Your Backend API
  - Node.js / Express
  - Python / FastAPI / Django
  - Java / Spring Boot
  - Go / Gin
  - PHP / Laravel
        ↓
Your Database
```

### Example: Express backend + Korlix frontend

**Backend (server.js)**
```js
const express = require('express');
const app = express();
app.use(express.json());

app.get('/api/users', (req, res) => {
  res.json([{ id: 1, name: 'Sachin' }, { id: 2, name: 'Arun' }]);
});

app.post('/api/users', (req, res) => {
  const { name, email } = req.body;
  // save to DB...
  res.json({ id: 3, name, email });
});

app.listen(4000);
```

**Frontend (.klx)**
```klx
page users route "/users":
  data users = get "http://localhost:4000/api/users":
    loading spinner
    error alert type="danger" "Could not load users"
    empty empty-state icon="user" title="No users"

  for user in users:
    div .flex .items-center .gap-3 .py-3 .border-b:
      avatar name=user.name size="sm"
      p user.name
```

### CORS on your backend
Your backend must allow requests from the Korlix dev server origin:
```js
// Express
const cors = require('cors');
app.use(cors({ origin: 'http://localhost:3000' }));
```

### Environment variables
Put your API base URL in `korlix.config.json`:
```json
{
  "env": {
    "API_BASE": "https://api.yourapp.com"
  }
}
```

Use it in `.klx`:
```klx
data products = get env.API_BASE + "/products":
  loading skeleton-card count=4
```

---

## 18. SPA vs Static Mode

### Static mode (default)
- Generates one HTML file per page
- Best for: landing pages, blogs, docs
- No JS router needed

```bash
korlix build
korlix build --mode static
```

### SPA mode
- Generates one `index.html` + JS router
- Best for: dashboards, web apps
- Client-side navigation, no page reloads

```bash
korlix build --mode spa
```

```json
// korlix.config.json
{ "mode": "spa" }
```

### SSG mode (coming in v0.2)
- Pre-renders known routes at build time
- Best for: blogs with dynamic data, product pages

---

## 19. Hot Drop (Dev Server)

Hot drop updates your browser instantly as you edit `.klx` files.

### Start the dev server
```bash
korlix dev
# ◈  Korlix dev server
# →  http://localhost:3000
# ⚡ Hot drop enabled
```

### What happens when you save a file

| Change type | What hot drop does |
|-------------|-------------------|
| Class name changed | CSS recompiled → browser refreshes styles only, no reload, state preserved |
| Page content changed | Route recompiled → current route reloads |
| `app.klx` or `main.klx` changed | Full browser reload |
| Syntax error | Error overlay shown in browser |
| Error fixed | Overlay disappears, page continues |

### Change the port
```json
// korlix.config.json
{ "server": { "port": 4000 } }
```

---

## 20. Configuration Reference

Full `korlix.config.json`:
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
    "port": 3000,
    "host": "0.0.0.0"
  },
  "budget": {
    "runtime": "20kb",
    "css": "80kb",
    "page": "150kb"
  }
}
```

---

## 21. CLI Reference

| Command | Description |
|---------|-------------|
| `korlix new <name>` | Create a new Korlix project |
| `korlix dev` | Start dev server with hot drop on `localhost:3000` |
| `korlix build` | Build for production (static mode) |
| `korlix build --mode spa` | Build as Single Page App |
| `korlix build --mode ssg` | Pre-render all routes (coming v0.2) |
| `korlix check` | Lint + validate all `.klx` files |
| `korlix check --ast` | Print AST as JSON |
| `korlix check --a11y` | Accessibility checks only |
| `korlix check --seo` | SEO checks only |
| `korlix check --security` | Security checks only |
| `korlix preview` | Serve the `dist/` folder locally |
| `korlix preview --port 4173` | Preview on custom port |

---

## 22. Error Codes

| Code | Type | Description |
|------|------|-------------|
| `KX-E001` | Error | Unexpected token — syntax error |
| `KX-E002` | Error | Missing path after `from` in import |
| `KX-E010` | Error | Unknown component used |
| `KX-E011` | Error | Required prop missing (e.g. `alt` on image) |
| `KX-E012` | Error | Wrong prop type |
| `KX-E020` | Error | Unknown route |
| `KX-E021` | Error | Duplicate route |
| `KX-E201` | Warning | Unknown utility class (with suggestions) |
| `KX-S101` | Error | Unsafe raw HTML without sanitize() |
| `KX-S102` | Error | `javascript:` URL blocked |
| `KX-A011` | Warning | Image missing `alt` text |
| `KX-A021` | Warning | Button missing accessible label |
| `KX-A051` | Warning | Heading order skipped |
| `KX-SEO001` | Warning | Page missing `meta: title` |
| `KX-SEO002` | Warning | Page missing `meta: description` |
| `KX-P001` | Warning | CSS bundle exceeds size budget |

---

*Korlix v0.1.0 — Phase 1 · MIT License*
