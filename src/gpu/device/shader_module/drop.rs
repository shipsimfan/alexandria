use crate::gpu::VulkanShaderModule;
use std::ptr::null;

impl Drop for VulkanShaderModule {
    fn drop(&mut self) {
        unsafe {
            (self.device.functions().shader_module.destroy_semaphore)(
                self.device.handle(),
                self.handle,
                null(),
            );
        }
    }
}
