use crate::{
    Error, PackedMap, Result,
    gpu::{VulkanCommandPool, VulkanCommandPoolCreateFlags, VulkanDevice},
};
use std::ptr::null_mut;
use vulkan::{VkCommandPool, VkCommandPoolCreateInfo, try_vulkan};

impl VulkanCommandPool {
    /// Create a new [`VulkanCommandPool`]
    pub(in crate::gpu::device) fn new(
        queue_family: u32,
        flags: VulkanCommandPoolCreateFlags,
        device: &VulkanDevice,
    ) -> Result<VulkanCommandPool> {
        let create_info = VkCommandPoolCreateInfo {
            flags,
            queue_family_index: queue_family,
            ..Default::default()
        };

        let mut handle = VkCommandPool::null();
        try_vulkan!((device.functions().command_pool.create_command_pool)(
            device.handle(),
            &create_info,
            null_mut(),
            &mut handle,
        ))
        .map_err(|vk| Error::new_with("unable to create command pool", vk))?;

        Ok(VulkanCommandPool {
            handle,
            command_buffers: PackedMap::new(),
            device: device.clone(),
        })
    }
}
