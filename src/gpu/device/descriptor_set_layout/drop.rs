use crate::gpu::VulkanDescriptorSetLayout;
use std::ptr::null;

impl Drop for VulkanDescriptorSetLayout {
    fn drop(&mut self) {
        unsafe {
            (self
                .device
                .functions()
                .descriptor_set_layout
                .destroy_descriptor_set_layout)(
                self.device.handle(), self.handle, null()
            );
        }
    }
}
