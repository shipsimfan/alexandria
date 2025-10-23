use crate::{
    pretty_print::{PrettyFormatter, PrettyPrint},
    Program,
};

impl PrettyPrint for Program {
    fn fmt(&self, depth: usize, f: &mut PrettyFormatter) -> std::fmt::Result {
        for r#type in &self.types {
            r#type.fmt(depth, f)?;
        }

        Ok(())
    }
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.pretty_print().fmt(f)
    }
}
