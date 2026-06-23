use crate::colors::build_palette;
use crate::tokens::*;
use indexmap::IndexMap;
use once_cell::sync::Lazy;

/// A resolved CSS rule: selector → declarations
#[derive(Debug, Clone)]
pub struct CssRule {
    pub selector: String,
    pub declarations: Vec<(String, String)>,
    pub media_query: Option<String>,
}

impl CssRule {
    pub fn new(selector: impl Into<String>) -> Self {
        Self {
            selector: selector.into(),
            declarations: vec![],
            media_query: None,
        }
    }

    pub fn prop(mut self, key: impl Into<String>, val: impl Into<String>) -> Self {
        self.declarations.push((key.into(), val.into()));
        self
    }

    pub fn media(mut self, mq: impl Into<String>) -> Self {
        self.media_query = Some(mq.into());
        self
    }

    pub fn to_css(&self) -> String {
        let decls = self
            .declarations
            .iter()
            .map(|(k, v)| format!("  {}:{};", k, v))
            .collect::<Vec<_>>()
            .join("\n");

        let rule = format!(".kx-{}{{\n{}\n}}", escape_selector(&self.selector), decls);

        if let Some(mq) = &self.media_query {
            format!("{}{{\n{}\n}}", mq, rule)
        } else {
            rule
        }
    }
}

