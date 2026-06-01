use crate::compile_shader::CompileShaderInput;
use proc_macro_util::{Parse, Parser, Result, Token};

impl<'a> Parse<'a> for CompileShaderInput<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let path = parser.parse()?;

        let mut entry_points = Vec::new();
        while !parser.empty() {
            parser.parse::<Token![,]>()?;
            entry_points.push(parser.parse()?);
        }

        Ok(CompileShaderInput { path, entry_points })
    }
}
