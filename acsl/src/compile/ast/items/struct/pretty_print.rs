use crate::{
    compile::ast::items::Struct,
    pretty_print::{PrettyFormatter, PrettyPrint},
};

impl<'a> PrettyPrint for Struct<'a> {
    fn fmt(&self, depth: usize, f: &mut PrettyFormatter) -> std::fmt::Result {
        f.print_prefix(depth)?;
        writeln!(f, "struct {} {{", self.name)?;

        let mut first = true;
        for field in &self.fields {
            if !first && field.attributes().len() > 0 {
                writeln!(f)?;
            }

            field.fmt(depth + 1, f)?;
            first = false;
        }

        f.print_prefix(depth)?;
        writeln!(f, "}}")
    }
}
