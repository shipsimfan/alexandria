mod pretty_print;

/// Metadata effecting a field in a structure
#[derive(Debug)]
pub enum StructFieldMeta {
    /// The field holds a position
    Position,
}
