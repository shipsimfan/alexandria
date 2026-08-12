use crate::{
    Result,
    gpu::{
        VulkanDescriptorPool, VulkanDescriptorPoolCreateFlags, VulkanDescriptorPoolSize,
        VulkanDevice,
    },
};

impl VulkanDevice {
    /// Create a new [`VulkanDescriptorPool`]
    pub fn create_descriptor_pool<F: Into<VulkanDescriptorPoolCreateFlags>>(
        &self,
        flags: F,
        max_sets: u32,
        pool_sizes: &[VulkanDescriptorPoolSize],
    ) -> Result<VulkanDescriptorPool> {
        VulkanDescriptorPool::new(flags.into(), max_sets, pool_sizes, self)
    }
}
