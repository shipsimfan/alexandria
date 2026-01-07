use adapter::GraphicsAdapterFunctions;
use inner::GraphicsInstanceInner;
use std::sync::Arc;

mod adapter;
mod builder;
mod debug_messenger;
mod inner;
mod layer;

mod deref;
mod enumerate_all_extensions;
mod enumerate_all_layers;
mod enumerate_layers;

pub use adapter::{GraphicsAdapter, GraphicsAdapterKind};
pub use builder::GraphicsInstanceBuilder;
pub use debug_messenger::*;
pub use layer::GraphicsInstanceLayer;

/// The context for interacting with Vulkan
pub struct GraphicsInstance {
    /// Reference to the graphics instance
    inner: Arc<GraphicsInstanceInner>,
}
