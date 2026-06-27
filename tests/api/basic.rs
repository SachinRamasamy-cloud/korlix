//! API parsing tests: `get`, `post`, `put`, `patch`, `delete`, `reload`.

#[cfg(test)]
mod tests {
    use korlix_lexer::lex;
    use korlix_parser::Parser;
    use korlix_ast::node::Node;
    use std::path::PathBuf;

    fn parse(source: &str) -> korlix_ast::program::Module {
        let (tokens, lex_diag) = lex(source, 0);
        assert!(
            !lex_diag.has_errors(),
            "Lex errors: {:?}",
            lex_diag
        );
        let parser = Parser::new(&tokens, 0);
        let (module, parse_diag) = parser.parse(PathBuf::from("<test>"));
        assert!(
            !parse_diag.has_errors(),
            "Parse errors: {:?}",
            parse_diag
        );
        module
    }

    // ── get ────────────────────────────────────────────────────────────────

    #[test]
    fn parses_get_api_query() {
        let module = parse(r#"
page Users route "/users":
  get users "/api/users"
"#);
        assert!(!module.items.is_empty(), "Should parse a page");

        if let korlix_ast::program::Item::Page(page) = &module.items[0] {
            let has_query = page.body.iter().any(|n| matches!(n, Node::ApiQuery(q) if q.name == "users"));
            assert!(has_query, "Expected ApiQuery(users) in page body, got: {:?}", page.body);
        } else {
            panic!("Expected Item::Page");
        }
    }

    #[test]
    fn api_query_captures_url() {
        let module = parse(r#"
page Test route "/test":
  get items "/api/items"
"#);
        if let korlix_ast::program::Item::Page(page) = &module.items[0] {
            let query = page.body.iter().find_map(|n| {
                if let Node::ApiQuery(q) = n { Some(q) } else { None }
            });
            let q = query.expect("ApiQuery not found");
            assert_eq!(q.name, "items");
            assert_eq!(q.url, "/api/items");
        }
    }

    // ── post / mutation ────────────────────────────────────────────────────

    #[test]
    fn parses_api_mutation_inside_action() {
        let module = parse(r#"
page Users route "/users":
  action save:
    post "/api/users" { name: name }
    reload users
"#);
        if let korlix_ast::program::Item::Page(page) = &module.items[0] {
            let action = page.body.iter().find_map(|n| {
                if let Node::Action(a) = n { Some(a) } else { None }
            });
            let a = action.expect("Action not found");

            // Should contain a post mutation
            let has_post = a.body.iter().any(|n| {
                matches!(n, Node::ApiMutation(m) if m.url == "/api/users")
            });
            assert!(has_post, "Expected ApiMutation(post) in action body");

            // Should contain a reload
            let has_reload = a.body.iter().any(|n| {
                matches!(n, Node::ApiReload(r) if r.target == "users")
            });
            assert!(has_reload, "Expected ApiReload(users) in action body");
        }
    }

    #[test]
    fn parses_delete_without_body() {
        let module = parse(r#"
page Users route "/users":
  action remove:
    delete "/api/users/1"
"#);
        if let korlix_ast::program::Item::Page(page) = &module.items[0] {
            let action = page.body.iter().find_map(|n| {
                if let Node::Action(a) = n { Some(a) } else { None }
            });
            let a = action.expect("Action not found");
            let mutation = a.body.iter().find_map(|n| {
                if let Node::ApiMutation(m) = n { Some(m) } else { None }
            });
            let m = mutation.expect("ApiMutation not found");
            assert!(m.body.is_none(), "DELETE should have no body");
        }
    }

    #[test]
    fn parses_put_mutation() {
        let module = parse(r#"
page Edit route "/edit":
  action update:
    put "/api/users/1" { name: newName }
"#);
        if let korlix_ast::program::Item::Page(page) = &module.items[0] {
            let action = page.body.iter().find_map(|n| {
                if let Node::Action(a) = n { Some(a) } else { None }
            });
            let a = action.expect("Action not found");
            let has_put = a.body.iter().any(|n| {
                matches!(n, Node::ApiMutation(m)
                    if m.url == "/api/users/1"
                    && m.method == korlix_ast::api::HttpMethod::Put)
            });
            assert!(has_put, "Expected ApiMutation(put) in action body");
        }
    }

    // ── reload ─────────────────────────────────────────────────────────────

    #[test]
    fn parses_reload_statement() {
        let module = parse(r#"
page Users route "/users":
  action refresh:
    reload users
"#);
        if let korlix_ast::program::Item::Page(page) = &module.items[0] {
            let action = page.body.iter().find_map(|n| {
                if let Node::Action(a) = n { Some(a) } else { None }
            });
            let a = action.expect("Action not found");
            let has_reload = a.body.iter().any(|n| {
                matches!(n, Node::ApiReload(r) if r.target == "users")
            });
            assert!(has_reload, "Expected ApiReload(users) in action body");
        }
    }

    // ── lexer sanity ────────────────────────────────────────────────────────

    #[test]
    fn lexes_api_keywords() {
        use korlix_lexer::token::TokenKind;
        let keywords = ["get", "post", "put", "patch", "delete", "reload"];
        for kw in &keywords {
            let (tokens, _) = lex(kw, 0);
            let has_keyword = tokens.iter().any(|t| matches!(
                &t.kind,
                TokenKind::Get | TokenKind::Post | TokenKind::Put
                | TokenKind::Patch | TokenKind::Delete | TokenKind::Reload
            ));
            assert!(has_keyword, "Expected API keyword token for `{}`", kw);
        }
    }
}
