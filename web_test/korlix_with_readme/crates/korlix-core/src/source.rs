use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct SourceFile {
    pub id: u32,
    pub path: PathBuf,
    pub content: String,
}

impl SourceFile {
    pub fn new(id: u32, path: PathBuf, content: String) -> Self {
        Self { id, path, content }
    }

    pub fn line(&self, line_num: usize) -> Option<&str> {
        self.content.lines().nth(line_num.saturating_sub(1))
    }

    pub fn name(&self) -> &str {
        self.path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
    }
}

#[derive(Debug, Default)]
pub struct SourceMap {
    files: Vec<SourceFile>,
}

impl SourceMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, path: PathBuf, content: String) -> u32 {
        let id = self.files.len() as u32;
        self.files.push(SourceFile::new(id, path, content));
        id
    }

    pub fn get(&self, id: u32) -> Option<&SourceFile> {
        self.files.get(id as usize)
    }
}
