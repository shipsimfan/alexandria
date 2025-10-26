mod display;
mod pretty_print;

/// Metadata effecting a field in a structure
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StructFieldMeta {
    /// The field holds a position
    Position,
}
