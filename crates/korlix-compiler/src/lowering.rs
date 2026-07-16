use korlix_ast::{
    declarations::PropDecl,
    element::{ComponentNode, ElementNode, EventHandler, Prop},
    expression::{Expr, StringPart},
    node::{AssignNode, CallNode, ForNode, IfNode, Node, TextNode},
    program::{ComponentDecl, Item, Program},
};
use korlix_components::{expander::expand_component, registry::is_component};
use std::collections::{HashMap, HashSet};

pub fn collect_user_components(program: &Program) -> HashMap<String, ComponentDecl> {
    let mut components = HashMap::new();
    for module in &program.modules {
        for item in &module.items {
            if let Item::Component(component) = item {
                components.insert(component.name.clone(), component.clone());
            }
        }
    }
    components
}

pub fn validate_component_names(
    nodes: &[Node],
    user_components: &HashMap<String, ComponentDecl>,
    errors: &mut Vec<String>,
) {
    for node in nodes {
        match node {
            Node::Component(component) => {
                if !is_component(&component.name) && !user_components.contains_key(&component.name)
                {
                    errors.push(format!(
                        "KX-S201: Unknown component `{}`. Declare it with `component {}` or use a registered component.",
                        component.name, component.name
                    ));
                }
                validate_component_names(&component.children, user_components, errors);
                for event in &component.events {
                    validate_component_names(&event.body, user_components, errors);
                }
            }
            Node::Element(element) => {
                validate_component_names(&element.children, user_components, errors);
                for event in &element.events {
                    validate_component_names(&event.body, user_components, errors);
                }
            }
            Node::If(statement) => {
                validate_component_names(&statement.then_body, user_components, errors);
                if let Some(else_body) = &statement.else_body {
                    validate_component_names(else_body, user_components, errors);
                }
            }
            Node::For(statement) => {
                validate_component_names(&statement.body, user_components, errors)
            }
            Node::Action(action) => validate_component_names(&action.body, user_components, errors),
            _ => {}
        }
    }
}

pub fn lower_nodes(nodes: &[Node], user_components: &HashMap<String, ComponentDecl>) -> Vec<Node> {
    lower_nodes_inner(nodes, user_components, &mut HashSet::new(), 0)
}

fn lower_nodes_inner(
    nodes: &[Node],
    user_components: &HashMap<String, ComponentDecl>,
    stack: &mut HashSet<String>,
    depth: usize,
) -> Vec<Node> {
    if depth > 64 {
        return vec![];
    }

    let mut result = Vec::new();
    for node in nodes {
        match node {
            Node::Component(component) if user_components.contains_key(&component.name) => {
                if stack.contains(&component.name) {
                    continue;
                }
                stack.insert(component.name.clone());
                let expanded = expand_user_component(component, &user_components[&component.name]);
                result.extend(lower_nodes_inner(
                    &expanded,
                    user_components,
                    stack,
                    depth + 1,
                ));
                stack.remove(&component.name);
            }
            Node::Component(component) => {
                let expanded = expand_component(component);
                result.extend(lower_nodes_inner(
                    &[expanded],
                    user_components,
                    stack,
                    depth + 1,
                ));
            }
            Node::Element(element) => {
                let mut lowered = element.clone();
                lowered.children =
                    lower_nodes_inner(&element.children, user_components, stack, depth + 1);
                lowered.events = lower_events(&element.events, user_components, stack, depth + 1);
                result.push(Node::Element(lowered));
            }
            Node::If(statement) => {
                result.push(Node::If(IfNode {
                    condition: statement.condition.clone(),
                    then_body: lower_nodes_inner(
                        &statement.then_body,
                        user_components,
                        stack,
                        depth + 1,
                    ),
                    else_body: statement
                        .else_body
                        .as_ref()
                        .map(|body| lower_nodes_inner(body, user_components, stack, depth + 1)),
                    span: statement.span,
                }));
            }
            Node::For(statement) => {
                result.push(Node::For(ForNode {
                    var: statement.var.clone(),
                    iterable: statement.iterable.clone(),
                    body: lower_nodes_inner(&statement.body, user_components, stack, depth + 1),
                    span: statement.span,
                }));
            }
            Node::Action(action) => {
                let mut lowered = action.clone();
                lowered.body = lower_nodes_inner(&action.body, user_components, stack, depth + 1);
                result.push(Node::Action(lowered));
            }
            other => result.push(other.clone()),
        }
    }
    result
}

