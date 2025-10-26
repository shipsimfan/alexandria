use lct_diagnostics::Span;

use crate::compile::ast::expressions::Expression;

mod parse;
mod pretty_print;

/// A field defined in the creation of a struct
#[derive(Debug)]
pub(in crate::compile) struct StructCreationField<'a> {
    /// The name of the field being defined
    name: &'a str,

    /// The value to fill the field with
    value: Expression<'a>,

    /// The location of this field
    #[allow(unused)]
    span: Span,
}
