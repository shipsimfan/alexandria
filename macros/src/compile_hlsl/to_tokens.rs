use crate::compile_hlsl::CompileHlsl;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl ToTokens for CompileHlsl {
    fn to_tokens(self, generator: &mut Generator) {
        let CompileHlsl {
            vertex_content,
            pixel_content,
        } = self;

        to_tokens! { generator
            ::alexandria::acsl::D3DProgram::new(
                ::std::borrow::Cow::Borrowed(#vertex_content),
                ::std::borrow::Cow::Borrowed(#pixel_content),
            )
        }
    }
}
