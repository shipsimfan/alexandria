use vulkan::try_vulkan;

use crate::{
    Error, Result,
    gpu::{VulkanDescriptorPool, VulkanDescriptorSet},
};

impl VulkanDescriptorPool {
    /// Free a [`VulkanDescriptorSet`] back to this descriptor pool
    pub fn free_descriptor_set(&mut self, descriptor_set: VulkanDescriptorSet) -> Result<()> {
        try_vulkan!((self
            .device
            .functions()
            .descriptor_set
            .free_descriptor_sets)(
            self.device.handle(),
            self.handle,
            1,
            &descriptor_set.handle()
        ))
        .map(|_| ())
        .map_err(|error| Error::new_with("unable to free a descriptor set", error))
    }
}
