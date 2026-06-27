use crate::{
    declarations::{ActionDecl, DataDecl, DerivedDecl, LetDecl, StateDecl},
    element::{ComponentNode, ElementNode},
    expression::Expr,
};
use korlix_core::Span;
use serde::{Deserialize, Serialize};

use create :: api::{ApiMutation, ApiQueryNode, ApiReloadNode, ApiRouteNode, HttpMethod};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Node {
    Api(ApiMutation),
    ApiQuery(ApiQueryNode),
    ApiReload(ApiReloadNode),
    ApiRoute(ApiRouteNode),
    HttpMethod(HttpMethod),
    Element(ElementNode),
    Component(ComponentNode),
    Text(TextNode),
    State(StateDecl),
    Let(LetDecl),
    Derived(DerivedDecl),
    If(IfNode),
    For(ForNode),
    Action(ActionDecl),
    Data(DataDecl),
    Assign(AssignNode),
    Call(CallNode),
    Slot(SlotNode),
    Raw(RawNode),
    Comment(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextNode {
    pub value: Expr,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IfNode {
    pub condition: Expr,
    pub then_body: Vec<Node>,
    pub else_body: Option<Vec<Node>>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForNode {
    pub var: String,
    pub iterable: Expr,
    pub body: Vec<Node>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignNode {
    pub target: String,
    pub value: Expr,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallNode {
    pub callee: String,
    pub args: Vec<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotNode {
    pub name: Option<String>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawNode {
    pub html: String,
    pub span: Span,
}
