use korlix_compiler::Project;
use korlix_dev_server::DevServer;

pub async fn run() -> anyhow::Result<()> {
    let root = std::env::current_dir()?;
    let project = Project::load(root).map_err(|e| anyhow::anyhow!(e))?;
    DevServer::new(project).run().await
}
