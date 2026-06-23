use korlix_ast::{
    declarations::StateDecl,
    expression::Expr,
    node::Node,
    program::{Item, Module, PageDecl},
};
use korlix_resolver::route_resolver::RouteEntry;
use std::collections::HashMap;

pub fn generate_app_js(module: &Module, routes: &HashMap<String, RouteEntry>) -> String {
    let mut js = String::new();
    let mut page_code: Vec<String> = vec![];

    for item in &module.items {
        match item {
            Item::AppDecl(_) => {
                js.push_str(&gen_route_manifest(routes));
            }
            Item::Page(p) => {
                page_code.push(gen_page_js(p));
            }
            _ => {}
        }
    }
    for code in page_code {
        js.push_str(&code);
    }
    js
}

fn gen_route_manifest(routes: &HashMap<String, RouteEntry>) -> String {
    let entries: Vec<String> = routes
        .values()
        .map(|r| format!("  {:?}: {{ id: {:?}, path: {:?} }}", r.path, r.id, r.path))
        .collect();
    format!(
        "window.__KORLIX_ROUTES__ = {{\n{}\n}};\n\n",
        entries.join(",\n")
    )
}

fn gen_page_js(page: &PageDecl) -> String {
    let mut js = String::new();
    let mut states = Vec::new();
    collect_states(&page.body, &mut states);

    if !states.is_empty() {
        js.push_str(&format!("// Page: {}\n(function() {{\n", page.name));
        js.push_str("  if (typeof KorlixRuntime === 'undefined') return;\n");
        js.push_str("  var __state = KorlixRuntime.createState({\n");
        for s in &states {
            js.push_str(&format!(
                "    {}: {},\n",
                s.name,
                expr_to_js_literal(&s.value)
            ));
        }
        js.push_str("  });\n");
        js.push_str("  window.__KORLIX_STATE__ = __state;\n");
        js.push_str("})();\n\n");
    }
    js
}

fn collect_states<'a>(nodes: &'a [Node], out: &mut Vec<&'a StateDecl>) {
    for node in nodes {
        match node {
            Node::State(s) => out.push(s),
            Node::Element(el) => collect_states(&el.children, out),
            Node::Component(c) => collect_states(&c.children, out),
            Node::If(i) => {
                collect_states(&i.then_body, out);
                if let Some(e) = &i.else_body {
                    collect_states(e, out);
                }
            }
            Node::For(f) => collect_states(&f.body, out),
            _ => {}
        }
    }
}

#[allow(dead_code)]
fn has_event_bindings(nodes: &[Node]) -> bool {
    nodes.iter().any(|node| match node {
        Node::Element(el) => !el.events.is_empty() || has_event_bindings(&el.children),
        Node::Component(c) => !c.events.is_empty() || has_event_bindings(&c.children),
        Node::If(i) => {
            has_event_bindings(&i.then_body)
                || i.else_body
                    .as_ref()
                    .map(|e| has_event_bindings(e))
                    .unwrap_or(false)
        }
        Node::For(f) => has_event_bindings(&f.body),
        _ => false,
    })
}

#[allow(dead_code)]
fn gen_event_bindings(nodes: &[Node], js: &mut String) {
    for node in nodes {
        match node {
            Node::Element(el) => {
                for ev in &el.events {
                    let handler = gen_handler_body(&ev.body);
                    js.push_str(&format!(
                        "  KorlixRuntime.bindEvent('[data-on-{}]', '{}', function(__state){{ {} }}, __state);\n",
                        ev.event, ev.event, handler
                    ));
                }
                gen_event_bindings(&el.children, js);
            }
            Node::Component(c) => {
                for ev in &c.events {
                    let handler = gen_handler_body(&ev.body);
                    js.push_str(&format!(
                        "  KorlixRuntime.bindEvent('[data-on-{}]', '{}', function(__state){{ {} }}, __state);\n",
                        ev.event, ev.event, handler
                    ));
                }
                gen_event_bindings(&c.children, js);
            }
            Node::If(i) => {
                gen_event_bindings(&i.then_body, js);
                if let Some(e) = &i.else_body {
                    gen_event_bindings(e, js);
                }
            }
            Node::For(f) => gen_event_bindings(&f.body, js),
            _ => {}
        }
    }
}

