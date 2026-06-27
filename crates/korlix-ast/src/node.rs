use crate::{
    declarations::{ActionDecl, DataDecl, DerivedDecl, LetDecl, StateDecl},
    element::{ComponentNode, ElementNode},
    expression::Expr,
};
use crate::api::{ApiMutationNode, ApiQueryNode, ApiReloadNode};
use korlix_core::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Node {
    // ── API ──────────────────────────────────────────────────────────────
    /// `get users "/api/users"` at page/component top-level
    ApiQuery(ApiQueryNode),
    /// `post "/api/url" body`, `put`, `patch`, `delete` inside action bodies
    ApiMutation(ApiMutationNode),
    /// `reload users` inside action bodies
    ApiReload(ApiReloadNode),

    // ── UI structure ─────────────────────────────────────────────────────
    Element(ElementNode),
    Component(ComponentNode),
    Text(TextNode),

    // ── Declarations ─────────────────────────────────────────────────────
    State(StateDecl),
    Let(LetDecl),
    Derived(DerivedDecl),
    Action(ActionDecl),
    Data(DataDecl),

    // ── Control flow ─────────────────────────────────────────────────────
    If(IfNode),
    For(ForNode),

    // ── Statements ───────────────────────────────────────────────────────
    Assign(AssignNode),
    Call(CallNode),
    Slot(SlotNode),
    Raw(RawNode),

    // ── Misc ─────────────────────────────────────────────────────────────
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
