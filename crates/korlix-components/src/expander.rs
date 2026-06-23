//! Expand KLX ComponentNodes into HTML ElementNodes at compile time.

use korlix_ast::{
    element::{ClassRef, ComponentNode, ElementNode, Prop},
    expression::Expr,
    node::Node,
};

use crate::registry::get_component;

pub fn expand_component(node: &ComponentNode) -> Node {
    let schema = match get_component(&node.name) {
        Some(s) => s,
        None    => return expand_unknown(node),
    };

    let mut classes: Vec<ClassRef> = schema.default_classes.iter()
        .map(|c| ClassRef::new(c.clone(), node.span))
        .collect();
    classes.extend(node.classes.clone());

    // Special expansions per component
    match node.name.as_str() {
        "btn" | "button" => expand_button(node, classes),
        "icon"           => expand_icon(node, classes),
        "image"          => expand_image(node, classes),
        "avatar"         => expand_avatar(node, classes),
        "spinner"        => expand_spinner(node, classes),
        "skeleton"       => expand_skeleton(node, classes),
        "skeleton-card"  => expand_skeleton_card(node, classes),
        "empty-state"    => expand_empty_state(node, classes),
        "toast"          => expand_toast_trigger(node, classes),
        "modal"          => expand_modal(node, classes),
        "pagination"     => expand_pagination(node, classes),
        "progress"       => expand_progress(node, classes),
        "badge"          => expand_badge(node, classes),
        "alert"          => expand_alert(node, classes),
        "card"           => expand_card(node, classes),
        "navbar"         => expand_navbar(node, classes),
        "footer"         => expand_footer(node, classes),
        "container"      => expand_container(node, classes),
        "section"        => expand_section(node, classes),
        "link"           => expand_link(node, classes),
        "hero"           => expand_hero(node, classes),
        _                => expand_generic(node, schema.html_tag.clone(), classes),
    }
}

fn get_prop_str<'a>(node: &'a ComponentNode, key: &str) -> Option<&'a str> {
    node.props.iter().find(|p| p.key == key)?.value.as_string()
}
fn get_prop_num(node: &ComponentNode, key: &str) -> Option<f64> {
    node.props.iter().find(|p| p.key == key)?.value.as_number()
}
fn get_prop_bool(node: &ComponentNode, key: &str) -> Option<bool> {
    node.props.iter().find(|p| p.key == key)?.value.as_bool()
}

fn expand_button(node: &ComponentNode, classes: Vec<ClassRef>) -> Node {
    let disabled = get_prop_bool(node, "disabled").unwrap_or(false);
    let loading  = get_prop_bool(node, "loading").unwrap_or(false);
    let btn_type = get_prop_str(node, "type").unwrap_or("button");

    let mut children = node.children.clone();
    if loading {
        children.insert(0, Node::Element(ElementNode {
            tag: "span".into(), classes: vec![ClassRef::new("kx-spinner kx-spinner--sm", node.span)],
            props: vec![], events: vec![], children: vec![], span: node.span,
        }));
    }

    let mut props = vec![
        Prop::new("type", Expr::String(btn_type.to_string()), node.span),
    ];
    if disabled {
        props.push(Prop::new("disabled", Expr::Bool(true), node.span));
    }

    Node::Element(ElementNode {
        tag: "button".into(), classes, props, events: node.events.clone(),
        children, span: node.span,
    })
}

