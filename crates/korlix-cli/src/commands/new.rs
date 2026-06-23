use crate::output::*;
use colored::Colorize;
use std::path::Path;

pub fn run(name: &str) -> anyhow::Result<()> {
    let project_dir = Path::new(name);
    if project_dir.exists() {
        print_error(&format!("Directory `{}` already exists.", name));
        return Ok(());
    }

    print_banner();
    println!("  Creating new Korlix project: {}", name.cyan().bold());
    println!();

    // Create directory structure
    let dirs = [
        "", "public/assets", "src/pages", "src/layouts",
        "src/components", "src/theme", "src", "dist",
    ];
    for dir in &dirs {
        std::fs::create_dir_all(project_dir.join(dir))?;
    }

    // korlix.config.json
    std::fs::write(project_dir.join("korlix.config.json"), CONFIG_JSON
        .replace("{{name}}", name))?;

    // package.json
    std::fs::write(project_dir.join("package.json"), PKG_JSON
        .replace("{{name}}", name))?;

    // .gitignore
    std::fs::write(project_dir.join(".gitignore"), GITIGNORE)?;

    // public/index.html
    std::fs::write(project_dir.join("public/index.html"), PUBLIC_HTML)?;

    // src/main.klx
    std::fs::write(project_dir.join("src/main.klx"), MAIN_KLX)?;

    // src/app.klx
    std::fs::write(project_dir.join("src/app.klx"), APP_KLX)?;

    // src/pages/index.klx
    std::fs::write(project_dir.join("src/pages/index.klx"), PAGE_INDEX_KLX
        .replace("{{name}}", name))?;

    // src/pages/about.klx
    std::fs::write(project_dir.join("src/pages/about.klx"), PAGE_ABOUT_KLX)?;

    // src/layouts/main.klx
    std::fs::write(project_dir.join("src/layouts/main.klx"), LAYOUT_MAIN_KLX
        .replace("{{name}}", name))?;

    // src/components/hero.klx
    std::fs::write(project_dir.join("src/components/hero.klx"), COMPONENT_HERO_KLX)?;

    // src/theme/tokens.klx
    std::fs::write(project_dir.join("src/theme/tokens.klx"), TOKENS_KLX)?;

    println!("  {}", "Project created successfully!".green().bold());
    println!();
    println!("  Next steps:");
    println!("    {} cd {}", "$".dimmed(), name.cyan());
    println!("    {} korlix dev", "$".dimmed());
    println!();
    Ok(())
}

// ── Template files ───────────────────────────────────────────────────────
const CONFIG_JSON: &str = r#"{
  "name": "{{name}}",
  "version": "0.1.0",
  "src": "src",
  "public": "public",
  "dist": "dist",
  "mode": "spa",
  "theme": { "default": "dark", "dark": true },
  "server": { "port": 3000 }
}
"#;

const PKG_JSON: &str = r#"{
  "name": "{{name}}",
  "version": "0.1.0",
  "private": true,
  "scripts": {
    "dev": "korlix dev",
    "build": "korlix build",
    "preview": "korlix preview"
  }
}
"#;

const GITIGNORE: &str = "dist/\nnode_modules/\n.DS_Store\n*.log\n";

const PUBLIC_HTML: &str = r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>Korlix App</title>
</head>
<body>
  <div id="korlix-root"></div>
</body>
</html>
"#;

const MAIN_KLX: &str = r##"import App from "./app.klx"

mount App to "#korlix-root"
"##;

const APP_KLX: &str = r#"import MainLayout from "./layouts/main.klx"

app:
  layout MainLayout

  theme:
    default "dark"
    dark true

  routes:
    page "/" from "./pages/index.klx"
    page "/about" from "./pages/about.klx"

  providers:
    toast
    modal
    theme
"#;

const PAGE_INDEX_KLX: &str = r#"page index route "/":
  section .min-h-screen .flex .flex-col .items-center .justify-center .bg-background .text-foreground .p-8:

    h1 .text-6xl .font-bold .text-primary .mb-4 "{{name}}"
    p .text-xl .text-muted .mb-8 "Built with Korlix — ultra-light frontend language"

    div .flex .gap-4:
      btn .primary "Get Started" on:click:
        navigate("/about")

      btn .ghost "Learn More" on:click:
        toast success "Korlix is working! 🎉"
"#;

const PAGE_ABOUT_KLX: &str = r#"page about route "/about":
  section .min-h-screen .flex .flex-col .items-center .justify-center .bg-background .text-foreground .p-8:
    h1 .text-4xl .font-bold .mb-4 "About"
    p .text-lg .text-muted .mb-8 "This project was built with Korlix."

    state count: int = 0

    card .p-8 .rounded-xl .shadow-lg .bg-surface .mb-6:
      p .text-center .text-muted .mb-4 "Counter demo:"
      p .text-5xl .text-primary .font-bold .text-center .mb-4:
        text count
      div .flex .gap-3 .justify-center:
        btn .primary "Increment" on:click:
          count = count + 1
        btn .danger "Reset" on:click:
          count = 0

    link .text-primary href="/":
      "← Back to Home"
"#;

const LAYOUT_MAIN_KLX: &str = r#"layout main:
  navbar .bg-surface .border-b .border-border .sticky .top-0 .z-50:
    div .max-w-7xl .mx-auto .px-6 .py-4 .flex .items-center .justify-between:
      link href="/" .text-primary .font-bold .text-xl "{{name}}"
      div .flex .gap-6:
        link href="/" .text-muted "Home"
        link href="/about" .text-muted "About"

  main .flex-1:
    slot

  footer .bg-surface .border-t .border-border .py-8 .text-center .text-muted:
    p "Built with Korlix ◈"
"#;

const COMPONENT_HERO_KLX: &str = r#"component hero:
  prop title: string
  prop subtitle: string

  section .hero .min-h-screen .flex .flex-col .items-center .justify-center .text-center .p-8:
    h1 .text-6xl .font-bold .text-primary .mb-4 title
    p .text-xl .text-muted subtitle
"#;

const TOKENS_KLX: &str = r#"theme:
  default "dark"
  dark true
"#;
