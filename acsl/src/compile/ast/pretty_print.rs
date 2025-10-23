use crate::{
    compile::Ast,
    pretty_print::{PrettyFormatter, PrettyPrint},
};

impl<'a> PrettyPrint for Ast<'a> {
    fn fmt(&self, depth: usize, f: &mut PrettyFormatter) -> std::fmt::Result {
        let mut first = true;
        for item in &self.items {
            if first {
                first = false;
            } else {
                writeln!(f)?;
            }

            item.fmt(depth, f)?;
        }
        Ok(())
    }
}
