pub use crate::feature::RuntimeFeature;
use std::collections::HashSet;
pub fn required_features(providers: &[String], has_routes: bool, has_state: bool) -> HashSet<RuntimeFeature> {
    let mut f = HashSet::new();
    f.insert(RuntimeFeature::Core);
    if has_routes { f.insert(RuntimeFeature::Router); }
    if has_state  { f.insert(RuntimeFeature::State); }
    for p in providers {
        match p.as_str() {
            "toast"  => { f.insert(RuntimeFeature::Toast); }
            "modal"  => { f.insert(RuntimeFeature::Overlay); }
            "theme"  => { f.insert(RuntimeFeature::Theme); }
            "motion" => { f.insert(RuntimeFeature::Motion); }
            _ => {}
        }
    }
    f
}
