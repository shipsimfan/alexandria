use crate::compile::tokens::KeywordKind;

impl KeywordKind {
    /// Tries to parse `content` into a [`KeywordKind`]
    pub(in crate::compile::tokens::keyword) fn parse(content: &str) -> Option<Self> {
        Some(match content {
            "fn" => KeywordKind::Fn,
            "struct" => KeywordKind::Struct,
            _ => return None,
        })
    }
}
