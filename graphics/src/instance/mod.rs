use functions::GraphicsInstanceFunctions;
use vulkan::VkInstance;

mod builder;
mod functions;

mod drop;
mod enumerate_all_extensions;
mod enumerate_all_layers;

pub use builder::GraphicsInstanceBuilder;

/// The context for interacting with Vulkan
pub struct GraphicsInstance {
    /// A handle to the underlying graphics intance
    instance: VkInstance,

    /// The functions loaded for this instance
    functions: GraphicsInstanceFunctions,
}

unsafe impl Send for GraphicsInstance {}
unsafe impl Sync for GraphicsInstance {}
