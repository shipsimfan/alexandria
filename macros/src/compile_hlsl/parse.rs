use crate::compile_hlsl::{input::CompileHlslInput, CompileHlsl};
use acsl::HlslProgram;
use proc_macro_util::{tokens::Literal, Parse, Parser, Result};

fn strip_string(literal: &Literal) -> Result<String> {
    let content = literal.to_string();
    match content.strip_prefix("r\"") {
        Some(content) => Ok(content[..content.len() - 1].to_string()),
        None => match content.strip_prefix("\"") {
            Some(content) => Ok(content[..content.len() - 1].to_string()),
            None => Err(literal.span().error("expected a string literals")),
        },
    }
}

impl<'a> Parse<'a> for CompileHlsl {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let input = CompileHlslInput::parse(parser)?;

        let program = HlslProgram::new(
            strip_string(input.content())?,
            strip_string(input.vertex_main())?,
            strip_string(input.pixel_main())?,
        );

        let compiled_program = acsl::d3dcompile::<()>(&program).map_err(|error| {
            input
                .content()
                .span()
                .error(format!("unable to compile program - {}", error))
        })?;

        Ok(CompileHlsl {
            vertex_content: compiled_program.vertex_content().into(),
            pixel_content: compiled_program.pixel_content().into(),
        })
    }
}
