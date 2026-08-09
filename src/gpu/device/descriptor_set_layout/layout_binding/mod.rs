use vulkan::VkDescriptorSetLayoutBinding;

mod get;
mod new;
mod set;

/// The description of a single binding in a descriptor set layout
#[repr(transparent)]
pub struct VulkanDescriptorSetLayoutBinding {
    /// The underlying Vulkan descriptor set layout binding
    inner: VkDescriptorSetLayoutBinding,
}
