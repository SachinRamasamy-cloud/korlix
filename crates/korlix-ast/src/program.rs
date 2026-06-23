use crate::declarations::{ImportDecl, PropDecl, RouteDecl, ThemeDecl, MetaBlock};
use crate::node::Node;
use korlix_core::Span;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub modules: Vec<Module>,
}

impl Program {
    pub fn new() -> Self {
        Self { modules: vec![] }
    }

    pub fn add_module(&mut self, m: Module) {
        self.modules.push(m);
    }
}

impl Default for Program {
    fn default() -> Self { Self::new() }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub file_id: u32,
    pub path: PathBuf,
    pub imports: Vec<ImportDecl>,
    pub items: Vec<Item>,
}

impl Module {
    pub fn new(file_id: u32, path: PathBuf) -> Self {
        Self { file_id, path, imports: vec![], items: vec![] }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Item {
    MountDecl(MountDecl),
    AppDecl(AppDecl),
    Page(PageDecl),
    Layout(LayoutDecl),
    Component(ComponentDecl),
    ThemeDecl(ThemeDecl),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountDecl {
    pub component: String,
    pub selector: String,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppDecl {
    pub layout: Option<String>,
    pub routes: Vec<RouteDecl>,
    pub providers: Vec<String>,
    pub theme: Option<ThemeDecl>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageDecl {
    pub name: String,
    pub route: Option<String>,
    pub layout: Option<String>,
    pub meta: Option<MetaBlock>,
    pub body: Vec<Node>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutDecl {
    pub name: String,
    pub body: Vec<Node>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentDecl {
    pub name: String,
    pub props: Vec<PropDecl>,
    pub body: Vec<Node>,
    pub span: Span,
}
