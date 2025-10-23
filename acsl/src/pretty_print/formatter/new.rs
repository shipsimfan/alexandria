use crate::pretty_print::PrettyFormatter;

impl<'a, 'b> PrettyFormatter<'a, 'b> {
    /// Create a new [`PrettyFormatter`]
    pub(in crate::pretty_print) const fn new(
        f: &'a mut std::fmt::Formatter<'b>,
        indent: usize,
        base_indent: usize,
    ) -> Self {
        PrettyFormatter {
            f,
            indent,
            base_indent,
        }
    }
}
