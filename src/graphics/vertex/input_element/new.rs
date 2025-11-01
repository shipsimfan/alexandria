use crate::graphics::{InputElement, InputElementType};

impl InputElement {
    /// Creates a new [`InputElement`] from parts
    pub const fn new(
        semantic_name: &'static str,
        semantic_index: u8,
        r#type: InputElementType,
    ) -> Self {
        InputElement {
            semantic_name,
            semantic_index,
            r#type,
        }
    }
}