fn expand_icon(node: &ComponentNode, classes: Vec<ClassRef>) -> Node {
    let name  = get_prop_str(node, "name").unwrap_or("circle");
    let _label = get_prop_str(node, "name");
    let decorative = get_prop_bool(node, "decorative").unwrap_or(false);

    let mut props = vec![
        Prop::new("data-icon", Expr::String(name.to_string()), node.span),
    ];
    if decorative {
        props.push(Prop::new("aria-hidden", Expr::String("true".into()), node.span));
    } else if let Some(l) = get_prop_str(node, "label") {
        props.push(Prop::new("aria-label", Expr::String(l.to_string()), node.span));
    }
    // SVG placeholder — real icon set can be injected at runtime
    let svg_content = format!("<svg width=\"1em\" height=\"1em\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" aria-hidden=\"true\"><use href=\"#icon-{}\"/></svg>", name);
    Node::Element(ElementNode {
        tag: "span".into(), classes, props, events: node.events.clone(),
        children: vec![Node::Raw(korlix_ast::node::RawNode { html: svg_content, span: node.span })],
        span: node.span,
    })
}

fn expand_image(node: &ComponentNode, classes: Vec<ClassRef>) -> Node {
    let src  = get_prop_str(node, "src").unwrap_or("");
    let alt  = get_prop_str(node, "alt").unwrap_or("");
    let lazy = get_prop_bool(node, "lazy").unwrap_or(true);

    let mut props = vec![
        Prop::new("src", Expr::String(src.to_string()), node.span),
        Prop::new("alt", Expr::String(alt.to_string()), node.span),
    ];
    if lazy {
        props.push(Prop::new("loading", Expr::String("lazy".into()), node.span));
    }
    if let Some(w) = get_prop_num(node, "width") {
        props.push(Prop::new("width", Expr::Number(w), node.span));
    }
    if let Some(h) = get_prop_num(node, "height") {
        props.push(Prop::new("height", Expr::Number(h), node.span));
    }
    Node::Element(ElementNode { tag: "img".into(), classes, props, events: node.events.clone(), children: vec![], span: node.span })
}

fn expand_avatar(node: &ComponentNode, mut classes: Vec<ClassRef>) -> Node {
    let src  = get_prop_str(node, "src");
    let name = get_prop_str(node, "name").unwrap_or("?");
    let size = get_prop_str(node, "size").unwrap_or("md");
    let size_cls = format!("kx-avatar--{}", size);
    classes.push(ClassRef::new(size_cls, node.span));

    let initials: String = name.split_whitespace()
        .filter_map(|w| w.chars().next())
        .take(2).collect::<String>().to_uppercase();

    let inner = if let Some(s) = src {
        Node::Element(ElementNode {
            tag: "img".into(),
            classes: vec![ClassRef::new("kx-avatar__img", node.span)],
            props: vec![
                Prop::new("src", Expr::String(s.to_string()), node.span),
                Prop::new("alt", Expr::String(name.to_string()), node.span),
            ],
            events: vec![], children: vec![], span: node.span,
        })
    } else {
        Node::Element(ElementNode {
            tag: "span".into(),
            classes: vec![ClassRef::new("kx-avatar__initials", node.span)],
            props: vec![],
            events: vec![],
            children: vec![Node::Text(korlix_ast::node::TextNode {
                value: Expr::String(initials),
                span: node.span,
            })],
            span: node.span,
        })
    };

    Node::Element(ElementNode { tag: "div".into(), classes, props: vec![], events: node.events.clone(), children: vec![inner], span: node.span })
}

fn expand_spinner(node: &ComponentNode, classes: Vec<ClassRef>) -> Node {
    Node::Element(ElementNode {
        tag: "div".into(), classes,
        props: vec![
            Prop::new("role", Expr::String("status".into()), node.span),
            Prop::new("aria-label", Expr::String("Loading".into()), node.span),
        ],
        events: node.events.clone(), children: vec![], span: node.span,
    })
}

fn expand_skeleton(node: &ComponentNode, mut classes: Vec<ClassRef>) -> Node {
    let w = get_prop_str(node, "width").unwrap_or("100%");
    let h = get_prop_str(node, "height").unwrap_or("1rem");
    classes.push(ClassRef::new("kx-skeleton", node.span));
    Node::Element(ElementNode {
        tag: "div".into(), classes,
        props: vec![
            Prop::new("style", Expr::String(format!("width:{};height:{}", w, h)), node.span),
            Prop::new("aria-hidden", Expr::String("true".into()), node.span),
        ],
        events: vec![], children: vec![], span: node.span,
    })
}

