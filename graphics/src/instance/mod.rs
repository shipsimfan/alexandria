use vulkan::VkInstance;

mod enumerate_all_extensions;
mod enumerate_all_layers;

/// The context for interacting with Vulkan
pub struct GraphicsInstance {
    /// A handle to the underlying graphics intance
    instance: VkInstance,
}

unsafe impl Send for GraphicsInstance {}
unsafe impl Sync for GraphicsInstance {}
