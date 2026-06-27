use crate::parser::Parser;
use korlix_ast::{
    declarations::{
        ActionDecl, DataDecl, DerivedDecl, ImportDecl, LetDecl, MetaBlock, RouteDecl, StateDecl,
        ThemeDecl,ApiMutation,ApiQueryNode,ApiReloadNode,ApiRouteNode,HttpMethod,Node,
    },
    expression::Expr,
    node::{AssignNode, CallNode, ForNode, IfNode, Node, TextNode},
    program::{AppDecl, ComponentDecl, Item, LayoutDecl, MountDecl, PageDecl},
    types::KType,
};
use korlix_lexer::token::TokenKind;

impl<'t> Parser<'t> {
    pub fn parse_top_level_item(&mut self) -> Option<Item> {
        self.skip_newlines();
        match self.current_kind() {
            TokenKind::Import => self
                .parse_import()
                .map(|_i| {
                    // imports stored in module.imports — just skip here
                    None::<Item>
                })
                .flatten(),
            TokenKind::Mount => self.parse_mount().map(Item::MountDecl),
            TokenKind::App => self.parse_app().map(Item::AppDecl),
            TokenKind::Page => self.parse_page().map(Item::Page),
            TokenKind::Layout => self.parse_layout().map(Item::Layout),
            TokenKind::Component => self.parse_component().map(Item::Component),
            _ => None,
        }
    }

    // ── import ─────────────────────────────────────────────────────────
    pub fn parse_import(&mut self) -> Option<ImportDecl> {
        let span_start = self.current_span();
        self.expect(&TokenKind::Import);

        // import Name from "path"  or  import "path"
        if let TokenKind::StringLit(path) = self.current_kind() {
            let path = path.clone();
            self.advance();
            return Some(ImportDecl {
                name: None,
                path,
                span: span_start,
            });
        }

        let name = self.expect_ident()?;
        self.expect(&TokenKind::From);
        let path = if let TokenKind::StringLit(p) = self.current_kind() {
            let p = p.clone();
            self.advance();
            p
        } else {
            self.diagnostics
                .error("KX-E002", "Expected string path after `from`");
            return None;
        };
        Some(ImportDecl {
            name: Some(name),
            path,
            span: span_start,
        })
    }

    // ── mount ─────────────────────────────────────────────────────────
    fn parse_mount(&mut self) -> Option<MountDecl> {
        let span = self.current_span();
        self.advance(); // mount
        let component = self.expect_ident()?;
        self.expect(&TokenKind::To);
        let selector = if let TokenKind::StringLit(s) = self.current_kind() {
            let s = s.clone();
            self.advance();
            s
        } else {
            "#korlix-root".into()
        };
        Some(MountDecl {
            component,
            selector,
            span,
        })
    }

    // ── app ───────────────────────────────────────────────────────────
    fn parse_app(&mut self) -> Option<AppDecl> {
        let span = self.current_span();
        self.advance(); // app
        self.expect(&TokenKind::Colon);

        let mut layout = None;
        let mut routes = vec![];
        let mut providers = vec![];
        let mut theme = None;

        self.skip_newlines();
        if self.check(&TokenKind::Indent) {
            self.advance();
            loop {
                self.skip_newlines();
                if self.check(&TokenKind::Dedent) || self.is_eof() {
                    break;
                }

                match self.current_kind() {
                    TokenKind::Ident(ref s) if s == "layout" => {
                        self.advance();
                        layout = self.expect_ident();
                    }
                    TokenKind::Routes => {
                        self.advance();
                        self.expect(&TokenKind::Colon);
                        self.skip_newlines();
                        if self.check(&TokenKind::Indent) {
                            self.advance();
                            loop {
                                self.skip_newlines();
                                if self.check(&TokenKind::Dedent) || self.is_eof() {
                                    break;
                                }
                                if let Some(r) = self.parse_route_decl() {
                                    routes.push(r);
                                }
                            }
                            if self.check(&TokenKind::Dedent) {
                                self.advance();
                            }
                        }
                    }
                    TokenKind::Providers => {
                        self.advance();
                        self.expect(&TokenKind::Colon);
                        self.skip_newlines();
                        if self.check(&TokenKind::Indent) {
                            self.advance();
                            loop {
                                self.skip_newlines();
                                if self.check(&TokenKind::Dedent) || self.is_eof() {
                                    break;
                                }
                                if let Some(p) = self.expect_ident() {
                                    providers.push(p);
                                }
                            }
                            if self.check(&TokenKind::Dedent) {
                                self.advance();
                            }
                        }
                    }
                    TokenKind::Theme => {
                        self.advance();
                        self.expect(&TokenKind::Colon);
                        theme = Some(self.parse_theme_block());
                    }
                    _ => {
                        self.advance();
                    }
                }
            }
            if self.check(&TokenKind::Dedent) {
                self.advance();
            }
        }

        Some(AppDecl {
            layout,
            routes,
            providers,
            theme,
            span,
        })
    }

