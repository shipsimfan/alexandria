use crate::compile_hlsl_file::{input::CompileHlslInput, CompileHlslFile};
use acsl::HlslProgram;
use proc_macro_util::{tokens::Literal, Parse, Parser, Result};
use std::path::Path;

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

impl<'a> Parse<'a> for CompileHlslFile {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let input = CompileHlslInput::parse(parser)?;

        let path = Path::new(&input.file_name().span().file())
            .parent()
            .unwrap_or(Path::new(""))
            .join(strip_string(input.file_name())?);
        let content = std::fs::read_to_string(&path).map_err(|error| {
            input.file_name().span().error(format!(
                "unable to read \"{}\" - {}",
                path.display(),
                error
            ))
        })?;

        let program = HlslProgram::new(
            content,
            strip_string(input.vertex_main())?,
            strip_string(input.pixel_main())?,
        );

        let compiled_program = acsl::d3dcompile::<(), ()>(&program).map_err(|error| {
            input
                .file_name()
                .span()
                .error(format!("unable to compile program - {}", error))
        })?;

        Ok(CompileHlslFile {
            vertex_content: compiled_program.vertex_content().into(),
            pixel_content: compiled_program.pixel_content().into(),
        })
    }
}
