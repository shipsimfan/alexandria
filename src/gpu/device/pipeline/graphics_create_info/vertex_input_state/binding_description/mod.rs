use vulkan::VkVertexInputBindingDescription;

mod get;
mod new;
mod set;

/// The descriptor for a vertex input binding in a graphics pipeline
#[repr(transparent)]
pub struct VulkanVertexInputBindingDescription {
    /// The inner Vulkan vertex input binding description
    inner: VkVertexInputBindingDescription,
}
