# Colors & Utility Classes

## Overview

Korlix has a JIT utility class engine. Only the classes you actually use are included in your final CSS — no dead code.

## Color System

### Semantic Colors

Use these in your design for consistent theming:

```klx
h1 .text-primary "Hello"           # Primary brand color
p .text-muted "Subtitle"           # Muted text
div .bg-surface "Card"             # Card background
div .bg-background "Page"          # Page background
div .border .border-border         # Standard border
btn .bg-success "Done"             # Success state
btn .bg-danger "Delete"            # Danger state
```

All semantic colors: `primary`, `primary-light`, `primary-dark`, `secondary`, `accent`, `success`, `danger`, `warning`, `info`, `muted`, `surface`, `background`, `foreground`, `border`, `dark`, `light`, `white`, `black`, `transparent`

### Color Palette (50–950)

Full Tailwind-compatible color palette:

```klx
div .bg-blue-500        # Standard blue
div .bg-slate-900       # Dark slate
div .text-emerald-400   # Emerald text
div .border-purple-300  # Purple border
```

Available palettes: `slate`, `gray`, `zinc`, `neutral`, `stone`, `red`, `orange`, `amber`, `yellow`, `lime`, `green`, `emerald`, `teal`, `cyan`, `sky`, `blue`, `indigo`, `violet`, `purple`, `fuchsia`, `pink`, `rose`

### Color Utility Families

For every color token, these utilities are generated:

```text
.text-{color}         Color the text
.bg-{color}           Set background color
.border-{color}       Set border color
.ring-{color}         Set focus ring color
.fill-{color}         SVG fill
.stroke-{color}       SVG stroke
.outline-{color}      Outline color
.caret-{color}        Text caret color
.placeholder-{color}  Placeholder text color
```

## Variants

```klx
# Responsive
div .sm:hidden .md:flex .lg:grid-cols-3

# State
btn .hover:bg-primary-dark .focus:ring .disabled:opacity-50

# Dark mode
div .dark:bg-slate-900 .light:bg-white

# Group
div .group:
  icon .group-hover:text-primary
```

Available variants: `sm:`, `md:`, `lg:`, `xl:`, `2xl:`, `hover:`, `focus:`, `active:`, `disabled:`, `checked:`, `dark:`, `light:`, `group-hover:`, `peer-checked:`, `data-open:`, `motion-safe:`, `print:`

## Arbitrary Values

Use any valid CSS value:

```klx
div .w-[432px] .h-[calc(100vh-4rem)]
div .bg-[#0f1c24] .text-[#f1f5f9]
div .grid-cols-[240px_1fr_300px]
div .mt-[clamp(1rem,5vw,3rem)]
```

## Common Utilities Quick Reference

### Layout
```klx
.flex .inline-flex .grid .hidden .block .inline-block .contents
.relative .absolute .fixed .sticky .static
.flex-row .flex-col .flex-wrap
.items-start .items-center .items-end .items-stretch
.justify-start .justify-center .justify-between .justify-around
.gap-4 .gap-x-6 .gap-y-2
.grid-cols-3 .grid-cols-12 .col-span-2
```

### Spacing
```klx
.p-4 .px-6 .py-3 .pt-8 .pr-4 .pb-4 .pl-4
.m-auto .mx-auto .my-8 .mt-4 .mr-2 .mb-4 .ml-2
.space-x-4 .space-y-2
```

### Typography
```klx
.text-xs .text-sm .text-base .text-lg .text-xl .text-2xl .text-3xl
.text-4xl .text-5xl .text-6xl .text-7xl .text-8xl .text-9xl
.font-thin .font-light .font-normal .font-medium
.font-semibold .font-bold .font-extrabold .font-black
.text-left .text-center .text-right .text-justify
.uppercase .lowercase .capitalize .italic .underline .line-through
.leading-tight .leading-normal .leading-relaxed .leading-loose
.tracking-tight .tracking-normal .tracking-wide .tracking-wider
.truncate .whitespace-nowrap
```

### Sizing
```klx
.w-full .w-screen .w-auto .w-1/2 .w-1/3 .w-1/4 .w-3/4
.h-full .h-screen .h-auto
.min-h-screen .min-h-full
.max-w-sm .max-w-md .max-w-lg .max-w-xl .max-w-2xl
.max-w-3xl .max-w-4xl .max-w-5xl .max-w-6xl .max-w-7xl
```

### Effects
```klx
.rounded .rounded-sm .rounded-md .rounded-lg .rounded-xl
.rounded-2xl .rounded-3xl .rounded-full
.shadow .shadow-sm .shadow-md .shadow-lg .shadow-xl .shadow-2xl
.opacity-0 .opacity-25 .opacity-50 .opacity-75 .opacity-100
.blur .blur-sm .blur-md .blur-lg .blur-xl
.transition .transition-colors .transition-transform
.duration-150 .duration-200 .duration-300
.scale-95 .scale-100 .scale-105 .scale-110
.rotate-0 .rotate-3 .rotate-6 .rotate-12 .rotate-45 .rotate-90
```
