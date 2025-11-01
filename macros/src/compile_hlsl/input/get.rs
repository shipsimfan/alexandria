use crate::compile_hlsl::input::CompileHlslInput;
use proc_macro_util::tokens::Literal;

impl CompileHlslInput {
    /// Get the provided content
    pub fn content(&self) -> &Literal {
        &self.content
    }

    /// Get the provided name of the main vertex function
    pub fn vertex_main(&self) -> &Literal {
        &self.vertex_main
    }

    /// Get the provided name of the main pixel function
    pub fn pixel_main(&self) -> &Literal {
        &self.pixel_main
    }
}
