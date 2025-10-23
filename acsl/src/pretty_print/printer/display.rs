use crate::pretty_print::{PrettyFormatter, PrettyPrint, PrettyPrinter};

impl<'a, T: PrettyPrint + ?Sized> std::fmt::Display for PrettyPrinter<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut new_f = PrettyFormatter::new(f, self.indent, self.base_indent);

        self.element.fmt(0, &mut new_f)
    }
}
