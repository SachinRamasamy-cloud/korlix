use crate::output::*;
use colored::Colorize;
use korlix_compiler::{compile, write_dist, Project};
use std::time::Instant;

pub fn run(mode: Option<&str>) -> anyhow::Result<()> {
    print_banner();
    let root = std::env::current_dir()?;
    let project = Project::load(root).map_err(|e| anyhow::anyhow!(e))?;
    let mode = mode.unwrap_or("static");

    println!("  {} Building in {} mode…", "◈".cyan().bold(), mode.cyan());
    let start = Instant::now();

    match compile(&project, mode) {
        Ok(output) => {
            write_dist(&output, &project).map_err(|e| anyhow::anyhow!(e))?;
            let elapsed = start.elapsed();
            println!();
            println!(
                "  {} Build complete in {:.0}ms",
                "✓".green().bold(),
                elapsed.as_millis()
            );
            println!();
            println!(
                "  {} {}",
                "→".dimmed(),
                format!("dist/ ({} pages)", output.pages.len()).cyan()
            );
            println!(
                "  {} CSS:  {:.1}kb",
                " ".dimmed(),
                output.css.len() as f64 / 1024.0
            );
            println!(
                "  {} JS:   {:.1}kb",
                " ".dimmed(),
                (output.app_js.len() + output.runtime_js.len()) as f64 / 1024.0
            );
            println!();
        }
        Err(e) => {
            print_error(&format!("Build failed: {}", e));
            std::process::exit(1);
        }
    }
    Ok(())
}
