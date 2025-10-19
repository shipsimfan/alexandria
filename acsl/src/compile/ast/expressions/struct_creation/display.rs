use crate::compile::ast::{display_prefix, expressions::StructCreation};

impl<'a> StructCreation<'a> {
    /// Pretty-prints a [`StructCreation`]
    pub(in crate::compile) fn display(
        &self,
        depth: usize,
        top_level: bool,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{} {{", self.name)?;
        if top_level {
            writeln!(f)?;
        } else {
            write!(f, " ")?;
        }

        let mut first = true;
        for field in &self.fields {
            if !top_level {
                if first {
                    first = false;
                } else {
                    write!(f, ", ")?;
                }
            }

            field.display(depth + 1, top_level, f)?;
            if top_level {
                writeln!(f, ",")?;
            }
        }

        if top_level {
            display_prefix(depth, f)?;
        }
        write!(f, "}}")
    }
}

impl<'a> std::fmt::Display for StructCreation<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display(0, true, f)
    }
}
