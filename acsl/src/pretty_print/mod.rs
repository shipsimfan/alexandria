//! Utilities for pretty printing syntactic elements

mod formatter;
mod printer;

pub use formatter::PrettyFormatter;
pub use printer::PrettyPrinter;

/// A trait that allows a syntax element to be pretty printed
pub trait PrettyPrint {
    /// Display the value using pretty printing
    fn fmt<'a, 'b>(&self, depth: usize, f: &mut PrettyFormatter<'a, 'b>) -> std::fmt::Result;

    /// Print the value using pretty printing
    fn pretty_print<'a>(&'a self) -> PrettyPrinter<'a, Self> {
        PrettyPrinter::new(self)
    }
}
