//! Compile the repository's complete Korlix showcase as a conformance fixture.

#[cfg(test)]
mod tests {
    use korlix_compiler::{compile, project::Project};
    use std::path::PathBuf;

    #[test]
    fn complete_showcase_compiles_as_static_multipage_project() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("examples/complete-showcase");
        let project = Project::load(root).expect("complete showcase project loads");
        let output = compile(&project, "static").expect("complete showcase compiles");

        assert_eq!(output.pages.len(), 11, "all showcase routes should compile");
        assert!(output.pages.iter().any(|page| page.route == "/"));
        assert!(output.pages.iter().any(|page| page.route == "/components"));
        assert!(output.pages.iter().any(|page| page.route == "/api"));
        assert!(output.pages.iter().any(|page| page.route == "/themes"));

        let html = output
            .pages
            .iter()
            .map(|page| page.html.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(html.contains("data-kx-pagination"));
        assert!(html.contains("data-kx-theme-toggle"));
        assert!(html.contains("kx-modal"));
        assert!(html.contains("Korlix Complete Showcase"));

        assert!(output.css.contains("kx-surface-violet-7"));
        assert!(output.css.contains("kx-content-content-muted"));
        assert!(output.app_js.contains("KorlixRuntime.api.query"));
        assert!(output.app_js.contains("KorlixRuntime.api.request"));
        assert!(output.runtime_js.contains("Pagination"));
        assert!(output.runtime_js.contains("Theme"));
        assert!(output.runtime_js.contains("Overlay"));
    }
}
