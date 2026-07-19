use crate::parser::Parser;
use korlix_lexer::token::TokenKind;

impl<'t> Parser<'t> {
    /// Skip tokens until we reach a newline or end-of-file.
    pub fn recover_to_newline(&mut self) {
        while !self.is_eof() && !self.check(&TokenKind::Newline) {
            self.advance();
        }
    }

    /// Skip tokens until we reach a dedent, indicating end of block.
    pub fn recover_to_dedent(&mut self) {
        let mut depth = 0usize;
        while !self.is_eof() {
            match self.current_kind() {
                TokenKind::Indent => {
                    depth += 1;
                    self.advance();
                }
                TokenKind::Dedent => {
                    if depth == 0 {
                        break;
                    }
                    depth -= 1;
                    self.advance();
                }
                TokenKind::Eof => break,
                _ => {
                    self.advance();
                }
            }
        }
    }
}
