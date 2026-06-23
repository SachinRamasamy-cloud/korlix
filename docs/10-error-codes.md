# Error Code Reference

## Compiler Errors (KX-E###)

| Code | Description | Example |
|------|-------------|---------|
| KX-E001 | Unexpected token | `Expected \`:\ `, found \`div\`` |
| KX-E002 | Missing string path after `from` | `import Foo from` ← missing path |
| KX-E003 | Unknown identifier | `navigate(undefined_var)` |
| KX-E010 | Undefined component | `my-widget` not in registry |
| KX-E011 | Missing required prop | `image` without `alt` |
| KX-E012 | Wrong prop type | `count="five"` when `int` expected |
| KX-E020 | Unknown route | `page "/foo/:id" from "./pages/foo.klx"` file missing |
| KX-E021 | Duplicate route | Two pages with same route |
| KX-E030 | Invalid state assignment | `count = "hello"` when `int` |
| KX-E040 | Circular import | File A imports B which imports A |
| KX-E050 | Empty page body | Page with no content |
| KX-E060 | Invalid expression | Syntax error in expression |

## Style Warnings (KX-E2##)

| Code | Description | Suggestion |
|------|-------------|------------|
| KX-E201 | Unknown utility class | `.bg-blu` → Did you mean `.bg-blue-500`? |
| KX-E202 | Unknown variant prefix | `.xl2:flex` → Did you mean `.2xl:flex`? |
| KX-E203 | Unsafe arbitrary value | `.bg-[javascript:...]` blocked |

## Security Warnings (KX-S###)

| Code | Severity | Description |
|------|----------|-------------|
| KX-S101 | Error | Unsafe raw HTML without sanitize() |
| KX-S102 | Error | `javascript:` URL detected |
| KX-S103 | Warning | External script without SRI hash |
| KX-S104 | Warning | `target="_blank"` without `rel="noopener noreferrer"` |
| KX-S105 | Warning | iframe without sandbox attribute |
| KX-S106 | Warning | Inline script usage |
| KX-S107 | Error | DOM clobbering risk |

## Accessibility Warnings (KX-A###)

| Code | Severity | Description |
|------|----------|-------------|
| KX-A011 | Warning | Image missing `alt` text |
| KX-A012 | Error | `alt=""` on non-decorative image |
| KX-A021 | Warning | Button has no accessible label |
| KX-A022 | Warning | Icon missing `label` and not `decorative=true` |
| KX-A031 | Warning | Form field missing associated label |
| KX-A041 | Error | Modal missing focus trap |
| KX-A051 | Warning | Heading order skipped (h1 → h3) |
| KX-A061 | Info | Low color contrast ratio (< 4.5:1) |
| KX-A071 | Warning | Custom interactive element missing ARIA role |
| KX-A081 | Info | Animation without `motion-reduce` variant |

## SEO Warnings (KX-SEO###)

| Code | Severity | Description |
|------|----------|-------------|
| KX-SEO001 | Warning | Page missing `meta: title` |
| KX-SEO002 | Warning | Page missing `meta: description` |
| KX-SEO003 | Info | Description longer than 160 characters |
| KX-SEO004 | Warning | Page missing Open Graph image |
| KX-SEO005 | Info | Multiple `h1` elements on page |

## Performance Warnings (KX-P###)

| Code | Severity | Description |
|------|----------|-------------|
| KX-P001 | Warning | CSS bundle exceeds budget |
| KX-P002 | Warning | JS runtime exceeds budget |
| KX-P003 | Warning | Page exceeds size budget |
| KX-P004 | Info | Image missing explicit width/height (layout shift risk) |
| KX-P005 | Info | Large image without lazy loading |

## Reading Error Output

```
error [KX-E201]: Unknown utility class `.bg-blu`
  → src/pages/index.klx:14:5
  hint: Did you mean: bg-blue-500, bg-blue-400, bg-blue-600?
```

Fix the class name and re-run `korlix check` or `korlix build`.