#[allow(dead_code)]
fn gen_handler_body(nodes: &[Node]) -> String {
    nodes
        .iter()
        .map(|n| match n {
            Node::Assign(a) => format!("__state.{}={};", a.target, expr_to_js_state(&a.value)),
            Node::Call(c) => {
                let args = c
                    .args
                    .iter()
                    .map(|e| expr_to_js(e))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("KorlixRuntime.call('{}', [{}]);", c.callee, args)
            }
            Node::Component(c) => {
                let args = c
                    .children
                    .iter()
                    .filter_map(|child| {
                        if let Node::Text(t) = child {
                            Some(expr_to_js(&t.value))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(",");
                format!("KorlixRuntime.call('{}', [{}]);", c.name, args)
            }
            _ => String::new(),
        })
        .collect()
}

pub fn expr_to_js(e: &Expr) -> String {
    match e {
        Expr::String(s) => format!("\"{}\"", s.replace('"', "\\\"")),
        Expr::Number(n) => n.to_string(),
        Expr::Bool(b) => b.to_string(),
        Expr::Null => "null".into(),
        Expr::Identifier(s) => s.clone(),
        Expr::Member { object, field } => format!("{}.{}", expr_to_js(object), field),
        Expr::Binary { left, op, right } => {
            use korlix_ast::expression::BinaryOp;
            let op_str = match op {
                BinaryOp::Add => "+",
                BinaryOp::Sub => "-",
                BinaryOp::Mul => "*",
                BinaryOp::Div => "/",
                BinaryOp::Mod => "%",
                BinaryOp::Eq => "===",
                BinaryOp::Ne => "!==",
                BinaryOp::Lt => "<",
                BinaryOp::Le => "<=",
                BinaryOp::Gt => ">",
                BinaryOp::Ge => ">=",
                BinaryOp::And => "&&",
                BinaryOp::Or => "||",
            };
            format!("({} {} {})", expr_to_js(left), op_str, expr_to_js(right))
        }
        Expr::Call { callee, args } => {
            let a = args.iter().map(expr_to_js).collect::<Vec<_>>().join(", ");
            format!("{}({})", expr_to_js(callee), a)
        }
        _ => "null".into(),
    }
}

fn expr_to_js_literal(e: &Expr) -> String {
    expr_to_js(e)
}

#[allow(dead_code)]
fn expr_to_js_state(e: &Expr) -> String {
    match e {
        Expr::Identifier(s) => format!("__state.{}", s),
        Expr::Member { object, field } => format!("{}.{}", expr_to_js_state(object), field),
        Expr::Binary { left, op, right } => {
            use korlix_ast::expression::BinaryOp;
            let op_str = match op {
                BinaryOp::Add => "+",
                BinaryOp::Sub => "-",
                BinaryOp::Mul => "*",
                BinaryOp::Div => "/",
                BinaryOp::Mod => "%",
                BinaryOp::Eq => "===",
                BinaryOp::Ne => "!==",
                BinaryOp::Lt => "<",
                BinaryOp::Le => "<=",
                BinaryOp::Gt => ">",
                BinaryOp::Ge => ">=",
                BinaryOp::And => "&&",
                BinaryOp::Or => "||",
            };
            format!(
                "({} {} {})",
                expr_to_js_state(left),
                op_str,
                expr_to_js_state(right)
            )
        }
        Expr::Call { callee, args } => {
            let a = args
                .iter()
                .map(expr_to_js_state)
                .collect::<Vec<_>>()
                .join(", ");
            format!("{}({})", expr_to_js_state(callee), a)
        }
        _ => expr_to_js(e),
    }
}
