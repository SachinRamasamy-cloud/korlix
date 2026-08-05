//! Authoritative HTML/SVG element registry used by the Korlix parser.
//!
//! Korlix accepts modern, non-obsolete HTML elements directly. Components
//! remain separate and are resolved through `korlix-components`.

pub const HTML_TAGS: &[&str] = &[
    // document metadata
    "html",
    "head",
    "body",
    "base",
    "link",
    "meta",
    "style",
    "title",
    // sectioning / semantic
    "address",
    "article",
    "aside",
    "footer",
    "header",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "hgroup",
    "main",
    "nav",
    "search",
    "section",
    // text content
    "blockquote",
    "dd",
    "div",
    "dl",
    "dt",
    "figcaption",
    "figure",
    "hr",
    "li",
    "menu",
    "ol",
    "p",
    "pre",
    "ul",
    // inline text semantics
    "a",
    "abbr",
    "b",
    "bdi",
    "bdo",
    "br",
    "cite",
    "code",
    "data",
    "dfn",
    "em",
    "i",
    "kbd",
    "mark",
    "q",
    "rp",
    "rt",
    "ruby",
    "s",
    "samp",
    "small",
    "span",
    "strong",
    "sub",
    "sup",
    "time",
    "u",
    "var",
    "wbr",
    // image and multimedia
    "area",
    "audio",
    "img",
    "map",
    "track",
    "video",
    // embedded content
    "embed",
    "fencedframe",
    "iframe",
    "object",
    "picture",
    "source",
    // scripting / edits
    "canvas",
    "noscript",
    "script",
    "del",
    "ins",
    // table
    "caption",
    "col",
    "colgroup",
    "table",
    "tbody",
    "td",
    "tfoot",
    "th",
    "thead",
    "tr",
    // forms
    "button",
    "datalist",
    "fieldset",
    "form",
    "input",
    "label",
    "legend",
    "meter",
    "optgroup",
    "option",
    "output",
    "progress",
    "select",
    "textarea",
    // interactive / web components
    "details",
    "dialog",
    "summary",
    "slot",
    "template",
    // common SVG elements supported directly inside KLX
    "svg",
    "g",
    "defs",
    "symbol",
    "use",
    "path",
    "rect",
    "circle",
    "ellipse",
    "line",
    "polyline",
    "polygon",
    "text",
    "tspan",
    "clipPath",
    "mask",
    "pattern",
    "marker",
    "linearGradient",
    "radialGradient",
    "stop",
    "filter",
    "feGaussianBlur",
    "foreignObject",
];

pub const VOID_TAGS: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "source", "track",
    "wbr",
];

pub fn is_html_tag(name: &str) -> bool {
    HTML_TAGS.contains(&name)
}

pub fn is_void_tag(name: &str) -> bool {
    VOID_TAGS.contains(&name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn includes_modern_semantic_and_form_tags() {
        for tag in [
            "search", "dialog", "datalist", "picture", "template", "slot",
        ] {
            assert!(is_html_tag(tag), "missing HTML tag: {tag}");
        }
    }

    #[test]
    fn identifies_void_tags() {
        assert!(is_void_tag("img"));
        assert!(!is_void_tag("div"));
    }
}
