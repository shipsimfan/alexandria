use lct_diagnostics::Span;

mod field;

mod get;
mod parse;
mod pretty_print;

pub(in crate::compile) use field::StructCreationField;

/// The direct creation of a struct
#[derive(Debug)]
pub(in crate::compile) struct StructCreation<'a> {
    /// The name of struct being created
    name: &'a str,

    /// The fields with their values to fill the struct
    fields: Vec<StructCreationField<'a>>,

    /// The location of this [`StructCreation`] expression
    span: Span,
}
