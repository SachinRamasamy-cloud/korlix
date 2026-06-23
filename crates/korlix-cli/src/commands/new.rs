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
        "",
        "public/assets",
        "src/pages",
        "src/layouts",
        "src/components",
        "src/theme",
        "src",
        "dist",
    ];
    for dir in &dirs {
        std::fs::create_dir_all(project_dir.join(dir))?;
    }

    // korlix.config.json
    std::fs::write(
        project_dir.join("korlix.config.json"),
        CONFIG_JSON.replace("{{name}}", name),
    )?;

    // package.json
    std::fs::write(
        project_dir.join("package.json"),
        PKG_JSON.replace("{{name}}", name),
    )?;

    // .gitignore
    std::fs::write(project_dir.join(".gitignore"), GITIGNORE)?;

    // public/index.html
    std::fs::write(project_dir.join("public/index.html"), PUBLIC_HTML)?;

    // src/main.klx
    std::fs::write(project_dir.join("src/main.klx"), MAIN_KLX)?;

    // src/app.klx
    std::fs::write(project_dir.join("src/app.klx"), APP_KLX)?;

    // src/pages/index.klx
    std::fs::write(
        project_dir.join("src/pages/index.klx"),
        PAGE_INDEX_KLX.replace("{{name}}", name),
    )?;

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

// ── Template files ──────────────────────────────────────────────────────────

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
    "check": "korlix check",
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

const APP_KLX: &str = r#"app:
  theme:
    default "dark"
    dark true
  routes:
    page "/" from "./pages/index.klx"
  providers:
    toast
    modal
    theme
"#;

const PAGE_INDEX_KLX: &str = r##"page index route "/":
  meta:
    title "{{name}} - Korlix App"
    description "A clean Korlix starter app."

  section .min-h-screen .bg-[#070b12] .text-white .overflow-hidden:
    div .absolute .inset-0 .bg-[radial-gradient(circle_at_20%_20%,rgba(45,212,191,0.18),transparent_32%)] .pointer-events-none
    div .absolute .inset-0 .bg-[radial-gradient(circle_at_80%_10%,rgba(249,115,22,0.16),transparent_30%)] .pointer-events-none

    nav .relative .z-10 .max-w-7xl .mx-auto .px-6 .py-5 .flex .items-center .justify-between:
      div .flex .items-center .gap-3:
        div .w-9 .h-9 .rounded-lg .bg-[linear-gradient(135deg,#2dd4bf,#f97316)] .flex .items-center .justify-center:
          span .font-black .text-[#071014] "K"
        span .font-bold .text-xl "{{name}}"
      div .flex .gap-3:
        button .px-4 .py-2 .rounded-lg .border .border-[rgba(255,255,255,0.18)] .text-sm .text-[#cbd5e1] .hover:bg-[rgba(255,255,255,0.08)] "Preview" on:click:
          scrollTo("#preview")
        button .px-4 .py-2 .rounded-lg .bg-[#2dd4bf] .text-[#071014] .font-semibold .text-sm "Toast" on:click:
          toast success "Korlix buttons are working."

    div .relative .z-10 .max-w-7xl .mx-auto .px-6 .pt-20 .pb-24 .grid .grid-cols-2 .gap-12 .items-center:
      div:
        div .inline-flex .items-center .gap-2 .px-3 .py-1 .rounded-full .border .border-[rgba(45,212,191,0.3)] .bg-[rgba(45,212,191,0.08)] .mb-6:
          span .w-2 .h-2 .rounded-full .bg-[#2dd4bf] .animate-pulse
          span .text-[#99f6e4] .text-sm "Korlix starter"
        h1 .text-[clamp(2.75rem,6vw,5.5rem)] .leading-none .font-black .mb-6 "Build a clean app fast."
        p .text-[#94a3b8] .text-xl .leading-relaxed .max-w-2xl .mb-8 "This starter ships with a polished page, working actions, SPA mode, and a small structure that is easy to edit."
        div .flex .flex-wrap .gap-4:
          button .px-6 .py-3 .rounded-lg .bg-[linear-gradient(135deg,#2dd4bf,#f97316)] .text-[#071014] .font-bold .shadow-[0_16px_50px_rgba(45,212,191,0.22)] "Get started" on:click:
            toast success "Edit src/pages/index.klx to start building."
          button .px-6 .py-3 .rounded-lg .border .border-[rgba(255,255,255,0.16)] .text-[#e2e8f0] .hover:bg-[rgba(255,255,255,0.08)] "See preview" on:click:
            scrollTo("#preview")

      div .rounded-2xl .border .border-[rgba(255,255,255,0.12)] .bg-[rgba(15,23,42,0.78)] .shadow-[0_24px_80px_rgba(0,0,0,0.35)] .overflow-hidden id="preview":
        div .flex .items-center .gap-2 .px-5 .py-3 .border-b .border-[rgba(255,255,255,0.08)]:
          span .w-3 .h-3 .rounded-full .bg-[#f97316]
          span .w-3 .h-3 .rounded-full .bg-[#facc15]
          span .w-3 .h-3 .rounded-full .bg-[#2dd4bf]
          span .ml-3 .text-[#64748b] .text-sm "src/pages/index.klx"
        div .p-6 .font-mono .text-sm .leading-relaxed:
          p .text-[#94a3b8] "page index route \"/\":"
          p .text-[#2dd4bf] "  state count: int = 0"
          p .text-[#c4b5fd] "  button \"Click\" on:click:"
          p .text-[#fbbf24] "    toast success \"It works\""

    section .relative .z-10 .max-w-7xl .mx-auto .px-6 .pb-20:
      div .grid .grid-cols-3 .gap-5:
        div .p-6 .rounded-xl .border .border-[rgba(45,212,191,0.18)] .bg-[rgba(45,212,191,0.06)]:
          h2 .text-xl .font-bold .mb-2 "Clean structure"
          p .text-[#94a3b8] "Start from src/app.klx, src/pages, public assets, and theme tokens."
        div .p-6 .rounded-xl .border .border-[rgba(249,115,22,0.18)] .bg-[rgba(249,115,22,0.06)]:
          h2 .text-xl .font-bold .mb-2 "Working buttons"
          p .text-[#94a3b8] "Toast and scroll actions are wired through the Korlix runtime."
        div .p-6 .rounded-xl .border .border-[rgba(255,255,255,0.12)] .bg-[rgba(255,255,255,0.04)]:
          h2 .text-xl .font-bold .mb-2 "Ready scripts"
          p .text-[#94a3b8] "Run npm run dev, npm run build, npm run check, or npm run preview."
"##;

const TOKENS_KLX: &str = r#"theme:
  default "dark"
  dark true
"#;
