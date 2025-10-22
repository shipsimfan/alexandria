use crate::{HlslProgram, InputLayout, Program};
use types::lower_types;

mod types;

impl HlslProgram {
    /// Lower `program` into an [`HlslProgram`]
    pub fn lower(program: Program) -> Self {
        let mut content = String::new();

        lower_types(&mut content, program.types());

        HlslProgram {
            content,
            input_layout: InputLayout::new(),
            vertex_entry: String::new(),
            pixel_entry: String::new(),
        }
    }
}
