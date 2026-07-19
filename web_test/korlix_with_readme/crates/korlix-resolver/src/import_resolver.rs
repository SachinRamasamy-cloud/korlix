// Phase 1: basic import resolution
pub fn resolve_import(base: &std::path::Path, path: &str) -> std::path::PathBuf {
    crate::resolve_relative(base, path)
}
