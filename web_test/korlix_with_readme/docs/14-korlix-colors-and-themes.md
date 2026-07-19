# Korlix Colors and Themes

Korlix V2 adds its own color vocabulary while retaining V1 utility classes for compatibility.

## Native color classes

```klx
card .surface-blue-1 .content-blue-10 .outline-blue-3
button .surface-violet-7 .content-white
```

The native prefixes are:

- `surface-*` — background color
- `content-*` — text color
- `outline-*` — border color
- `accent-*` — component accent variable
- `fill-*`, `stroke-*` — SVG color
- `ring-color-*` — focus-ring color
- `caret-color-*` — input caret color

Color families include neutral aliases and chromatic palettes such as slate, gray, zinc, red, orange, amber, yellow, lime, green, emerald, mint, teal, cyan, sky, blue, indigo, violet, purple, magenta, pink, rose and coral. Numeric levels run from `0` (light endpoint) through `12` (dark endpoint). Arbitrary values are also supported, for example `.surface-[#17324d]` and `.content-[oklch(72%_0.18_260)]`.

## Semantic colors

Semantic classes automatically follow the active theme:

```klx
section .surface-canvas .content-content
card .surface-raised .outline-outline
p .content-content-muted
button .surface-brand
```

## Light, dark and automatic mode

```klx
app MyApp
  theme auto
```

Allowed modes are `light`, `dark` and `auto`. Add a built-in switch with:

```klx
theme-toggle
```

The runtime stores a manual selection in `localStorage`, follows `prefers-color-scheme` in automatic mode and applies `data-kx-theme` before page rendering to reduce theme flash.

Dark-only variants use the existing variant form:

```klx
card .surface-white .dark:surface-slate-10
```
