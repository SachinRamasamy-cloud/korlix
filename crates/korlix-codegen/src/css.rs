use korlix_style::{generator::generate_css, scanner::scan_classes};
use korlix_ast::program::Module;
use std::collections::HashSet;

pub fn generate_css_for_module(module: &Module) -> String {
    let used = scan_classes(module);
    generate_css(&used)
}

pub fn generate_css_for_classes(classes: &HashSet<String>) -> String {
    generate_css(classes)
}
