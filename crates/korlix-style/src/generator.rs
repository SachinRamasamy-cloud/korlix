use crate::{
    colors::{build_css_vars, build_palette},
    registry::{lookup, CssRule, UTILITY_REGISTRY},
    variants::{apply_variant, parse_variant},
};
use std::collections::HashSet;

/// Generate only the CSS rules needed for the given set of class names.
pub fn generate_css(classes: &HashSet<String>) -> String {
    let palette = build_palette();
    let mut css = String::new();

    // CSS reset + base styles
    css.push_str(BASE_CSS);

    // CSS variables (design tokens)
    css.push_str(&build_css_vars(&palette));

    // Component / animation CSS
    css.push_str(COMPONENT_CSS);

    // JIT: only include rules for used classes
    let mut rules: Vec<CssRule> = vec![];

    for class in classes {
        if let Some(rule) = resolve_class(class) {
            rules.push(rule);
        }
    }

    // Sort for determinism
    rules.sort_by(|a, b| a.selector.cmp(&b.selector));

    // Group media queries to the end
    let (plain, media): (Vec<_>, Vec<_>) = rules.into_iter().partition(|r| r.media_query.is_none());

    for rule in plain {
        css.push_str(&rule.to_css());
        css.push('\n');
    }

    // Group media rules
    let mut mq_groups: std::collections::HashMap<String, Vec<CssRule>> = Default::default();
    for rule in media {
        mq_groups
            .entry(rule.media_query.clone().unwrap())
            .or_default()
            .push(rule);
    }
    let mut mq_keys: Vec<_> = mq_groups.keys().cloned().collect();
    mq_keys.sort();
    for mq in mq_keys {
        css.push_str(&format!("{}{{\n", mq));
        for rule in &mq_groups[&mq] {
            css.push_str(&rule.to_css());
            css.push('\n');
        }
        css.push_str("}\n");
    }

    css
}

fn resolve_class(class: &str) -> Option<CssRule> {
    // Try direct lookup first
    if let Some(rule) = lookup(class) {
        return Some(rule.clone());
    }

    // Try variant: hover:bg-blue-500 → base = bg-blue-500, variant = hover
    if let Some((prefix, base)) = parse_variant(class) {
        if let Some(base_rule) = lookup(base) {
            let rule = base_rule.clone();
            return Some(apply_variant(prefix, base, rule));
        }
    }

    // Arbitrary value: w-[320px], h-[calc(100vh-4rem)], bg-[#0f1c24]
    if let Some(rule) = resolve_arbitrary(class) {
        return Some(rule);
    }

    None
}

fn resolve_arbitrary(class: &str) -> Option<CssRule> {
    let bracket_start = class.find('[')?;
    let bracket_end = class.rfind(']')?;
    if bracket_end <= bracket_start {
        return None;
    }

    let prefix = &class[..bracket_start];
    let value = &class[bracket_start + 1..bracket_end];

    // Safety: block JS and expressions
    if value.contains("javascript:") || value.contains("expression(") {
        return None;
    }

    // Store formatted strings for translate variants to avoid lifetime issues
    let translate_x_val;
    let translate_y_val;

    let (prop, val): (&str, &str) = match prefix.trim_end_matches('-') {
        "w" => ("width", value),
        "h" => ("height", value),
        "min-w" => ("min-width", value),
        "min-h" => ("min-height", value),
        "max-w" => ("max-width", value),
        "max-h" => ("max-height", value),
        "p" => ("padding", value),
        "px" => ("padding-inline", value),
        "py" => ("padding-block", value),
        "m" => ("margin", value),
        "mx" => ("margin-inline", value),
        "my" => ("margin-block", value),
        "gap" => ("gap", value),
        "text" => ("font-size", value),
        "bg" => ("background-color", value),
        "border" => ("border-color", value),
        "top" => ("top", value),
        "right" => ("right", value),
        "bottom" => ("bottom", value),
        "left" => ("left", value),
        "grid-cols" => ("grid-template-columns", value),
        "z" => ("z-index", value),
        "opacity" => ("opacity", value),
        "translate-x" => {
            translate_x_val = format!("translateX({})", value);
            ("transform", &translate_x_val)
        }
        "translate-y" => {
            translate_y_val = format!("translateY({})", value);
            ("transform", &translate_y_val)
        }
        _ => return None,
    };

    Some(CssRule::new(class).prop(prop, val))
}

/// Generate a full (all classes) CSS for preview / docs.
pub fn generate_all_css() -> String {
    let all_classes: HashSet<String> = UTILITY_REGISTRY.keys().cloned().collect();
    generate_css(&all_classes)
}

const BASE_CSS: &str = r#"
/* Korlix Base Reset */
*, *::before, *::after { box-sizing: border-box; }
html { line-height: 1.5; -webkit-text-size-adjust: 100%; font-family: ui-sans-serif,system-ui,-apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,sans-serif; }
body { margin: 0; padding: 0; }
img, video { max-width: 100%; height: auto; display: block; }
button, [type='button'], [type='reset'], [type='submit'] { -webkit-appearance: button; background-color: transparent; background-image: none; }
input, optgroup, select, textarea { font-family: inherit; font-size: 100%; }
[hidden] { display: none !important; }
#korlix-root { min-height: 100vh; }
.kx-truncate { overflow:hidden; text-overflow:ellipsis; white-space:nowrap; }
"#;

