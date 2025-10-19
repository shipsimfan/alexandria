use crate::compile::ast::{display_prefix, items::StructField};

impl<'a> StructField<'a> {
    /// Pretty-prints a [`StructField`]
    pub(in crate::compile) fn display(
        &self,
        depth: usize,
        first: bool,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if !first && self.attributes.len() > 0 {
            writeln!(f)?;
        }

        for attribute in &self.attributes {
            attribute.display(depth, f)?;
        }

        display_prefix(depth, f)?;
        writeln!(f, "{}: {},", self.name, self.r#type)
    }
}

impl<'a> std::fmt::Display for StructField<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display(0, true, f)
    }
}
