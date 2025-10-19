use crate::compile::ast::{display_prefix, items::Fn};

impl<'a> Fn<'a> {
    /// Pretty-prints a [`Fn`]
    pub(in crate::compile) fn display(
        &self,
        depth: usize,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        for attribute in &self.attributes {
            attribute.display(depth, f)?;
        }

        display_prefix(0, f)?;
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

        writeln!(f, "{{")?;

        for statement in &self.body {
            statement.display(depth + 1, true, f)?;
        }

        if let Some(statement) = &self.return_statement {
            statement.display(depth + 1, false, f)?;
        }

        display_prefix(depth, f)?;
        writeln!(f, "}}")
    }
}

impl<'a> std::fmt::Display for Fn<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display(0, f)
    }
}
