use crate::gpu::VulkanSurface;
use std::ptr::null;

impl Drop for VulkanSurface {
    fn drop(&mut self) {
        unsafe {
            (self.instance.functions().surface().destroy_surface)(
                self.instance.handle(),
                self.handle,
                null(),
            )
        }
    }
}
