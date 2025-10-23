use crate::pretty_print::PrettyFormatter;

impl<'a, 'b> PrettyFormatter<'a, 'b> {
    /// Prints the prefix for newlines
    pub fn print_prefix(&mut self, depth: usize) -> std::fmt::Result {
        for _ in 0..self.base_indent {
            write!(self.f, " ")?;
        }

        for _ in 0..depth {
            for _ in 0..self.indent {
                write!(self.f, " ")?;
            }
        }

        Ok(())
    }
}
