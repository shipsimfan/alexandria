//! Alexandria Common Shader Language

#[cfg(feature = "compile")]
mod compile;
#[cfg(feature = "d3d")]
mod d3d;
#[cfg(feature = "d3dcompile")]
mod d3dcompile;
#[cfg(feature = "hlsl")]
mod hlsl;

#[cfg(feature = "program")]
pub mod pretty_print;
#[cfg(feature = "program")]
#[allow(unused)]
pub mod program;

#[cfg(feature = "compile")]
pub use compile::compile;
#[cfg(feature = "d3d")]
pub use d3d::D3DProgram;
#[cfg(feature = "d3dcompile")]
pub use d3dcompile::{d3dcompile, D3DCompileError};
#[cfg(feature = "hlsl")]
pub use hlsl::HlslProgram;
#[cfg(feature = "program")]
pub use program::Program;
