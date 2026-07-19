# HTML Support

Korlix recognizes modern HTML elements directly. Native elements compile without component wrappers.

```klx
article
  header
    h1 "Article title"
    time datetime="2026-07-16" "16 July 2026"
  p "Article content"
  figure
    img src="/photo.jpg" alt="Road traffic"
    figcaption "Traffic observation"
```

## Supported groups

- Document and metadata: `html`, `head`, `body`, `title`, `base`, `link`, `meta`, `style`
- Sections: `header`, `footer`, `main`, `nav`, `section`, `article`, `aside`, `address`, `search`
- Text: headings, `p`, `div`, `span`, `pre`, `blockquote`, `figure`, `figcaption`, `code`, `kbd`, `time`, `mark`, `strong`, `em` and related inline tags
- Lists: `ul`, `ol`, `li`, `dl`, `dt`, `dd`, `menu`
- Forms: `form`, `label`, `input`, `textarea`, `button`, `select`, `option`, `optgroup`, `fieldset`, `legend`, `datalist`, `output`, `progress`, `meter`
- Tables: `table`, `caption`, `thead`, `tbody`, `tfoot`, `tr`, `th`, `td`, `colgroup`, `col`
- Media: `img`, `picture`, `source`, `audio`, `video`, `track`, `map`, `area`, `canvas`, `svg`, `iframe`, `embed`, `object`
- Interactive and templates: `details`, `summary`, `dialog`, `template`, `noscript`, `script`, `slot`
- Common SVG primitives are accepted for inline graphics.

Boolean attributes can be written without `=true`:

```klx
input type=email required autofocus
video src="/demo.mp4" controls muted playsinline
```

Void elements such as `img`, `input`, `meta`, `source`, `track`, `br` and `hr` are emitted without closing tags.
