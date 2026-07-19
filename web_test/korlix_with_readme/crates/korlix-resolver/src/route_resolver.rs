use korlix_ast::declarations::RouteDecl;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteEntry {
    pub id: String,
    pub path: String,
    pub source: String,
}

pub fn build_route_map(routes: &[RouteDecl]) -> HashMap<String, RouteEntry> {
    routes
        .iter()
        .map(|r| {
            let id = r
                .path
                .trim_matches('/')
                .replace('/', "_")
                .replace(':', "param_");
            let id = if id.is_empty() {
                "index".to_string()
            } else {
                id
            };
            (
                r.path.clone(),
                RouteEntry {
                    id,
                    path: r.path.clone(),
                    source: r.source.clone(),
                },
            )
        })
        .collect()
}
