use crate::output::*;
use colored::Colorize;
use korlix_compiler::Project;
use korlix_core::DiagnosticSet;
use korlix_lexer::lexer::lex;
use korlix_parser::parser::Parser;
use korlix_resolver::file_resolver::find_klx_files;
use korlix_style::{scanner::scan_classes, validator::validate_classes};

pub fn run(a11y: bool, security: bool, seo: bool, ast: bool) -> anyhow::Result<()> {
    print_banner();
    let root = std::env::current_dir()?;
    let project = Project::load(root).map_err(|e| anyhow::anyhow!(e))?;
    let klx_files = find_klx_files(&project.src_dir);

    println!(
        "  {} Checking {} files…",
        "◈".cyan().bold(),
        klx_files.len()
    );
    let mut total_errors = 0usize;
    let mut total_warnings = 0usize;
    let mut source_map = korlix_core::SourceMap::new();

    for path in &klx_files {
        let source = std::fs::read_to_string(path)?;
        let file_id = source_map.add(path.clone(), source.clone());
        let (tokens, lex_diag) = lex(&source, file_id);

        let parser = Parser::new(&tokens, file_id);
        let (module, parse_diag) = parser.parse(path.clone());

        let mut diag = DiagnosticSet::new();
        diag.diagnostics.extend(lex_diag.diagnostics);
        diag.diagnostics.extend(parse_diag.diagnostics);

        // Style validation
        let classes = scan_classes(&module);
        validate_classes(&classes, &mut diag);

        if ast {
            let json = serde_json::to_string_pretty(&module).unwrap_or_default();
            println!("\n  AST for {}:\n{}", path.display(), json);
        }

        if diag.has_errors() || diag.warning_count() > 0 {
            let rel = path.strip_prefix(&project.src_dir).unwrap_or(path);
            println!("  {} {}", "→".dimmed(), rel.display().to_string().cyan());
            for d in &diag.diagnostics {
                let sym = match d.severity {
                    korlix_core::Severity::Error => "✕".red().bold(),
                    korlix_core::Severity::Warning => "⚠".yellow().bold(),
                    _ => "◦".blue().bold(),
                };
                println!("    {} [{}] {}", sym, d.code.dimmed(), d.message);
                if let Some(h) = &d.hint {
                    println!("      {} {}", "hint:".green(), h);
                }
            }
        }

        total_errors += diag.error_count();
        total_warnings += diag.warning_count();
    }

    println!();
    if total_errors > 0 {
        println!(
            "  {} {} error(s), {} warning(s)",
            "✕".red().bold(),
            total_errors,
            total_warnings
        );
        std::process::exit(1);
    } else if total_warnings > 0 {
        println!(
            "  {} {} warning(s) found",
            "⚠".yellow().bold(),
            total_warnings
        );
    } else {
        println!("  {} All checks passed", "✓".green().bold());
    }
    println!();
    Ok(())
}
