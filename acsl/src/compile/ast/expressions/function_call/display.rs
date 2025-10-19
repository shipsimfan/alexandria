use crate::compile::ast::expressions::FunctionCall;

impl<'a> std::fmt::Display for FunctionCall<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}(", self.path)?;
        let mut first = true;
        for parameter in &self.parameters {
            if first {
                first = false;
            } else {
                write!(f, ", ")?;
            }

            parameter.fmt(f)?;
        }
        write!(f, ")")
    }
}
