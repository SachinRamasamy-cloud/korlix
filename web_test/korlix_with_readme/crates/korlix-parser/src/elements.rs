use crate::parser::Parser;
use korlix_ast::{
    element::{ClassRef, ComponentNode, ElementNode, EventHandler, Prop},
    expression::Expr,
    node::{CallNode, Node, TextNode},
};
use korlix_lexer::token::TokenKind;

use crate::html::is_html_tag;

impl<'t> Parser<'t> {
    pub fn parse_element_or_component(&mut self) -> Option<Node> {
        let span = self.current_span();
        let name = self.current_kind().as_ident_str()?.to_string();
        self.advance();

        // Collect .classes
        let mut classes = vec![];
        while self.check(&TokenKind::Class("".into())) {
            if let TokenKind::Class(c) = self.current_kind() {
                classes.push(ClassRef::new(c.clone(), self.current_span()));
                self.advance();
            }
        }

        // Collect prop_key=value pairs and inline text/expression content.
        let mut props = vec![];
        let mut inline_children = vec![];
        let mut events = vec![];

        loop {
            match self.current_kind() {
                // Classes may appear after props in author code:
                // a href="#features" .text-primary "Features"
                TokenKind::Class(c) => {
                    classes.push(ClassRef::new(c.clone(), self.current_span()));
                    self.advance();
                }
                // HTML/Korlix boolean property: `required`, `disabled`, `url-sync`.
                _ if self.current_kind().is_ident_like()
                    && is_boolean_property(
                        self.current_kind().as_ident_str().unwrap_or_default(),
                    ) =>
                {
                    let key = self.expect_ident().unwrap_or_default();
                    props.push(Prop::new(key, Expr::Bool(true), self.current_span()));
                }
                // prop=value
                _ if self.current_kind().is_ident_like()
                    && self.peek_ahead(1).kind == TokenKind::Equals =>
                {
                    let key = self.expect_ident().unwrap_or_default();
                    self.advance(); // =
                    let val = self.parse_expr().unwrap_or(Expr::Null);
                    let prop_span = span;
                    if is_event_property(&key) {
                        if let Some(body) = event_expression_to_body(&val, prop_span) {
                            events.push(EventHandler {
                                event: normalize_event_name(&key),
                                body,
                                span: prop_span,
                            });
                        } else {
                            props.push(Prop::new(key, val, prop_span));
                        }
                    } else {
                        props.push(Prop::new(key, val, prop_span));
                    }
                }
                // inline text / binding content
                TokenKind::StringLit(_)
                | TokenKind::Number(_)
                | TokenKind::Bool(_)
                | TokenKind::Null => {
                    let expr = self
                        .parse_expr()
                        .unwrap_or(korlix_ast::expression::Expr::Null);
                    inline_children.push(Node::Text(TextNode { value: expr, span }));
                }
                _ if self.current_kind().is_ident_like() => {
                    let expr = self
                        .parse_expr()
                        .unwrap_or(korlix_ast::expression::Expr::Null);
                    inline_children.push(Node::Text(TextNode { value: expr, span }));
                }
                // on:event
                TokenKind::OnEvent(ev) => {
                    let ev = ev.clone();
                    let ev_span = self.current_span();
                    self.advance();
                    self.expect(&TokenKind::Colon);
                    let body = self.parse_block();
                    events.push(EventHandler {
                        event: ev,
                        body,
                        span: ev_span,
                    });
                    break;
                }
                _ => break,
            }
        }

        // Check for an indented child block. Korlix V2 makes the trailing
        // colon optional, while V1 source remains valid.
        let has_colon = self.check(&TokenKind::Colon);
        let mut i = self.pos + usize::from(has_colon);
        while i < self.tokens.len() && matches!(self.tokens[i].kind, TokenKind::Newline) {
            i += 1;
        }
        let has_block = i < self.tokens.len() && matches!(self.tokens[i].kind, TokenKind::Indent);

        if has_colon {
            self.advance();
        }
        let children = if has_block {
            self.parse_block()
        } else {
            vec![]
        };

        let mut all_children = children;
        if !inline_children.is_empty() {
            inline_children.extend(all_children);
            all_children = inline_children;
        }

        if is_html_tag(&name) {
            Some(Node::Element(ElementNode {
                tag: name,
                classes,
                props,
                events,
                children: all_children,
                span,
            }))
        } else {
            // Korlix component
            Some(Node::Component(ComponentNode {
                name,
                classes,
                props,
                slots: vec![],
                events,
                children: all_children,
                span,
            }))
        }
    }
}

fn is_event_property(name: &str) -> bool {
    matches!(
        name,
        "click"
            | "double-click"
            | "input"
            | "change"
            | "submit"
            | "focus"
            | "blur"
            | "keydown"
            | "keyup"
            | "mouseenter"
            | "mouseleave"
            | "scroll"
            | "load"
            | "error"
            | "drag"
            | "drop"
            | "touch-start"
            | "touch-end"
    )
}

fn normalize_event_name(name: &str) -> String {
    match name {
        "double-click" => "dblclick".into(),
        "touch-start" => "touchstart".into(),
        "touch-end" => "touchend".into(),
        other => other.into(),
    }
}

fn event_expression_to_body(expr: &Expr, span: korlix_core::Span) -> Option<Vec<Node>> {
    match expr {
        Expr::Identifier(name) => Some(vec![Node::Call(CallNode {
            callee: name.clone(),
            args: vec![],
            span,
        })]),
        Expr::Call { callee, args } => {
            let callee = match callee.as_ref() {
                Expr::Identifier(name) => name.clone(),
                other => other.to_string(),
            };
            Some(vec![Node::Call(CallNode {
                callee,
                args: args.clone(),
                span,
            })])
        }
        _ => None,
    }
}

fn is_boolean_property(name: &str) -> bool {
    matches!(
        name,
        "allowfullscreen"
            | "async"
            | "autofocus"
            | "autoplay"
            | "checked"
            | "controls"
            | "default"
            | "defer"
            | "disabled"
            | "formnovalidate"
            | "hidden"
            | "inert"
            | "ismap"
            | "itemscope"
            | "loop"
            | "multiple"
            | "muted"
            | "nomodule"
            | "novalidate"
            | "open"
            | "playsinline"
            | "readonly"
            | "required"
            | "reversed"
            | "selected"
            | "url-sync"
    )
}
