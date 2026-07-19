# Component System

## Built-In Components

Korlix ships a complete component registry. Every component is expanded at compile time into clean HTML — no heavy JS component system in the browser.

## Primitives

```klx
# Button
btn .primary "Save" on:click:
  saveData()

btn .ghost "Cancel"
btn .danger "Delete"
btn .secondary "Export"
btn .primary loading=true "Saving..."
btn disabled=true "Not Available"

# Link (SPA-aware)
link href="/about" "About"
link href="https://example.com" external=true "External"

# Icon
icon name="user" .size-6 .text-primary label="User profile"
icon name="star" .size-4 decorative=true

# Image
image src="/hero.jpg" alt="Hero image" .rounded-xl .w-full lazy=true
image src="/thumb.jpg" alt="Thumb" width=400 height=300

# Container / Section
container size="lg":
  h1 "Centered content"

section .py-20 .bg-surface:
  h2 "Section"
```

## Avatar & Profile

```klx
# Avatar with image
avatar src="/profile.jpg" name="Sachin" size="lg"

# Avatar with initials fallback (no src)
avatar name="John Doe" size="md"

# Avatar with status
avatar src="/pic.jpg" name="Alice" status="online"

# Profile card
profile-card name="Sachin" avatar="/profile.jpg" role="Full-Stack Developer":
  slot:actions:
    btn .primary "Follow"
    btn .ghost "Message"
```

## Navigation

```klx
# Navbar
navbar .sticky .top-0:
  div .flex .items-center .justify-between .max-w-7xl .mx-auto .px-6 .py-4:
    link href="/" .font-bold .text-xl "My Site"
    div .flex .gap-6:
      link href="/"       "Home"
      link href="/about"  "About"
      link href="/contact" "Contact"

# Pagination
pagination page=1 total=100 perPage=10 on:change:
  loadPage(page)

# Breadcrumb
breadcrumb items=[
  { label: "Home", href: "/" },
  { label: "Products", href: "/products" },
  { label: "Item #42" }
]

# Tabs
tabs active="overview":
  div data-tab="overview":
    p "Overview content"
  div data-tab="specs":
    p "Specs content"
```

## Feedback

```klx
# Toast (triggered from event)
btn "Save" on:click:
  toast success "Saved successfully"
  toast error "Failed to save"
  toast warning "Check your input"
  toast info "Loading data..."

# Alert (inline)
alert type="success" title="Done" "Your file was uploaded."
alert type="danger" dismissible=true "Something went wrong."

# Badge
badge .primary "New"
badge .success "Active"
badge .danger "Urgent"
```

## Loaders & Placeholders

```klx
# Spinner
spinner size="lg"
spinner size="sm" color="primary"

# Skeleton
skeleton width="100%" height="2rem"
skeleton width="60%" height="1rem"

# Skeleton card (for lists)
skeleton-card count=6 lines=3

# Empty state
empty-state icon="inbox" title="No messages" description="Your inbox is empty"

# Progress bar
progress value=65 max=100 label=true
```

## Overlay

```klx
# Modal
btn "Open Modal" on:click:
  openModal("confirm")

modal id="confirm" title="Confirm Action":
  p "Are you sure you want to delete this item?"
  slot:footer:
    btn .danger "Delete" on:click:
      closeModal("confirm")
    btn .ghost "Cancel" on:click:
      closeModal("confirm")

# Drawer
btn "Open Drawer" on:click:
  openDrawer("settings")

drawer id="settings" title="Settings" side="right":
  p "Drawer content here"

# Tooltip
tooltip content="This is a helpful tip" placement="top":
  icon name="info" .size-4
```

## Forms

```klx
form on:submit:
  submitForm()

  form-field:
    label "Email address"
    input type="email" placeholder="you@example.com"
    form-help "We'll never share your email."

  form-field:
    label "Password"
    input type="password" placeholder="••••••••"

  form-field:
    checkbox label="Remember me" checked=false

  form-field:
    switch label="Enable notifications" checked=true

  form-field:
    label "Country"
    select options=[
      { label: "India", value: "IN" },
      { label: "USA", value: "US" }
    ]

  btn .primary type="submit" "Sign in"
```

## Data Display

```klx
# Table
table striped=true hoverable=true:
  thead:
    tr:
      th "Name"
      th "Email"
      th "Status"
  tbody:
    for user in users:
      tr:
        td user.name
        td user.email
        td:
          badge user.status

# Accordion
accordion:
  div data-title="What is Korlix?":
    p "Korlix is a frontend-first programming language..."
  div data-title="How does it compare to React?":
    p "Korlix compiles to native HTML/CSS/JS..."
```

## Marketing

```klx
hero variant="centered" size="xl":
  h1 .text-7xl .font-black "Build Faster"
  p .text-xl .text-muted "The frontend language that gets out of your way."
  slot:actions:
    btn .primary "Get Started →"
    btn .ghost "View Demo"

# Feature grid
div .grid .grid-cols-3 .gap-8:
  for feature in features:
    card .p-6:
      icon name=feature.icon .size-8 .text-primary .mb-4
      h3 .font-semibold .mb-2 feature.title
      p .text-muted feature.description
```

## Component Props Reference

Every component accepts these universal props:

```klx
component-name
  .class-name          # Utility classes
  prop-name=value      # Component-specific props
  on:click:            # Event handler
    action()
```
