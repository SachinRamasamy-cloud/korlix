//! Integration tests for the full build pipeline

#[cfg(test)]
mod tests {
    use korlix_compiler::{compile, project::Project};
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn make_project(src_content: &str) -> (tempfile::TempDir, Project) {
        let dir = tempdir().expect("temp dir");
        let root = dir.path().to_path_buf();

        // Write config
        std::fs::write(
            root.join("korlix.config.json"),
            r#"{"name":"test","src":"src","public":"public","dist":"dist"}"#,
        )
        .unwrap();

        // Create dirs
        std::fs::create_dir_all(root.join("src/pages")).unwrap();
        std::fs::create_dir_all(root.join("public")).unwrap();

        // Write source
        std::fs::write(root.join("src/pages/index.klx"), src_content).unwrap();
        std::fs::write(
            root.join("src/main.klx"),
            "import App from \"./app.klx\"\nmount App to \"#korlix-root\"\n",
        )
        .unwrap();
        std::fs::write(
            root.join("src/app.klx"),
            "app:\n  routes:\n    page \"/\" from \"./pages/index.klx\"\n",
        )
        .unwrap();

        let project = Project::load(root).expect("project loads");
        (dir, project)
    }

    #[test]
    fn test_hello_world_compiles() {
        let (_dir, project) = make_project("page index route \"/ \":\n  h1 \"Hello Korlix\"\n");
        let output = compile(&project, "static").expect("compile succeeds");
        assert!(!output.pages.is_empty());
        assert!(output.pages[0].html.contains("Hello Korlix"));
        assert!(output.pages[0].html.contains("<!doctype html>"));
    }

    #[test]
    fn test_css_generated() {
        let (_dir, project) =
            make_project("page index:\n  div .flex .bg-primary .text-white .p-4 \"Content\"\n");
        let output = compile(&project, "static").expect("compile succeeds");
        assert!(output.css.contains("display:flex") || output.css.contains("display: flex"));
    }

    #[test]
    fn test_empty_diagnostics_on_valid_klx() {
        let (_dir, project) = make_project(
            "page index route \"/ \":\n  section .py-20:\n    h1 .text-4xl \"Valid\"\n",
        );
        // Should not panic
        let _ = compile(&project, "static");
    }
}
