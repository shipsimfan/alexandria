#![allow(mixed_script_confusables)]

mod graphics;
mod input;
mod math;
mod mesh;
mod shader;
mod window;

pub use crate::graphics::{GraphicsCreationError, RenderError};
pub use math::*;
pub use mesh::{Mesh, MeshCreationError};
pub use shader::{Format, SetConstantBufferError, Shader, ShaderCB, ShaderCreationError};
pub use window::{Window, WindowCreationError};

pub use win32::ID3D11Device as Device;