    fn parse_route_decl(&mut self) -> Option<RouteDecl> {
        let span = self.current_span();
        // page "/" from "./pages/index.klx"
        let _kw = self.expect_ident()?; // "page"
        let path_str = if let TokenKind::StringLit(s) = self.current_kind() {
            let s = s.clone();
            self.advance();
            s
        } else {
            return None;
        };
        // from "…"
        if let TokenKind::From = self.current_kind() {
            self.advance();
        }
        let source = if let TokenKind::StringLit(s) = self.current_kind() {
            let s = s.clone();
            self.advance();
            s
        } else {
            return None;
        };
        Some(RouteDecl {
            path: path_str,
            source,
            span,
        })
    }

    fn parse_theme_block(&mut self) -> ThemeDecl {
        let span = self.current_span();
        let mut default_mode = None;
        let mut dark_enabled = false;
        self.skip_newlines();
        if self.check(&TokenKind::Indent) {
            self.advance();
            loop {
                self.skip_newlines();
                if self.check(&TokenKind::Dedent) || self.is_eof() {
                    break;
                }
                let key = self.expect_ident().unwrap_or_default();
                match key.as_str() {
                    "default" => {
                        if let TokenKind::StringLit(s) = self.current_kind() {
                            default_mode = Some(s.clone());
                            self.advance();
                        }
                    }
                    "dark" => {
                        if let TokenKind::Bool(b) = self.current_kind() {
                            dark_enabled = b;
                            self.advance();
                        }
                    }
                    _ => {}
                }
            }
            if self.check(&TokenKind::Dedent) {
                self.advance();
            }
        }
        ThemeDecl {
            default_mode,
            dark_enabled,
            span,
        }
    }

    // ── page ──────────────────────────────────────────────────────────
    fn parse_page(&mut self) -> Option<PageDecl> {
        let span = self.current_span();
        self.advance(); // page
        let name = self.expect_ident().unwrap_or("index".into());

        // optional: route "/path"
        let mut route = None;
        if self.check(&TokenKind::Route) {
            self.advance();
            if let TokenKind::StringLit(s) = self.current_kind() {
                route = Some(s.clone());
                self.advance();
            }
        }

        self.expect(&TokenKind::Colon);
        let body_raw = self.parse_block();

        // extract layout and meta from body
        let mut layout = None;
        let mut meta = None;
        let mut body = vec![];

        for node in body_raw {
            match &node {
                Node::Element(el) if el.tag == "layout" => {
                    if let Some(prop) = el.props.first() {
                        if let Expr::String(s) | Expr::Identifier(s) = &prop.value {
                            layout = Some(s.clone());
                        }
                    }
                }
                Node::Element(el) if el.tag == "meta" => {
                    meta = Some(extract_meta_block(&node));
                }
                Node::Component(c) if c.name == "meta" => {
                    meta = Some(extract_meta_block(&node));
                }
                _ => body.push(node),
            }
        }

        Some(PageDecl {
            name,
            route,
            layout,
            meta,
            body,
            span,
        })
    }

    // ── layout ────────────────────────────────────────────────────────
    fn parse_layout(&mut self) -> Option<LayoutDecl> {
        let span = self.current_span();
        self.advance(); // layout
        let name = self.expect_ident()?;
        self.expect(&TokenKind::Colon);
        let body = self.parse_block();
        Some(LayoutDecl { name, body, span })
    }

    // ── component ─────────────────────────────────────────────────────
    fn parse_component(&mut self) -> Option<ComponentDecl> {
        let span = self.current_span();
        self.advance(); // component
        let name = self.expect_ident()?;
        self.expect(&TokenKind::Colon);
        let body_raw = self.parse_block();

        // extract prop declarations from body
        let props = vec![];
        let mut body = vec![];

        for node in body_raw {
            match node {
                Node::State(_s) => {
                    // treat as prop
                }
                _ => body.push(node),
            }
        }

        Some(ComponentDecl {
            name,
            props,
            body,
            span,
        })
    }

