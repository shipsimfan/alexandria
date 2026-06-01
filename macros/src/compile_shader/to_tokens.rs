use crate::compile_shader::CompileShader;
use proc_macro_util::{Generator, ToTokens};

impl ToTokens for CompileShader {
    fn to_tokens(self, generator: &mut Generator) {
        self.data.to_tokens(generator);
    }
}
