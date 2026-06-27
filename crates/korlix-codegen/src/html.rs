use korlix_ast::{element::ElementNode, expression::Expr, node::Node};
use korlix_components::expander::expand_component;

pub fn render_nodes(nodes: &[Node]) -> String {
    nodes.iter().map(render_node).collect::<Vec<_>>().join("")
}

pub fn render_node(node: &Node) -> String {
    match node {
        Node::Element(el) => render_element(el),
        Node::Component(c) => render_node(&expand_component(c)),
        Node::Text(t) => render_expr(&t.value),
        Node::If(i) => render_if_static(i),
        Node::For(f) => format!("<!-- for {} in ... -->", f.var),
        Node::Slot(s) => format!(
            r#"<div data-slot="{}" class="kx-slot"></div>"#,
            s.name.as_deref().unwrap_or("default")
        ),
        Node::Raw(r) => r.html.clone(),
        Node::State(_) => String::new(),
        Node::Let(_) => String::new(),
        Node::Derived(_) => String::new(),
        Node::Action(_) => String::new(),
        Node::Data(_) => String::new(),
        Node::Assign(_) => String::new(),
        Node::Call(c) => format!("<!-- call {} -->", c.callee),
        Node::Comment(s) => format!("<!-- {} -->", s),
        // API nodes produce no HTML — they are declarations/statements handled
        // by the JS codegen and runtime.
        Node::ApiQuery(_) => String::new(),
        Node::ApiMutation(_) => String::new(),
        Node::ApiReload(_) => String::new(),
    }
}