const COMPONENT_CSS: &str = r#"
/* Korlix Toast */
.kx-toast-root { position:fixed; z-index:9999; pointer-events:none; padding:1rem; }
.kx-toast-root.top-right  { top:0; right:0; }
.kx-toast-root.top-left   { top:0; left:0; }
.kx-toast-root.top-center { top:0; left:50%; transform:translateX(-50%); }
.kx-toast-root.bottom-right  { bottom:0; right:0; }
.kx-toast-root.bottom-left   { bottom:0; left:0; }
.kx-toast-root.bottom-center { bottom:0; left:50%; transform:translateX(-50%); }
.kx-toast { display:flex; align-items:center; gap:.75rem; padding:.75rem 1rem; border-radius:.5rem;
  font-size:.875rem; line-height:1.25rem; box-shadow:0 4px 12px rgba(0,0,0,.15);
  pointer-events:all; min-width:240px; max-width:400px; margin-bottom:.5rem;
  animation:kx-slide-in .2s ease; }
.kx-toast.success { background:#166534; color:#dcfce7; }
.kx-toast.error   { background:#991b1b; color:#fee2e2; }
.kx-toast.warning { background:#92400e; color:#fef3c7; }
.kx-toast.info    { background:#1e3a8a; color:#dbeafe; }
.kx-toast.loading { background:#1e1e2e; color:#f1f5f9; }
.kx-toast-close   { margin-left:auto; cursor:pointer; opacity:.7; background:none; border:none;
  color:inherit; font-size:1.1rem; padding:0; line-height:1; }
.kx-toast-close:hover { opacity:1; }

/* Korlix Modal */
.kx-modal-backdrop { position:fixed; inset:0; background:rgba(0,0,0,.6); z-index:1000;
  display:flex; align-items:center; justify-content:center; backdrop-filter:blur(2px); }
.kx-modal { background:var(--kx-surface,#1e1e2e); border-radius:.75rem; padding:1.5rem;
  min-width:360px; max-width:90vw; max-height:90vh; overflow-y:auto;
  box-shadow:0 20px 60px rgba(0,0,0,.5); animation:kx-fade-scale-in .2s ease; }
.kx-modal-header { display:flex; align-items:center; justify-content:space-between; margin-bottom:1rem; }
.kx-modal-close  { background:none; border:none; cursor:pointer; opacity:.7; font-size:1.5rem;
  color:var(--kx-foreground,#f1f5f9); line-height:1; }
.kx-modal-close:hover { opacity:1; }

/* Korlix Drawer */
.kx-drawer-backdrop { position:fixed; inset:0; background:rgba(0,0,0,.5); z-index:1000; }
.kx-drawer { position:fixed; top:0; bottom:0; background:var(--kx-surface,#1e1e2e);
  overflow-y:auto; z-index:1001; transition:transform .3s ease; }
.kx-drawer.right { right:0; width:380px; transform:translateX(100%); }
.kx-drawer.left  { left:0;  width:380px; transform:translateX(-100%); }
.kx-drawer.open  { transform:translateX(0); }

/* Korlix Skeleton */
@keyframes kx-shimmer { 0%{background-position:-200% 0} 100%{background-position:200% 0} }
.kx-skeleton { background:linear-gradient(90deg,#2d2d3d 25%,#3d3d4d 50%,#2d2d3d 75%);
  background-size:200% 100%; animation:kx-shimmer 1.5s infinite; border-radius:.375rem; }

/* Korlix Spinner */
@keyframes kx-spin { to{transform:rotate(360deg)} }
.kx-spinner { display:inline-block; width:1.5rem; height:1.5rem; border:3px solid rgba(255,255,255,.2);
  border-top-color:var(--kx-primary,#6366f1); border-radius:50%; animation:kx-spin .7s linear infinite; }

/* Korlix Animations */
@keyframes kx-slide-in   { from{opacity:0;transform:translateX(1rem)} to{opacity:1;transform:translateX(0)} }
@keyframes kx-fade-scale-in { from{opacity:0;transform:scale(.95)} to{opacity:1;transform:scale(1)} }
@keyframes kx-fade-in    { from{opacity:0} to{opacity:1} }
@keyframes kx-slide-up   { from{opacity:0;transform:translateY(.5rem)} to{opacity:1;transform:translateY(0)} }
@keyframes kx-bounce     { 0%,100%{transform:translateY(0)} 50%{transform:translateY(-.25rem)} }
.kx-fade-in  { animation:kx-fade-in .3s ease; }
.kx-slide-up { animation:kx-slide-up .3s ease; }
.kx-bounce   { animation:kx-bounce 1s ease infinite; }

/* Dark mode */
.dark { color-scheme: dark; }
.dark body { background:var(--kx-background,#0f0f1a); color:var(--kx-foreground,#f1f5f9); }

/* Pagination */
.kx-pagination { display:flex; align-items:center; gap:.5rem; }
.kx-pagination button { min-width:2.25rem; height:2.25rem; border-radius:.375rem; border:1px solid var(--kx-border,#2d2d3d);
  background:transparent; color:var(--kx-foreground,#f1f5f9); cursor:pointer; font-size:.875rem; }
.kx-pagination button:hover:not(:disabled) { background:var(--kx-primary,#6366f1); border-color:var(--kx-primary,#6366f1); }
.kx-pagination button.active { background:var(--kx-primary,#6366f1); border-color:var(--kx-primary,#6366f1); }
.kx-pagination button:disabled { opacity:.4; cursor:not-allowed; }
"#;
