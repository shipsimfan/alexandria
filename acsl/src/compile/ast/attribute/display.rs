use crate::compile::ast::{attribute::Attribute, display_prefix};

impl<'a> Attribute<'a> {
    /// Pretty-prints an [`Attribute`]
    pub(in crate::compile) fn display(
        &self,
        depth: usize,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        display_prefix(depth, f)?;
        writeln!(f, "#[{}]", self.name)
    }
}

impl<'a> std::fmt::Display for Attribute<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display(0, f)
    }
}
