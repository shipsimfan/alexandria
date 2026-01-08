use adapter::GraphicsAdapterFunctions;
use inner::GraphicsInstanceInner;
use std::sync::Arc;

mod adapter;
mod builder;
mod create_debug_messenger;
mod debug_messenger;
mod extension;
mod inner;

mod deref;
mod enumerate_all_extensions;
mod enumerate_all_layers;
mod enumerate_extensions;

pub use adapter::{GraphicsAdapter, GraphicsAdapterKind};
pub use builder::GraphicsInstanceBuilder;
pub use debug_messenger::*;
pub use extension::GraphicsInstanceExtension;

/// The context for interacting with Vulkan
pub struct GraphicsInstance {
    /// Reference to the graphics instance
    inner: Arc<GraphicsInstanceInner>,
}
