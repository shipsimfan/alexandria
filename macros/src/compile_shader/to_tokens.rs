use crate::compile_shader::CompileShader;
use proc_macro_util::{Generator, ToTokens, to_tokens};

impl<'a> ToTokens for CompileShader<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let CompileShader {
            attributes,
            visibility,
            identifier,
            data,
            length,
            entry_points,
        } = self;

        to_tokens! { generator
            #attributes
            #visibility const #identifier: ::alexandria::gpu::VulkanShaderModuleCode<#length> =
                ::alexandria::gpu::VulkanShaderModuleCode::new(*#data, &[#entry_points]);
        }
    }
}
