use crate::compile::tokens::KeywordKind;

impl std::fmt::Display for KeywordKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeywordKind::Struct => "struct",
            KeywordKind::Fn => "fn",
        }
        .fmt(f)
    }
}
