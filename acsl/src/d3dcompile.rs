use crate::{D3DProgram, HlslProgram};

/// A result the can occur when compiling HLSL
#[derive(Debug)]
pub enum D3DCompileError {
    /// The result is from the function call itself
    HResult(win32::Error),

    /// The result is in the provided code
    String(String),
}

/// Compile an [`HlslProgram`] into a [`D3DProgram`]
pub fn d3dcompile(program: &HlslProgram) -> Result<D3DProgram, D3DCompileError> {
    todo!()
}

impl std::error::Error for D3DCompileError {}

impl std::fmt::Display for D3DCompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            D3DCompileError::HResult(error) => error.fmt(f),
            D3DCompileError::String(string) => string.fmt(f),
        }
    }
}
