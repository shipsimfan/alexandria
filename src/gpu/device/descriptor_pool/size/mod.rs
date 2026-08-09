use vulkan::VkDescriptorPoolSize;

mod get;
mod new;
mod set;

/// The size of a descriptor pool
#[repr(transparent)]
pub struct VulkanDescriptorPoolSize {
    /// The underlying Vulkan descriptor pool size
    inner: VkDescriptorPoolSize,
}
