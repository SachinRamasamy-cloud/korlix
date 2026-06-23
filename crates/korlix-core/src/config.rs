use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KorlixConfig {
    pub name: Option<String>,
    pub version: Option<String>,
    pub src: Option<String>,
    pub public: Option<String>,
    pub dist: Option<String>,
    pub mode: Option<BuildMode>,
    pub theme: Option<ThemeConfig>,
    pub budget: Option<BudgetConfig>,
    pub server: Option<ServerConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BuildMode {
    Static,
    Spa,
    Ssg,
}

impl Default for BuildMode {
    fn default() -> Self {
        Self::Static
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub default: Option<String>,
    pub dark: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetConfig {
    pub runtime: Option<String>,
    pub css: Option<String>,
    pub page: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: Option<u16>,
    pub host: Option<String>,
}

impl Default for KorlixConfig {
    fn default() -> Self {
        Self {
            name: Some("my-korlix-app".into()),
            version: Some("0.1.0".into()),
            src: Some("src".into()),
            public: Some("public".into()),
            dist: Some("dist".into()),
            mode: Some(BuildMode::Static),
            theme: None,
            budget: None,
            server: None,
        }
    }
}

impl KorlixConfig {
    pub fn load(project_root: &Path) -> Result<Self, String> {
        let config_path = project_root.join("korlix.config.json");
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)
                .map_err(|e| format!("Failed to read korlix.config.json: {}", e))?;
            serde_json::from_str(&content).map_err(|e| format!("Invalid korlix.config.json: {}", e))
        } else {
            Ok(Self::default())
        }
    }

    pub fn src_dir(&self, root: &Path) -> PathBuf {
        root.join(self.src.as_deref().unwrap_or("src"))
    }

    pub fn public_dir(&self, root: &Path) -> PathBuf {
        root.join(self.public.as_deref().unwrap_or("public"))
    }

    pub fn dist_dir(&self, root: &Path) -> PathBuf {
        root.join(self.dist.as_deref().unwrap_or("dist"))
    }

    pub fn port(&self) -> u16 {
        self.server.as_ref().and_then(|s| s.port).unwrap_or(3000)
    }
}
