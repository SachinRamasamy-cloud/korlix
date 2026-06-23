use korlix_ast::program::Module;
use korlix_core::{DiagnosticSet, Span};
use korlix_lexer::token::{Token, TokenKind};

pub struct Parser<'t> {
    pub tokens: &'t [Token],
    pub pos: usize,
    pub file_id: u32,
    pub diagnostics: DiagnosticSet,
}

impl<'t> Parser<'t> {
    pub fn new(tokens: &'t [Token], file_id: u32) -> Self {
        Self {
            tokens,
            pos: 0,
            file_id,
            diagnostics: DiagnosticSet::new(),
        }
    }

    // ── token navigation ───────────────────────────────────────────────

    pub fn current(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(self.tokens.last().unwrap())
    }

    pub fn peek_ahead(&self, n: usize) -> &Token {
        self.tokens.get(self.pos + n).unwrap_or(self.tokens.last().unwrap())
    }

    pub fn current_kind(&self) -> TokenKind {
        self.current().kind.clone()
    }

    pub fn advance(&mut self) -> &Token {
        let t = &self.tokens[self.pos];
        if self.pos + 1 < self.tokens.len() {
            self.pos += 1;
        }
        t
    }

    pub fn check(&self, kind: &TokenKind) -> bool {
        std::mem::discriminant(&self.current().kind) == std::mem::discriminant(kind)
    }

    pub fn check_exact(&self, kind: &TokenKind) -> bool {
        &self.current().kind == kind
    }

    pub fn is_eof(&self) -> bool {
        matches!(self.current().kind, TokenKind::Eof)
    }

    pub fn current_span(&self) -> Span {
        self.current().span
    }

    /// Consume the current token if it matches `kind`, else emit an error.
    pub fn expect(&mut self, kind: &TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            let got = self.current_kind();
            let _span = self.current_span();
            self.diagnostics.error(
                "KX-E001",
                format!("Expected `{}`, found `{}`", kind, got),
            );
            false
        }
    }

    /// Consume if matches.
    pub fn consume_if(&mut self, kind: &TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    // ── entry point ────────────────────────────────────────────────────

    pub fn parse(mut self, path: std::path::PathBuf) -> (Module, DiagnosticSet) {
        let mut module = Module::new(self.file_id, path);

        self.skip_newlines();

        while !self.is_eof() {
            self.skip_newlines();
            if self.is_eof() { break; }

            if let Some(item) = self.parse_top_level_item() {
                module.items.push(item);
            } else {
                self.advance(); // skip unknown
            }
        }

        (module, self.diagnostics)
    }
}
