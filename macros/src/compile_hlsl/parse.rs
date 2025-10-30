use crate::compile_hlsl::{input::CompileHlslInput, CompileHlsl};
use acsl::{HlslProgram, InputLayout};
use proc_macro_util::{Parse, Parser, Result};

impl<'a> Parse<'a> for CompileHlsl {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let input = CompileHlslInput::parse(parser)?;

        let program = HlslProgram::new(
            input.content().to_string(),
            InputLayout::new(),
            input.vertex_main().to_string(),
            input.pixel_main().to_string(),
        );

        let compiled_program = acsl::d3dcompile(&program).map_err(|error| {
            input
                .content()
                .span()
                .error(format!("unable to compile program - {}", error))
        })?;

        Ok(CompileHlsl {
            vertex_content: compiled_program.vertex_content().into(),
            pixel_content: compiled_program.pixel_content().into(),
            input_layout: input.input_layout(),
        })
    }
}
