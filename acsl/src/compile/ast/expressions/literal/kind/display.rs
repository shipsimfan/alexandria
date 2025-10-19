use crate::compile::ast::expressions::LiteralKind;

impl std::fmt::Display for LiteralKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralKind::FloatingPoint(floating_point) => {
                floating_point.fmt(f)?;
                if floating_point.fract() == 0.0 {
                    write!(f, ".")?;
                }
                Ok(())
            }
            LiteralKind::Integer(integer) => integer.fmt(f),
        }
    }
}
