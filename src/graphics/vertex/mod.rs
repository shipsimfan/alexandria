mod input_element;

pub use input_element::{InputElement, InputElementType};

/// An item which can be used as a vertex
pub trait Vertex {
    /// The layout of the vertex on the GPU
    const INPUT_LAYOUT: &[InputElement];
}
