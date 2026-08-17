use crate::gpu::device::image::VulkanImageInner;
use std::ptr::null;

impl Drop for VulkanImageInner {
    fn drop(&mut self) {
        if self.destroy {
            unsafe {
                (self.device.functions().image.destroy_image)(
                    self.device.handle(),
                    self.handle,
                    null(),
                );
            }
        }
    }
}
