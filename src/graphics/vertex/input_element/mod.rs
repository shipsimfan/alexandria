mod r#type;

mod get;
mod new;

pub use r#type::InputElementType;

/// The description of a single input element of a Vertex
pub struct InputElement {
    /// The semantic name attached to the input element
    semantic_name: &'static str,

    /// The semantic index attached to the input element
    semantic_index: u8,

    /// The type of the input element
    r#type: InputElementType,
}
