mod display;

/// The kind of literal an expression is
#[derive(Debug)]
pub(in crate::compile) enum LiteralKind {
    /// The literal is a floating point value
    FloatingPoint(f64),

    /// The literal is an integer
    Integer(u64),
}
