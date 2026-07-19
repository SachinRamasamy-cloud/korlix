use korlix_ast::program::Program;
use korlix_core::{DiagnosticSet, SourceMap};
use korlix_resolver::symbol_table::SymbolTable;
use std::collections::HashSet;

pub struct CompileContext {
    pub source_map: SourceMap,
    pub program: Program,
    pub symbols: SymbolTable,
    pub diagnostics: DiagnosticSet,
    pub used_classes: HashSet<String>,
}

impl CompileContext {
    pub fn new() -> Self {
        Self {
            source_map: SourceMap::new(),
            program: Program::new(),
            symbols: SymbolTable::new(),
            diagnostics: DiagnosticSet::new(),
            used_classes: HashSet::new(),
        }
    }
}