    // ── node (inside a block) ─────────────────────────────────────────
    pub fn parse_node(&mut self) -> Option<Node> {
        self.skip_newlines();

        match self.current_kind() {
            TokenKind::State => self.parse_state().map(Node::State),
            TokenKind::Let => self.parse_let().map(Node::Let),
            TokenKind::Derived => self.parse_derived().map(Node::Derived),
            TokenKind::Action => self.parse_action().map(Node::Action),
            TokenKind::Data => self.parse_data().map(Node::Data),
            TokenKind::If => self.parse_if().map(Node::If),
            TokenKind::For => self.parse_for().map(Node::For),
            TokenKind::Slot => {
                let span = self.current_span();
                self.advance();
                let name = self.expect_ident();
                Some(Node::Slot(korlix_ast::node::SlotNode { name, span }))
            }
            _ if self.current_kind().is_ident_like() => self.parse_ident_node(),
            TokenKind::StringLit(_) => {
                let span = self.current_span();
                let expr = self.parse_expr()?;
                Some(Node::Text(TextNode { value: expr, span }))
            }
            _ => None,
        }
    }

    fn parse_ident_node(&mut self) -> Option<Node> {
        let span = self.current_span();
        let name = self.current_kind().as_ident_str()?.to_string();

        if name == "text" {
            self.advance();
            let value = self.parse_expr().unwrap_or(Expr::Null);
            return Some(Node::Text(TextNode { value, span }));
        }

        if self.peek_ahead(1).kind == TokenKind::Equals {
            self.advance();
            self.advance();
            let value = self.parse_expr().unwrap_or(Expr::Null);
            return Some(Node::Assign(AssignNode {
                target: name,
                value,
                span,
            }));
        }

        if self.peek_ahead(1).kind == TokenKind::LParen {
            let expr = self.parse_expr()?;
            if let Expr::Call { callee, args } = expr {
                let callee = match *callee {
                    Expr::Identifier(name) => name,
                    other => other.to_string(),
                };
                return Some(Node::Call(CallNode { callee, args, span }));
            }
        }

        self.parse_element_or_component()
    }

    fn parse_state(&mut self) -> Option<korlix_ast::declarations::StateDecl> {
        let span = self.current_span();
        self.advance(); // state
        let name = self.expect_ident()?;
        let type_ann = if self.check(&TokenKind::Colon) {
            self.advance();
            self.expect_ident().map(|s| KType::from_str(&s))
        } else {
            None
        };
        self.expect(&TokenKind::Equals);
        let value = self.parse_expr()?;
        Some(StateDecl {
            name,
            type_ann,
            value,
            span,
        })
    }

    fn parse_let(&mut self) -> Option<LetDecl> {
        let span = self.current_span();
        self.advance();
        let name = self.expect_ident()?;
        let type_ann = if self.check(&TokenKind::Colon) {
            self.advance();
            self.expect_ident().map(|s| KType::from_str(&s))
        } else {
            None
        };
        self.expect(&TokenKind::Equals);
        let value = self.parse_expr()?;
        Some(LetDecl {
            name,
            type_ann,
            value,
            span,
        })
    }

    fn parse_derived(&mut self) -> Option<DerivedDecl> {
        let span = self.current_span();
        self.advance();
        let name = self.expect_ident()?;
        self.expect(&TokenKind::Equals);
        let value = self.parse_expr()?;
        Some(DerivedDecl { name, value, span })
    }

    fn parse_action(&mut self) -> Option<ActionDecl> {
        let span = self.current_span();
        self.advance();
        let name = self.expect_ident()?;
        self.expect(&TokenKind::Colon);
        let body = self.parse_block();
        Some(ActionDecl {
            name,
            params: vec![],
            body,
            span,
        })
    }

    fn parse_data(&mut self) -> Option<DataDecl> {
        let span = self.current_span();
        self.advance(); // data
        let name = self.expect_ident()?;
        self.expect(&TokenKind::Equals);
        // method url
        let method = self.expect_ident().unwrap_or("get".into());
        let url = self.parse_expr().unwrap_or(Expr::Null);
        self.expect(&TokenKind::Colon);

        // parse loading/error/empty blocks
        let mut loading = None;
        let mut error = None;
        let mut empty = None;
        self.skip_newlines();
        if self.check(&TokenKind::Indent) {
            self.advance();
            loop {
                self.skip_newlines();
                if self.check(&TokenKind::Dedent) || self.is_eof() {
                    break;
                }
                let key = self.expect_ident().unwrap_or_default();
                let nodes = vec![]; // simplified
                match key.as_str() {
                    "loading" => loading = Some(nodes),
                    "error" => error = Some(nodes),
                    "empty" => empty = Some(nodes),
                    _ => {}
                }
            }
            if self.check(&TokenKind::Dedent) {
                self.advance();
            }
        }

        Some(DataDecl {
            name,
            method,
            url,
            loading,
            error,
            empty,
            span,
        })
    }

