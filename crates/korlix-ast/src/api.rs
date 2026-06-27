use serde :: {Deserialize, Serialize};

use create:: expression::Expr;
use Korlix_core::span::Span;

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