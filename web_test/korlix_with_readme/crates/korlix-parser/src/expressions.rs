use crate::parser::Parser;
use korlix_ast::expression::{BinaryOp, Expr, StringPart, UnaryOp};
use korlix_lexer::token::TokenKind;

impl<'t> Parser<'t> {
    pub fn parse_expr(&mut self) -> Option<Expr> {
        self.parse_or_expr()
    }

    fn parse_or_expr(&mut self) -> Option<Expr> {
        let mut left = self.parse_and_expr()?;
        while self.check(&TokenKind::PipePipe) {
            self.advance();
            let right = self.parse_and_expr()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::Or,
                right: Box::new(right),
            };
        }
        Some(left)
    }

    fn parse_and_expr(&mut self) -> Option<Expr> {
        let mut left = self.parse_eq_expr()?;
        while self.check(&TokenKind::AmpAmp) {
            self.advance();
            let right = self.parse_eq_expr()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::And,
                right: Box::new(right),
            };
        }
        Some(left)
    }

    fn parse_eq_expr(&mut self) -> Option<Expr> {
        let mut left = self.parse_cmp_expr()?;
        loop {
            let op = match self.current_kind() {
                TokenKind::EqEq => BinaryOp::Eq,
                TokenKind::BangEq => BinaryOp::Ne,
                _ => break,
            };
            self.advance();
            let right = self.parse_cmp_expr()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Some(left)
    }

    fn parse_cmp_expr(&mut self) -> Option<Expr> {
        let mut left = self.parse_add_expr()?;
        loop {
            let op = match self.current_kind() {
                TokenKind::Lt => BinaryOp::Lt,
                TokenKind::LtEq => BinaryOp::Le,
                TokenKind::Gt => BinaryOp::Gt,
                TokenKind::GtEq => BinaryOp::Ge,
                _ => break,
            };
            self.advance();
            let right = self.parse_add_expr()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Some(left)
    }

    fn parse_add_expr(&mut self) -> Option<Expr> {
        let mut left = self.parse_mul_expr()?;
        loop {
            let op = match self.current_kind() {
                TokenKind::Plus => BinaryOp::Add,
                TokenKind::Minus => BinaryOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_mul_expr()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Some(left)
    }

    fn parse_mul_expr(&mut self) -> Option<Expr> {
        let mut left = self.parse_unary_expr()?;
        loop {
            let op = match self.current_kind() {
                TokenKind::Star => BinaryOp::Mul,
                TokenKind::Slash => BinaryOp::Div,
                TokenKind::Percent => BinaryOp::Mod,
                _ => break,
            };
            self.advance();
            let right = self.parse_unary_expr()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Some(left)
    }

    fn parse_unary_expr(&mut self) -> Option<Expr> {
        if self.check(&TokenKind::Bang) {
            self.advance();
            let expr = self.parse_unary_expr()?;
            return Some(Expr::Unary {
                op: UnaryOp::Not,
                operand: Box::new(expr),
            });
        }
        self.parse_postfix_expr()
    }

    fn parse_postfix_expr(&mut self) -> Option<Expr> {
        let mut expr = self.parse_primary()?;
        loop {
            if self.check(&TokenKind::Dot) {
                self.advance();
                if let Some(field) = self.expect_ident() {
                    expr = Expr::Member {
                        object: Box::new(expr),
                        field,
                    };
                } else {
                    break;
                }
            } else if self.check(&TokenKind::LParen) {
                self.advance();
                let mut args = vec![];
                while !self.check(&TokenKind::RParen) && !self.is_eof() {
                    if let Some(a) = self.parse_expr() {
                        args.push(a);
                    }
                    if self.check(&TokenKind::Comma) {
                        self.advance();
                    }
                }
                self.expect(&TokenKind::RParen);
                expr = Expr::Call {
                    callee: Box::new(expr),
                    args,
                };
            } else if self.check(&TokenKind::LBracket) {
                self.advance();
                let idx = self.parse_expr()?;
                self.expect(&TokenKind::RBracket);
                expr = Expr::Index {
                    object: Box::new(expr),
                    index: Box::new(idx),
                };
            } else {
                break;
            }
        }
        Some(expr)
    }

    fn parse_primary(&mut self) -> Option<Expr> {
        let tok = self.current().clone();
        match &tok.kind {
            TokenKind::StringLit(s) => {
                let s = s.clone();
                self.advance();
                Some(parse_string_expression(&s))
            }
            TokenKind::Number(n) => {
                let n = *n;
                self.advance();
                Some(Expr::Number(n))
            }
            TokenKind::Bool(b) => {
                let b = *b;
                self.advance();
                Some(Expr::Bool(b))
            }
            TokenKind::Null => {
                self.advance();
                Some(Expr::Null)
            }
            TokenKind::LBracket => {
                self.advance();
                let mut items = vec![];
                while !self.check(&TokenKind::RBracket) && !self.is_eof() {
                    if let Some(e) = self.parse_expr() {
                        items.push(e);
                    }
                    if self.check(&TokenKind::Comma) {
                        self.advance();
                    }
                }
                self.expect(&TokenKind::RBracket);
                Some(Expr::List(items))
            }
            TokenKind::LBrace => {
                self.advance();
                let mut pairs = vec![];
                while !self.check(&TokenKind::RBrace) && !self.is_eof() {
                    let key = self.expect_ident().unwrap_or_default();
                    self.expect(&TokenKind::Colon);
                    let val = self.parse_expr().unwrap_or(Expr::Null);
                    pairs.push((key, val));
                    if self.check(&TokenKind::Comma) {
                        self.advance();
                    }
                }
                self.expect(&TokenKind::RBrace);
                Some(Expr::Object(pairs))
            }
            TokenKind::LParen => {
                self.advance();
                let expr = self.parse_expr();
                self.expect(&TokenKind::RParen);
                expr
            }
            _ if tok.kind.is_ident_like() => {
                let name = tok.kind.as_ident_str().unwrap_or("").to_string();
                self.advance();
                Some(Expr::Identifier(name))
            }
            _ => None,
        }
    }

    pub fn expect_ident(&mut self) -> Option<String> {
        let tok = self.current().clone();
        if tok.kind.is_ident_like() {
            let name = tok.kind.as_ident_str().unwrap_or("").to_string();
            self.advance();
            Some(name)
        } else if let TokenKind::Ident(s) = &tok.kind {
            let s = s.clone();
            self.advance();
            Some(s)
        } else {
            None
        }
    }
}

fn parse_string_expression(value: &str) -> Expr {
    if !value.contains('{') {
        return Expr::String(value.into());
    }

    let mut parts = Vec::new();
    let mut rest = value;
    while let Some(open) = rest.find('{') {
        let (before, after_open) = rest.split_at(open);
        if !before.is_empty() {
            parts.push(StringPart::Literal(before.into()));
        }
        let after_open = &after_open[1..];
        let Some(close) = after_open.find('}') else {
            parts.push(StringPart::Literal(format!("{{{}", after_open)));
            rest = "";
            break;
        };
        let (binding, after_close) = after_open.split_at(close);
        if binding.is_empty()
            || !binding.split('.').all(|part| {
                !part.is_empty() && part.chars().all(|c| c.is_alphanumeric() || c == '_')
            })
        {
            parts.push(StringPart::Literal(format!("{{{binding}}}")));
        } else {
            let mut segments = binding.split('.');
            let mut expr = Expr::Identifier(segments.next().unwrap_or_default().into());
            for field in segments {
                expr = Expr::Member {
                    object: Box::new(expr),
                    field: field.into(),
                };
            }
            parts.push(StringPart::Expr(expr));
        }
        rest = &after_close[1..];
    }
    if !rest.is_empty() {
        parts.push(StringPart::Literal(rest.into()));
    }

    if parts.iter().any(|part| matches!(part, StringPart::Expr(_))) {
        Expr::Interpolated(parts)
    } else {
        Expr::String(value.into())
    }
}

#[cfg(test)]
mod interpolation_tests {
    use super::*;

    #[test]
    fn parses_simple_and_member_interpolation() {
        let value = parse_string_expression("Hello {user.name}, count {count}");
        match value {
            Expr::Interpolated(parts) => assert_eq!(parts.len(), 4),
            _ => panic!("expected interpolated expression"),
        }
    }
}
