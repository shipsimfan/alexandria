use crate::compile_hlsl_file::input::CompileHlslInput;
use proc_macro_util::{Parse, Parser, Result, Token};

impl<'a> Parse<'a> for CompileHlslInput {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let file_name = parser.parse()?;
        parser.parse::<Token![,]>()?;
        let vertex_main = parser.parse()?;
        parser.parse::<Token![,]>()?;
        let pixel_main = parser.parse()?;

        Ok(CompileHlslInput {
            file_name,
            vertex_main,
            pixel_main,
        })
    }
}
