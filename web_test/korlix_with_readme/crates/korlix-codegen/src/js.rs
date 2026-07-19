use korlix_ast::{
    declarations::StateDecl,
    expression::{Expr, StringPart},
    node::Node,
    program::{Item, Module, PageDecl},
};
use korlix_resolver::route_resolver::RouteEntry;
use std::collections::{HashMap, HashSet};

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

    let api_init = generate_api_init_from_nodes(&page.body);
    let mut actions = Vec::new();
    collect_actions(&page.body, &mut actions);

    if !states.is_empty() || !api_init.is_empty() || !actions.is_empty() {
        let route = page.route.as_deref().unwrap_or("/");
        js.push_str(&format!("// Page: {}\n(function() {{\n", page.name));
        js.push_str("  if (typeof KorlixRuntime === 'undefined') return;\n");
        js.push_str(&format!("  var __route = {:?};\n", route));
        js.push_str(r"  var __path = window.location.pathname.replace(/\/$/, '') || '/';");
        js.push('\n');
        js.push_str(r"  var __expected = __route.replace(/\/$/, '') || '/';");
        js.push('\n');
        js.push_str("  if (__path !== __expected) return;\n");

        if !states.is_empty() || !api_init.is_empty() || !actions.is_empty() {
            js.push_str("  var __state = KorlixRuntime.createState({\n");
            for s in &states {
                js.push_str(&format!(
                    "    {}: {},\n",
                    s.name,
                    expr_to_js_literal(&s.value)
                ));
            }
            js.push_str("  });\n");
            js.push_str(
                "  window.__KORLIX_STATE_BY_PAGE__ = window.__KORLIX_STATE_BY_PAGE__ || {};\n",
            );
            js.push_str(&format!(
                "  window.__KORLIX_STATE_BY_PAGE__[{:?}] = __state;\n",
                page.name
            ));
            js.push_str("  window.__KORLIX_STATE__ = __state;\n");
        }

        if !actions.is_empty() {
            js.push_str("\n  // Page actions\n");
            for act in &actions {
                js.push_str(&format!("  {}\n", act));
            }
        }

        if !api_init.is_empty() {
            js.push_str("\n  // Korlix API queries\n");
            for line in api_init.lines() {
                js.push_str("  ");
                js.push_str(line);
                js.push('\n');
            }
        }

        js.push_str("})();\n\n");
    }
    js
}

