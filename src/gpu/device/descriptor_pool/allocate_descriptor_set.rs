use crate::{
    Result,
    gpu::{VulkanDescriptorPool, VulkanDescriptorSet, VulkanDescriptorSetLayout},
};

impl VulkanDescriptorPool {
    /// Allocate a [`VulkanDescriptorSet`] from this descriptor pool
    pub fn allocate_descriptor_set(
        &mut self,
        set_layout: &VulkanDescriptorSetLayout,
    ) -> Result<VulkanDescriptorSet> {
        VulkanDescriptorSet::new(set_layout, self)
    }
}
