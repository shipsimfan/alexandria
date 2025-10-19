use crate::compile::tokens::KeywordKind;

impl std::fmt::Display for KeywordKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeywordKind::Fn => "fn",
            KeywordKind::Struct => "struct",
        }
        .fmt(f)
    }
}