fn lower_events(
    events: &[EventHandler],
    user_components: &HashMap<String, ComponentDecl>,
    stack: &mut HashSet<String>,
    depth: usize,
) -> Vec<EventHandler> {
    events
        .iter()
        .map(|event| EventHandler {
            event: event.event.clone(),
            body: lower_nodes_inner(&event.body, user_components, stack, depth),
            span: event.span,
        })
        .collect()
}

fn expand_user_component(call: &ComponentNode, declaration: &ComponentDecl) -> Vec<Node> {
    let values = component_prop_values(call, &declaration.props);
    let mut body = substitute_nodes(&declaration.body, &values, &call.children);

    if body.len() == 1 {
        merge_call_surface(&mut body[0], call);
        return body;
    }

    if call.classes.is_empty() && call.props.is_empty() && call.events.is_empty() {
        return body;
    }

    vec![Node::Element(ElementNode {
        tag: "div".into(),
        classes: call.classes.clone(),
        props: call.props.clone(),
        events: call.events.clone(),
        children: body,
        span: call.span,
    })]
}

fn component_prop_values(call: &ComponentNode, declarations: &[PropDecl]) -> HashMap<String, Expr> {
    let mut values = HashMap::new();
    for declaration in declarations {
        if let Some(default) = &declaration.default {
            values.insert(declaration.name.clone(), default.clone());
        }
    }
    for prop in &call.props {
        values.insert(prop.key.clone(), prop.value.clone());
    }
    values
}

fn merge_call_surface(node: &mut Node, call: &ComponentNode) {
    match node {
        Node::Element(element) => {
            element.classes.extend(call.classes.clone());
            element.props.extend(call.props.clone());
            element.events.extend(call.events.clone());
        }
        Node::Component(component) => {
            component.classes.extend(call.classes.clone());
            component.props.extend(call.props.clone());
            component.events.extend(call.events.clone());
        }
        _ => {}
    }
}

fn substitute_nodes(
    nodes: &[Node],
    values: &HashMap<String, Expr>,
    default_slot: &[Node],
) -> Vec<Node> {
    let mut result = Vec::new();
    for node in nodes {
        match node {
            Node::Slot(slot) if slot.name.is_none() => result.extend(default_slot.to_vec()),
            Node::Slot(_) => {}
            Node::Text(text) => result.push(Node::Text(TextNode {
                value: substitute_expr(&text.value, values),
                span: text.span,
            })),
            Node::Element(element) => result.push(Node::Element(ElementNode {
                tag: element.tag.clone(),
                classes: element.classes.clone(),
                props: substitute_props(&element.props, values),
                events: substitute_events(&element.events, values, default_slot),
                children: substitute_nodes(&element.children, values, default_slot),
                span: element.span,
            })),
            Node::Component(component) => result.push(Node::Component(ComponentNode {
                name: component.name.clone(),
                classes: component.classes.clone(),
                props: substitute_props(&component.props, values),
                slots: component.slots.clone(),
                events: substitute_events(&component.events, values, default_slot),
                children: substitute_nodes(&component.children, values, default_slot),
                span: component.span,
            })),
            Node::If(statement) => result.push(Node::If(IfNode {
                condition: substitute_expr(&statement.condition, values),
                then_body: substitute_nodes(&statement.then_body, values, default_slot),
                else_body: statement
                    .else_body
                    .as_ref()
                    .map(|body| substitute_nodes(body, values, default_slot)),
                span: statement.span,
            })),
            Node::For(statement) => result.push(Node::For(ForNode {
                var: statement.var.clone(),
                iterable: substitute_expr(&statement.iterable, values),
                body: substitute_nodes(&statement.body, values, default_slot),
                span: statement.span,
            })),
            Node::Assign(assign) => result.push(Node::Assign(AssignNode {
                target: assign.target.clone(),
                value: substitute_expr(&assign.value, values),
                span: assign.span,
            })),
            Node::Call(call) => result.push(Node::Call(CallNode {
                callee: call.callee.clone(),
                args: call
                    .args
                    .iter()
                    .map(|expr| substitute_expr(expr, values))
                    .collect(),
                span: call.span,
            })),
            other => result.push(other.clone()),
        }
    }
    result
}

fn substitute_events(
    events: &[EventHandler],
    values: &HashMap<String, Expr>,
    default_slot: &[Node],
) -> Vec<EventHandler> {
    events
        .iter()
        .map(|event| EventHandler {
            event: event.event.clone(),
            body: substitute_nodes(&event.body, values, default_slot),
            span: event.span,
        })
        .collect()
}

