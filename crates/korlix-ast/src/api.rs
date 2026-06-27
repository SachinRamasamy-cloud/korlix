use serde::{Deserialize, Serialize};

use crate::expression::Expr;
use korlix_core::Span;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Options,
    Head,
}

impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Options => "OPTIONS",
            HttpMethod::Head => "HEAD",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "get" => Some(HttpMethod::Get),
            "post" => Some(HttpMethod::Post),
            "put" => Some(HttpMethod::Put),
            "delete" => Some(HttpMethod::Delete),
            "patch" => Some(HttpMethod::Patch),
            "options" => Some(HttpMethod::Options),
            "head" => Some(HttpMethod::Head),
            _ => None,
        }
    }
}

/// `get users "/api/users"` — declares a named GET query bound to a URL.
/// The runtime exposes `users`, `usersLoading`, and `usersError` as reactive state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiQueryNode {
    pub name: String,
    pub url: String,
    pub span: Span,
}

/// `post "/api/users" { name: name, email: email }` — a write mutation inside an action body.
/// Also handles `put`, `patch`, `delete`. Body is optional (DELETE typically has none).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMutationNode {
    pub method: HttpMethod,
    pub url: String,
    pub body: Option<Expr>,
    pub span: Span,
}

/// `reload users` — re-fetches the named query (must have been declared via `get`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiReloadNode {
    pub target: String,
    pub span: Span,
}