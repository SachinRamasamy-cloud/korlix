//! Basic lexer tests

#[cfg(test)]
mod tests {
    use korlix_lexer::{lex, token::TokenKind};

    #[test]
    fn test_lex_keywords() {
        let (tokens, diag) = lex("page index route "/":", 0);
        assert!(!diag.has_errors(), "No lex errors expected");
        assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::Page)));
        assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::Route)));
    }

    #[test]
    fn test_lex_class() {
        let (tokens, _) = lex("div .flex .bg-blue-500", 0);
        let classes: Vec<_> = tokens.iter()
            .filter_map(|t| if let TokenKind::Class(c) = &t.kind { Some(c.as_str()) } else { None })
            .collect();
        assert_eq!(classes, vec!["flex", "bg-blue-500"]);
    }

    #[test]
    fn test_lex_string() {
        let (tokens, _) = lex(r#"h1 "Hello Korlix""#, 0);
        let strings: Vec<_> = tokens.iter()
            .filter_map(|t| if let TokenKind::StringLit(s) = &t.kind { Some(s.as_str()) } else { None })
            .collect();
        assert_eq!(strings, vec!["Hello Korlix"]);
    }

    #[test]
    fn test_lex_on_event() {
        let (tokens, _) = lex("btn on:click:", 0);
        assert!(tokens.iter().any(|t| matches!(&t.kind, TokenKind::OnEvent(e) if e == "click")));
    }

    #[test]
    fn test_lex_indentation() {
        let src = "page index:
  h1 "Hello"
  p "World"";
        let (tokens, _) = lex(src, 0);
        let indent_count = tokens.iter().filter(|t| t.kind == TokenKind::Indent).count();
        let dedent_count = tokens.iter().filter(|t| t.kind == TokenKind::Dedent).count();
        assert_eq!(indent_count, 1, "One indent block");
        assert_eq!(dedent_count, 1, "One dedent block");
    }

    #[test]
    fn test_lex_bool_literals() {
        let (tokens, _) = lex("true false", 0);
        let bools: Vec<bool> = tokens.iter()
            .filter_map(|t| if let TokenKind::Bool(b) = t.kind { Some(b) } else { None })
            .collect();
        assert_eq!(bools, vec![true, false]);
    }

    #[test]
    fn test_lex_number() {
        let (tokens, _) = lex("42 3.14", 0);
        let nums: Vec<f64> = tokens.iter()
            .filter_map(|t| if let TokenKind::Number(n) = t.kind { Some(n) } else { None })
            .collect();
        assert!((nums[0] - 42.0).abs() < f64::EPSILON);
        assert!((nums[1] - 3.14).abs() < 0.001);
    }

    #[test]
    fn test_lex_comment() {
        let (tokens, _) = lex("# This is a comment
h1 "Hello"", 0);
        // Comment should be skipped
        assert!(tokens.iter().any(|t| matches!(&t.kind, TokenKind::Ident(s) if s == "h1")));
    }

    #[test]
    fn test_lex_arbitrary_class() {
        let (tokens, _) = lex("div .w-[320px]", 0);
        assert!(tokens.iter().any(|t| matches!(&t.kind, TokenKind::Class(c) if c == "w-[320px]")));
    }
}
