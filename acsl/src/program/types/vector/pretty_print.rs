use crate::{
    pretty_print::{PrettyFormatter, PrettyPrint},
    program::types::Vector,
};

impl PrettyPrint for Vector {
    fn fmt(&self, depth: usize, f: &mut PrettyFormatter) -> std::fmt::Result {
        f.print_prefix(depth)?;
        writeln!(
            f,
            "Vector {} #{} [{}; {}]",
            self.name(),
            self.id(),
            self.r#type(),
            self.size()
        )
    }
}
