use crate::compile_hlsl_file::CompileHlslFile;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl ToTokens for CompileHlslFile {
    fn to_tokens(self, generator: &mut Generator) {
        let CompileHlslFile {
            vertex_content,
            pixel_content,
            input_layout,
        } = self;

        to_tokens! { generator
            ::alexandria::acsl::D3DProgram::new(
                ::std::borrow::Cow::Borrowed(#vertex_content),
                ::std::borrow::Cow::Borrowed(#pixel_content),
                #input_layout,
            )
        }
    }
}
