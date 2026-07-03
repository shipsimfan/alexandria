use vulkan::VkVertexInputAttributeDescription;

mod get;
mod new;
mod set;

/// The descriptor for a vertex input attribute in a graphics pipeline
#[repr(transparent)]
pub struct VulkanVertexInputAttributeDescription {
    /// The inner Vulkan vertex input attribute description
    inner: VkVertexInputAttributeDescription,
}
