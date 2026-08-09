use crate::{
    Error, Result,
    gpu::{VulkanDescriptorPool, VulkanDescriptorSet, VulkanDescriptorSetLayout},
};
use vulkan::{VkDescriptorSet, VkDescriptorSetAllocateInfo, try_vulkan};

impl VulkanDescriptorSet {
    /// Allocate a new [`VulkanDescriptorSet`]
    pub(in crate::gpu::device) fn new(
        set_layout: &VulkanDescriptorSetLayout,
        decriptor_pool: &mut VulkanDescriptorPool,
    ) -> Result<VulkanDescriptorSet> {
        let allocate_info = VkDescriptorSetAllocateInfo {
            descriptor_pool: decriptor_pool.handle(),
            descriptor_set_count: 1,
            set_layouts: &set_layout.handle(),
            ..Default::default()
        };

        let mut handle = VkDescriptorSet::null();
        try_vulkan!((decriptor_pool
            .device()
            .functions()
            .descriptor_set
            .allocate_descriptor_sets)(
            decriptor_pool.device().handle(),
            &allocate_info,
            &mut handle
        ))
        .map_err(|error| Error::new_with("unable to allocate a descriptor set", error))?;

        Ok(VulkanDescriptorSet { handle })
    }
}
