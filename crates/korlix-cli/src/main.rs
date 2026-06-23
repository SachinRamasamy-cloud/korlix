use clap::{Parser, Subcommand};
use colored::Colorize;

mod commands;
mod output;

#[derive(Parser)]
#[command(
    name    = "korlix",
    version = "0.1.0",
    author  = "Korlix Team",
    about   = "◈  Korlix — ultra-light frontend language compiler",
    long_about = None,
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Korlix project
    New {
        /// Project name / directory
        name: String,
    },
    /// Start the development server with hot drop
    Dev,
    /// Build the project for production
    Build {
        /// Build mode: static|spa|ssg
        #[arg(long, default_value = "static")]
        mode: String,
    },
    /// Type-check and lint all .klx files
    Check {
        /// Check accessibility rules
        #[arg(long)]
        a11y: bool,
        /// Check security rules
        #[arg(long)]
        security: bool,
        /// Check SEO rules
        #[arg(long)]
        seo: bool,
        /// Print AST to stdout
        #[arg(long)]
        ast: bool,
    },
    /// Preview the production build locally
    Preview {
        /// Port to serve on
        #[arg(long, short, default_value = "4173")]
        port: u16,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        None => {
            output::print_banner();
            println!("  Run {} for available commands.", "korlix --help".cyan());
            println!();
        }
        Some(Commands::New { name }) => {
            if let Err(e) = commands::new::run(&name) {
                output::print_error(&format!("{}", e));
                std::process::exit(1);
            }
        }
        Some(Commands::Dev) => {
            if let Err(e) = commands::dev::run().await {
                output::print_error(&format!("{}", e));
                std::process::exit(1);
            }
        }
        Some(Commands::Build { mode }) => {
            if let Err(e) = commands::build::run(Some(&mode)) {
                output::print_error(&format!("{}", e));
                std::process::exit(1);
            }
        }
        Some(Commands::Check { a11y, security, seo, ast }) => {
            if let Err(e) = commands::check::run(a11y, security, seo, ast) {
                output::print_error(&format!("{}", e));
                std::process::exit(1);
            }
        }
        Some(Commands::Preview { port }) => {
            if let Err(e) = commands::preview::run(Some(port)).await {
                output::print_error(&format!("{}", e));
                std::process::exit(1);
            }
        }
    }
}
