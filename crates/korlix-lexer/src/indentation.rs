//! Indentation tracker — produces INDENT/DEDENT tokens like Python.

use crate::token::{Token, TokenKind};
use korlix_core::{Pos, Span};

pub struct IndentStack {
    stack: Vec<usize>,
}

impl IndentStack {
    pub fn new() -> Self {
        Self { stack: vec![0] }
    }

    pub fn current(&self) -> usize {
        *self.stack.last().unwrap_or(&0)
    }

    /// Returns a list of (kind, indent_level) events.
    pub fn process(&mut self, indent: usize, file_id: u32, line: usize) -> Vec<Token> {
        let mut tokens = vec![];
        let current = self.current();
        let dummy_span = Span::new(Pos::new(line, 1, 0), Pos::new(line, 1, 0), file_id);

        if indent > current {
            self.stack.push(indent);
            tokens.push(Token::new(TokenKind::Indent, dummy_span, "INDENT"));
        } else if indent < current {
            while self.current() > indent {
                self.stack.pop();
                tokens.push(Token::new(TokenKind::Dedent, dummy_span, "DEDENT"));
            }
        }

        tokens
    }

    pub fn flush_dedents(&mut self, file_id: u32, line: usize) -> Vec<Token> {
        let dummy_span = Span::new(Pos::new(line, 1, 0), Pos::new(line, 1, 0), file_id);
        let mut tokens = vec![];
        while self.current() > 0 {
            self.stack.pop();
            tokens.push(Token::new(TokenKind::Dedent, dummy_span, "DEDENT"));
        }
        tokens
    }
}