fn expand_skeleton_card(node: &ComponentNode, classes: Vec<ClassRef>) -> Node {
    let count = get_prop_num(node, "count").unwrap_or(1.0) as usize;
    let mut children = vec![];
    for _ in 0..count.min(10) {
        children.push(Node::Element(ElementNode {
            tag: "div".into(),
            classes: vec![ClassRef::new("kx-skeleton-card__inner", node.span)],
            props: vec![Prop::new("aria-hidden", Expr::String("true".into()), node.span)],
            events: vec![],
            children: vec![
                Node::Element(ElementNode { tag: "div".into(), classes: vec![ClassRef::new("kx-skeleton", node.span)],
                    props: vec![Prop::new("style", Expr::String("width:100%;height:180px;margin-bottom:.75rem".into()), node.span)],
                    events: vec![], children: vec![], span: node.span }),
                Node::Element(ElementNode { tag: "div".into(), classes: vec![ClassRef::new("kx-skeleton", node.span)],
                    props: vec![Prop::new("style", Expr::String("width:70%;height:1rem;margin-bottom:.5rem".into()), node.span)],
                    events: vec![], children: vec![], span: node.span }),
                Node::Element(ElementNode { tag: "div".into(), classes: vec![ClassRef::new("kx-skeleton", node.span)],
                    props: vec![Prop::new("style", Expr::String("width:90%;height:.75rem".into()), node.span)],
                    events: vec![], children: vec![], span: node.span }),
            ],
            span: node.span,
        }));
    }
    Node::Element(ElementNode { tag: "div".into(), classes, props: vec![], events: vec![], children, span: node.span })
}

fn expand_empty_state(node: &ComponentNode, classes: Vec<ClassRef>) -> Node {
    let title = get_prop_str(node, "title").unwrap_or("Nothing here");
    let desc  = get_prop_str(node, "description");
    let icon  = get_prop_str(node, "icon");

    let mut children = vec![];
    if let Some(ic) = icon {
        children.push(Node::Element(ElementNode {
            tag: "div".into(), classes: vec![ClassRef::new("kx-empty-state__icon", node.span)],
            props: vec![Prop::new("data-icon", Expr::String(ic.to_string()), node.span)],
            events: vec![], children: vec![], span: node.span,
        }));
    }
    children.push(Node::Element(ElementNode {
        tag: "h3".into(), classes: vec![ClassRef::new("kx-empty-state__title", node.span)],
        props: vec![], events: vec![],
        children: vec![Node::Text(korlix_ast::node::TextNode { value: Expr::String(title.to_string()), span: node.span })],
        span: node.span,
    }));
    if let Some(d) = desc {
        children.push(Node::Element(ElementNode {
            tag: "p".into(), classes: vec![ClassRef::new("kx-empty-state__desc", node.span)],
            props: vec![], events: vec![],
            children: vec![Node::Text(korlix_ast::node::TextNode { value: Expr::String(d.to_string()), span: node.span })],
            span: node.span,
        }));
    }
    Node::Element(ElementNode { tag: "div".into(), classes, props: vec![
        Prop::new("role", Expr::String("status".into()), node.span),
    ], events: vec![], children, span: node.span })
}

fn expand_toast_trigger(node: &ComponentNode, classes: Vec<ClassRef>) -> Node {
    // toast is emitted via JS; render nothing structural
    let msg  = get_prop_str(node, "message").or_else(|| node.children.first().and_then(|c| if let Node::Text(t) = c { t.value.as_string().map(|s| s) } else { None })).unwrap_or("").to_string();
    let kind = get_prop_str(node, "type").unwrap_or("info").to_string();
    Node::Element(ElementNode {
        tag: "div".into(), classes,
        props: vec![
            Prop::new("data-kx-toast", Expr::String(kind), node.span),
            Prop::new("data-message", Expr::String(msg), node.span),
            Prop::new("style", Expr::String("display:none".into()), node.span),
        ],
        events: vec![], children: vec![], span: node.span,
    })
}