fn render_element(el: &ElementNode) -> String {
    let classes = el
        .classes
        .iter()
        .map(|c| format!("kx-{}", sanitize_class(&c.name)))
        .collect::<Vec<_>>()
        .join(" ");

    let mut attrs = String::new();
    if !classes.is_empty() {
        attrs.push_str(&format!(r#" class="{}""#, html_escape_attr(&classes)));
    }
    for prop in &el.props {
        let val = render_expr_attr(&prop.value);
        let key = &prop.key;
        // Skip JS-injection risks
        if key.starts_with("on") && val.to_lowercase().contains("javascript:") {
            continue;
        }
        attrs.push_str(&format!(
            r#" {}="{}""#,
            html_attr_key(key),
            html_escape_attr(&val)
        ));
    }

    // Encode events as data attributes for runtime
    for ev in &el.events {
        let body_js = nodes_to_js_stub(&ev.body);
        attrs.push_str(&format!(
            r#" data-on-{}="{}""#,
            ev.event,
            html_escape_attr(&body_js)
        ));
    }

    let tag = &el.tag;
    if is_void_element(tag) {
        return format!("<{}{}>", tag, attrs);
    }
    let inner = render_nodes(&el.children);
    format!("<{}{}>{}</{}>", tag, attrs, inner, tag)
}

fn render_if_static(i: &korlix_ast::node::IfNode) -> String {
    // At compile time we emit both branches with data- markers;
    // the runtime shows/hides them based on state.
    let then_html = render_nodes(&i.then_body);
    let else_html = i
        .else_body
        .as_ref()
        .map(|e| render_nodes(e))
        .unwrap_or_default();
    let cond = render_expr(&i.condition);
    format!(
        r#"<template data-kx-if="{}">{}</template><template data-kx-else="{}">{}</template>"#,
        html_escape_attr(&cond),
        then_html,
        html_escape_attr(&cond),
        else_html
    )
}

fn render_expr(e: &Expr) -> String {
    match e {
        Expr::String(s) => html_escape(s),
        Expr::Number(n) => n.to_string(),
        Expr::Bool(b) => b.to_string(),
        Expr::Null => String::new(),
        Expr::Identifier(s) => format!(r#"<span data-kx-bind="{}"></span>"#, s),
        Expr::Member { object, field } => {
            format!(
                r#"<span data-kx-bind="{}.{}"></span>"#,
                render_expr_raw(object),
                field
            )
        }
        Expr::Call { callee, args } => {
            let a = args
                .iter()
                .map(render_expr_raw)
                .collect::<Vec<_>>()
                .join(",");
            format!(
                r#"<span data-kx-expr="{}({})"></span>"#,
                render_expr_raw(callee),
                a
            )
        }
        _ => format!(r#"<span data-kx-expr="{}"></span>"#, render_expr_raw(e)),
    }
}

pub fn render_expr_raw(e: &Expr) -> String {
    match e {
        Expr::String(s) => format!("\"{}\"", s.replace('"', "\\\"")),
        Expr::Number(n) => n.to_string(),
        Expr::Bool(b) => b.to_string(),
        Expr::Null => "null".into(),
        Expr::Identifier(s) => s.clone(),
        Expr::Member { object, field } => format!("{}.{}", render_expr_raw(object), field),
        Expr::Binary { left, op, right } => {
            use korlix_ast::expression::BinaryOp;
            let op_str = match op {
                BinaryOp::Add => "+",
                BinaryOp::Sub => "-",
                BinaryOp::Mul => "*",
                BinaryOp::Div => "/",
                BinaryOp::Eq => "===",
                BinaryOp::Ne => "!==",
                BinaryOp::Lt => "<",
                BinaryOp::Le => "<=",
                BinaryOp::Gt => ">",
                BinaryOp::Ge => ">=",
                BinaryOp::And => "&&",
                BinaryOp::Or => "||",
                _ => "+",
            };
            format!(
                "({} {} {})",
                render_expr_raw(left),
                op_str,
                render_expr_raw(right)
            )
        }
        Expr::Call { callee, args } => {
            let a = args
                .iter()
                .map(render_expr_raw)
                .collect::<Vec<_>>()
                .join(", ");
            format!("{}({})", render_expr_raw(callee), a)
        }
        _ => String::new(),
    }
}

fn render_expr_attr(e: &Expr) -> String {
    match e {
        Expr::String(s) => s.clone(),
        Expr::Number(n) => n.to_string(),
        Expr::Bool(b) => b.to_string(),
        Expr::Null => String::new(),
        _ => render_expr_raw(e),
    }
}

fn nodes_to_js_stub(nodes: &[Node]) -> String {
    nodes
        .iter()
        .map(|n| {
            // API mutation/reload nodes emit async JS for inline event handlers.
            // We use the codegen api module for consistency.
            if let Some(api_js) = korlix_codegen_api_stub(n) {
                return api_js;
            }
            match n {
                Node::Assign(a) => format!("__state.{} = {}", a.target, render_expr_state(&a.value)),
                Node::Call(c) => format!(
                    "KorlixRuntime.call('{}', [{}])",
                    c.callee,
                    c.args
                        .iter()
                        .map(render_expr_state)
                        .collect::<Vec<_>>()
                        .join(",")
                ),
                Node::Component(c) if c.name == "toast" => {
                    let args = c
                        .children
                        .iter()
                        .filter_map(|child| {
                            if let Node::Text(t) = child {
                                Some(match &t.value {
                                    Expr::Identifier(s) => format!("\"{}\"", s.replace('"', "\\\"")),
                                    other => render_expr_state(other),
                                })
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(",");
                    format!("KorlixRuntime.call('toast', [{}])", args)
                }
                Node::Component(c) => {
                    let args = c
                        .children
                        .iter()
                        .filter_map(|child| {
                            if let Node::Text(t) = child {
                                Some(render_expr_state(&t.value))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(",");
                    format!("KorlixRuntime.call('{}', [{}])", c.name, args)
                }
                _ => String::new(),
            }
        })
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(";")
}

/// Generate inline JS for API nodes appearing in event handler bodies.
fn korlix_codegen_api_stub(node: &Node) -> Option<String> {
    match node {
        Node::ApiMutation(m) => {
            let method = format!("{:?}", m.method.as_str());
            let url = format!("{:?}", m.url.as_str());
            let body = match &m.body {
                Some(b) => render_expr_raw(b),
                None => "undefined".into(),
            };
            Some(format!("KorlixRuntime.api.request({},{},{})", method, url, body))
        }
        Node::ApiReload(r) => {
            let name = format!("{:?}", r.target.as_str());
            Some(format!("KorlixRuntime.api.reload({})", name))
        }
        _ => None,
    }
}

fn render_expr_state(e: &Expr) -> String {
    match e {
        Expr::Identifier(s) => format!("__state.{}", s),
        Expr::Member { object, field } => format!("{}.{}", render_expr_state(object), field),
        Expr::Binary { left, op, right } => {
            use korlix_ast::expression::BinaryOp;
            let op_str = match op {
                BinaryOp::Add => "+",
                BinaryOp::Sub => "-",
                BinaryOp::Mul => "*",
                BinaryOp::Div => "/",
                BinaryOp::Eq => "===",
                BinaryOp::Ne => "!==",
                BinaryOp::Lt => "<",
                BinaryOp::Le => "<=",
                BinaryOp::Gt => ">",
                BinaryOp::Ge => ">=",
                BinaryOp::And => "&&",
                BinaryOp::Or => "||",
                _ => "+",
            };
            format!(
                "({} {} {})",
                render_expr_state(left),
                op_str,
                render_expr_state(right)
            )
        }
        Expr::Call { callee, args } => {
            let a = args
                .iter()
                .map(render_expr_state)
                .collect::<Vec<_>>()
                .join(", ");
            format!("{}({})", render_expr_state(callee), a)
        }
        _ => render_expr_raw(e),
    }
}

fn is_void_element(tag: &str) -> bool {
    matches!(
        tag,
        "area"
            | "base"
            | "br"
            | "col"
            | "embed"
            | "hr"
            | "img"
            | "input"
            | "link"
            | "meta"
            | "param"
            | "source"
            | "track"
            | "wbr"
    )
}

fn sanitize_class(c: &str) -> String {
    c.to_string()
}

fn html_attr_key(k: &str) -> &str {
    k
}

pub fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
pub fn html_escape_attr(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
}
