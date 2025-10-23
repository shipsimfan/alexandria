use crate::{
    compile::ast::Expression,
    pretty_print::{PrettyFormatter, PrettyPrint},
};

impl<'a> Expression<'a> {
    /// Pretty-prints an [`Expression`]
    pub(in crate::compile) fn pretty_print(
        &self,
        depth: usize,
        top_level: bool,
        f: &mut PrettyFormatter,
    ) -> std::fmt::Result {
        match self {
            Expression::FunctionCall(function_call) => function_call.fmt(depth, f),
            Expression::Literal(literal) => write!(f, "{}", literal),
            Expression::StructCreation(struct_creation) => {
                struct_creation.pretty_print(depth, top_level, f)
            }
        }
    }
}
