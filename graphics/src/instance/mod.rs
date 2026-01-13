use adapter::GraphicsAdapterFunctions;
use inner::GraphicsInstanceInner;
use std::sync::Arc;
#[cfg(target_os = "windows")]
use surface::Win32WindowSurfaceFunctions;
use surface::WindowSurfaceFunctions;

mod adapter;
mod builder;
mod create_debug_messenger;
mod create_window_surface;
mod debug_messenger;
mod extension;
mod inner;
mod surface;

mod deref;
mod enumerate_all_extensions;
mod enumerate_all_layers;
mod enumerate_extensions;

pub use adapter::*;
pub use builder::GraphicsInstanceBuilder;
pub use debug_messenger::*;
pub use extension::GraphicsInstanceExtension;
pub use surface::WindowSurface;

/// The context for interacting with Vulkan
pub struct GraphicsInstance {
    /// Reference to the graphics instance
    inner: Arc<GraphicsInstanceInner>,
}
