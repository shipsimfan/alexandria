use crate::gpu::{VulkanDevice, VulkanWriteDescriptorSet};

impl VulkanDevice {
    /// Update descriptor sets on a device
    pub fn update_descriptor_sets(
        &self,
        writes: &[VulkanWriteDescriptorSet],
        copies: &[VulkanWriteDescriptorSet],
    ) {
        unsafe {
            (self.functions().descriptor_set.update_descriptor_sets)(
                self.inner.handle(),
                writes.len() as u32,
                writes.as_ptr().cast(),
                copies.len() as u32,
                copies.as_ptr().cast(),
            );
        }
    }
}
