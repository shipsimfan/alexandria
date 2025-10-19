use crate::compile::Ast;

impl<'a> Ast<'a> {
    /// Pretty-prints the AST
    pub(in crate::compile) fn display(
        &self,
        depth: usize,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let mut first = true;
        for item in &self.items {
            if first {
                first = false;
            } else {
                writeln!(f)?;
            }

            item.display(depth, f)?;
        }
        Ok(())
    }
}

impl<'a> std::fmt::Display for Ast<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display(0, f)
    }
}
