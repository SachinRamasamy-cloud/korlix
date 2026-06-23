use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
    routing::get,
    Router,
};
use colored::Colorize;
use futures::{sink::SinkExt, stream::StreamExt};
use korlix_compiler::{compile, write_dist, Project};
use std::{net::SocketAddr, path::PathBuf};
use tower_http::services::ServeDir;

use crate::hot_drop::{send_clear_error, send_error, send_full_reload};
use crate::websocket::{create_hmr_channel, HmrSender};

pub struct DevServer {
    pub project: Project,
    pub port: u16,
}

#[derive(Clone)]
struct AppState {
    dist_dir: PathBuf,
    hmr_tx: HmrSender,
}

impl DevServer {
    pub fn new(project: Project) -> Self {
        let port = project.config.port();
        Self { project, port }
    }

    pub async fn run(self) -> anyhow::Result<()> {
        let (hmr_tx, _) = create_hmr_channel(64);
        let state = AppState {
            dist_dir: self.project.dist_dir.clone(),
            hmr_tx: hmr_tx.clone(),
        };

        // Initial build
        println!("{} Building project...", "◈".cyan().bold());
        match compile(&self.project, "spa") {
            Ok(output) => {
                if let Err(e) = write_dist(&output, &self.project) {
                    eprintln!("{} {}", "error:".red().bold(), e);
                } else {
                    println!("{} Build complete", "✓".green().bold());
                }
            }
            Err(e) => eprintln!("{} Initial build failed: {}", "error:".red().bold(), e),
        }

        // File watcher
        let proj_clone = self.project.clone();
        let tx_clone = hmr_tx.clone();
        let src_dir = self.project.src_dir.clone();
        std::thread::spawn(move || {
            use crate::watcher::watch_project;
            use std::time::{Duration, Instant};
            let mut last = Instant::now();
            let _watcher = watch_project(&src_dir, move |_ev| {
                if last.elapsed() < Duration::from_millis(200) {
                    return;
                }
                last = Instant::now();
                println!("{} File changed — rebuilding…", "↻".yellow().bold());
                match compile(&proj_clone, "spa") {
                    Ok(output) => {
                        if let Err(e) = write_dist(&output, &proj_clone) {
                            send_error(&tx_clone, e);
                        } else {
                            println!("{} Hot drop sent", "⚡".cyan().bold());
                            send_clear_error(&tx_clone);
                            send_full_reload(&tx_clone);
                        }
                    }
                    Err(e) => send_error(&tx_clone, e),
                }
            });
            loop {
                std::thread::sleep(Duration::from_secs(3600));
            }
        });

        // Axum router
        let app = Router::new()
            .route("/__kx_hmr", get(ws_handler))
            .fallback_service(
                ServeDir::new(&self.project.dist_dir).append_index_html_on_directories(true),
            )
            .with_state(state);

        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        println!();
        println!("  {} Korlix dev server", "◈".cyan().bold());
        println!("  {} http://localhost:{}", "→".green().bold(), self.port);
        println!("  {} Hot drop enabled", "⚡".yellow().bold());
        println!();

        let listener = tokio::net::TcpListener::bind(&addr).await?;
        axum::serve(listener, app).await?;
        Ok(())
    }
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(move |socket| handle_ws(socket, state))
}

async fn handle_ws(socket: WebSocket, state: AppState) {
    let mut rx = state.hmr_tx.subscribe();
    let (mut sender, _) = socket.split();
    loop {
        match rx.recv().await {
            Ok(msg) => {
                if sender.send(Message::Text(msg)).await.is_err() {
                    break;
                }
            }
            Err(_) => break,
        }
    }
}
