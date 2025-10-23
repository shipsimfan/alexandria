use crate::{
    pretty_print::{PrettyFormatter, PrettyPrint},
    program::types::StructField,
};

impl PrettyPrint for StructField {
    fn fmt(&self, depth: usize, f: &mut PrettyFormatter) -> std::fmt::Result {
        for meta in self.meta() {
            meta.fmt(depth, f)?;
        }

        f.print_prefix(depth)?;
        writeln!(f, "{}: {}", self.name(), self.r#type())
    }
}
