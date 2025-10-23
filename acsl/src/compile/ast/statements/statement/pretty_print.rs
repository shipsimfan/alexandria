use crate::{
    compile::ast::Statement,
    pretty_print::{PrettyFormatter, PrettyPrint},
};

impl<'a> PrettyPrint for Statement<'a> {
    fn fmt(&self, depth: usize, f: &mut PrettyFormatter) -> std::fmt::Result {
        match self {
            Statement::Expression(expression) => {
                f.print_prefix(depth)?;
                expression.pretty_print(depth, true, f)
            }
        }
    }
}
