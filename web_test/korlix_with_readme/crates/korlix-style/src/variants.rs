use crate::registry::CssRule;
use crate::tokens::BREAKPOINTS;

/// State variant prefixes and their CSS pseudo-class/attribute.
pub const STATE_VARIANTS: &[(&str, &str)] = &[
    ("hover", ":hover"),
    ("focus", ":focus"),
    ("focus-within", ":focus-within"),
    ("focus-visible", ":focus-visible"),
    ("active", ":active"),
    ("visited", ":visited"),
    ("disabled", ":disabled"),
    ("checked", ":checked"),
    ("indeterminate", ":indeterminate"),
    ("placeholder", "::placeholder"),
    ("first", ":first-child"),
    ("last", ":last-child"),
    ("odd", ":nth-child(odd)"),
    ("even", ":nth-child(even)"),
    ("required", ":required"),
    ("invalid", ":invalid"),
    ("valid", ":valid"),
    ("read-only", ":read-only"),
    ("empty", ":empty"),
    ("group-hover", ".group:hover "),
    ("peer-checked", ".peer:checked ~ "),
    ("dark", ".dark "),
    ("data-open", "[data-open] "),
    ("data-active", "[data-active] "),
    (
        "motion-safe",
        "@media (prefers-reduced-motion: no-preference)",
    ),
    ("motion-reduce", "@media (prefers-reduced-motion: reduce)"),
    ("print", "@media print"),
];

/// Parse a class string like "hover:bg-blue-500" or "sm:flex" into
/// (variant_prefix, base_class).
pub fn parse_variant(class: &str) -> Option<(&str, &str)> {
    let mut bracket_depth = 0usize;
    let mut separator = None;

    for (idx, ch) in class.char_indices() {
        match ch {
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            ':' if bracket_depth == 0 => separator = Some(idx),
            _ => {}
        }
    }

    let idx = separator?;
    Some((&class[..idx], &class[idx + 1..]))
}

/// Check if a prefix is a responsive breakpoint.
pub fn is_breakpoint(prefix: &str) -> bool {
    BREAKPOINTS.iter().any(|(bp, _)| *bp == prefix)
}

/// Build the media query string for a breakpoint prefix.
pub fn breakpoint_media(prefix: &str) -> Option<String> {
    BREAKPOINTS
        .iter()
        .find(|(bp, _)| *bp == prefix)
        .map(|(_, w)| format!("@media (min-width:{}px)", w))
}

/// Apply a variant to an existing CssRule, modifying selector or wrapping in @media.
pub fn apply_variant(prefix: &str, base_class: &str, mut rule: CssRule) -> CssRule {
    if is_breakpoint(prefix) {
        rule.selector = format!("{}:{}", prefix, base_class);
        rule.media_query = breakpoint_media(prefix);
        return rule;
    }

    let escaped_variant_class = format!("{}\\:{}", prefix, base_class);
    match prefix {
        "dark" => {
            rule.selector = format!("[data-kx-theme=\"dark\"] .kx-{}", escaped_variant_class);
            rule.selector_is_raw = true;
        }
        "group-hover" => {
            rule.selector = format!(".kx-group:hover .kx-{}", escaped_variant_class);
            rule.selector_is_raw = true;
        }
        "peer-checked" => {
            rule.selector = format!(".kx-peer:checked ~ .kx-{}", escaped_variant_class);
            rule.selector_is_raw = true;
        }
        "data-open" => {
            rule.selector = format!(".kx-{}[data-open]", escaped_variant_class);
            rule.selector_is_raw = true;
        }
        "data-active" => {
            rule.selector = format!(".kx-{}[data-active]", escaped_variant_class);
            rule.selector_is_raw = true;
        }
        _ => {
            for (variant, pseudo) in STATE_VARIANTS {
                if *variant != prefix {
                    continue;
                }
                if pseudo.starts_with("@media") {
                    rule.selector = format!("{}:{}", prefix, base_class);
                    rule.media_query = Some((*pseudo).into());
                } else {
                    rule.selector = format!("{}:{}{}", prefix, base_class, pseudo);
                }
                break;
            }
        }
    }
    rule
}
