use crate::{
    pretty_print::{PrettyFormatter, PrettyPrint},
    program::types::StructFieldMeta,
};

impl PrettyPrint for StructFieldMeta {
    fn fmt(&self, depth: usize, f: &mut PrettyFormatter) -> std::fmt::Result {
        f.print_prefix(depth)?;
        writeln!(
            f,
            "#[{}]",
            match self {
                StructFieldMeta::Position => "position",
            }
        )
    }
}
