use crate::{compile::ast::expressions::StructCreationField, pretty_print::PrettyFormatter};

impl<'a> StructCreationField<'a> {
    /// Pretty-prints a [`StructCreationField`]
    pub(in crate::compile) fn pretty_print(
        &self,
        depth: usize,
        top_level: bool,
        f: &mut PrettyFormatter,
    ) -> std::fmt::Result {
        if top_level {
            f.print_prefix(depth)?;
        }
        write!(f, "{}: ", self.name)?;
        self.value.pretty_print(depth, top_level, f)
    }
}
