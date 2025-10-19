use crate::compile::ast::Path;

impl<'a> std::fmt::Display for Path<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.first.fmt(f)?;

        if let Some(second) = self.second {
            write!(f, "::{}", second)?;
        }

        Ok(())
    }
}
