//! Lightweight semantic validation for the stable Korlix V2 surface.
//!
//! This pass deliberately validates contracts that can be proven without a
//! full control-flow type engine: duplicate local declarations, literal type
//! compatibility, and required user-component props. It is designed to grow
//! into the complete type checker without moving validation into codegen.

use korlix_ast::{
    declarations::{LetDecl, StateDecl},
    expression::Expr,
    node::Node,
    program::{ComponentDecl, Item, Program},
    types::KType,
};
use std::collections::{HashMap, HashSet};

pub fn validate_semantics(
    program: &Program,
    user_components: &HashMap<String, ComponentDecl>,
) -> Vec<String> {
    let mut errors = Vec::new();
    for module in &program.modules {
        for item in &module.items {
            match item {
                Item::Page(page) => {
                    validate_scope(&page.name, &page.body, user_components, &mut errors)
                }
                Item::Layout(layout) => {
                    validate_scope(&layout.name, &layout.body, user_components, &mut errors)
                }
                Item::Component(component) => {
                    let mut names = HashSet::new();
                    for prop in &component.props {
                        if !names.insert(prop.name.clone()) {
                            errors.push(format!(
                                "KX-S111: Duplicate prop `{}` in component `{}`.",
                                prop.name, component.name
                            ));
                        }
                        if let Some(default) = &prop.default {
                            validate_type(
                                &format!("prop `{}`", prop.name),
                                &prop.type_ann,
                                default,
                                &mut errors,
                            );
                        }
                    }
                    validate_scope(
                        &component.name,
                        &component.body,
                        user_components,
                        &mut errors,
                    );
                }
                _ => {}
            }
        }
    }
    errors.sort();
    errors.dedup();
    errors
}

fn validate_scope(
    scope_name: &str,
    nodes: &[Node],
    user_components: &HashMap<String, ComponentDecl>,
    errors: &mut Vec<String>,
) {
    let mut declarations = HashSet::new();
    for node in nodes {
        let name = match node {
            Node::State(value) => Some(("state", value.name.as_str())),
            Node::Let(value) => Some(("let", value.name.as_str())),
            Node::Derived(value) => Some(("derived", value.name.as_str())),
            Node::Action(value) => Some(("function", value.name.as_str())),
            Node::ApiQuery(value) => Some(("query", value.name.as_str())),
            _ => None,
        };
        if let Some((kind, name)) = name {
            if !declarations.insert(name.to_string()) {
                errors.push(format!(
                    "KX-S110: Duplicate symbol `{name}` in `{scope_name}` ({kind})."
                ));
            }
        }

        match node {
            Node::State(value) => validate_state(value, errors),
            Node::Let(value) => validate_let(value, errors),
            _ => {}
        }
    }
    validate_nodes(nodes, user_components, errors);
}

fn validate_state(value: &StateDecl, errors: &mut Vec<String>) {
    if let Some(expected) = &value.type_ann {
        validate_type(
            &format!("state `{}`", value.name),
            expected,
            &value.value,
            errors,
        );
    }
}

fn validate_let(value: &LetDecl, errors: &mut Vec<String>) {
    if let Some(expected) = &value.type_ann {
        validate_type(
            &format!("let `{}`", value.name),
            expected,
            &value.value,
            errors,
        );
    }
}

fn validate_type(label: &str, expected: &KType, value: &Expr, errors: &mut Vec<String>) {
    let Some(actual) = literal_type(value) else {
        return;
    };
    if !type_compatible(expected, &actual) {
        errors.push(format!(
            "KX-T101: Type mismatch for {label}: expected `{expected}`, found `{actual}`."
        ));
    }
}

fn literal_type(value: &Expr) -> Option<KType> {
    match value {
        Expr::String(_) | Expr::Interpolated(_) => Some(KType::String),
        Expr::Number(value) => Some(if value.fract() == 0.0 {
            KType::Int
        } else {
            KType::Float
        }),
        Expr::Bool(_) => Some(KType::Bool),
        Expr::Null => Some(KType::Null),
        Expr::List(items) => {
            let item = items.iter().find_map(literal_type).unwrap_or(KType::Any);
            Some(KType::List(Box::new(item)))
        }
        Expr::Object(_) => Some(KType::Record),
        _ => None,
    }
}

fn type_compatible(expected: &KType, actual: &KType) -> bool {
    match (expected, actual) {
        (KType::Any | KType::Unknown | KType::Json, _) => true,
        (KType::Number | KType::Float, KType::Int | KType::Float | KType::Number) => true,
        (
            KType::String | KType::Email | KType::Url | KType::Color | KType::Image | KType::Icon,
            KType::String,
        ) => true,
        (KType::List(expected), KType::List(actual)) => type_compatible(expected, actual),
        (left, right) => left == right,
    }
}

fn validate_nodes(
    nodes: &[Node],
    user_components: &HashMap<String, ComponentDecl>,
    errors: &mut Vec<String>,
) {
    for node in nodes {
        match node {
            Node::Component(call) => {
                if let Some(component) = user_components.get(&call.name) {
                    let provided: HashSet<&str> =
                        call.props.iter().map(|prop| prop.key.as_str()).collect();
                    for prop in &component.props {
                        if prop.required && !provided.contains(prop.name.as_str()) {
                            errors.push(format!(
                                "KX-S210: Component `{}` requires prop `{}`.",
                                call.name, prop.name
                            ));
                        }
                    }
                }
                validate_nodes(&call.children, user_components, errors);
                for event in &call.events {
                    validate_nodes(&event.body, user_components, errors);
                }
            }
            Node::Element(element) => {
                validate_nodes(&element.children, user_components, errors);
                for event in &element.events {
                    validate_nodes(&event.body, user_components, errors);
                }
            }
            Node::If(statement) => {
                validate_nodes(&statement.then_body, user_components, errors);
                if let Some(else_body) = &statement.else_body {
                    validate_nodes(else_body, user_components, errors);
                }
            }
            Node::For(statement) => validate_nodes(&statement.body, user_components, errors),
            Node::Action(action) => validate_nodes(&action.body, user_components, errors),
            _ => {}
        }
    }
}
