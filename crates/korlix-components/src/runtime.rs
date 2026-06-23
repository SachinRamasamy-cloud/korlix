use crate::schema::RuntimeFeature;
use korlix_ast::{
    node::Node,
    program::{Item, Module},
};
use std::collections::HashSet;

pub fn analyze_features(module: &Module) -> HashSet<RuntimeFeature> {
    let mut features = HashSet::new();
    features.insert(RuntimeFeature::Core);

    for item in &module.items {
        match item {
            Item::AppDecl(app) => {
                if !app.routes.is_empty() {
                    features.insert(RuntimeFeature::Router);
                }
                for p in &app.providers {
                    match p.as_str() {
                        "toast" => {
                            features.insert(RuntimeFeature::Toast);
                        }
                        "modal" => {
                            features.insert(RuntimeFeature::Overlay);
                        }
                        "theme" => {
                            features.insert(RuntimeFeature::Theme);
                        }
                        _ => {}
                    }
                }
            }
            Item::Page(p) => scan_nodes_features(&p.body, &mut features),
            Item::Layout(l) => scan_nodes_features(&l.body, &mut features),
            Item::Component(c) => scan_nodes_features(&c.body, &mut features),
            _ => {}
        }
    }
    features
}

fn scan_nodes_features(nodes: &[Node], features: &mut HashSet<RuntimeFeature>) {
    for node in nodes {
        match node {
            Node::Component(c) => {
                match c.name.as_str() {
                    "toast" => {
                        features.insert(RuntimeFeature::Toast);
                    }
                    "modal" | "drawer" | "tooltip" => {
                        features.insert(RuntimeFeature::Overlay);
                    }
                    "image" | "avatar" => {
                        features.insert(RuntimeFeature::Media);
                    }
                    "input" | "form" | "select" | "textarea" | "checkbox" | "switch" => {
                        features.insert(RuntimeFeature::Forms);
                    }
                    _ => {}
                }
                scan_nodes_features(&c.children, features);
            }
            Node::Element(el) => {
                for _ev in &el.events {
                    features.insert(RuntimeFeature::Core);
                }
                scan_nodes_features(&el.children, features);
            }
            Node::State(_) | Node::Derived(_) => {
                features.insert(RuntimeFeature::State);
            }
            Node::If(i) => {
                scan_nodes_features(&i.then_body, features);
                if let Some(e) = &i.else_body {
                    scan_nodes_features(e, features);
                }
            }
            Node::For(f) => scan_nodes_features(&f.body, features),
            _ => {}
        }
    }
}
