use crate::compile_shader::{CompileShader, CompileShaderInput};
use proc_macro_util::{Parse, Parser, Result, Token, tokens::Literal};
use std::{
    ffi::CString,
    path::Path,
    process::{Command, Stdio},
};

impl<'a> Parse<'a> for CompileShader<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let input = CompileShaderInput::parse(parser)?;

        let raw_path = input.path.to_string();
        let input_path = raw_path.trim_matches('\"');

        let raw_file_path = input.path.span().file();
        let file_path = match Path::new(&raw_file_path).parent() {
            Some(path) => path,
            None => Path::new("."),
        };

        let full_input_path = file_path.join(input_path);

        let mut command = Command::new("slangc");
        command
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .arg(full_input_path)
            .args([
                "-target",
                "spirv",
                "-profile",
                "spirv_1_4",
                "-emit-spirv-directly",
                "-fvk-use-entrypoint-name",
            ]);
        let mut entry_points = Vec::with_capacity(input.entry_points.len());
        for entry_point_ident in input.entry_points {
            let entry_point = entry_point_ident.to_string();
            command.args(["-entry", &entry_point]);

            entry_points.push((
                Literal::new(
                    CString::new(entry_point)
                        .map_err(|_| entry_point_ident.span().error("entry point contains null"))?
                        .as_c_str(),
                ),
                Token![,](),
            ));
        }

        let output = command
            .output()
            .map_err(|error| parser.error(format!("Failed to execute \"slangc\" - {error}")))?;

        if !output.status.success() {
            return Err(parser.error(format!(
                "Shader compiler returned an error\n{}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(CompileShader {
            attributes: input.attributes,
            visibility: input.visibility,
            identifier: input.identifier,
            data: Literal::new(output.stdout.as_slice()),
            length: output.stdout.len(),
            entry_points,
        })
    }
}
