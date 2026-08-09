# Colors & Utility Classes

Korlix includes a JIT utility-class engine.

Only the utility classes referenced by your application are emitted into the final stylesheet, which keeps generated CSS focused on the styles actually used by the project.

---

## Utility Generation

When Korlix compiles your application, it scans the source for utility classes such as:

```klx
div .flex .items-center .gap-4
h1 .text-3xl .font-bold .text-primary
btn .bg-primary .hover:bg-primary-dark
```

Only the detected utilities are included in the generated CSS.

Conceptually:

```text
Korlix source
    │
    ▼
Utility scan
    │
    ▼
Used classes detected
    │
    ▼
CSS generated
    │
    ▼
Unused utilities omitted
```

This avoids shipping a complete utility framework when only a subset of classes is required.

---

# Color System

Korlix provides two main ways to work with color:

1. Semantic color tokens
2. Palette-based color utilities

Use semantic colors for application-level design decisions and palette colors when you need a specific shade.

---

## Semantic Colors

Semantic colors describe the purpose of a color rather than a fixed shade.

Example:

```klx
h1 .text-primary "Hello"
p .text-muted "Subtitle"

div .bg-surface "Card"
div .bg-background "Page"

div .border .border-border

btn .bg-success "Done"
btn .bg-danger "Delete"
```

This makes theme changes easier because components reference semantic intent instead of hard-coded palette values.

For example:

```text
primary
```

represents the application's main brand color, while:

```text
danger
```

represents destructive or error-related actions.

---

## Available Semantic Colors

Korlix provides the following semantic color tokens:

```text
primary
primary-light
primary-dark

secondary
accent

success
danger
warning
info

muted

surface
background
foreground
border

dark
light

white
black
transparent
```

Typical usage:

```klx
btn .bg-primary .text-white "Continue"

p .text-muted "Optional information"

div .bg-surface .border .border-border:
  p .text-foreground "Card content"

btn .bg-danger .text-white "Delete"
```

---

## Semantic Colors vs Palette Colors

Use semantic tokens when the color represents a design role:

```klx
btn .bg-primary
p .text-muted
div .bg-surface
div .border-border
```

Use palette colors when you need a specific visual shade:

```klx
div .bg-blue-500
p .text-slate-600
div .border-emerald-300
```

A practical rule is:

```text
Application UI
     │
     ├── Design meaning
     │      └── Semantic colors
     │
     └── Exact visual shade
            └── Palette colors
```

---

# Color Palette

Korlix includes a Tailwind-compatible color palette using shade values from `50` through `950`.

Example:

```klx
div .bg-blue-500
div .bg-slate-900

p .text-emerald-400

div .border-purple-300
```

---

## Available Palettes

```text
slate
gray
zinc
neutral
stone

red
orange
amber
yellow

lime
green
emerald
teal

cyan
sky
blue
indigo

violet
purple
fuchsia
pink
rose
```

Each palette can be combined with supported shade values.

Example:

```klx
div .bg-slate-50
div .bg-slate-100
div .bg-slate-500
div .bg-slate-900
div .bg-slate-950
```

---

# Color Utility Families

Color tokens can be used across several utility families.

```text
.text-{color}
.bg-{color}
.border-{color}
.ring-{color}
.fill-{color}
.stroke-{color}
.outline-{color}
.caret-{color}
.placeholder-{color}
```

---

## Text Color

```klx
p .text-primary "Primary text"
p .text-slate-600 "Slate text"
p .text-emerald-500 "Success-related text"
```

Generated form:

```text
.text-{color}
```

---

## Background Color

```klx
div .bg-background
div .bg-surface
div .bg-blue-500
```

Generated form:

```text
.bg-{color}
```

---

## Border Color

```klx
div .border .border-border
div .border .border-slate-300
```

Generated form:

```text
.border-{color}
```

---

## Ring Color

```klx
input .focus:ring .ring-primary
```

Generated form:

```text
.ring-{color}
```

---

## SVG Fill and Stroke

```klx
icon .fill-primary
icon .stroke-slate-500
```

Generated forms:

```text
.fill-{color}
.stroke-{color}
```

---

## Outline Color

```klx
input .outline-primary
```

Generated form:

```text
.outline-{color}
```

---

## Caret Color

```klx
input .caret-primary
```

Generated form:

```text
.caret-{color}
```

---

## Placeholder Color

```klx
input .placeholder-muted
```

Generated form:

```text
.placeholder-{color}
```

---

# Variants

Variants apply a utility only under a particular condition.

Korlix supports responsive, interaction-state, theme, group, peer, data-state, motion, and print variants.

