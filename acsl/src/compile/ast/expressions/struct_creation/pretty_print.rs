use crate::{compile::ast::expressions::StructCreation, pretty_print::PrettyFormatter};

impl<'a> StructCreation<'a> {
    /// Pretty-prints a [`StructCreation`]
    pub(in crate::compile) fn pretty_print(
        &self,
        depth: usize,
        top_level: bool,
        f: &mut PrettyFormatter,
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

            field.pretty_print(depth + 1, top_level, f)?;
            if top_level {
                writeln!(f, ",")?;
            }
        }

        if top_level {
            f.print_prefix(depth)?;
        }
        write!(f, "}}")
    }
}
