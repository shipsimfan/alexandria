use crate::compile_shader::{CompileShader, CompileShaderInput};
use proc_macro_util::{Parse, Parser, Result, tokens::Literal};
use std::{
    path::Path,
    process::{Command, Stdio},
};

impl<'a> Parse<'a> for CompileShader {
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
        for entry_point in input.entry_points {
            command.args(["-entry", &entry_point.to_string()]);
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
            data: Literal::new(output.stdout.as_slice()),
        })
    }
}
