use crate::{
    pretty_print::{PrettyFormatter, PrettyPrint},
    program::types::Matrix,
};

impl PrettyPrint for Matrix {
    fn fmt(&self, depth: usize, f: &mut PrettyFormatter) -> std::fmt::Result {
        f.print_prefix(depth)?;
        writeln!(
            f,
            "Matrix {} #{} [[{}; {}]; {}]",
            self.name(),
            self.id(),
            self.r#type(),
            self.columns(),
            self.rows()
        )
    }
}
