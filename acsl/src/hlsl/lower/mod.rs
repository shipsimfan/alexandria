use crate::{HlslProgram, Program};
use types::lower_types;

mod types;

impl HlslProgram {
    /// Lower `program` into an [`HlslProgram`]
    pub fn lower(program: Program) -> Self {
        let mut content = String::new();

        lower_types(&mut content, program.types());

        HlslProgram {
            content: content.into(),
            vertex_entry: String::new().into(),
            pixel_entry: String::new().into(),
        }
    }
}
