use crate::{
    compile::ast::items::Fn,
    pretty_print::{PrettyFormatter, PrettyPrint},
};

impl<'a> PrettyPrint for Fn<'a> {
    fn fmt(&self, depth: usize, f: &mut PrettyFormatter) -> std::fmt::Result {
        for attribute in &self.attributes {
            attribute.fmt(depth, f)?;
        }

        f.print_prefix(depth)?;
        write!(f, "fn {}(", self.name)?;

        let mut first = true;
        for parameter in &self.parameters {
            if first {
                first = false;
            } else {
                write!(f, ", ")?;
            }

            write!(f, "{}", parameter)?;
        }

        write!(f, ")")?;

        if let Some(return_type) = self.return_type {
            write!(f, " -> {}", return_type)?;
        }

        writeln!(f, " {{")?;

        for statement in &self.body {
            statement.fmt(depth + 1, f)?;
            writeln!(f, ";")?;
        }

        if let Some(statement) = &self.return_statement {
            statement.fmt(depth + 1, f)?;
            writeln!(f)?;
        }

        f.print_prefix(depth)?;
        writeln!(f, "}}")
    }
}
