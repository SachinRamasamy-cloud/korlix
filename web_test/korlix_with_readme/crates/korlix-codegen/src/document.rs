use crate::html::{html_escape, render_nodes};
use korlix_ast::program::PageDecl;

pub struct BuildOutput {
    pub pages: Vec<PageOutput>,
    pub css: String,
    pub js: String,
    pub route_manifest: String,
}

pub struct PageOutput {
    pub route: String,
    pub filename: String,
    pub html: String,
}

pub fn generate_document(
    page: &PageDecl,
    layout_html: Option<String>,
    css_path: &str,
    js_paths: &[&str],
    app_name: &str,
    theme_mode: &str,
) -> String {
    let page_body = render_nodes(&page.body);
    let content = if let Some(layout) = layout_html {
        layout.replace(
            r#"<div data-slot="default" class="kx-slot"></div>"#,
            &page_body,
        )
    } else {
        page_body
    };

    let title = page
        .meta
        .as_ref()
        .and_then(|m| m.title.as_ref())
        .and_then(|t| t.as_string().map(|s| s.to_string()))
        .unwrap_or_else(|| format!("{} | Korlix", app_name));

    let description = page
        .meta
        .as_ref()
        .and_then(|m| m.description.as_ref())
        .and_then(|d| d.as_string().map(|s| s.to_string()))
        .unwrap_or_default();

    let js_tags: String = js_paths
        .iter()
        .map(|p| format!(r#"<script src="{}" defer></script>"#, p))
        .collect::<Vec<_>>()
        .join("\n    ");

    let theme_mode = match theme_mode {
        "light" | "dark" | "auto" => theme_mode,
        _ => "auto",
    };
    let theme_bootstrap = format!(
        r#"<script>(function(){{var m={mode:?};var s=localStorage.getItem('kx-theme');var t=s||(m==='auto'?(matchMedia('(prefers-color-scheme: dark)').matches?'dark':'light'):m);document.documentElement.dataset.kxTheme=t;}})();</script>"#,
        mode = theme_mode
    );

    format!(
        r#"<!doctype html>
<html lang="en" data-kx-theme="{theme_mode}">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>{title}</title>
  {desc}
  {theme_bootstrap}
  <link rel="stylesheet" href="{css}" />
</head>
<body>
  <div id="korlix-root">{content}</div>
  {js}
</body>
</html>"#,
        title = html_escape(&title),
        desc = if description.is_empty() {
            String::new()
        } else {
            format!(
                r#"<meta name="description" content="{}" />"#,
                html_escape(&description)
            )
        },
        theme_mode = theme_mode,
        theme_bootstrap = theme_bootstrap,
        css = css_path,
        content = content,
        js = js_tags,
    )
}

pub fn generate_spa_shell(css_path: &str, js_paths: &[&str], app_name: &str) -> String {
    let js_tags: String = js_paths
        .iter()
        .map(|p| format!(r#"<script src="{}" defer></script>"#, p))
        .collect::<Vec<_>>()
        .join("\n    ");

    format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>{}</title>
  <link rel="stylesheet" href="{}" />
</head>
<body class="dark">
  <div id="korlix-root"><div class="kx-page-loading" aria-live="polite"></div></div>
  {}
</body>
</html>"#,
        app_name, css_path, js_tags
    )
}

pub fn generate_build_manifest(pages: &[PageOutput], css_size: usize, js_size: usize) -> String {
    let page_entries: Vec<String> = pages
        .iter()
        .map(|p| {
            format!(
                r#"    {{ "route": "{}", "file": "{}" }}"#,
                p.route, p.filename
            )
        })
        .collect();

    format!(
        r#"{{
  "version": "0.1.0",
  "compiler": "korlix",
  "pages": [
{}
  ],
  "assets": {{
    "css": {{ "size": {} }},
    "js":  {{ "size": {} }}
  }}
}}"#,
        page_entries.join(",\n"),
        css_size,
        js_size,
    )
}
