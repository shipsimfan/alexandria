use crate::compile::ast::{display_prefix, Statement};

impl<'a> Statement<'a> {
    /// Pretty-prints a [`Statement`]
    pub(in crate::compile) fn display(
        &self,
        depth: usize,
        semicolon: bool,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Statement::Expression(expression) => {
                display_prefix(depth, f)?;
                expression.display(depth, true, f)?;
                if semicolon {
                    write!(f, ";")?;
                }
                writeln!(f)
            }
        }
    }
}

impl<'a> std::fmt::Display for Statement<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display(0, true, f)
    }
}
