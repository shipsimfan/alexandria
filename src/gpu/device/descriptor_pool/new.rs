use crate::{
    Error, Result,
    gpu::{
        VulkanDescriptorPool, VulkanDescriptorPoolCreateFlags, VulkanDescriptorPoolSize,
        VulkanDevice,
    },
};
use std::ptr::null;
use vulkan::{VkDescriptorPool, VkDescriptorPoolCreateInfo, try_vulkan};

impl VulkanDescriptorPool {
    /// Create a new [`VulkanDescriptorPool`]
    pub(in crate::gpu::device) fn new(
        flags: VulkanDescriptorPoolCreateFlags,
        max_sets: u32,
        pool_sizes: &[VulkanDescriptorPoolSize],
        device: &VulkanDevice,
    ) -> Result<VulkanDescriptorPool> {
        let create_info = VkDescriptorPoolCreateInfo {
            flags,
            max_sets,
            pool_size_count: pool_sizes.len() as u32,
            pool_sizes: pool_sizes.as_ptr().cast(),
            ..Default::default()
        };

        let mut handle = VkDescriptorPool::null();
        try_vulkan!((device.functions().descriptor_pool.create_descriptor_pool)(
            device.handle(),
            &create_info,
            null(),
            &mut handle
        ))
        .map_err(|error| Error::new_with("unable to create a descriptor pool", error))?;

        Ok(VulkanDescriptorPool {
            handle,
            device: device.clone(),
        })
    }
}
