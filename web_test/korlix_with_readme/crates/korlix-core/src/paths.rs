use std::path::{Path, PathBuf};

pub fn normalize(path: &Path) -> PathBuf {
    dunce::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

pub fn relative_to(base: &Path, target: &Path) -> PathBuf {
    pathdiff::diff_paths(target, base).unwrap_or_else(|| target.to_path_buf())
}

pub fn route_from_path(path: &Path, pages_dir: &Path) -> String {
    let rel = match path.strip_prefix(pages_dir) {
        Ok(r) => r,
        Err(_) => return "/".into(),
    };

    let mut route = String::from("/");
    let components: Vec<_> = rel.components().collect();

    for (i, comp) in components.iter().enumerate() {
        use std::path::Component::Normal;
        if let Normal(s) = comp {
            let s = s.to_string_lossy();
            let part = if s == "index.klx" {
                if i == 0 {
                    return "/".into();
                }
                continue;
            } else if s.ends_with(".klx") {
                let base = s.trim_end_matches(".klx");
                if base.starts_with('[') && base.ends_with(']') {
                    let param = &base[1..base.len() - 1];
                    format!(":{}", param)
                } else {
                    base.to_string()
                }
            } else {
                s.to_string()
            };

            if !route.ends_with('/') {
                route.push('/');
            }
            route.push_str(&part);
        }
    }

    route
}
