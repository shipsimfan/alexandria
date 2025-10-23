use crate::{
    pretty_print::{PrettyFormatter, PrettyPrint},
    program::types::Primitive,
};

impl PrettyPrint for Primitive {
    fn fmt(&self, depth: usize, f: &mut PrettyFormatter) -> std::fmt::Result {
        f.print_prefix(depth)?;
        writeln!(f, "Primitive {} #{}", self.name(), self.id())
    }
}
