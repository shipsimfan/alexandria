use crate::compile::Lexer;
use lct_diagnostics::Source;

impl<'a> Lexer<'a> {
    /// Create a new [`Lexer`] over `source`
    pub fn new(source: &'a Source) -> Self {
        let stream = source.char_stream();

        Lexer { stream }
    }
}
