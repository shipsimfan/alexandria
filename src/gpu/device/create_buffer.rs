use crate::{
    Result,
    gpu::{
        VulkanBuffer, VulkanBufferCreateFlags, VulkanBufferUsageFlags, VulkanDevice,
        VulkanSharingMode,
    },
};

impl VulkanDevice {
    /// Create a new [`VulkanBuffer`]
    pub fn create_buffer<F: Into<VulkanBufferCreateFlags>, U: Into<VulkanBufferUsageFlags>>(
        &self,
        flags: F,
        size: u64,
        usage: U,
        sharing_mode: VulkanSharingMode,
        queue_family_indices: &[u32],
    ) -> Result<VulkanBuffer> {
        VulkanBuffer::new(
            flags.into(),
            size,
            usage.into(),
            sharing_mode,
            queue_family_indices,
            self,
        )
    }
}
