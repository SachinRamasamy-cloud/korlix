//! KLX Lexer — tokenises .klx source files.

use crate::{
    indentation::IndentStack,
    keywords::lookup_keyword,
    token::{Token, TokenKind},
};
use korlix_core::{DiagnosticSet, Pos, Span};

#[allow(dead_code)]
pub struct Lexer<'a> {
    source: &'a str,
    chars: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
    file_id: u32,
    indent_stack: IndentStack,
    pub diagnostics: DiagnosticSet,
    /// tokens pending emit (e.g. INDENT / DEDENT batches)
    pending: Vec<Token>,
    at_line_start: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str, file_id: u32) -> Self {
        Self {
            source,
            chars: source.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
            file_id,
            indent_stack: IndentStack::new(),
            diagnostics: DiagnosticSet::new(),
            pending: vec![],
            at_line_start: true,
        }
    }

    // ── helpers ────────────────────────────────────────────────────────

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn peek2(&self) -> Option<char> {
        self.chars.get(self.pos + 1).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.chars.get(self.pos).copied()?;
        self.pos += 1;
        if ch == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(ch)
    }

    fn pos_now(&self) -> Pos {
        Pos::new(self.line, self.col, self.pos)
    }

    fn make_span(&self, start: Pos) -> Span {
        Span::new(start, self.pos_now(), self.file_id)
    }

    fn skip_comment(&mut self) {
        // # single-line comment
        while let Some(c) = self.peek() {
            if c == '\n' {
                break;
            }
            self.advance();
        }
    }

    // ── tokenise one logical token ─────────────────────────────────────

    fn next_token(&mut self) -> Option<Token> {
        // return pending first
        if !self.pending.is_empty() {
            return Some(self.pending.remove(0));
        }

        loop {
            let ch = self.peek()?;

            // ── line start: measure indentation ────────────────────────
            if self.at_line_start && ch != '\n' && ch != '\r' {
                self.at_line_start = false;

                let _start_pos = self.pos_now();
                let mut indent = 0usize;
                loop {
                    match self.peek() {
                        Some(' ') => {
                            self.advance();
                            indent += 1;
                        }
                        Some('\t') => {
                            self.advance();
                            indent += 4;
                        }
                        _ => break,
                    }
                }

                // after leading spaces, handle blank / comment lines
                match self.peek() {
                    Some('\n') | Some('\r') | None => {
                        // skip blank line without emitting INDENT/DEDENT
                        continue;
                    }
                    Some('#') => {
                        self.skip_comment();
                        continue;
                    }
                    _ => {}
                }

                let indent_toks = self.indent_stack.process(indent, self.file_id, self.line);
                if !indent_toks.is_empty() {
                    let first = indent_toks[0].clone();
                    self.pending.extend(indent_toks.into_iter().skip(1));
                    return Some(first);
                }
                continue;
            }

            // ── skip horizontal whitespace ──────────────────────────────
            if ch == ' ' || ch == '\t' || ch == '\r' {
                self.advance();
                continue;
            }

            // ── newline ────────────────────────────────────────────────
            if ch == '\n' {
                let start = self.pos_now();
                self.advance();
                self.at_line_start = true;
                let span = self.make_span(start);
                return Some(Token::new(TokenKind::Newline, span, "\\n"));
            }

            // ── comment ────────────────────────────────────────────────
            if ch == '#' {
                self.advance();
                self.skip_comment();
                continue;
            }

            let start = self.pos_now();

            // ── on:event ───────────────────────────────────────────────
            if ch == 'o' && self.peek2() == Some('n') {
                // look ahead for on:
                let saved_pos = self.pos;
                let saved_line = self.line;
                let saved_col = self.col;
                self.advance(); // o
                self.advance(); // n
                if self.peek() == Some(':') {
                    self.advance(); // :
                    let mut event_name = String::new();
                    while let Some(c) = self.peek() {
                        if c.is_alphanumeric() || c == '-' || c == '_' {
                            event_name.push(c);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    if !event_name.is_empty() {
                        let span = self.make_span(start);
                        return Some(Token::new(
                            TokenKind::OnEvent(event_name),
                            span,
                            format!("on:{}", ""),
                        ));
                    }
                    // not an event — restore
                    self.pos = saved_pos;
                    self.line = saved_line;
                    self.col = saved_col;
                } else {
                    self.pos = saved_pos;
                    self.line = saved_line;
                    self.col = saved_col;
                }
            }

            // ── class (.something) ─────────────────────────────────────
            if ch == '.' {
                // check next char is ident-start
                if let Some(nc) = self.peek2() {
                    if nc.is_alphabetic() || nc == '_' || nc == '-' {
                        self.advance(); // consume '.'
                        let mut class = String::new();
                        // class chars: alphanumeric, -, _, :, [, ], / (for responsive variants).
                        // A trailing ':' starts a KLX block, so only keep ':' when it is
                        // followed by another class-name segment, as in hover:bg-red-500.
                        let mut bracket_depth = 0;
                        loop {
                            match self.peek() {
                                Some(':')
                                    if bracket_depth == 0
                                        && self
                                            .peek2()
                                            .map(is_class_segment_char)
                                            .unwrap_or(false) =>
                                {
                                    class.push(':');
                                    self.advance();
                                }
                                Some(c) if bracket_depth > 0 => {
                                    if c == '[' {
                                        bracket_depth += 1;
                                    } else if c == ']' {
                                        bracket_depth -= 1;
                                    }
                                    class.push(c);
                                    self.advance();
                                }
                                Some(c)
                                    if is_class_segment_char(c)
                                        || matches!(
                                            c,
                                            '-' | '_' | '[' | ']' | '/' | '.' | '#' | '%'
                                        ) =>
                                {
                                    if c == '[' {
                                        bracket_depth += 1;
                                    }
                                    class.push(c);
                                    self.advance();
                                }
                                _ => break,
                            }
                        }
                        let span = self.make_span(start);
                        return Some(Token::new(
                            TokenKind::Class(class.clone()),
                            span,
                            format!(".{}", class),
                        ));
                    }
                }
                // lone dot
                self.advance();
                let span = self.make_span(start);
                return Some(Token::new(TokenKind::Dot, span, "."));
            }

            // ── string literal ─────────────────────────────────────────
            if ch == '"' || ch == '\'' {
                let quote = ch;
                self.advance();
                let mut s = String::new();
                let mut escaped = false;
                loop {
                    match self.peek() {
                        None => break,
                        Some('\n') => break,
                        Some(c) => {
                            self.advance();
                            if escaped {
                                match c {
                                    'n' => s.push('\n'),
                                    't' => s.push('\t'),
                                    '\\' => s.push('\\'),
                                    '"' => s.push('"'),
                                    '\'' => s.push('\''),
                                    _ => {
                                        s.push('\\');
                                        s.push(c);
                                    }
                                }
                                escaped = false;
                            } else if c == '\\' {
                                escaped = true;
                            } else if c == quote {
                                break;
                            } else {
                                s.push(c);
                            }
                        }
                    }
                }
                let span = self.make_span(start);
                return Some(Token::new(
                    TokenKind::StringLit(s.clone()),
                    span,
                    format!("\"{}\"", s),
                ));
            }

            // ── number ─────────────────────────────────────────────────
            if ch.is_ascii_digit()
                || (ch == '-' && self.peek2().map(|c| c.is_ascii_digit()).unwrap_or(false))
            {
                let mut num = String::new();
                if ch == '-' {
                    num.push('-');
                    self.advance();
                }
                while let Some(c) = self.peek() {
                    if c.is_ascii_digit() || c == '.' {
                        num.push(c);
                        self.advance();
                    } else {
                        break;
                    }
                }
                let val: f64 = num.parse().unwrap_or(0.0);
                let span = self.make_span(start);
                return Some(Token::new(TokenKind::Number(val), span, num));
            }

            // ── identifier / keyword ───────────────────────────────────
            if ch.is_alphabetic() || ch == '_' {
                let mut ident = String::new();
                while let Some(c) = self.peek() {
                    if c.is_alphanumeric() || c == '_' || c == '-' {
                        ident.push(c);
                        self.advance();
                    } else {
                        break;
                    }
                }
                let span = self.make_span(start);
                let kind = lookup_keyword(&ident).unwrap_or(TokenKind::Ident(ident.clone()));
                return Some(Token::new(kind, span, ident));
            }

            // ── two-char operators ─────────────────────────────────────
            self.advance();
            let kind = match (ch, self.peek()) {
                ('=', Some('=')) => {
                    self.advance();
                    TokenKind::EqEq
                }
                ('=', Some('>')) => {
                    self.advance();
                    TokenKind::Arrow
                }
                ('!', Some('=')) => {
                    self.advance();
                    TokenKind::BangEq
                }
                ('<', Some('=')) => {
                    self.advance();
                    TokenKind::LtEq
                }
                ('>', Some('=')) => {
                    self.advance();
                    TokenKind::GtEq
                }
                ('&', Some('&')) => {
                    self.advance();
                    TokenKind::AmpAmp
                }
                ('|', Some('|')) => {
                    self.advance();
                    TokenKind::PipePipe
                }
                // ── single-char ───────────────────────────────────
                (':', _) => TokenKind::Colon,
                (',', _) => TokenKind::Comma,
                ('=', _) => TokenKind::Equals,
                ('+', _) => TokenKind::Plus,
                ('-', _) => TokenKind::Minus,
                ('*', _) => TokenKind::Star,
                ('/', _) => TokenKind::Slash,
                ('%', _) => TokenKind::Percent,
                ('<', _) => TokenKind::Lt,
                ('>', _) => TokenKind::Gt,
                ('!', _) => TokenKind::Bang,
                ('?', _) => TokenKind::Question,
                (';', _) => TokenKind::Semicolon,
                ('(', _) => TokenKind::LParen,
                (')', _) => TokenKind::RParen,
                ('[', _) => TokenKind::LBracket,
                (']', _) => TokenKind::RBracket,
                ('{', _) => TokenKind::LBrace,
                ('}', _) => TokenKind::RBrace,
                _ => {
                    // skip unknown char
                    continue;
                }
            };
            let span = self.make_span(start);
            return Some(Token::new(kind, span, ch.to_string()));
        }
    }

    // ── public API ─────────────────────────────────────────────────────

    pub fn tokenize(mut self) -> (Vec<Token>, DiagnosticSet) {
        let mut tokens: Vec<Token> = vec![];

        loop {
            match self.next_token() {
                None => break,
                Some(t) if t.kind == TokenKind::Eof => {
                    tokens.push(t);
                    break;
                }
                Some(t) => tokens.push(t),
            }
        }

        // flush remaining dedents
        let line = self.line;
        let dedents = self.indent_stack.flush_dedents(self.file_id, line);
        tokens.extend(dedents);

        let eof_span = Span::new(
            Pos::new(self.line, self.col, self.pos),
            Pos::new(self.line, self.col, self.pos),
            self.file_id,
        );
        tokens.push(Token::new(TokenKind::Eof, eof_span, ""));

        (tokens, self.diagnostics)
    }
}

pub fn lex(source: &str, file_id: u32) -> (Vec<Token>, DiagnosticSet) {
    Lexer::new(source, file_id).tokenize()
}

fn is_class_segment_char(c: char) -> bool {
    c.is_alphanumeric()
}
