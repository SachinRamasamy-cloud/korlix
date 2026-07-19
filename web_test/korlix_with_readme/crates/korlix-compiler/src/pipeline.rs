use crate::{
    context::CompileContext,
    lowering::{collect_user_components, lower_nodes, validate_component_names},
    output::{CompileOutput, PageFile},
    project::Project,
    semantic::validate_semantics,
};
use korlix_ast::program::{ComponentDecl, Item, LayoutDecl, Program};
use korlix_codegen::{
    css::generate_css_for_classes,
    document::{generate_build_manifest, generate_document, PageOutput},
    routes::generate_route_manifest,
};
use korlix_lexer::lexer::lex;
use korlix_parser::parser::Parser;
use korlix_resolver::{
    file_resolver::find_klx_files,
    route_resolver::{build_route_map, RouteEntry},
};
use korlix_style::scanner::scan_classes;
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};

pub fn compile(project: &Project, mode: &str) -> Result<CompileOutput, String> {
    let mut ctx = CompileContext::new();

    // 1. Discover, lex, and parse every .klx module.
    let klx_files = find_klx_files(&project.src_dir);
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

    // 2. Build the declaration environment, including import aliases.
    let mut user_components = collect_user_components(&ctx.program);
    let mut imported_layouts: HashMap<String, LayoutDecl> = HashMap::new();
    add_import_aliases(&ctx.program, &mut user_components, &mut imported_layouts);

    // 3. Semantic checks that must run before destructive lowering.
    let mut semantic_errors = validate_program(&ctx.program, &user_components);
    semantic_errors.extend(validate_semantics(&ctx.program, &user_components));
    semantic_errors.extend(validate_routes(&ctx.program));
    if !semantic_errors.is_empty() {
        return Err(semantic_errors.join("\n"));
    }

    // 4. Lower user components and built-in components to ordinary elements.
    for module in &mut ctx.program.modules {
        for item in &mut module.items {
            match item {
                Item::Page(page) => page.body = lower_nodes(&page.body, &user_components),
                Item::Layout(layout) => layout.body = lower_nodes(&layout.body, &user_components),
                Item::Component(component) => {
                    component.body = lower_nodes(&component.body, &user_components)
                }
                _ => {}
            }
        }
    }

    // 5. Collect application configuration and routes.
    let mut routes: HashMap<String, RouteEntry> = HashMap::new();
    let mut app_default_layout: Option<String> = None;
    let mut theme_mode = "auto".to_string();
    let mut layout_map: HashMap<String, String> = HashMap::new();

    for module in &ctx.program.modules {
        for item in &module.items {
            match item {
                Item::AppDecl(app) => {
                    routes.extend(build_route_map(&app.routes));
                    if app_default_layout.is_none() {
                        app_default_layout = app.layout.clone();
                    }
                    if let Some(theme) = &app.theme {
                        theme_mode = theme.default_mode.clone().unwrap_or_else(|| {
                            if theme.dark_enabled { "dark" } else { "light" }.into()
                        });
                    }
                }
                Item::Layout(layout) => {
                    layout_map.insert(
                        layout.name.clone(),
                        korlix_codegen::html::render_nodes(&layout.body),
                    );
                }
                Item::Page(page) => {
                    let route = page.route.clone().unwrap_or_else(|| "/".into());
                    routes.entry(route.clone()).or_insert_with(|| RouteEntry {
                        id: route_id(&route),
                        path: route,
                        source: module.path.to_string_lossy().into_owned(),
                    });
                }
                _ => {}
            }
        }
    }

    for (alias, mut layout) in imported_layouts {
        layout.body = lower_nodes(&layout.body, &user_components);
        layout_map.insert(alias, korlix_codegen::html::render_nodes(&layout.body));
    }

    // 6. Scan the lowered tree. This includes component-generated classes.
    for module in &ctx.program.modules {
        ctx.used_classes.extend(scan_classes(module));
    }
    let css = generate_css_for_classes(&ctx.used_classes);

    // 7. Emit pages. Static is the stable mode; spa currently uses the same
    // page artifacts plus the route manifest while the runtime router evolves.
    let normalized_mode = match mode {
        "static" | "spa" => mode,
        other => {
            return Err(format!(
                "KX-C301: Unsupported build mode `{other}`. Use `static` or `spa`."
            ))
        }
    };

    let mut pages: Vec<PageFile> = vec![];
    for module in &ctx.program.modules {
        for item in &module.items {
            if let Item::Page(page) = item {
                let route = page.route.clone().unwrap_or_else(|| "/".to_string());
                let selected_layout = page
                    .layout
                    .as_ref()
                    .or(app_default_layout.as_ref())
                    .and_then(|name| layout_map.get(name))
                    .cloned();

                let html = generate_document(
                    page,
                    selected_layout,
                    "/assets/korlix.css",
                    &["/assets/korlix.runtime.js", "/assets/app.js"],
                    project.config.name.as_deref().unwrap_or("Korlix App"),
                    &theme_mode,
                );
                pages.push(PageFile {
                    route: route.clone(),
                    filename: route_to_filename(&route),
                    html,
                });
            }
        }
    }

    // 8. Generate JavaScript from the lowered program. Runtime initialisation is
    // route-gated, so page state no longer overwrites unrelated page state.
    let mut app_js = format!("window.__KORLIX_BUILD_MODE__ = {:?};\n", normalized_mode);
    for module in &ctx.program.modules {
        app_js.push_str(&korlix_codegen::js::generate_app_js(module, &routes));
    }

    let route_manifest = generate_route_manifest(&routes);
    let page_outputs: Vec<PageOutput> = pages
        .iter()
        .map(|page| PageOutput {
            route: page.route.clone(),
            filename: page.filename.clone(),
            html: page.html.clone(),
        })
        .collect();
    let build_manifest = generate_build_manifest(&page_outputs, css.len(), app_js.len());

    Ok(CompileOutput {
        pages,
        css,
        app_js,
        runtime_js: RUNTIME_JS.to_string(),
        route_manifest,
        build_manifest,
    })
}

