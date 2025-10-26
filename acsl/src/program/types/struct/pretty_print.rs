use crate::{
    pretty_print::{PrettyFormatter, PrettyPrint},
    program::types::Struct,
};

impl PrettyPrint for Struct {
    fn fmt(&self, depth: usize, f: &mut PrettyFormatter) -> std::fmt::Result {
        f.print_prefix(depth)?;
        writeln!(f, "Struct {} #{} {{", self.name(), self.id())?;
        for field in self.fields() {
            field.fmt(depth + 1, f)?;
        }
        writeln!(f, "}}")
    }
}
