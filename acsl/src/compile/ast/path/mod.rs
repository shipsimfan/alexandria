use lct_diagnostics::Span;

mod display;

/// A path to an item
#[derive(Debug)]
pub(in crate::compile) struct Path<'a> {
    /// The first item in the path
    first: &'a str,

    /// The second item in the path
    second: Option<&'a str>,

    /// The location of this [`Path`]
    span: Span,
}