fn expand_modal(node: &ComponentNode, classes: Vec<ClassRef>) -> Node {
    let id     = get_prop_str(node, "id").unwrap_or("modal");
    let title  = get_prop_str(node, "title");
    let mut children = vec![];
    // header
    if let Some(t) = title {
        children.push(Node::Element(ElementNode {
            tag: "div".into(), classes: vec![ClassRef::new("kx-modal-header", node.span)],
            props: vec![], events: vec![],
            children: vec![
                Node::Element(ElementNode {
                    tag: "h2".into(), classes: vec![], props: vec![], events: vec![],
                    children: vec![Node::Text(korlix_ast::node::TextNode { value: Expr::String(t.to_string()), span: node.span })],
                    span: node.span,
                }),
                Node::Element(ElementNode {
                    tag: "button".into(),
                    classes: vec![ClassRef::new("kx-modal-close", node.span)],
                    props: vec![
                        Prop::new("data-kx-close-modal", Expr::String(id.to_string()), node.span),
                        Prop::new("aria-label", Expr::String("Close".into()), node.span),
                    ],
                    events: vec![], children: vec![Node::Text(korlix_ast::node::TextNode { value: Expr::String("×".into()), span: node.span })],
                    span: node.span,
                }),
            ],
            span: node.span,
        }));
    }
    children.extend(node.children.clone());
    Node::Element(ElementNode {
        tag: "div".into(),
        classes,
        props: vec![
            Prop::new("id", Expr::String(format!("kx-modal-{}", id)), node.span),
            Prop::new("role", Expr::String("dialog".into()), node.span),
            Prop::new("aria-modal", Expr::String("true".into()), node.span),
            Prop::new("hidden", Expr::String("true".into()), node.span),
        ],
        events: node.events.clone(), children, span: node.span,
    })
}

fn expand_pagination(node: &ComponentNode, classes: Vec<ClassRef>) -> Node {
    let page    = get_prop_num(node, "page").unwrap_or(1.0) as i64;
    let total   = get_prop_num(node, "total").unwrap_or(0.0) as i64;
    let per_page = get_prop_num(node, "perPage").unwrap_or(10.0) as i64;
    let _pages  = ((total + per_page - 1) / per_page).max(1);

    Node::Element(ElementNode {
        tag: "div".into(), classes,
        props: vec![
            Prop::new("data-kx-pagination", Expr::String("true".into()), node.span),
            Prop::new("data-page", Expr::Number(page as f64), node.span),
            Prop::new("data-total", Expr::Number(total as f64), node.span),
            Prop::new("data-per-page", Expr::Number(per_page as f64), node.span),
            Prop::new("role", Expr::String("navigation".into()), node.span),
            Prop::new("aria-label", Expr::String("Pagination".into()), node.span),
        ],
        events: node.events.clone(), children: vec![], span: node.span,
    })
}

fn expand_progress(node: &ComponentNode, classes: Vec<ClassRef>) -> Node {
    let value = get_prop_num(node, "value").unwrap_or(0.0);
    let max   = get_prop_num(node, "max").unwrap_or(100.0);
    let pct   = (value / max * 100.0).min(100.0).max(0.0);
    Node::Element(ElementNode {
        tag: "div".into(), classes,
        props: vec![
            Prop::new("role", Expr::String("progressbar".into()), node.span),
            Prop::new("aria-valuenow", Expr::Number(value), node.span),
            Prop::new("aria-valuemin", Expr::Number(0.0), node.span),
            Prop::new("aria-valuemax", Expr::Number(max), node.span),
            Prop::new("style", Expr::String(format!("--kx-progress:{:.2}%", pct)), node.span),
        ],
        events: node.events.clone(), children: vec![], span: node.span,
    })
}

