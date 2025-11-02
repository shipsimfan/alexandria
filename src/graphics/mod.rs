//! Graphics context and rendering supports

mod adapter;
mod graphics_context;
mod mesh;
mod render_context;
mod shader;
mod vertex;

pub use adapter::{Adapter, Output, OutputResolution};
pub use graphics_context::GraphicsContext;
pub use mesh::Mesh;
pub use render_context::{RenderContext, RenderFrame};
pub use shader::Shader;
pub use vertex::{InputElement, InputElementType, Vertex};
