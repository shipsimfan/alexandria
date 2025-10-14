//! Graphics context and rendering supports

mod adapter;
mod graphics_context;
mod render_context;

pub use adapter::{Adapter, Output, OutputResolution};
pub use graphics_context::GraphicsContext;
pub use render_context::{RenderContext, RenderFrame};
