use crate::pretty_print::PrettyPrint;

mod display;
mod new;

/// Able to use [`std::fmt::Display`] to pretty print syntactic elements
pub struct PrettyPrinter<'a, T: PrettyPrint + ?Sized> {
    /// The number of spaces to add for each level of indentation
    indent: usize,

    /// The number of spaces to add before each line
    base_indent: usize,

    /// The element to pretty print
    element: &'a T,
}

impl<'a, T: PrettyPrint + ?Sized> PrettyPrinter<'a, T> {
    const DEFAULT_INDENT: usize = 4;
}
