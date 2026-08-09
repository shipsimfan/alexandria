use crate::{
    Result,
    gpu::{
        VulkanDescriptorSetLayout, VulkanDescriptorSetLayoutBinding,
        VulkanDescriptorSetLayoutCreateFlags, VulkanDevice,
    },
};

impl VulkanDevice {
    /// Create a new [`VulkanDescriptorSetLayout`]
    pub fn create_descriptor_set_layout<F: Into<VulkanDescriptorSetLayoutCreateFlags>>(
        &self,
        flags: F,
        bindings: &[VulkanDescriptorSetLayoutBinding],
    ) -> Result<VulkanDescriptorSetLayout> {
        VulkanDescriptorSetLayout::new(flags.into(), bindings, self)
    }
}
