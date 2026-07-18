use crate::gpu::VulkanBufferCopy;
use vulkan::VkBufferCopy;

impl VulkanBufferCopy {
    /// Create a new [`VulkanBufferCopy`]
    pub fn new(src_offset: u64, dst_offset: u64, size: u64) -> VulkanBufferCopy {
        VulkanBufferCopy {
            inner: VkBufferCopy {
                src_offset,
                dst_offset,
                size,
            },
        }
    }
}
