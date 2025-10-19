use crate::compile::ast::{display_prefix, expressions::StructCreationField};

impl<'a> StructCreationField<'a> {
    /// Pretty-prints a [`StructCreationField`]
    pub(in crate::compile) fn display(
        &self,
        depth: usize,
        top_level: bool,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if top_level {
            display_prefix(depth, f)?;
        }
        write!(f, "{}: ", self.name)?;
        self.value.display(depth, top_level, f)
    }
}

impl<'a> std::fmt::Display for StructCreationField<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display(0, true, f)
    }
}