    fn parse_if(&mut self) -> Option<IfNode> {
        let span = self.current_span();
        self.advance(); // if
        let condition = self.parse_expr()?;
        self.expect(&TokenKind::Colon);
        let then_body = self.parse_block();

        let else_body = if self.check_exact(&TokenKind::Else) {
            self.advance();
            self.expect(&TokenKind::Colon);
            Some(self.parse_block())
        } else {
            None
        };

        Some(IfNode {
            condition,
            then_body,
            else_body,
            span,
        })
    }

    fn parse_for(&mut self) -> Option<ForNode> {
        let span = self.current_span();
        self.advance(); // for
        let var = self.expect_ident()?;
        self.expect(&TokenKind::In);
        let iterable = self.parse_expr()?;
        self.expect(&TokenKind::Colon);
        let body = self.parse_block();
        Some(ForNode {
            var,
            iterable,
            body,
            span,
        })
    }

    fn parse_api_query(&mut self)-> KorlixResult<Node>
    {
        let start = self.current_span();
        self.export(TokenKind:Get,"expected 'get'")?;
        let name =self.expect_identifier("expected API variable name after 'get'")?;
        let url = self.export_string("expected URL after API variable name")?;
        Ok(Node::ApiQuery(ApiQueryNode { name, url, span: start }))
        
        self.skip_newlines();

        Ok(Node::ApiQuery(ApiQueryNode { name, url, span: start }))
    }

    fn parse_api_mutation(&mut self) -> KorlixResult<Node> {
        let start = self.current_sapn();

        let method = match self.current_kind(){
            TokenKind::Post=>{
                seld.advance()
                HttpMethod::Post
            }
            TokenKind::Put=>{
                self.advance();
                HttpMethod::Put
            }
            TokenKind::Delete=>{
                self.advance();
                HttpMethod::Delete
            }
            TokenKind::Patch=>{
                self.advance();
                HttpMethod::Patch
            }
            TokenKind::Options=>{
                self.advance();
                HttpMethod::Options
            }
            TokenKind::Head=>{
                self.advance();
                HttpMethod::Head
            }
            _=>{
                return self.error_current("expected API mutation method (post, put, delete, patch, options, head)");
            };

            let url=self.export_string("expected API URL")?;

            let body=if method == HttpMethod::Delete{
                None
            }else{
                Some{self.parse_expression()?}
            };

            self.skip_newlines();

            ok(Node::ApiMutation(ApiMutationNode{
                method,
                url,
                body,
                span:start,
            }))
        }

    }

    fn parse_api_reload(&mut self)->
    korlixResult<Node>{
        let start=self.current.sapn();
        self.export(TokenKind::Reload,"expected 'reload'")?;

        let target=self.export_indentifier("exported api target after 'reload'")?;

        self.skip_newlines();

        ok(Node::ApiReload(ApiReloadNode{
            target,
            sapn:start,
        }))
    }
}

fn extract_meta_block(node: &Node) -> MetaBlock {
    let (children, span) = match node {
        Node::Element(el) => (&el.children, el.span),
        Node::Component(c) => (&c.children, c.span),
        _ => unreachable!("extract_meta_block only accepts meta nodes"),
    };

    let mut meta = MetaBlock {
        title: None,
        description: None,
        canonical: None,
        og_image: None,
        extras: vec![],
        span,
    };

    for child in children {
        let (name, child_children, child_props) = match child {
            Node::Element(el) => (el.tag.as_str(), &el.children, Some(&el.props)),
            Node::Component(c) => (c.name.as_str(), &c.children, None),
            _ => continue,
        };

        let value = child_children.iter().find_map(|n| {
            if let Node::Text(text) = n {
                Some(text.value.clone())
            } else {
                None
            }
        });

        match name {
            "title" => meta.title = value,
            "description" => meta.description = value,
            "canonical" => meta.canonical = value,
            "og_image" | "og-image" => meta.og_image = value,
            _ => {
                if let Some(props) = child_props {
                    meta.extras.extend(props.clone());
                }
            }
        }
    }

    meta
}
