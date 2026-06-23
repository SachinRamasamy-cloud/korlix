use crate::parser::Parser;
use korlix_ast::node::Node;
use korlix_lexer::token::TokenKind;

impl<'t> Parser<'t> {
    /// Parse an indented block: INDENT nodes* DEDENT
    pub fn parse_block(&mut self) -> Vec<Node> {
        self.skip_newlines();
        if !self.check(&TokenKind::Indent) {
            return vec![];
        }
        self.advance(); // consume INDENT

        let mut nodes = vec![];
        loop {
            self.skip_newlines();
            if self.check(&TokenKind::Dedent) || self.is_eof() {
                break;
            }
            if let Some(node) = self.parse_node() {
                nodes.push(node);
            } else {
                // error recovery: skip one token
                self.advance();
            }
        }

        if self.check(&TokenKind::Dedent) {
            self.advance(); // consume DEDENT
        }

        nodes
    }

    pub fn skip_newlines(&mut self) {
        while self.check(&TokenKind::Newline) {
            self.advance();
        }
    }
}
