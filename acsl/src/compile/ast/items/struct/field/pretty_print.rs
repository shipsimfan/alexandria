use crate::{
    compile::ast::items::StructField,
    pretty_print::{PrettyFormatter, PrettyPrint},
};

impl<'a> PrettyPrint for StructField<'a> {
    fn fmt(&self, depth: usize, f: &mut PrettyFormatter) -> std::fmt::Result {
        for attribute in &self.attributes {
            attribute.fmt(depth, f)?;
        }

        f.print_prefix(depth)?;
        writeln!(f, "{}: {},", self.name, self.r#type)
    }
}
