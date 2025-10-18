use crate::compile::tokens::Identifier;

impl<'a> PartialEq for Identifier<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.content.eq(other.content)
    }
}
