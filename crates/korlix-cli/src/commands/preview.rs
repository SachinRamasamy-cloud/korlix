use crate::output::*;
use colored::Colorize;
use korlix_compiler::Project;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

pub async fn run(port: Option<u16>) -> anyhow::Result<()> {
    let root = std::env::current_dir()?;
    let project = Project::load(root).map_err(|e| anyhow::anyhow!(e))?;
    let port = port.unwrap_or(4173);

    if !project.dist_dir.exists() {
        print_error("No dist/ directory found. Run `korlix build` first.");
        std::process::exit(1);
    }

    let app = axum::Router::new()
        .fallback_service(ServeDir::new(&project.dist_dir).append_index_html_on_directories(true));

    println!();
    println!("  {} Preview server", "◈".cyan().bold());
    println!("  {} http://localhost:{}", "→".green().bold(), port);
    println!();

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