fn validate_program(
    program: &Program,
    user_components: &HashMap<String, ComponentDecl>,
) -> Vec<String> {
    let mut errors = vec![];
    let mut declarations = HashSet::new();

    for module in &program.modules {
        for item in &module.items {
            let declaration = match item {
                Item::Page(page) => Some(("page", page.name.as_str())),
                Item::Layout(layout) => Some(("layout", layout.name.as_str())),
                Item::Component(component) => Some(("component", component.name.as_str())),
                _ => None,
            };
            if let Some((kind, name)) = declaration {
                let key = format!("{kind}:{name}");
                if !declarations.insert(key) {
                    errors.push(format!("KX-S101: Duplicate {kind} declaration `{name}`."));
                }
            }

            match item {
                Item::Page(page) => {
                    validate_component_names(&page.body, user_components, &mut errors)
                }
                Item::Layout(layout) => {
                    validate_component_names(&layout.body, user_components, &mut errors)
                }
                Item::Component(component) => {
                    validate_component_names(&component.body, user_components, &mut errors)
                }
                _ => {}
            }
        }
    }

    errors.sort();
    errors.dedup();
    errors
}

fn validate_routes(program: &Program) -> Vec<String> {
    let mut seen: HashMap<String, String> = HashMap::new();
    let mut errors = vec![];
    for module in &program.modules {
        for item in &module.items {
            if let Item::Page(page) = item {
                let route = page.route.clone().unwrap_or_else(|| "/".into());
                if let Some(first) = seen.insert(route.clone(), page.name.clone()) {
                    errors.push(format!(
                        "KX-S102: Duplicate route `{route}` used by pages `{first}` and `{}`.",
                        page.name
                    ));
                }
            }
        }
    }
    errors
}

fn add_import_aliases(
    program: &Program,
    components: &mut HashMap<String, ComponentDecl>,
    layouts: &mut HashMap<String, LayoutDecl>,
) {
    for module in &program.modules {
        for import in &module.imports {
            let Some(alias) = &import.name else {
                continue;
            };
            let target_path = resolve_import_path(&module.path, &import.path);
            let Some(target) = program
                .modules
                .iter()
                .find(|candidate| same_path(&candidate.path, &target_path))
            else {
                continue;
            };

            for item in &target.items {
                match item {
                    Item::Component(component) => {
                        let mut aliased = component.clone();
                        aliased.name = alias.clone();
                        components.insert(alias.clone(), aliased);
                        break;
                    }
                    Item::Layout(layout) => {
                        let mut aliased = layout.clone();
                        aliased.name = alias.clone();
                        layouts.insert(alias.clone(), aliased);
                        break;
                    }
                    _ => {}
                }
            }
        }
    }
}

fn resolve_import_path(module_path: &Path, import_path: &str) -> PathBuf {
    let path = module_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(import_path);
    std::fs::canonicalize(&path).unwrap_or(path)
}

fn same_path(left: &Path, right: &Path) -> bool {
    let left = std::fs::canonicalize(left).unwrap_or_else(|_| left.to_path_buf());
    let right = std::fs::canonicalize(right).unwrap_or_else(|_| right.to_path_buf());
    left == right
}

fn route_id(route: &str) -> String {
    let id = route
        .trim_matches('/')
        .replace('/', "_")
        .replace(':', "param_");
    if id.is_empty() {
        "index".into()
    } else {
        id
    }
}

fn route_to_filename(route: &str) -> String {
    if route == "/" {
        return "index.html".to_string();
    }
    let clean = route.trim_matches('/');
    format!("{clean}/index.html")
}

pub fn write_dist(output: &CompileOutput, project: &Project) -> Result<(), String> {
    let dist = &project.dist_dir;
    let assets_dir = dist.join("assets");
    std::fs::create_dir_all(&assets_dir).map_err(|e| format!("Cannot create dist/: {}", e))?;

    korlix_codegen::assets::copy_public_assets(&project.public_dir, dist)
        .map_err(|e| format!("Cannot copy public assets: {}", e))?;

    std::fs::write(assets_dir.join("korlix.css"), &output.css)
        .map_err(|e| format!("Cannot write CSS: {}", e))?;
    std::fs::write(assets_dir.join("korlix.runtime.js"), &output.runtime_js)
        .map_err(|e| format!("Cannot write runtime: {}", e))?;
    std::fs::write(assets_dir.join("app.js"), &output.app_js)
        .map_err(|e| format!("Cannot write app.js: {}", e))?;
    std::fs::write(dist.join("korlix.routes.json"), &output.route_manifest)
        .map_err(|e| format!("Cannot write routes: {}", e))?;
    std::fs::write(dist.join("korlix.manifest.json"), &output.build_manifest)
        .map_err(|e| format!("Cannot write manifest: {}", e))?;

    for page in &output.pages {
        let page_path = if page.filename == "index.html" {
            dist.join("index.html")
        } else {
            let full = dist.join(&page.filename);
            if let Some(parent) = full.parent() {
                std::fs::create_dir_all(parent).map_err(|e| format!("Cannot create dir: {}", e))?;
            }
            full
        };
        std::fs::write(&page_path, &page.html)
            .map_err(|e| format!("Cannot write {}: {}", page_path.display(), e))?;
    }

    Ok(())
}

const RUNTIME_JS: &str = include_str!("../runtime-bundle/korlix.runtime.js");
