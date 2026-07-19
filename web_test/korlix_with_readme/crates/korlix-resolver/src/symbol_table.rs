use std::collections::HashMap;
#[derive(Debug, Default)]
pub struct SymbolTable {
    pub components: HashMap<String, String>,
    pub layouts: HashMap<String, String>,
    pub pages: HashMap<String, String>,
}
impl SymbolTable {
    pub fn new() -> Self {
        Self::default()
    }
}
