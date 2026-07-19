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
  "theme": { "default": "light", "dark": false },
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
    default "light"
    dark false
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
    description "A warm, polished Korlix starter app."

  state count: int = 0

  section .min-h-screen .bg-[#F7F6F3] .text-[#1C1917]:
    nav .bg-[#111110] .border-b .border-[rgba(255,255,255,0.08)]:
      div .max-w-7xl .mx-auto .px-6 .py-4 .flex .items-center .justify-between:
        div .flex .items-center .gap-3:
          svg .w-10 .h-10 viewBox="0 0 64 64" role="img" aria-label="Korlix KLX logo":
            rect x="4" y="4" width="56" height="56" rx="12" fill="#FEFDFB"
            path d="M18 18V46" stroke="#6366F1" stroke-width="5" stroke-linecap="round"
            path d="M20 32L34 18" stroke="#F97316" stroke-width="5" stroke-linecap="round" stroke-linejoin="round"
            path d="M20 32L36 46" stroke="#F97316" stroke-width="5" stroke-linecap="round" stroke-linejoin="round"
            path d="M43 18L52 32L43 46" fill="none" stroke="#14B8A6" stroke-width="5" stroke-linecap="round" stroke-linejoin="round"
          div:
            p .text-white .font-bold .text-xl "{{name}}"
            p .text-[#A8A29E] .text-sm "Korlix starter"
        div .flex .items-center .gap-4:
          a href="#features" .text-[#A8A29E] .hover:text-[#6366F1] .text-sm "Features"
          a href="#docs" .text-[#A8A29E] .hover:text-[#6366F1] .text-sm "Docs"
          button .px-4 .py-2 .rounded-lg .bg-[#6366F1] .text-white .font-semibold .text-sm .hover:bg-[#4F46E5] "Toast" on:click:
            toast success "Korlix is ready."

    header .bg-[#111110] .text-white:
      div .max-w-7xl .mx-auto .px-6 .py-20 .grid .grid-cols-1 .md:grid-cols-2 .gap-12 .items-center:
        div:
          div .inline-flex .items-center .gap-2 .px-3 .py-1 .rounded-lg .bg-[#F97316] .text-white .font-semibold .text-sm .mb-6:
            span "v0.1 starter"
          h1 .text-[clamp(2.75rem,6vw,5.5rem)] .leading-none .font-black .mb-6 "Build with KLX, ship clean UI."
          p .text-[#A8A29E] .text-xl .leading-relaxed .max-w-2xl .mb-8 "A warm starter using indigo actions, orange callouts, teal success states, and a React-style counter wired through Korlix state."
          div .flex .flex-wrap .gap-4:
            button .px-6 .py-3 .rounded-lg .bg-[#6366F1] .text-white .font-bold .shadow-[0_18px_50px_rgba(99,102,241,0.35)] .hover:bg-[#4F46E5] "Start editing" on:click:
              scrollTo("#docs")
            button .px-6 .py-3 .rounded-lg .border .border-[#14B8A6] .text-[#14B8A6] .font-semibold .hover:bg-[rgba(20,184,166,0.12)] "Show success" on:click:
              toast success "Teal is your success accent."

        div .bg-[#1C1917] .border .border-[rgba(255,255,255,0.1)] .rounded-lg .overflow-hidden .shadow-[0_24px_80px_rgba(0,0,0,0.35)]:
          div .flex .items-center .gap-2 .px-5 .py-3 .border-b .border-[rgba(255,255,255,0.08)]:
            span .w-3 .h-3 .rounded-full .bg-[#F97316]
            span .w-3 .h-3 .rounded-full .bg-[#6366F1]
            span .w-3 .h-3 .rounded-full .bg-[#14B8A6]
            span .ml-3 .text-[#A8A29E] .text-sm "src/pages/index.klx"
          pre .p-6 .font-mono .text-sm .leading-relaxed:
            code .text-[#A8A29E] "page index route \"/\":"
            code .block .text-[#6366F1] "  state count: int = 0"
            code .block .text-[#F97316] "  button \"count is \" count on:click:"
            code .block .text-[#14B8A6] "    count = count + 1"

    main .max-w-7xl .mx-auto .px-6 .py-16:
      section .grid .grid-cols-1 .md:grid-cols-3 .gap-5 .mb-16 id="features":
        div .bg-[#FEFDFB] .border .border-[rgba(99,102,241,0.18)] .rounded-lg .p-6 .shadow-sm:
          div .w-10 .h-10 .rounded-lg .bg-[rgba(99,102,241,0.12)] .text-[#6366F1] .flex .items-center .justify-center .font-black .mb-4 "K"
          h2 .text-xl .font-bold .mb-2 "Compiler-first"
          p .text-[#57534E] "Indigo marks primary actions, links, active states, and code accents."
        div .bg-[#FEFDFB] .border .border-[rgba(249,115,22,0.2)] .rounded-lg .p-6 .shadow-sm:
          div .w-10 .h-10 .rounded-lg .bg-[rgba(249,115,22,0.12)] .text-[#F97316] .flex .items-center .justify-center .font-black .mb-4 "J"
          h2 .text-xl .font-bold .mb-2 "JIT styling"
          p .text-[#57534E] "Orange is reserved for badges, callouts, warnings, and framing problem areas."
        div .bg-[#FEFDFB] .border .border-[rgba(20,184,166,0.22)] .rounded-lg .p-6 .shadow-sm:
          div .w-10 .h-10 .rounded-lg .bg-[rgba(20,184,166,0.12)] .text-[#14B8A6] .flex .items-center .justify-center .font-black .mb-4 "A"
          h2 .text-xl .font-bold .mb-2 "API ready"
          p .text-[#57534E] "Teal supports success states, secondary CTAs, and positive sections."

      section .grid .grid-cols-1 .md:grid-cols-2 .gap-8 .items-start id="docs":
        div .bg-[#FEFDFB] .border .border-[rgba(28,25,23,0.08)] .rounded-lg .p-8:
          span .inline-flex .px-3 .py-1 .rounded-lg .bg-[rgba(249,115,22,0.12)] .text-[#F97316] .font-semibold .text-sm .mb-4 "Problem solved"
          h2 .text-3xl .font-black .mb-4 "Edit one KLX file."
          p .text-[#57534E] .text-lg .leading-relaxed .mb-6 "Start in src/pages/index.klx. The starter already includes routing, state, events, generated CSS, and a working toast provider."
          div .flex .flex-wrap .gap-3:
            a href="https://www.npmjs.com/package/korlix" .px-5 .py-3 .rounded-lg .bg-[#6366F1] .text-white .font-semibold "NPM package"
            button .px-5 .py-3 .rounded-lg .border .border-[#14B8A6] .text-[#14B8A6] .font-semibold "Success toast" on:click:
              toast success "Your starter palette is active."

        div .bg-[#FEFDFB] .border .border-[rgba(20,184,166,0.28)] .rounded-lg .p-8:
          h2 .text-2xl .font-black .mb-2 "React-style counter"
          p .text-[#57534E] .mb-6 "This button updates Korlix state and re-renders the bound value."
          p .text-[#A8A29E] .text-sm .mb-2 "Current count"
          p .text-6xl .font-black .text-[#6366F1] .mb-6 count
          div .flex .flex-wrap .gap-3:
            button .px-6 .py-3 .rounded-lg .bg-[#6366F1] .text-white .font-bold .hover:bg-[#4F46E5] "count is " count on:click:
              count = count + 1
            button .px-6 .py-3 .rounded-lg .border .border-[#F97316] .text-[#F97316] .font-semibold .hover:bg-[rgba(249,115,22,0.1)] "Reset" on:click:
              count = 0

    section .bg-[#111110] .text-white:
      div .max-w-7xl .mx-auto .px-6 .py-12 .flex .items-center .justify-between:
        div:
          h2 .text-3xl .font-black .mb-2 "Ready to build?"
          p .text-[#A8A29E] "Use indigo for the main path, orange for attention, and teal for success."
        button .px-6 .py-3 .rounded-lg .bg-[#14B8A6] .text-[#111110] .font-bold "Open docs" on:click:
          scrollTo("#docs")
"##;

const TOKENS_KLX: &str = r#"theme:
  default "light"
  dark false
"#;
