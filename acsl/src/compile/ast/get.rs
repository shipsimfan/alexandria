use crate::compile::{ast::Item, Ast};

impl<'a> Ast<'a> {
    /// Get the items that make up this AST
    pub const fn items(&self) -> &[Item<'a>] {
        self.items.as_slice()
    }
}