fn collect_actions(nodes: &[Node], out: &mut Vec<String>) {
    for node in nodes {
        if let Node::Action(action) = node {
            let mut locals: HashSet<String> = action
                .params
                .iter()
                .map(|param| param.name.clone())
                .collect();
            let body_js = gen_handler_body(&action.body, &mut locals);
            let params = action
                .params
                .iter()
                .map(|param| param.name.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            out.push(format!(
                "window.{} = async function({}) {{\n    {}\n  }};",
                action.name, params, body_js
            ));
        }
    }
}

/// Collect API query registrations from a flat node list (top-level of a page body).
fn generate_api_init_from_nodes(nodes: &[Node]) -> String {
    use crate::api::generate_api_init;
    use korlix_ast::program::{Item, Module, PageDecl};
    use korlix_core::Span;
    use std::path::PathBuf;

    // Build a minimal synthetic module containing just these nodes so we
    // can reuse the shared generate_api_init logic.
    let mut module = Module::new(0, PathBuf::new());
    module.items.push(Item::Page(PageDecl {
        name: "__synthetic".into(),
        route: None,
        layout: None,
        meta: None,
        body: nodes.to_vec(),
        span: Span::default(),
    }));
    generate_api_init(&module)
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
                    let handler = gen_handler_body(&ev.body, &mut HashSet::new());
                    js.push_str(&format!(
                        "  KorlixRuntime.bindEvent('[data-on-{}]', '{}', function(__state){{ {} }}, __state);\n",
                        ev.event, ev.event, handler
                    ));
                }
                gen_event_bindings(&el.children, js);
            }
            Node::Component(c) => {
                for ev in &c.events {
                    let handler = gen_handler_body(&ev.body, &mut HashSet::new());
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

fn gen_handler_body(nodes: &[Node], locals: &mut HashSet<String>) -> String {
    let mut output = String::new();
    for node in nodes {
        let statement = match node {
            Node::Let(declaration) => {
                let value = expr_to_js_scoped(&declaration.value, locals);
                locals.insert(declaration.name.clone());
                format!("let {} = {};", declaration.name, value)
            }
            Node::Assign(assignment) => {
                let target = if locals.contains(&assignment.target) {
                    assignment.target.clone()
                } else {
                    format!("__state.{}", assignment.target)
                };
                format!(
                    "{} = {};",
                    target,
                    expr_to_js_scoped(&assignment.value, locals)
                )
            }
            Node::Call(call) => {
                let args = call
                    .args
                    .iter()
                    .map(|expr| expr_to_js_scoped(expr, locals))
                    .collect::<Vec<_>>()
                    .join(", ");
                if let Some(method) = call.callee.strip_prefix("api.") {
                    format!("await KorlixRuntime.api.{}({});", method, args)
                } else {
                    format!("KorlixRuntime.call({:?}, [{}]);", call.callee, args)
                }
            }
            Node::ApiMutation(mutation) => {
                let body = mutation
                    .body
                    .as_ref()
                    .map(|body| expr_to_js_scoped(body, locals))
                    .unwrap_or_else(|| "undefined".into());
                format!(
                    "await KorlixRuntime.api.request({:?}, {:?}, {});",
                    mutation.method.as_str(),
                    mutation.url,
                    body
                )
            }
            Node::ApiReload(reload) => {
                format!("await KorlixRuntime.api.reload({:?});", reload.target)
            }
            Node::If(statement) => {
                let mut then_locals = locals.clone();
                let then_body = gen_handler_body(&statement.then_body, &mut then_locals);
                let else_body = statement.else_body.as_ref().map(|body| {
                    let mut else_locals = locals.clone();
                    gen_handler_body(body, &mut else_locals)
                });
                if let Some(else_body) = else_body {
                    format!(
                        "if ({}) {{ {} }} else {{ {} }}",
                        expr_to_js_scoped(&statement.condition, locals),
                        then_body,
                        else_body
                    )
                } else {
                    format!(
                        "if ({}) {{ {} }}",
                        expr_to_js_scoped(&statement.condition, locals),
                        then_body
                    )
                }
            }
            Node::For(statement) => {
                let mut loop_locals = locals.clone();
                loop_locals.insert(statement.var.clone());
                let body = gen_handler_body(&statement.body, &mut loop_locals);
                format!(
                    "for (const {} of {}) {{ {} }}",
                    statement.var,
                    expr_to_js_scoped(&statement.iterable, locals),
                    body
                )
            }
            Node::Component(component) => {
                let args = component
                    .children
                    .iter()
                    .filter_map(|child| match child {
                        Node::Text(text) => Some(expr_to_js_scoped(&text.value, locals)),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("KorlixRuntime.call({:?}, [{}]);", component.name, args)
            }
            _ => String::new(),
        };
        if !statement.is_empty() {
            output.push_str(&statement);
            output.push('\n');
        }
    }
    output
}

fn expr_to_js_scoped(expr: &Expr, locals: &HashSet<String>) -> String {
    match expr {
        Expr::Identifier(name) => {
            if name == "event" || locals.contains(name) {
                name.clone()
            } else {
                format!("__state.{}", name)
            }
        }
        Expr::Interpolated(parts) => parts
            .iter()
            .map(|part| match part {
                StringPart::Literal(value) => format!("{:?}", value),
                StringPart::Expr(value) => format!("String({})", expr_to_js_scoped(value, locals)),
            })
            .collect::<Vec<_>>()
            .join(" + "),
        Expr::Member { object, field } => {
            format!("{}.{}", expr_to_js_scoped(object, locals), field)
        }
        Expr::Index { object, index } => format!(
            "{}[{}]",
            expr_to_js_scoped(object, locals),
            expr_to_js_scoped(index, locals)
        ),
        Expr::Binary { left, op, right } => {
            use korlix_ast::expression::BinaryOp;
            let operator = match op {
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
                expr_to_js_scoped(left, locals),
                operator,
                expr_to_js_scoped(right, locals)
            )
        }
        Expr::Unary { op, operand } => {
            let operator = match op {
                korlix_ast::expression::UnaryOp::Not => "!",
                korlix_ast::expression::UnaryOp::Neg => "-",
            };
            format!("{}{}", operator, expr_to_js_scoped(operand, locals))
        }
        Expr::Call { callee, args } => format!(
            "{}({})",
            expr_to_js_scoped(callee, locals),
            args.iter()
                .map(|arg| expr_to_js_scoped(arg, locals))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Expr::Object(entries) => format!(
            "{{ {} }}",
            entries
                .iter()
                .map(|(key, value)| format!("{}: {}", key, expr_to_js_scoped(value, locals)))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Expr::List(items) => format!(
            "[{}]",
            items
                .iter()
                .map(|item| expr_to_js_scoped(item, locals))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        _ => expr_to_js(expr),
    }
}

pub fn expr_to_js(e: &Expr) -> String {
    match e {
        Expr::String(s) => format!("\"{}\"", s.replace('"', "\\\"")),
        Expr::Number(n) => n.to_string(),
        Expr::Bool(b) => b.to_string(),
        Expr::Null => "null".into(),
        Expr::Interpolated(parts) => parts
            .iter()
            .map(|part| match part {
                StringPart::Literal(value) => format!("{:?}", value),
                StringPart::Expr(value) => format!("String({})", expr_to_js(value)),
            })
            .collect::<Vec<_>>()
            .join(" + "),
        Expr::Identifier(s) => s.clone(),
        Expr::Member { object, field } => format!("{}.{}", expr_to_js(object), field),
        Expr::Index { object, index } => {
            format!("{}[{}]", expr_to_js(object), expr_to_js(index))
        }
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
        Expr::Object(pairs) => {
            let p: Vec<String> = pairs
                .iter()
                .map(|(k, v)| format!("{}: {}", k, expr_to_js(v)))
                .collect();
            format!("{{ {} }}", p.join(", "))
        }
        Expr::List(items) => {
            let it: Vec<String> = items.iter().map(|item| expr_to_js(item)).collect();
            format!("[{}]", it.join(", "))
        }
        _ => "null".into(),
    }
}

fn expr_to_js_literal(e: &Expr) -> String {
    expr_to_js(e)
}

#[allow(dead_code)]
pub fn expr_to_js_state(expr: &Expr) -> String {
    expr_to_js_scoped(expr, &HashSet::new())
}
