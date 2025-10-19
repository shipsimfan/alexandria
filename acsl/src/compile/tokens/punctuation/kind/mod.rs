mod display;

/// The kind of a punctuation a token is
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::compile) enum PunctuationKind {
    /// A colon ':'
    Colon,

    /// A comma ','
    Comma,

    /// A dash '-'
    Dash,

    /// A dot '.'
    Dot,

    /// A hash character '#'
    Hash,

    /// A semicolon ';'
    Semicolon,

    /// An opening brace '{'
    OpenBrace,

    /// A closing brace '}'
    CloseBrace,

    /// An opening bracket '['
    OpenBracket,

    /// A closing bracket ']'
    CloseBracket,

    /// An opening parentheses '('
    OpenParentheses,

    /// A closing parentheses ')'
    CloseParentheses,

    /// A fat arrow '->'
    FatArrow,

    /// Two colons back-to-back '::'
    DoubleColon,
}