---

## Responsive Variants

Responsive variants apply utilities at specific breakpoints.

```klx
div .sm:hidden
div .md:flex
div .lg:grid-cols-3
```

Example:

```klx
div .grid .grid-cols-1 .md:grid-cols-2 .lg:grid-cols-3
```

The layout can therefore change as the viewport size increases.

---

## Interaction State Variants

Apply utilities in response to user interaction or component state.

```klx
btn .hover:bg-primary-dark
btn .focus:ring
btn .active:scale-95
btn .disabled:opacity-50
```

Example:

```klx
btn
  .bg-primary
  .hover:bg-primary-dark
  .focus:ring
  .disabled:opacity-50
  "Continue"
```

---

## Theme Variants

Use `dark:` and `light:` for theme-specific styles.

```klx
div .dark:bg-slate-900
div .light:bg-white
```

Example:

```klx
div
  .light:bg-white
  .dark:bg-slate-900
```

---

## Group Variants

A parent can expose its interaction state to descendants with `.group`.

```klx
div .group:
  icon .group-hover:text-primary
```

This allows a child element to react when the parent is hovered.

---

## Peer Variants

Peer variants allow one element's state to affect another element.

```klx
input .peer
div .peer-checked:block
```

---

## Data-State Variants

Korlix can apply utilities based on supported data states.

```klx
div .data-open:block
```

---

## Motion Variants

Use motion variants when a style or animation should only apply when motion is allowed.

```klx
div .motion-safe:transition
```

---

## Print Variant

Use `print:` for print-specific presentation.

```klx
nav .print:hidden
```

---

## Available Variants

```text
sm:
md:
lg:
xl:
2xl:

hover:
focus:
active:
disabled:
checked:

dark:
light:

group-hover:
peer-checked:
data-open:

motion-safe:
print:
```

Variants can be combined with compatible utilities.

---

# Arbitrary Values

When a predefined utility is not enough, Korlix supports arbitrary CSS values using square-bracket syntax.

---

## Width and Height

```klx
div .w-[432px]
div .h-[calc(100vh-4rem)]
```

---

## Arbitrary Colors

```klx
div .bg-[#0f1c24]
p .text-[#f1f5f9]
```

For application-wide colors, prefer semantic tokens where possible. Arbitrary colors are better suited to isolated or highly specific design requirements.

---

## Grid Definitions

```klx
div .grid-cols-[240px_1fr_300px]
```

This can be used for layouts such as:

```text
Sidebar        Main content        Inspector
240px              1fr               300px
```

---

## CSS Functions

Any supported CSS value can be used inside the brackets.

```klx
div .mt-[clamp(1rem,5vw,3rem)]
```

Other examples:

```klx
div .w-[min(90vw,1200px)]
div .top-[calc(100%-2rem)]
```

---

# Common Utility Reference

The following utilities cover common layout, spacing, typography, sizing, and visual-effect patterns.

---

## Layout

### Display

```klx
.flex
.inline-flex
.grid
.hidden
.block
.inline-block
.contents
```

### Position

```klx
.relative
.absolute
.fixed
.sticky
.static
```

### Flex Direction and Wrapping

```klx
.flex-row
.flex-col
.flex-wrap
```

### Alignment

```klx
.items-start
.items-center
.items-end
.items-stretch
```

### Distribution

```klx
.justify-start
.justify-center
.justify-between
.justify-around
```

### Gap

```klx
.gap-4
.gap-x-6
.gap-y-2
```

### Grid

```klx
.grid-cols-3
.grid-cols-12
.col-span-2
```

Example:

```klx
div .grid .grid-cols-12 .gap-6:
  div .col-span-2
  div .col-span-10
```

---

# Spacing

## Padding

```klx
.p-4

.px-6
.py-3

.pt-8
.pr-4
.pb-4
.pl-4
```

## Margin

```klx
.m-auto

.mx-auto
.my-8

.mt-4
.mr-2
.mb-4
.ml-2
```

## Sibling Spacing

```klx
.space-x-4
.space-y-2
```

Example:

```klx
div .flex .space-x-4
```

or:

```klx
div .flex .flex-col .space-y-2
```

---

# Typography

## Font Size

```klx
.text-xs
.text-sm
.text-base
.text-lg
.text-xl
.text-2xl
.text-3xl
.text-4xl
.text-5xl
.text-6xl
.text-7xl
.text-8xl
.text-9xl
```

---

## Font Weight

```klx
.font-thin
.font-light
.font-normal
.font-medium
.font-semibold
.font-bold
.font-extrabold
.font-black
```

