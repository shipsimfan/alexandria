use crate::compile::tokens::Identifier;

impl<'a> std::fmt::Display for Identifier<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "identifier \"{}\"", self.content)
    }
}
