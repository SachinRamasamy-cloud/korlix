use crate::parser::Parser;
use korlix_ast::{
    element::{ClassRef, ComponentNode, ElementNode, EventHandler, Prop},
    node::{Node, TextNode},
};
use korlix_lexer::token::TokenKind;

/// HTML-native tags that map directly to HTML elements.
const HTML_TAGS: &[&str] = &[
    "div", "span", "p", "h1", "h2", "h3", "h4", "h5", "h6",
    "a", "ul", "ol", "li", "table", "thead", "tbody", "tr", "td", "th",
    "form", "input", "textarea", "select", "option", "button", "label",
    "header", "footer", "main", "nav", "section", "article", "aside",
    "figure", "figcaption", "img", "video", "audio", "canvas", "svg",
    "code", "pre", "blockquote", "br", "hr", "strong", "em", "small",
    "sub", "sup", "del", "ins", "mark", "abbr", "cite", "q",
    "details", "summary", "dialog", "template",
];

fn is_html_tag(name: &str) -> bool {
    HTML_TAGS.contains(&name)
}

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
                // prop=value
                _ if self.current_kind().is_ident_like() && self.peek_ahead(1).kind == TokenKind::Equals => {
                    let key = self.expect_ident().unwrap_or_default();
                    self.advance(); // =
                    let val = self.parse_expr().unwrap_or(korlix_ast::expression::Expr::Null);
                    let prop_span = span;
                    props.push(Prop::new(key, val, prop_span));
                }
                // inline text / binding content
                TokenKind::StringLit(_) | TokenKind::Number(_) | TokenKind::Bool(_) | TokenKind::Null => {
                    let expr = self.parse_expr().unwrap_or(korlix_ast::expression::Expr::Null);
                    inline_children.push(Node::Text(TextNode { value: expr, span }));
                }
                _ if self.current_kind().is_ident_like() => {
                    let expr = self.parse_expr().unwrap_or(korlix_ast::expression::Expr::Null);
                    inline_children.push(Node::Text(TextNode { value: expr, span }));
                }
                // on:event
                TokenKind::OnEvent(ev) => {
                    let ev = ev.clone();
                    let ev_span = self.current_span();
                    self.advance();
                    self.expect(&TokenKind::Colon);
                    let body = self.parse_block();
                    events.push(EventHandler { event: ev, body, span: ev_span });
                    break;
                }
                _ => break,
            }
        }

        // Check for a block (children)
        let has_block = self.check(&TokenKind::Colon) && {
            // lookahead: does the next non-newline token after colon start a block?
            let mut i = self.pos + 1;
            while i < self.tokens.len() {
                match &self.tokens[i].kind {
                    TokenKind::Newline => i += 1,
                    TokenKind::Indent  => break,
                    _                  => { break; }
                }
            }
            i < self.tokens.len() && matches!(self.tokens[i].kind, TokenKind::Indent)
        };

        let children = if has_block {
            self.expect(&TokenKind::Colon);
            self.parse_block()
        } else if self.check(&TokenKind::Colon) {
            self.advance(); // consume lone colon (e.g. on:click: inside same line)
            vec![]
        } else {
            vec![]
        };

        let mut all_children = children;
        if !inline_children.is_empty() {
            inline_children.extend(all_children);
            all_children = inline_children;
        }

        if is_html_tag(&name) {
            Some(Node::Element(ElementNode { tag: name, classes, props, events, children: all_children, span }))
        } else {
            // Korlix component
            Some(Node::Component(ComponentNode { name, classes, props, slots: vec![], events, children: all_children, span }))
        }
    }
}