fn escape_selector(s: &str) -> String {
    s.replace(':', r"\:")
        .replace('[', r"\[")
        .replace(']', r"\]")
        .replace('/', r"\/")
        .replace('.', r"\.")
        .replace('%', r"\%")
        .replace('#', r"\#")
        .replace('(', r"\(")
        .replace(')', r"\)")
        .replace(',', r"\,")
        .replace('"', r#"\""#)
}

// ── Global registry: class_name → CssRule ─────────────────────────────
pub static UTILITY_REGISTRY: Lazy<IndexMap<String, CssRule>> = Lazy::new(build_registry);

fn build_registry() -> IndexMap<String, CssRule> {
    let mut map = IndexMap::new();
    let palette = build_palette();

    macro_rules! add {
        ($name:expr, $rule:expr) => {
            map.insert($name.to_string(), $rule);
        };
    }

    // ── Display ──────────────────────────────────────────────────────
    for (cls, val) in [
        ("block", "block"),
        ("inline-block", "inline-block"),
        ("inline", "inline"),
        ("flex", "flex"),
        ("inline-flex", "inline-flex"),
        ("grid", "grid"),
        ("inline-grid", "inline-grid"),
        ("hidden", "none"),
        ("contents", "contents"),
        ("table", "table"),
        ("table-cell", "table-cell"),
        ("table-row", "table-row"),
    ] {
        add!(cls, CssRule::new(cls).prop("display", val));
    }

    // ── Position ─────────────────────────────────────────────────────
    for (cls, val) in [
        ("static", "static"),
        ("relative", "relative"),
        ("absolute", "absolute"),
        ("fixed", "fixed"),
        ("sticky", "sticky"),
    ] {
        add!(cls, CssRule::new(cls).prop("position", val));
    }

    // ── Flex ─────────────────────────────────────────────────────────
    for (cls, prop, val) in [
        ("flex-row", "flex-direction", "row"),
        ("flex-col", "flex-direction", "column"),
        ("flex-row-reverse", "flex-direction", "row-reverse"),
        ("flex-col-reverse", "flex-direction", "column-reverse"),
        ("flex-wrap", "flex-wrap", "wrap"),
        ("flex-nowrap", "flex-wrap", "nowrap"),
        ("flex-1", "flex", "1 1 0%"),
        ("flex-auto", "flex", "1 1 auto"),
        ("flex-none", "flex", "none"),
        ("flex-shrink", "flex-shrink", "1"),
        ("flex-shrink-0", "flex-shrink", "0"),
        ("flex-grow", "flex-grow", "1"),
        ("flex-grow-0", "flex-grow", "0"),
        ("items-start", "align-items", "flex-start"),
        ("items-end", "align-items", "flex-end"),
        ("items-center", "align-items", "center"),
        ("items-baseline", "align-items", "baseline"),
        ("items-stretch", "align-items", "stretch"),
        ("justify-start", "justify-content", "flex-start"),
        ("justify-end", "justify-content", "flex-end"),
        ("justify-center", "justify-content", "center"),
        ("justify-between", "justify-content", "space-between"),
        ("justify-around", "justify-content", "space-around"),
        ("justify-evenly", "justify-content", "space-evenly"),
        ("self-auto", "align-self", "auto"),
        ("self-start", "align-self", "flex-start"),
        ("self-end", "align-self", "flex-end"),
        ("self-center", "align-self", "center"),
        ("self-stretch", "align-self", "stretch"),
        ("center", "place-items", "center"),
    ] {
        add!(cls, CssRule::new(cls).prop(prop, val));
    }

    // ── Grid ─────────────────────────────────────────────────────────
    for cols in 1..=12 {
        let cls = format!("grid-cols-{}", cols);
        add!(
            cls.clone(),
            CssRule::new(&cls).prop(
                "grid-template-columns",
                format!("repeat({},minmax(0,1fr))", cols)
            )
        );
        let cls2 = format!("col-span-{}", cols);
        add!(
            cls2.clone(),
            CssRule::new(&cls2).prop("grid-column", format!("span {}/span {}", cols, cols))
        );
    }
    add!(
        "grid-cols-none",
        CssRule::new("grid-cols-none").prop("grid-template-columns", "none")
    );
    for rows in 1..=6 {
        let cls = format!("grid-rows-{}", rows);
        add!(
            cls.clone(),
            CssRule::new(&cls).prop(
                "grid-template-rows",
                format!("repeat({},minmax(0,1fr))", rows)
            )
        );
    }

    // ── Spacing (p, m, gap) ─────────────────────────────────────────
    for (scale, px) in SPACING {
        let v = px.to_string();
        // padding all sides
        add!(
            format!("p-{}", scale),
            CssRule::new(format!("p-{}", scale)).prop("padding", &v)
        );
        add!(
            format!("px-{}", scale),
            CssRule::new(format!("px-{}", scale))
                .prop("padding-left", &v)
                .prop("padding-right", &v)
        );
        add!(
            format!("py-{}", scale),
            CssRule::new(format!("py-{}", scale))
                .prop("padding-top", &v)
                .prop("padding-bottom", &v)
        );
        add!(
            format!("pt-{}", scale),
            CssRule::new(format!("pt-{}", scale)).prop("padding-top", &v)
        );
        add!(
            format!("pr-{}", scale),
            CssRule::new(format!("pr-{}", scale)).prop("padding-right", &v)
        );
        add!(
            format!("pb-{}", scale),
            CssRule::new(format!("pb-{}", scale)).prop("padding-bottom", &v)
        );
        add!(
            format!("pl-{}", scale),
            CssRule::new(format!("pl-{}", scale)).prop("padding-left", &v)
        );
        // margin
        add!(
            format!("m-{}", scale),
            CssRule::new(format!("m-{}", scale)).prop("margin", &v)
        );
        add!(
            format!("mx-{}", scale),
            CssRule::new(format!("mx-{}", scale))
                .prop("margin-left", &v)
                .prop("margin-right", &v)
        );
        add!(
            format!("my-{}", scale),
            CssRule::new(format!("my-{}", scale))
                .prop("margin-top", &v)
                .prop("margin-bottom", &v)
        );
        add!(
            format!("mt-{}", scale),
            CssRule::new(format!("mt-{}", scale)).prop("margin-top", &v)
        );
        add!(
            format!("mr-{}", scale),
            CssRule::new(format!("mr-{}", scale)).prop("margin-right", &v)
        );
        add!(
            format!("mb-{}", scale),
            CssRule::new(format!("mb-{}", scale)).prop("margin-bottom", &v)
        );
        add!(
            format!("ml-{}", scale),
            CssRule::new(format!("ml-{}", scale)).prop("margin-left", &v)
        );
        // gap
        add!(
            format!("gap-{}", scale),
            CssRule::new(format!("gap-{}", scale)).prop("gap", &v)
        );
        add!(
            format!("gap-x-{}", scale),
            CssRule::new(format!("gap-x-{}", scale)).prop("column-gap", &v)
        );
        add!(
            format!("gap-y-{}", scale),
            CssRule::new(format!("gap-y-{}", scale)).prop("row-gap", &v)
        );
        // space-x / space-y (child selector)
        let sx = format!("space-x-{}", scale);
        add!(sx.clone(), CssRule::new(&sx).prop("--kx-space-x", &v));
        let sy = format!("space-y-{}", scale);
        add!(sy.clone(), CssRule::new(&sy).prop("--kx-space-y", &v));
    }
    // margin auto
    add!("m-auto", CssRule::new("m-auto").prop("margin", "auto"));
    add!(
        "mx-auto",
        CssRule::new("mx-auto")
            .prop("margin-left", "auto")
            .prop("margin-right", "auto")
    );
    add!(
        "my-auto",
        CssRule::new("my-auto")
            .prop("margin-top", "auto")
            .prop("margin-bottom", "auto")
    );

    // ── Sizing ───────────────────────────────────────────────────────
    for (scale, px) in SPACING {
        let v = px.to_string();
        add!(
            format!("w-{}", scale),
            CssRule::new(format!("w-{}", scale)).prop("width", &v)
        );
        add!(
            format!("h-{}", scale),
            CssRule::new(format!("h-{}", scale)).prop("height", &v)
        );
        add!(
            format!("min-w-{}", scale),
            CssRule::new(format!("min-w-{}", scale)).prop("min-width", &v)
        );
        add!(
            format!("min-h-{}", scale),
            CssRule::new(format!("min-h-{}", scale)).prop("min-height", &v)
        );
        add!(
            format!("max-w-{}", scale),
            CssRule::new(format!("max-w-{}", scale)).prop("max-width", &v)
        );
        add!(
            format!("max-h-{}", scale),
            CssRule::new(format!("max-h-{}", scale)).prop("max-height", &v)
        );
        add!(
            format!("size-{}", scale),
            CssRule::new(format!("size-{}", scale))
                .prop("width", &v)
                .prop("height", &v)
        );
    }
    for (cls, prop, val) in [
        ("w-full", "width", "100%"),
        ("w-screen", "width", "100vw"),
        ("w-auto", "width", "auto"),
        ("w-1/2", "width", "50%"),
        ("w-1/3", "width", "33.3333%"),
        ("w-2/3", "width", "66.6667%"),
        ("w-1/4", "width", "25%"),
        ("w-3/4", "width", "75%"),
        ("h-full", "height", "100%"),
        ("h-screen", "height", "100vh"),
        ("h-auto", "height", "auto"),
        ("min-h-screen", "min-height", "100vh"),
        ("min-h-full", "min-height", "100%"),
        ("max-w-sm", "max-width", "24rem"),
        ("max-w-md", "max-width", "28rem"),
        ("max-w-lg", "max-width", "32rem"),
        ("max-w-xl", "max-width", "36rem"),
        ("max-w-2xl", "max-width", "42rem"),
        ("max-w-3xl", "max-width", "48rem"),
        ("max-w-4xl", "max-width", "56rem"),
        ("max-w-5xl", "max-width", "64rem"),
        ("max-w-6xl", "max-width", "72rem"),
        ("max-w-7xl", "max-width", "80rem"),
        ("max-w-full", "max-width", "100%"),
        ("max-w-screen", "max-width", "100vw"),
    ] {
        add!(cls, CssRule::new(cls).prop(prop, val));
    }

    // ── Typography ───────────────────────────────────────────────────
    for (name, fs, lh) in FONT_SIZES {
        let cls = format!("text-{}", name);
        add!(
            cls.clone(),
            CssRule::new(&cls)
                .prop("font-size", *fs)
                .prop("line-height", *lh)
        );
    }
    for (name, val) in FONT_WEIGHTS {
        let cls = format!("font-{}", name);
        add!(cls.clone(), CssRule::new(&cls).prop("font-weight", *val));
    }
    for (cls, prop, val) in [
        ("font-sans", "font-family", "ui-sans-serif,system-ui,-apple-system,BlinkMacSystemFont,\"Segoe UI\",Roboto,sans-serif"),
        ("font-serif", "font-family", "ui-serif,Georgia,Cambria,\"Times New Roman\",Times,serif"),
        ("font-mono", "font-family", "ui-monospace,SFMono-Regular,Menlo,Monaco,Consolas,\"Liberation Mono\",\"Courier New\",monospace"),
        ("text-left", "text-align", "left"),
        ("text-center", "text-align", "center"),
        ("text-right", "text-align", "right"),
        ("text-justify", "text-align", "justify"),
        ("uppercase", "text-transform", "uppercase"),
        ("lowercase", "text-transform", "lowercase"),
        ("capitalize", "text-transform", "capitalize"),
        ("normal-case", "text-transform", "none"),
        ("italic", "font-style", "italic"),
        ("not-italic", "font-style", "normal"),
        ("underline", "text-decoration-line", "underline"),
        ("line-through", "text-decoration-line", "line-through"),
        ("no-underline", "text-decoration-line", "none"),
        ("bg-clip-text", "background-clip", "text"),
        ("truncate", "overflow", "hidden"),
        ("leading-none", "line-height", "1"),
        ("leading-tight", "line-height", "1.25"),
        ("leading-snug", "line-height", "1.375"),
        ("leading-normal", "line-height", "1.5"),
        ("leading-relaxed", "line-height", "1.625"),
        ("leading-loose", "line-height", "2"),
        ("tracking-tight", "letter-spacing", "-0.05em"),
        ("tracking-normal", "letter-spacing", "0"),
        ("tracking-wide", "letter-spacing", "0.025em"),
        ("tracking-wider", "letter-spacing", "0.05em"),
        ("tracking-widest", "letter-spacing", "0.1em"),
        ("whitespace-normal", "white-space", "normal"),
        ("whitespace-nowrap", "white-space", "nowrap"),
        ("whitespace-pre", "white-space", "pre"),
        ("break-words", "word-break", "break-word"),
        ("break-all", "word-break", "break-all"),
    ] {
        add!(cls, CssRule::new(cls).prop(prop, val));
    }

    // ── Colors (text-, bg-, border-, ring-, fill-, stroke-) ─────────
    for (color_name, hex) in &palette {
        let hex = hex.clone();

        for prefix in [
            "text",
            "bg",
            "border",
            "ring",
            "fill",
            "stroke",
            "outline",
            "caret",
            "placeholder",
        ] {
            let cls = format!("{}-{}", prefix, color_name);
            let rule = match prefix {
                "text" => CssRule::new(&cls).prop("color", &hex),
                "bg" => CssRule::new(&cls).prop("background-color", &hex),
                "border" => CssRule::new(&cls).prop("border-color", &hex),
                "ring" => CssRule::new(&cls).prop("--kx-ring-color", &hex),
                "fill" => CssRule::new(&cls).prop("fill", &hex),
                "stroke" => CssRule::new(&cls).prop("stroke", &hex),
                "outline" => CssRule::new(&cls).prop("outline-color", &hex),
                "caret" => CssRule::new(&cls).prop("caret-color", &hex),
                "placeholder" => CssRule::new(&cls).prop("--kx-placeholder-color", &hex),
                _ => continue,
            };
            add!(cls, rule);
        }
    }

    // ── Border ───────────────────────────────────────────────────────
    for (name, val) in RADIUS {
        let cls = if name.is_empty() {
            "rounded".to_string()
        } else {
            format!("rounded-{}", name)
        };
        add!(cls.clone(), CssRule::new(&cls).prop("border-radius", *val));
    }
    for w in [0, 1, 2, 4, 8] {
        let cls = if w == 1 {
            "border".to_string()
        } else {
            format!("border-{}", w)
        };
        let v = if w == 0 {
            "0px".to_string()
        } else {
            format!("{}px", w)
        };
        add!(cls.clone(), CssRule::new(&cls).prop("border-width", &v));
    }
    for (cls, prop, val) in [
        ("border-t", "border-top-width", "1px"),
        ("border-r", "border-right-width", "1px"),
        ("border-b", "border-bottom-width", "1px"),
        ("border-l", "border-left-width", "1px"),
        ("border-solid", "border-style", "solid"),
        ("border-dashed", "border-style", "dashed"),
        ("border-dotted", "border-style", "dotted"),
        ("border-none", "border-style", "none"),
    ] {
        add!(cls, CssRule::new(cls).prop(prop, val));
    }

    // ── Shadow ───────────────────────────────────────────────────────
    for (name, val) in SHADOWS {
        let cls = if name.is_empty() {
            "shadow".to_string()
        } else {
            format!("shadow-{}", name)
        };
        add!(cls.clone(), CssRule::new(&cls).prop("box-shadow", *val));
    }

    // ── Opacity ──────────────────────────────────────────────────────
    for (name, val) in OPACITY {
        let cls = format!("opacity-{}", name);
        add!(cls.clone(), CssRule::new(&cls).prop("opacity", *val));
    }
    add!(
        "opacity-15",
        CssRule::new("opacity-15").prop("opacity", "0.15")
    );

    // ── Overflow ─────────────────────────────────────────────────────
    for (cls, val) in [
        ("overflow-auto", "auto"),
        ("overflow-hidden", "hidden"),
        ("overflow-visible", "visible"),
        ("overflow-scroll", "scroll"),
        ("overflow-x-auto", "auto"),
        ("overflow-y-auto", "auto"),
        ("overflow-x-hidden", "hidden"),
        ("overflow-y-hidden", "hidden"),
    ] {
        let prop = if cls.contains("-x-") {
            "overflow-x"
        } else if cls.contains("-y-") {
            "overflow-y"
        } else {
            "overflow"
        };
        add!(cls, CssRule::new(cls).prop(prop, val));
    }

    // ── Z-Index ──────────────────────────────────────────────────────
    for (name, val) in Z_INDEX {
        let cls = format!("z-{}", name);
        add!(cls.clone(), CssRule::new(&cls).prop("z-index", *val));
    }

    // ── Transitions & animations ─────────────────────────────────────
    for (name, val) in TRANSITIONS {
        let cls = if name.is_empty() {
            "transition".to_string()
        } else {
            format!("transition-{}", name)
        };
        add!(cls.clone(), CssRule::new(&cls).prop("transition", *val));
    }
    for (name, val) in DURATIONS {
        let cls = format!("duration-{}", name);
        add!(
            cls.clone(),
            CssRule::new(&cls).prop("transition-duration", *val)
        );
    }

    // ── Cursor / Pointer ─────────────────────────────────────────────
    for (cls, val) in [
        ("cursor-auto", "auto"),
        ("cursor-default", "default"),
        ("cursor-pointer", "pointer"),
        ("cursor-wait", "wait"),
        ("cursor-text", "text"),
        ("cursor-not-allowed", "not-allowed"),
        ("cursor-grab", "grab"),
        ("cursor-grabbing", "grabbing"),
        ("pointer-events-none", "none"),
        ("pointer-events-auto", "auto"),
        ("select-none", "none"),
        ("select-text", "text"),
        ("select-all", "all"),
    ] {
        let prop = if cls.starts_with("pointer") {
            "pointer-events"
        } else if cls.starts_with("select") {
            "user-select"
        } else {
            "cursor"
        };
        add!(cls, CssRule::new(cls).prop(prop, val));
    }

    // ── Transform helpers ────────────────────────────────────────────
    for (cls, prop, val) in [
        ("scale-0", "transform", "scale(0)"),
        ("scale-50", "transform", "scale(.5)"),
        ("scale-75", "transform", "scale(.75)"),
        ("scale-90", "transform", "scale(.9)"),
        ("scale-95", "transform", "scale(.95)"),
        ("scale-100", "transform", "scale(1)"),
        ("scale-105", "transform", "scale(1.05)"),
        ("scale-110", "transform", "scale(1.1)"),
        ("scale-125", "transform", "scale(1.25)"),
        ("scale-150", "transform", "scale(1.5)"),
        ("rotate-0", "transform", "rotate(0deg)"),
        ("rotate-1", "transform", "rotate(1deg)"),
        ("rotate-2", "transform", "rotate(2deg)"),
        ("rotate-3", "transform", "rotate(3deg)"),
        ("rotate-6", "transform", "rotate(6deg)"),
        ("rotate-12", "transform", "rotate(12deg)"),
        ("rotate-45", "transform", "rotate(45deg)"),
        ("rotate-90", "transform", "rotate(90deg)"),
        ("rotate-180", "transform", "rotate(180deg)"),
        ("-rotate-1", "transform", "rotate(-1deg)"),
        ("-rotate-2", "transform", "rotate(-2deg)"),
        ("-rotate-6", "transform", "rotate(-6deg)"),
        ("-rotate-12", "transform", "rotate(-12deg)"),
    ] {
        add!(cls, CssRule::new(cls).prop(prop, val));
    }

    // ── Accessibility ────────────────────────────────────────────────
    add!(
        "sr-only",
        CssRule::new("sr-only")
            .prop("position", "absolute")
            .prop("width", "1px")
            .prop("height", "1px")
            .prop("padding", "0")
            .prop("margin", "-1px")
            .prop("overflow", "hidden")
            .prop("clip", "rect(0,0,0,0)")
            .prop("white-space", "nowrap")
            .prop("border-width", "0")
    );
    add!(
        "not-sr-only",
        CssRule::new("not-sr-only")
            .prop("position", "static")
            .prop("width", "auto")
            .prop("height", "auto")
            .prop("padding", "0")
            .prop("margin", "0")
            .prop("overflow", "visible")
            .prop("clip", "auto")
            .prop("white-space", "normal")
    );

    // ── Misc ──────────────────────────────────────────────────────────
    for (cls, prop, val) in [
        ("outline-none", "outline", "2px solid transparent"),
        ("group", "position", "relative"),
        (
            "animate-pulse",
            "animation",
            "kx-pulse 2s cubic-bezier(.4,0,.6,1) infinite",
        ),
        (
            "ring",
            "box-shadow",
            "0 0 0 3px var(--kx-ring-color,rgba(99,102,241,.5))",
        ),
        (
            "ring-2",
            "box-shadow",
            "0 0 0 2px var(--kx-ring-color,rgba(99,102,241,.5))",
        ),
        ("appearance-none", "appearance", "none"),
        ("resize", "resize", "both"),
        ("resize-none", "resize", "none"),
        ("resize-x", "resize", "horizontal"),
        ("resize-y", "resize", "vertical"),
        ("aspect-auto", "aspect-ratio", "auto"),
        ("aspect-square", "aspect-ratio", "1/1"),
        ("aspect-video", "aspect-ratio", "16/9"),
        ("object-cover", "object-fit", "cover"),
        ("object-contain", "object-fit", "contain"),
        ("object-fill", "object-fit", "fill"),
        ("object-none", "object-fit", "none"),
        ("object-center", "object-position", "center"),
        ("antialiased", "-webkit-font-smoothing", "antialiased"),
        ("subpixel-antialiased", "-webkit-font-smoothing", "auto"),
        ("list-none", "list-style-type", "none"),
        ("list-disc", "list-style-type", "disc"),
        ("list-decimal", "list-style-type", "decimal"),
        ("table-auto", "table-layout", "auto"),
        ("table-fixed", "table-layout", "fixed"),
        ("border-collapse", "border-collapse", "collapse"),
        ("border-separate", "border-collapse", "separate"),
        ("blur-none", "filter", "blur(0)"),
        ("blur-sm", "filter", "blur(4px)"),
        ("blur", "filter", "blur(8px)"),
        ("blur-md", "filter", "blur(12px)"),
        ("blur-lg", "filter", "blur(16px)"),
        ("blur-xl", "filter", "blur(24px)"),
        ("blur-2xl", "filter", "blur(40px)"),
        ("blur-3xl", "filter", "blur(64px)"),
        ("brightness-50", "filter", "brightness(.5)"),
        ("brightness-75", "filter", "brightness(.75)"),
        ("brightness-90", "filter", "brightness(.9)"),
        ("brightness-100", "filter", "brightness(1)"),
        ("brightness-110", "filter", "brightness(1.1)"),
        ("brightness-125", "filter", "brightness(1.25)"),
        ("backdrop-blur", "backdrop-filter", "blur(8px)"),
        ("backdrop-blur-sm", "backdrop-filter", "blur(4px)"),
        ("backdrop-blur-md", "backdrop-filter", "blur(12px)"),
        ("backdrop-blur-xl", "backdrop-filter", "blur(24px)"),
    ] {
        add!(cls, CssRule::new(cls).prop(prop, val));
    }

    // ── Top / right / bottom / left inset ────────────────────────────
    for (scale, px) in SPACING {
        for dir in ["inset", "top", "right", "bottom", "left"] {
            let cls = if dir == "inset" {
                format!("inset-{}", scale)
            } else {
                format!("{}-{}", dir, scale)
            };
            let prop = if dir == "inset" {
                "inset".to_string()
            } else {
                dir.to_string()
            };
            add!(cls.clone(), CssRule::new(&cls).prop(prop, *px));
        }
    }
    for (cls, prop, val) in [
        ("inset-x-0", "left", "0"),
        ("inset-y-0", "top", "0"),
        ("inset-full", "inset", "100%"),
        ("inset-auto", "inset", "auto"),
        ("left-1/2", "left", "50%"),
    ] {
        add!(cls, CssRule::new(cls).prop(prop, val));
    }

    // ── Responsive breakpoints wrapping ──────────────────────────────
    // (done in generator per-class; registry only stores base rules)

    map
}

pub fn all_class_names() -> Vec<String> {
    UTILITY_REGISTRY.keys().cloned().collect()
}

pub fn lookup(class: &str) -> Option<&'static CssRule> {
    UTILITY_REGISTRY.get(class)
}

pub fn suggest(class: &str) -> Vec<String> {
    let names = all_class_names();
    let mut scored: Vec<(usize, &String)> = names
        .iter()
        .filter_map(|n| {
            let dist = levenshtein(class, n);
            if dist <= 3 {
                Some((dist, n))
            } else {
                None
            }
        })
        .collect();
    scored.sort_by_key(|(d, _)| *d);
    scored.into_iter().take(3).map(|(_, n)| n.clone()).collect()
}

fn levenshtein(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let n = a.len();
    let m = b.len();
    let mut dp = vec![vec![0usize; m + 1]; n + 1];
    for i in 0..=n {
        dp[i][0] = i;
    }
    for j in 0..=m {
        dp[0][j] = j;
    }
    for i in 1..=n {
        for j in 1..=m {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            dp[i][j] = (dp[i - 1][j] + 1)
                .min(dp[i][j - 1] + 1)
                .min(dp[i - 1][j - 1] + cost);
        }
    }
    dp[n][m]
}
