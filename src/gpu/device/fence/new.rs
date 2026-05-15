use crate::{
    Error, Result,
    gpu::{VulkanDevice, VulkanFence},
};
use std::ptr::null;
use vulkan::{VkFence, VkFenceCreateFlag, VkFenceCreateFlags, VkFenceCreateInfo, try_vulkan};

impl VulkanFence {
    /// Create a new [`VulkanFence`]
    pub(in crate::gpu::device) fn new(
        device: VulkanDevice,
        signalled: bool,
    ) -> Result<VulkanFence> {
        let create_info = VkFenceCreateInfo {
            flags: if signalled {
                VkFenceCreateFlag::Signalled.into()
            } else {
                VkFenceCreateFlags::default()
            },
            ..Default::default()
        };

        let mut handle = VkFence::null();
        try_vulkan!((device.functions().fence.create_fence)(
            device.handle(),
            &create_info,
            null(),
            &mut handle,
        ))
        .map_err(|vk| Error::new_with("unable to create a fence", vk))?;

        Ok(VulkanFence { handle, device })
    }
}
