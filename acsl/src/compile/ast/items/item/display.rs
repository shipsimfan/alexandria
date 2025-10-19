use crate::compile::ast::Item;

impl<'a> Item<'a> {
    /// Pretty-prints an [`Item`]
    pub(in crate::compile) fn display(
        &self,
        depth: usize,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Item::Fn(r#fn) => r#fn.display(depth, f),
            Item::Struct(r#struct) => r#struct.display(depth, f),
        }
    }
}

impl<'a> std::fmt::Display for Item<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display(0, f)
    }
}
