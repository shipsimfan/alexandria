use crate::compile_shader::CompileShaderInput;
use proc_macro_util::{Parse, Parser, Result, Token};

impl<'a> Parse<'a> for CompileShaderInput<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let attributes = parser.parse()?;
        let visibility = parser.parse()?;
        parser.parse::<Token![const]>()?;
        let identifier = parser.parse()?;
        parser.parse::<Token![=]>()?;

        let path = parser.parse()?;

        let mut entry_points = Vec::new();
        while !parser.empty() {
            parser.parse::<Token![,]>()?;
            entry_points.push(parser.parse()?);
        }

        if entry_points.len() == 0 {
            parser.error("at least one entry point must be specified");
        }

        Ok(CompileShaderInput {
            attributes,
            visibility,
            identifier,
            path,
            entry_points,
        })
    }
}
