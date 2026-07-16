use crate::gpu::{VulkanBuffer, VulkanMemoryRequirements};
use vulkan::{VkBufferMemoryRequirementsInfo2, VkMemoryRequirements2};

impl VulkanBuffer {
    /// Get the memory requirements for this buffer
    pub fn get_memory_requirements(&self) -> VulkanMemoryRequirements {
        let mut memory_requirements = VkMemoryRequirements2::default();
        let info = VkBufferMemoryRequirementsInfo2 {
            buffer: self.handle,
            ..Default::default()
        };

        unsafe {
            (self.device.functions().buffer.get_memory_requirements2)(
                self.device.handle(),
                &info,
                &mut memory_requirements,
            )
        };

        VulkanMemoryRequirements::new(memory_requirements)
    }
}
