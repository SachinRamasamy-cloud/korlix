# Korlix V2 Component Catalog

The component registry contains more than 100 names. Components lower to semantic HTML during compilation and use shared component CSS and runtime modules.

## Main groups

### Navigation

`navbar`, `sidebar`, `nav`, `nav-item`, `nav-group`, `breadcrumb`, `tabs`, `tab`, `stepper`, `step`, `command-menu`, `mobile-menu`, `bottom-navigation`

### Content and cards

`card`, `profile-card`, `product-card`, `stat-card`, `pricing-card`, `article-card`, `testimonial-card`, `feature-card`, `image-card`, `timeline`, `timeline-item`, `accordion`, `accordion-item`, `list-group`, `list-item`, `avatar`, `avatar-group`, `badge`, `tag`, `chip`

### Forms

`input`, `text-field`, `number-field`, `password-field`, `email-field`, `search-box`, `textarea`, `textarea-field`, `select`, `select-field`, `multi-select`, `checkbox`, `checkbox-group`, `radio`, `radio-group`, `switch`, `slider`, `range-slider`, `date-picker`, `time-picker`, `file-upload`, `dropzone`, `color-picker`, `otp-input`, `form-group`, `field-error`

### Feedback and overlays

`alert`, `toast`, `notification`, `progress`, `progress-bar`, `progress-circle`, `spinner`, `skeleton`, `skeleton-grid`, `empty-state`, `result-state`, `status-indicator`, `modal`, `drawer`, `popover`, `tooltip`, `dropdown`, `context-menu`, `confirm-dialog`, `lightbox`, `sheet`

### Data and media

`table`, `data-table`, `pagination`, `data-grid`, `tree-view`, `calendar`, `chart-container`, `metric`, `key-value-list`, `code-block`, `carousel`, `gallery`, `image-viewer`, `video-player`, `audio-player`, `map-container`

### Layout

`container`, `row`, `column`, `grid`, `stack`, `split`, `divider`, `spacer`, `scroll-area`

## Variants

```klx
button "Save" variant=primary size=md
card variant=raised
navbar variant=glass
alert variant=danger
```

Generic V2 components accept `variant`, `size` and `disabled`. Specialized components expose additional props. The registry is schema-driven so future validation and editor completion can read the same definitions.
