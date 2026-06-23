use crate::expression::Expr;
use korlix_core::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassRef {
    pub name: String,
    pub span: Span,
}

impl ClassRef {
    pub fn new(name: impl Into<String>, span: Span) -> Self {
        Self { name: name.into(), span }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prop {
    pub key: String,
    pub value: Expr,
    pub span: Span,
}

impl Prop {
    pub fn new(key: impl Into<String>, value: Expr, span: Span) -> Self {
        Self { key: key.into(), value, span }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventHandler {
    pub event: String,
    pub body: Vec<super::node::Node>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotFill {
    pub slot_name: Option<String>,
    pub children: Vec<super::node::Node>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementNode {
    pub tag: String,
    pub classes: Vec<ClassRef>,
    pub props: Vec<Prop>,
    pub events: Vec<EventHandler>,
    pub children: Vec<super::node::Node>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentNode {
    pub name: String,
    pub classes: Vec<ClassRef>,
    pub props: Vec<Prop>,
    pub slots: Vec<SlotFill>,
    pub events: Vec<EventHandler>,
    pub children: Vec<super::node::Node>,
    pub span: Span,
}
