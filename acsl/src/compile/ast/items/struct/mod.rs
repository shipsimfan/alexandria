use lct_diagnostics::Span;

mod field;

mod display;

pub(in crate::compile) use field::StructField;

/// A structure definition
#[derive(Debug)]
pub(in crate::compile) struct Struct<'a> {
    /// The name of the structure
    name: &'a str,

    /// The fields making up the struct
    fields: Vec<StructField<'a>>,

    /// The location of the structure
    span: Span,
}
