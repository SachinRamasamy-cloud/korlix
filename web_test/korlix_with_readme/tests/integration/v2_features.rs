//! Korlix V2 language and compiler conformance tests.

#[cfg(test)]
mod tests {
    use korlix_compiler::{compile, project::Project};
    use std::fs;
    use tempfile::tempdir;

    fn project_with(files: &[(&str, &str)]) -> (tempfile::TempDir, Project) {
        let dir = tempdir().expect("temp dir");
        let root = dir.path();
        fs::write(
            root.join("korlix.config.json"),
            r#"{"name":"Korlix V2 Test","src":"src","public":"public","dist":"dist"}"#,
        )
        .unwrap();
        fs::create_dir_all(root.join("src")).unwrap();
        fs::create_dir_all(root.join("public")).unwrap();
        for (path, source) in files {
            let target = root.join("src").join(path);
            fs::create_dir_all(target.parent().unwrap()).unwrap();
            fs::write(target, source).unwrap();
        }
        let project = Project::load(root.to_path_buf()).expect("project loads");
        (dir, project)
    }

    #[test]
    fn v2_page_function_interpolation_and_compound_assignment_compile() {
        let (_dir, project) = project_with(&[(
            "pages/index.klx",
            r#"
page Home at "/"
  state count: int = 0
  h1 "Count: {count}"
  button "Increase" click=increment

  fn increment
    count += 1
"#,
        )]);
        let output = compile(&project, "static").expect("V2 source compiles");
        assert!(output.pages[0].html.contains("Count:"));
        assert!(output.app_js.contains("increment"));
        assert!(output.app_js.contains("count"));
    }

    #[test]
    fn app_layout_user_component_and_props_are_lowered() {
        let (_dir, project) = project_with(&[
            (
                "app.klx",
                r#"
app
  layout Shell
  theme auto

layout Shell
  header
    nav "Korlix"
  main
    slot
  footer "Footer"
"#,
            ),
            (
                "pages/index.klx",
                r#"
component user-card
  prop name: string
  prop role: string = "Member"
  card variant=raised
    h2 name
    p role

page Home at "/"
  user-card name="Sachin" role="Admin"
"#,
            ),
        ]);
        let output = compile(&project, "static").expect("layout/component source compiles");
        let html = &output.pages[0].html;
        assert!(html.contains("Korlix"), "default layout should render");
        assert!(html.contains("Footer"), "layout footer should render");
        assert!(html.contains("Sachin"), "component prop should render");
        assert!(
            html.contains("Admin"),
            "component default scope should render"
        );
        assert!(!html.contains("<user-card"), "component must be lowered");
        assert!(
            !html.contains("kx-kx-"),
            "class prefix must not be duplicated"
        );
    }

    #[test]
    fn theme_native_colors_and_pagination_are_generated() {
        let (_dir, project) = project_with(&[
            (
                "app.klx",
                r#"
app
  theme dark
"#,
            ),
            (
                "pages/index.klx",
                r#"
page Home at "/"
  section .surface-blue-1 .content-blue-10 .dark:surface-blue-10
    pagination page=2 pages=8 url-sync
"#,
            ),
        ]);
        let output = compile(&project, "static").expect("theme source compiles");
        assert!(output.pages[0].html.contains("data-kx-theme=\"dark\""));
        assert!(output.pages[0].html.contains("data-kx-pagination"));
        assert!(output.css.contains("kx-surface-blue-1"));
        assert!(output.css.contains("data-kx-theme=\"dark\""));
        assert!(!output.css.contains("@media (min-width:768px){\n@media"));
        assert!(output.runtime_js.contains("Pagination"));
        assert!(output.runtime_js.contains("Theme"));
    }

    #[test]
    fn duplicate_routes_are_rejected() {
        let (_dir, project) = project_with(&[
            ("pages/one.klx", "page One at \"/same\"\n  h1 \"One\"\n"),
            ("pages/two.klx", "page Two at \"/same\"\n  h1 \"Two\"\n"),
        ]);
        let error = match compile(&project, "static") {
            Ok(_) => panic!("duplicate routes must fail"),
            Err(error) => error,
        };
        assert!(error.contains("KX-S102"));
    }

    #[test]
    fn unknown_components_are_rejected() {
        let (_dir, project) = project_with(&[(
            "pages/index.klx",
            "page Home at \"/\"\n  does-not-exist \"Invalid\"\n",
        )]);
        let error = match compile(&project, "static") {
            Ok(_) => panic!("unknown component must fail"),
            Err(error) => error,
        };
        assert!(error.contains("Unknown component"));
    }
    #[test]
    fn literal_type_mismatches_and_missing_props_are_rejected() {
        let (_dir, project) = project_with(&[(
            "pages/index.klx",
            r#"
component profile-card
  prop name: string
  article
    h2 name

page Home at "/"
  state count: int = "wrong"
  profile-card
"#,
        )]);
        let error = match compile(&project, "static") {
            Ok(_) => panic!("invalid semantics must fail"),
            Err(error) => error,
        };
        assert!(error.contains("KX-T101"));
        assert!(error.contains("KX-S210"));
    }
}