fn expand_badge(node: &ComponentNode, classes: Vec<ClassRef>) -> Node {
    Node::Element(ElementNode { tag: "span".into(), classes, props: vec![], events: node.events.clone(), children: node.children.clone(), span: node.span })
}
fn expand_alert(node: &ComponentNode, mut classes: Vec<ClassRef>) -> Node {
    let kind = get_prop_str(node, "type").unwrap_or("info");
    classes.push(ClassRef::new(format!("kx-alert--{}", kind), node.span));
    Node::Element(ElementNode { tag: "div".into(), classes,
        props: vec![Prop::new("role", Expr::String("alert".into()), node.span)],
        events: node.events.clone(), children: node.children.clone(), span: node.span })
}
fn expand_card(node: &ComponentNode, classes: Vec<ClassRef>) -> Node {
    Node::Element(ElementNode { tag: "div".into(), classes, props: vec![], events: node.events.clone(), children: node.children.clone(), span: node.span })
}
fn expand_navbar(node: &ComponentNode, classes: Vec<ClassRef>) -> Node {
    Node::Element(ElementNode { tag: "nav".into(), classes,
        props: vec![Prop::new("role", Expr::String("navigation".into()), node.span)],
        events: node.events.clone(), children: node.children.clone(), span: node.span })
}
fn expand_footer(node: &ComponentNode, classes: Vec<ClassRef>) -> Node {
    Node::Element(ElementNode { tag: "footer".into(), classes, props: vec![], events: node.events.clone(), children: node.children.clone(), span: node.span })
}
fn expand_container(node: &ComponentNode, mut classes: Vec<ClassRef>) -> Node {
    let size = get_prop_str(node, "size").unwrap_or("lg");
    classes.push(ClassRef::new(format!("kx-container--{}", size), node.span));
    Node::Element(ElementNode { tag: "div".into(), classes, props: vec![], events: node.events.clone(), children: node.children.clone(), span: node.span })
}
fn expand_section(node: &ComponentNode, classes: Vec<ClassRef>) -> Node {
    Node::Element(ElementNode { tag: "section".into(), classes, props: vec![], events: node.events.clone(), children: node.children.clone(), span: node.span })
}
fn expand_link(node: &ComponentNode, classes: Vec<ClassRef>) -> Node {
    let href = get_prop_str(node, "href").unwrap_or("#").to_string();
    let mut props = vec![Prop::new("href", Expr::String(href), node.span)];
    let external = get_prop_bool(node, "external").unwrap_or(false);
    if external {
        props.push(Prop::new("target", Expr::String("_blank".into()), node.span));
        props.push(Prop::new("rel", Expr::String("noopener noreferrer".into()), node.span));
    } else {
        props.push(Prop::new("data-kx-link", Expr::String("true".into()), node.span));
    }
    Node::Element(ElementNode { tag: "a".into(), classes, props, events: node.events.clone(), children: node.children.clone(), span: node.span })
}
fn expand_hero(node: &ComponentNode, mut classes: Vec<ClassRef>) -> Node {
    let variant = get_prop_str(node, "variant").unwrap_or("centered");
    classes.push(ClassRef::new(format!("kx-hero--{}", variant), node.span));
    Node::Element(ElementNode { tag: "section".into(), classes, props: vec![], events: node.events.clone(), children: node.children.clone(), span: node.span })
}
fn expand_generic(node: &ComponentNode, tag: String, classes: Vec<ClassRef>) -> Node {
    Node::Element(ElementNode { tag, classes, props: node.props.clone(), events: node.events.clone(), children: node.children.clone(), span: node.span })
}

fn expand_unknown(node: &ComponentNode) -> Node {
    Node::Element(ElementNode {
        tag: "div".into(),
        classes: node.classes.clone(),
        props: node.props.clone(),
        events: node.events.clone(),
        children: node.children.clone(),
        span: node.span,
    })
}
