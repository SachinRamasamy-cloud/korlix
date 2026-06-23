pub struct CompileOutput {
    pub pages: Vec<PageFile>,
    pub css: String,
    pub app_js: String,
    pub runtime_js: String,
    pub route_manifest: String,
    pub build_manifest: String,
}

pub struct PageFile {
    pub route: String,
    pub filename: String,
    pub html: String,
}
