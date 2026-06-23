use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn find_klx_files(dir: &Path) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|x| x == "klx").unwrap_or(false))
        .map(|e| e.into_path())
        .collect()
}

pub fn resolve_relative(from: &Path, import_path: &str) -> PathBuf {
    let base = from.parent().unwrap_or(from);
    let resolved = base.join(import_path);
    if resolved.exists() {
        return resolved;
    }
    // Try with .klx extension
    let with_ext = base.join(format!("{}", import_path));
    if with_ext.exists() {
        return with_ext;
    }
    resolved
}
