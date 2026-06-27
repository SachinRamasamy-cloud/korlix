use korlix_ast::{
    api::{ApiMutationNode, ApiQueryNode, ApiReloadNode},
    expression::Expr,
    node::Node,
    program::{Item, Module},
};

use crate::js::expr_to_js;

// ── Public surface ─────────────────────────────────────────────────────────

/// Generate `KorlixRuntime.api.query(...)` calls for every `get` declaration
/// found anywhere in the module (pages, layouts, components).
///
/// These are emitted once at page-init time (before actions run).
pub fn generate_api_init(module: &Module) -> String {
    let mut js = String::new();

    for item in &module.items {
        let nodes = item_body(item);
        collect_api_queries(nodes, &mut js);
    }

    js
}

/// Try to generate JS for a single action-body node.
/// Returns `Some(js_string)` for `ApiMutation` and `ApiReload`, `None` otherwise.
pub fn generate_api_statement(node: &Node) -> Option<String> {
    match node {
        Node::ApiMutation(m) => Some(generate_mutation(m)),
        Node::ApiReload(r) => Some(generate_reload(r)),
        // ApiQuery is a top-level declaration, not an action statement
        _ => None,
    }
}

// ── Internal helpers ───────────────────────────────────────────────────────

fn collect_api_queries(nodes: &[Node], out: &mut String) {
    for node in nodes {
        if let Node::ApiQuery(q) = node {
            out.push_str(&generate_query(q));
            out.push('\n');
        }
        // Recurse into structural nodes (but NOT into action bodies —
        // queries inside actions would be re-declared on every call)
        match node {
            Node::Element(el) => collect_api_queries(&el.children, out),
            Node::Component(c) => collect_api_queries(&c.children, out),
            Node::If(i) => {
                collect_api_queries(&i.then_body, out);
                if let Some(else_nodes) = &i.else_body {
                    collect_api_queries(else_nodes, out);
                }
            }
            Node::For(f) => collect_api_queries(&f.body, out),
            // Intentionally skip Action bodies — mutations/reloads live there
            _ => {}
        }
    }
}

/// Borrow the top-level node slice for a given `Item`.
fn item_body(item: &Item) -> &[Node] {
    match item {
        Item::Page(p) => &p.body,
        Item::Layout(l) => &l.body,
        Item::Component(c) => &c.body,
        _ => &[],
    }
}

// ── Code generators ────────────────────────────────────────────────────────

/// `KorlixRuntime.api.query("users", "/api/users");`
fn generate_query(q: &ApiQueryNode) -> String {
    format!(
        "KorlixRuntime.api.query({}, {});",
        js_str(&q.name),
        js_str(&q.url)
    )
}

/// `await KorlixRuntime.api.request("POST", "/api/users", body);`
fn generate_mutation(m: &ApiMutationNode) -> String {
    let method = js_str(m.method.as_str());
    let url = js_str(&m.url);

    match &m.body {
        Some(body) => format!(
            "await KorlixRuntime.api.request({}, {}, {});",
            method,
            url,
            expr_to_api_js(body)
        ),
        None => format!(
            "await KorlixRuntime.api.request({}, {}, undefined);",
            method, url
        ),
    }
}

/// `await KorlixRuntime.api.reload("users");`
fn generate_reload(r: &ApiReloadNode) -> String {
    format!("await KorlixRuntime.api.reload({});", js_str(&r.target))
}

// ── Expression serialiser for API bodies ──────────────────────────────────

/// Convert an expression to JS suitable for an API request body.
/// Delegates to the shared `expr_to_js` from `js.rs` for consistency.
fn expr_to_api_js(expr: &Expr) -> String {
    expr_to_js(expr)
}

/// Format a Rust string as a JSON-safe JS string literal.
fn js_str(value: &str) -> String {
    format!("{:?}", value) // uses Rust's Debug formatting → "value" with escapes
}