fn substitute_props(props: &[Prop], values: &HashMap<String, Expr>) -> Vec<Prop> {
    props
        .iter()
        .map(|prop| Prop {
            key: prop.key.clone(),
            value: substitute_expr(&prop.value, values),
            span: prop.span,
        })
        .collect()
}

fn substitute_expr(expr: &Expr, values: &HashMap<String, Expr>) -> Expr {
    match expr {
        Expr::Identifier(name) => values.get(name).cloned().unwrap_or_else(|| expr.clone()),
        Expr::String(value) => substitute_string(value, values),
        Expr::List(items) => Expr::List(
            items
                .iter()
                .map(|item| substitute_expr(item, values))
                .collect(),
        ),
        Expr::Object(entries) => Expr::Object(
            entries
                .iter()
                .map(|(key, value)| (key.clone(), substitute_expr(value, values)))
                .collect(),
        ),
        Expr::Binary { left, op, right } => Expr::Binary {
            left: Box::new(substitute_expr(left, values)),
            op: op.clone(),
            right: Box::new(substitute_expr(right, values)),
        },
        Expr::Unary { op, operand } => Expr::Unary {
            op: op.clone(),
            operand: Box::new(substitute_expr(operand, values)),
        },
        Expr::Call { callee, args } => Expr::Call {
            callee: Box::new(substitute_expr(callee, values)),
            args: args
                .iter()
                .map(|arg| substitute_expr(arg, values))
                .collect(),
        },
        Expr::Member { object, field } => Expr::Member {
            object: Box::new(substitute_expr(object, values)),
            field: field.clone(),
        },
        Expr::Index { object, index } => Expr::Index {
            object: Box::new(substitute_expr(object, values)),
            index: Box::new(substitute_expr(index, values)),
        },
        Expr::Ternary {
            condition,
            then_expr,
            else_expr,
        } => Expr::Ternary {
            condition: Box::new(substitute_expr(condition, values)),
            then_expr: Box::new(substitute_expr(then_expr, values)),
            else_expr: Box::new(substitute_expr(else_expr, values)),
        },
        Expr::Interpolated(parts) => Expr::Interpolated(
            parts
                .iter()
                .map(|part| match part {
                    StringPart::Literal(value) => StringPart::Literal(value.clone()),
                    StringPart::Expr(value) => StringPart::Expr(substitute_expr(value, values)),
                })
                .collect(),
        ),
        _ => expr.clone(),
    }
}

fn substitute_string(value: &str, values: &HashMap<String, Expr>) -> Expr {
    if value.starts_with('{') && value.ends_with('}') && value.len() > 2 {
        let name = &value[1..value.len() - 1];
        if let Some(value) = values.get(name) {
            return value.clone();
        }
    }

    let mut result = value.to_string();
    for (name, replacement) in values {
        let placeholder = format!("{{{name}}}");
        let literal = match replacement {
            Expr::String(value) => Some(value.clone()),
            Expr::Number(value) => Some(value.to_string()),
            Expr::Bool(value) => Some(value.to_string()),
            _ => None,
        };
        if let Some(literal) = literal {
            result = result.replace(&placeholder, &literal);
        }
    }
    Expr::String(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use korlix_core::Span;

    #[test]
    fn substitutes_user_component_props_and_slots() {
        let span = Span::default();
        let declaration = ComponentDecl {
            name: "hello-card".into(),
            props: vec![PropDecl {
                name: "name".into(),
                type_ann: korlix_ast::types::KType::String,
                default: None,
                required: true,
                span,
            }],
            body: vec![Node::Element(ElementNode {
                tag: "h2".into(),
                classes: vec![],
                props: vec![],
                events: vec![],
                children: vec![Node::Text(TextNode {
                    value: Expr::Identifier("name".into()),
                    span,
                })],
                span,
            })],
            span,
        };
        let call = ComponentNode {
            name: "hello-card".into(),
            classes: vec![],
            props: vec![Prop::new("name", Expr::String("Korlix".into()), span)],
            slots: vec![],
            events: vec![],
            children: vec![],
            span,
        };
        let expanded = expand_user_component(&call, &declaration);
        match &expanded[0] {
            Node::Element(element) => match &element.children[0] {
                Node::Text(text) => assert_eq!(text.value.as_string(), Some("Korlix")),
                _ => panic!("expected text"),
            },
            _ => panic!("expected element"),
        }
    }
}
