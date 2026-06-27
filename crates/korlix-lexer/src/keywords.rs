use crate::token::TokenKind;

pub fn lookup_keyword(s: &str) -> Option<TokenKind> {
    match s {
        "import" => Some(TokenKind::Import),
        "from" => Some(TokenKind::From),
        "app" => Some(TokenKind::App),
        "page" => Some(TokenKind::Page),
        "layout" => Some(TokenKind::Layout),
        "component" => Some(TokenKind::Component),
        "prop" => Some(TokenKind::Prop),
        "state" => Some(TokenKind::State),
        "let" => Some(TokenKind::Let),
        "derived" => Some(TokenKind::Derived),
        "store" => Some(TokenKind::Store),
        "data" => Some(TokenKind::Data),
        "action" => Some(TokenKind::Action),
        "if" => Some(TokenKind::If),
        "else" => Some(TokenKind::Else),
        "for" => Some(TokenKind::For),
        "in" => Some(TokenKind::In),
        "meta" => Some(TokenKind::Meta),
        "theme" => Some(TokenKind::Theme),
        "mount" => Some(TokenKind::Mount),
        "to" => Some(TokenKind::To),
        "route" => Some(TokenKind::Route),
        "slot" => Some(TokenKind::Slot),
        "providers" => Some(TokenKind::Providers),
        "routes" => Some(TokenKind::Routes),
        "true" => Some(TokenKind::Bool(true)),
        "false" => Some(TokenKind::Bool(false)),
        "null" => Some(TokenKind::Null),
        // ── HTTP / API keywords ─────────────────────────────────────────
        // Use dedicated token variants (not HttpMethod(String)) so the parser
        // can match them by enum discriminant without allocating.
        "get" => Some(TokenKind::Get),
        "post" => Some(TokenKind::Post),
        "put" => Some(TokenKind::Put),
        "patch" => Some(TokenKind::Patch),
        "delete" => Some(TokenKind::Delete),
        "options" => Some(TokenKind::Options),
        "head" => Some(TokenKind::Head),
        "reload" => Some(TokenKind::Reload),
        _ => None,
    }
}
