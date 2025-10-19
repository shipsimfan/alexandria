use crate::compile::ast::Expression;

impl<'a> Expression<'a> {
    /// Pretty-prints an [`Expression`]
    pub(in crate::compile) fn display(
        &self,
        depth: usize,
        top_level: bool,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Expression::FunctionCall(function_call) => write!(f, "{}", function_call),
            Expression::Literal(literal) => write!(f, "{}", literal),
            Expression::StructCreation(struct_creation) => {
                struct_creation.display(depth, top_level, f)
            }
        }
    }
}

impl<'a> std::fmt::Display for Expression<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display(0, true, f)
    }
}
