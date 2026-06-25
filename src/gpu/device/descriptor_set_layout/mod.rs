use vulkan::VkDescriptorSetLayout;

mod get;

/// A set of resources that can be bound to a pipeline
pub struct VulkanDescriptorSetLayout {
    /// The handle to the underlying Vulkan descriptor set layout
    handle: VkDescriptorSetLayout,
}
