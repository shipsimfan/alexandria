use crate::compile::tokens::PunctuationKind;

impl std::fmt::Display for PunctuationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationKind::Colon => ":",
            PunctuationKind::Comma => ",",
            PunctuationKind::Dot => ".",
            PunctuationKind::Hash => "#",
            PunctuationKind::Semicolon => ";",
            PunctuationKind::OpenBrace => "{",
            PunctuationKind::CloseBrace => "}",
            PunctuationKind::OpenBracket => "[",
            PunctuationKind::CloseBracket => "]",
            PunctuationKind::OpenParentheses => "(",
            PunctuationKind::CloseParentheses => ")",
            PunctuationKind::FatArrow => "->",
            PunctuationKind::DoubleColon => "::",
        }
        .fmt(f)
    }
}
