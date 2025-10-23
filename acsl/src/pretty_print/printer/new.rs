use crate::pretty_print::{PrettyPrint, PrettyPrinter};

impl<'a, T: PrettyPrint + ?Sized> PrettyPrinter<'a, T> {
    /// Create a new [`PrettyPrinter`]
    pub(in crate::pretty_print) const fn new(element: &'a T) -> Self {
        PrettyPrinter {
            indent: Self::DEFAULT_INDENT,
            base_indent: 0,
            element,
        }
    }
}
