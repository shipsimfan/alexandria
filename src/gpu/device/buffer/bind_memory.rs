use crate::{
    Error, Result,
    gpu::{VulkanBuffer, VulkanDeviceMemory},
};
use vulkan::{VkBindBufferMemoryInfo, try_vulkan};

impl VulkanBuffer {
    /// Bind this buffer to a memory object
    pub fn bind_memory(&mut self, memory: &VulkanDeviceMemory, offset: u64) -> Result<()> {
        let info = VkBindBufferMemoryInfo {
            buffer: self.handle,
            memory: memory.handle(),
            memory_offset: offset,
            ..Default::default()
        };

        try_vulkan!((self.device.functions().buffer.bind_buffer_memory2)(
            self.device.handle(),
            1,
            &info
        ))
        .map(|_| ())
        .map_err(|error| Error::new_with("unable to bind memory to a buffer", error))
    }
}
