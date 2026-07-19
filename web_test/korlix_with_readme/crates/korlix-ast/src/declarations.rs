use crate::{element::Prop, expression::Expr, node::Node, types::KType};
use korlix_core::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportDecl {
    pub name: Option<String>,
    pub path: String,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropDecl {
    pub name: String,
    pub type_ann: KType,
    pub default: Option<Expr>,
    pub required: bool,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateDecl {
    pub name: String,
    pub type_ann: Option<KType>,
    pub value: Expr,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LetDecl {
    pub name: String,
    pub type_ann: Option<KType>,
    pub value: Expr,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivedDecl {
    pub name: String,
    pub value: Expr,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionDecl {
    pub name: String,
    pub params: Vec<PropDecl>,
    pub body: Vec<Node>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataDecl {
    pub name: String,
    pub method: String,
    pub url: Expr,
    pub loading: Option<Vec<Node>>,
    pub error: Option<Vec<Node>>,
    pub empty: Option<Vec<Node>>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaBlock {
    pub title: Option<Expr>,
    pub description: Option<Expr>,
    pub canonical: Option<Expr>,
    pub og_image: Option<Expr>,
    pub extras: Vec<Prop>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteDecl {
    pub path: String,
    pub source: String,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeDecl {
    pub default_mode: Option<String>,
    pub dark_enabled: bool,
    pub span: Span,
}
