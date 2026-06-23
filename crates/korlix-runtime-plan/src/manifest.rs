use crate::feature::RuntimeFeature;
use std::collections::HashSet;
pub fn runtime_modules(features: &HashSet<RuntimeFeature>) -> Vec<&'static str> {
    let mut mods = vec![];
    if features.contains(&RuntimeFeature::Core) {
        mods.push("core");
    }
    if features.contains(&RuntimeFeature::Router) {
        mods.push("router");
    }
    if features.contains(&RuntimeFeature::State) {
        mods.push("state");
    }
    if features.contains(&RuntimeFeature::Toast) {
        mods.push("toast");
    }
    if features.contains(&RuntimeFeature::Overlay) {
        mods.push("overlay");
    }
    if features.contains(&RuntimeFeature::Media) {
        mods.push("media");
    }
    if features.contains(&RuntimeFeature::Theme) {
        mods.push("theme");
    }
    if features.contains(&RuntimeFeature::Forms) {
        mods.push("forms");
    }
    mods
}
