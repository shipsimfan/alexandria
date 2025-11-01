use crate::compile_hlsl::input::CompileHlslInput;
use proc_macro_util::{Parse, Parser, Result, Token};

impl<'a> Parse<'a> for CompileHlslInput {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let content = parser.parse()?;
        parser.parse::<Token![,]>()?;
        let vertex_main = parser.parse()?;
        parser.parse::<Token![,]>()?;
        let pixel_main = parser.parse()?;

        Ok(CompileHlslInput {
            content,
            vertex_main,
            pixel_main,
        })
    }
}
