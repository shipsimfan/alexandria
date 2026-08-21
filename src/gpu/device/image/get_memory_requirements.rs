use crate::gpu::{VulkanImage, VulkanMemoryRequirements};
use vulkan::{VkMemoryRequirements, VkMemoryRequirements2};

impl VulkanImage {
    /// Get the memory requirements for this image
    pub fn get_memory_requirements(&self) -> VulkanMemoryRequirements {
        let mut memory_requirements = VkMemoryRequirements::default();
        unsafe {
            (self
                .inner
                .device()
                .functions()
                .image
                .get_image_memory_requirements)(
                self.inner.device().handle(),
                self.inner.handle(),
                &mut memory_requirements,
            )
        };

        VulkanMemoryRequirements::new(VkMemoryRequirements2 {
            memory_requirements,
            ..Default::default()
        })
    }
}
