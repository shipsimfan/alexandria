mod new;
mod print_prefix;
mod write;

/// The formatter used when pretty printing
pub struct PrettyFormatter<'a, 'b> {
    /// The underlying display formatter
    f: &'a mut std::fmt::Formatter<'b>,

    /// The number of spaces to add for each level of indentation
    indent: usize,

    /// The number of spaces to add before each line
    base_indent: usize,
}
