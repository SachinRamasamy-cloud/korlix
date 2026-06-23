use korlix_ast::{element::ClassRef, node::Node, program::{Item, Module}};
use std::collections::HashSet;

/// Walk the full AST and collect all class references used.
pub fn scan_classes(module: &Module) -> HashSet<String> {
    let mut found = HashSet::new();
    for item in &module.items {
        scan_item(item, &mut found);
    }
    found
}

fn scan_item(item: &Item, out: &mut HashSet<String>) {
    match item {
        Item::Page(p)      => scan_nodes(&p.body, out),
        Item::Layout(l)    => scan_nodes(&l.body, out),
        Item::Component(c) => scan_nodes(&c.body, out),
        Item::AppDecl(_)   => {}
        Item::MountDecl(_) => {}
        Item::ThemeDecl(_) => {}
    }
}

fn scan_nodes(nodes: &[Node], out: &mut HashSet<String>) {
    for node in nodes {
        scan_node(node, out);
    }
}

fn scan_node(node: &Node, out: &mut HashSet<String>) {
    match node {
        Node::Element(el) => {
            scan_classes_list(&el.classes, out);
            scan_nodes(&el.children, out);
            for ev in &el.events { scan_nodes(&ev.body, out); }
        }
        Node::Component(c) => {
            scan_classes_list(&c.classes, out);
            scan_nodes(&c.children, out);
        }
        Node::If(i) => {
            scan_nodes(&i.then_body, out);
            if let Some(e) = &i.else_body { scan_nodes(e, out); }
        }
        Node::For(f)    => scan_nodes(&f.body, out),
        Node::Action(a) => scan_nodes(&a.body, out),
        _ => {}
    }
}

fn scan_classes_list(classes: &[ClassRef], out: &mut HashSet<String>) {
    for c in classes { out.insert(c.name.clone()); }
}
