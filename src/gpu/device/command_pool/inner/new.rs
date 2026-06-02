use crate::{
    Error, Result,
    gpu::{VulkanDevice, device::command_pool::VulkanCommandPoolInner},
};
use std::ptr::null_mut;
use vulkan::{
    VkCommandPool, VkCommandPoolCreateFlag, VkCommandPoolCreateFlags, VkCommandPoolCreateInfo,
    try_vulkan,
};

impl VulkanCommandPoolInner {
    /// Create a new [`VulkanCommandPoolInner`]
    pub fn new(
        queue_family: u32,
        reset_command_buffer: bool,
        device: VulkanDevice,
    ) -> Result<VulkanCommandPoolInner> {
        let mut flags = VkCommandPoolCreateFlags::new();
        if reset_command_buffer {
            flags.set(VkCommandPoolCreateFlag::ResetCommandBuffer);
        }

        let create_info = VkCommandPoolCreateInfo {
            queue_family_index: queue_family,
            flags,
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

        Ok(VulkanCommandPoolInner { handle, device })
    }
}
