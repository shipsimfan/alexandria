use crate::compile_hlsl_file::input::CompileHlslInput;
use proc_macro_util::tokens::{Literal, TokenTree};

impl CompileHlslInput {
    /// Get the name of the file to read
    pub fn file_name(&self) -> &Literal {
        &self.file_name
    }

    /// Get the provided name of the main vertex function
    pub fn vertex_main(&self) -> &Literal {
        &self.vertex_main
    }

    /// Get the provided name of the main pixel function
    pub fn pixel_main(&self) -> &Literal {
        &self.pixel_main
    }

    /// Get the input layout to use for the final program
    pub fn input_layout(self) -> Vec<TokenTree> {
        self.input_layout
    }
}
