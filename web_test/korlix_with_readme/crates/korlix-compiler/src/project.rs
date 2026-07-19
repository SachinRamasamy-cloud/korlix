use korlix_core::config::KorlixConfig;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Project {
    pub root: PathBuf,
    pub config: KorlixConfig,
    pub src_dir: PathBuf,
    pub public_dir: PathBuf,
    pub dist_dir: PathBuf,
}

impl Project {
    pub fn load(root: PathBuf) -> Result<Self, String> {
        let config = KorlixConfig::load(&root)?;
        let src_dir = config.src_dir(&root);
        let public_dir = config.public_dir(&root);
        let dist_dir = config.dist_dir(&root);
        Ok(Self {
            root,
            config,
            src_dir,
            public_dir,
            dist_dir,
        })
    }
}