---

## Text Alignment

```klx
.text-left
.text-center
.text-right
.text-justify
```

---

## Text Transformation and Decoration

```klx
.uppercase
.lowercase
.capitalize

.italic

.underline
.line-through
```

---

## Line Height

```klx
.leading-tight
.leading-normal
.leading-relaxed
.leading-loose
```

---

## Letter Spacing

```klx
.tracking-tight
.tracking-normal
.tracking-wide
.tracking-wider
```

---

## Overflow and Wrapping

```klx
.truncate
.whitespace-nowrap
```

---

# Sizing

## Width

```klx
.w-full
.w-screen
.w-auto

.w-1/2
.w-1/3
.w-1/4
.w-3/4
```

---

## Height

```klx
.h-full
.h-screen
.h-auto
```

---

## Minimum Height

```klx
.min-h-screen
.min-h-full
```

---

## Maximum Width

```klx
.max-w-sm
.max-w-md
.max-w-lg
.max-w-xl
.max-w-2xl
.max-w-3xl
.max-w-4xl
.max-w-5xl
.max-w-6xl
.max-w-7xl
```

Example:

```klx
main .max-w-7xl .mx-auto
```

---

# Visual Effects

## Border Radius

```klx
.rounded
.rounded-sm
.rounded-md
.rounded-lg
.rounded-xl
.rounded-2xl
.rounded-3xl
.rounded-full
```

---

## Shadows

```klx
.shadow
.shadow-sm
.shadow-md
.shadow-lg
.shadow-xl
.shadow-2xl
```

---

## Opacity

```klx
.opacity-0
.opacity-25
.opacity-50
.opacity-75
.opacity-100
```

---

## Blur

```klx
.blur
.blur-sm
.blur-md
.blur-lg
.blur-xl
```

---

# Transitions

```klx
.transition
.transition-colors
.transition-transform
```

Transition duration:

```klx
.duration-150
.duration-200
.duration-300
```

Example:

```klx
btn
  .transition-colors
  .duration-200
  .bg-primary
  .hover:bg-primary-dark
```

---

# Transforms

## Scale

```klx
.scale-95
.scale-100
.scale-105
.scale-110
```

Example:

```klx
btn .active:scale-95
```

---

## Rotation

```klx
.rotate-0
.rotate-3
.rotate-6
.rotate-12
.rotate-45
.rotate-90
```

---

# Combined Example

A typical card using semantic colors, layout utilities, responsive variants, and state utilities could look like:

```klx
div
  .bg-surface
  .border
  .border-border
  .rounded-lg
  .p-6
  .shadow-sm
  .transition
  .hover:shadow-md:

  h2
    .text-xl
    .font-semibold
    .text-foreground
    "Project"

  p
    .mt-2
    .text-sm
    .text-muted
    "Project description"

  btn
    .mt-4
    .bg-primary
    .text-white
    .rounded-md
    .px-4
    .py-2
    .transition-colors
    .duration-200
    .hover:bg-primary-dark
    .disabled:opacity-50
    "Open Project"
```

For responsive layouts:

```klx
div
  .grid
  .grid-cols-1
  .gap-4
  .md:grid-cols-2
  .lg:grid-cols-3
```

---

# Recommended Usage

Prefer semantic utilities for reusable application components:

```klx
.bg-background
.bg-surface
.text-foreground
.text-muted
.border-border
.bg-primary
.bg-danger
```

Use palette colors when a design explicitly requires a specific shade:

```klx
.bg-slate-900
.text-blue-500
.border-emerald-300
```

Use arbitrary values only when the normal utility system cannot express the required value cleanly:

```klx
.w-[432px]
.grid-cols-[240px_1fr]
.bg-[#0f1c24]
```

This keeps application styling consistent while still allowing precise control when required.

---

# Summary

Korlix styling follows this model:

```text
Korlix Utility System
        │
        ├── Semantic colors
        │      ├── primary
        │      ├── surface
        │      ├── foreground
        │      └── danger
        │
        ├── Palette colors
        │      ├── slate-500
        │      ├── blue-600
        │      └── emerald-400
        │
        ├── Utility classes
        │      ├── layout
        │      ├── spacing
        │      ├── typography
        │      ├── sizing
        │      └── effects
        │
        ├── Variants
        │      ├── responsive
        │      ├── state
        │      ├── theme
        │      └── data/group/peer
        │
        └── Arbitrary values
               └── [...]
```

The JIT compiler emits only the utility classes used by the application, while semantic tokens, palette utilities, variants, and arbitrary values provide different levels of styling control.
