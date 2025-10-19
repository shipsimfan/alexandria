use crate::compile::ast::{display_prefix, items::Struct};

impl<'a> Struct<'a> {
    /// Pretty-prints a [`Struct`]
    pub(in crate::compile) fn display(
        &self,
        depth: usize,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        display_prefix(depth, f)?;
        writeln!(f, "struct {} {{", self.name)?;

        let mut first = true;
        for field in &self.fields {
            field.display(depth + 1, first, f);
            first = false;
        }

        display_prefix(depth, f)?;
        writeln!(f, "}}")
    }
}

impl<'a> std::fmt::Display for Struct<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display(0, f)
    }
}
