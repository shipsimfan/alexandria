use crate::{
    Error, Result,
    gpu::{
        VulkanBuffer, VulkanBufferCreateFlags, VulkanBufferUsageFlags, VulkanDevice,
        VulkanSharingMode,
    },
};
use std::ptr::null;
use vulkan::{VkBuffer, VkBufferCreateInfo, try_vulkan};

impl VulkanBuffer {
    /// Create a new [`VulkanBuffer`]
    pub(in crate::gpu::device) fn new(
        flags: VulkanBufferCreateFlags,
        size: u64,
        usage: VulkanBufferUsageFlags,
        sharing_mode: VulkanSharingMode,
        queue_family_indices: &[u32],
        device: &VulkanDevice,
    ) -> Result<VulkanBuffer> {
        let create_info = VkBufferCreateInfo {
            flags,
            size,
            usage,
            sharing_mode,
            queue_family_index_count: queue_family_indices.len() as _,
            queue_family_indices: queue_family_indices.as_ptr(),
            ..Default::default()
        };

        let mut handle = VkBuffer::null();
        try_vulkan!((device.functions().buffer.create_buffer)(
            device.handle(),
            &create_info,
            null(),
            &mut handle,
        ))
        .map(|_| VulkanBuffer {
            handle,
            device: device.clone(),
        })
        .map_err(|error| Error::new_with("unable to create a buffer", error))
    }
}
