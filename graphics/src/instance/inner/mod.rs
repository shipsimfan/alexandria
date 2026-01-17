use functions::GraphicsInstanceFunctions;
use vulkan::VkInstance;

mod functions;

mod create_debug_messenger;
mod create_window_surface;
mod drop;
mod enumerate_adapters;
mod get;
mod new;

/// The context for interacting with Vulkan
pub struct GraphicsInstanceInner {
    /// A handle to the underlying graphics intance
    handle: VkInstance,

    /// The functions loaded for this instance
    pub(crate) functions: GraphicsInstanceFunctions,
}
