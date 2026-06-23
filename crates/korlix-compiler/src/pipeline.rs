use crate::{context::CompileContext, output::{CompileOutput, PageFile}, project::Project};
use korlix_ast::program::Item;
use korlix_codegen::{
    css::generate_css_for_classes,
    document::{generate_document, generate_build_manifest, PageOutput},
    routes::generate_route_manifest,
};
use korlix_lexer::lexer::lex;
use korlix_parser::parser::Parser;
use korlix_resolver::{
    file_resolver::find_klx_files,
    route_resolver::{build_route_map, RouteEntry},
};
use korlix_style::scanner::scan_classes;
use std::collections::HashMap;

pub fn compile(project: &Project, _mode: &str) -> Result<CompileOutput, String> {
    let mut ctx = CompileContext::new();

    // 1. Find all .klx files
    let klx_files = find_klx_files(&project.src_dir);

    // 2. Lex + Parse each file
    for path in &klx_files {
        let source = std::fs::read_to_string(path)
            .map_err(|e| format!("Cannot read {}: {}", path.display(), e))?;
        let file_id = ctx.source_map.add(path.clone(), source.clone());
        let (tokens, lex_diag) = lex(&source, file_id);
        ctx.diagnostics.diagnostics.extend(lex_diag.diagnostics);

        let parser = Parser::new(&tokens, file_id);
        let (module, parse_diag) = parser.parse(path.clone());
        ctx.diagnostics.diagnostics.extend(parse_diag.diagnostics);
        ctx.program.add_module(module);
    }

    if ctx.diagnostics.has_errors() {
        ctx.diagnostics.print_all();
        return Err(format!("{} error(s) found", ctx.diagnostics.error_count()));
    }

    // 3. Collect all used CSS classes
    for module in &ctx.program.modules {
        let classes = scan_classes(module);
        ctx.used_classes.extend(classes);
    }

    // 4. Generate CSS
    let css = generate_css_for_classes(&ctx.used_classes);

    // 5. Find routes from app.klx
    let mut routes: HashMap<String, RouteEntry> = HashMap::new();
    let mut _app_providers: Vec<String> = vec![];
    let mut layout_map: HashMap<String, String> = HashMap::new(); // name → html

    for module in &ctx.program.modules {
        for item in &module.items {
            match item {
                Item::AppDecl(app) => {
                    routes = build_route_map(&app.routes);
                    _app_providers = app.providers.clone();
                }
                Item::Layout(layout) => {
                    use korlix_codegen::html::render_nodes;
                    let html = render_nodes(&layout.body);
                    layout_map.insert(layout.name.clone(), html);
                }
                _ => {}
            }
        }
    }

    // 6. Generate HTML pages
    let mut pages: Vec<PageFile> = vec![];
    for module in &ctx.program.modules {
        for item in &module.items {
            if let Item::Page(page) = item {
                let route = page.route.clone()
                    .unwrap_or_else(|| "/".to_string());
                let layout_html = page.layout.as_ref()
                    .and_then(|l| layout_map.get(l))
                    .cloned();

                let html = generate_document(
                    page,
                    layout_html,
                    "/assets/korlix.css",
                    &["/assets/korlix.runtime.js", "/assets/app.js"],
                    &project.config.name.as_deref().unwrap_or("Korlix App"),
                );
                let filename = route_to_filename(&route);
                pages.push(PageFile { route, filename, html });
            }
        }
    }

    // 7. Generate JS
    let mut app_js = String::new();
    for module in &ctx.program.modules {
        app_js.push_str(&korlix_codegen::js::generate_app_js(module, &routes));
    }

    // 8. Route manifest
    let route_manifest = generate_route_manifest(&routes);

    // 9. Build manifest
    let page_outputs: Vec<PageOutput> = pages.iter().map(|p| PageOutput {
        route: p.route.clone(),
        filename: p.filename.clone(),
        html: p.html.clone(),
    }).collect();
    let build_manifest = generate_build_manifest(
        &page_outputs,
        css.len(),
        app_js.len(),
    );

    // 10. Inline runtime (embedded)
    let runtime_js = RUNTIME_JS.to_string();

    Ok(CompileOutput {
        pages,
        css,
        app_js,
        runtime_js,
        route_manifest,
        build_manifest,
    })
}

fn route_to_filename(route: &str) -> String {
    if route == "/" {
        return "index.html".to_string();
    }
    let clean = route.trim_matches('/').replace('/', "/");
    format!("{}/index.html", clean)
}

pub fn write_dist(output: &CompileOutput, project: &Project) -> Result<(), String> {
    let dist = &project.dist_dir;
    let assets_dir = dist.join("assets");
    std::fs::create_dir_all(&assets_dir)
        .map_err(|e| format!("Cannot create dist/: {}", e))?;

    // Copy public assets first so compiler outputs take precedence for
    // reserved files such as index.html and assets/korlix.css.
    korlix_codegen::assets::copy_public_assets(&project.public_dir, dist)
        .map_err(|e| format!("Cannot copy public assets: {}", e))?;

    // CSS
    std::fs::write(assets_dir.join("korlix.css"), &output.css)
        .map_err(|e| format!("Cannot write CSS: {}", e))?;

    // Runtime JS
    std::fs::write(assets_dir.join("korlix.runtime.js"), &output.runtime_js)
        .map_err(|e| format!("Cannot write runtime: {}", e))?;

    // App JS
    std::fs::write(assets_dir.join("app.js"), &output.app_js)
        .map_err(|e| format!("Cannot write app.js: {}", e))?;

    // Route manifest
    std::fs::write(dist.join("korlix.routes.json"), &output.route_manifest)
        .map_err(|e| format!("Cannot write routes: {}", e))?;

    // Build manifest
    std::fs::write(dist.join("korlix.manifest.json"), &output.build_manifest)
        .map_err(|e| format!("Cannot write manifest: {}", e))?;

    // HTML pages
    for page in &output.pages {
        let page_path = if page.filename == "index.html" {
            dist.join("index.html")
        } else {
            let full = dist.join(&page.filename);
            if let Some(parent) = full.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("Cannot create dir: {}", e))?;
            }
            full
        };
        std::fs::write(&page_path, &page.html)
            .map_err(|e| format!("Cannot write {}: {}", page_path.display(), e))?;
    }

    Ok(())
}

// ── Embedded minimal runtime ─────────────────────────────────────────────
const RUNTIME_JS: &str = include_str!("../runtime-bundle/korlix.runtime.js");
