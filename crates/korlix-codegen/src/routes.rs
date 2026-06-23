use korlix_resolver::route_resolver::RouteEntry;
use serde_json::{json, Value};
use std::collections::HashMap;

pub fn generate_route_manifest(routes: &HashMap<String, RouteEntry>) -> String {
    let obj: Value = routes
        .iter()
        .map(|(k, v)| {
            (
                k.clone(),
                json!({ "id": v.id, "path": v.path, "source": v.source }),
            )
        })
        .collect::<serde_json::Map<_, _>>()
        .into();
    serde_json::to_string_pretty(&obj).unwrap_or_default()
}
