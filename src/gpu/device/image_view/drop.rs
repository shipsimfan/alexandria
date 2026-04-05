use crate::gpu::VulkanImageView;
use std::ptr::null;

impl Drop for VulkanImageView {
    fn drop(&mut self) {
        unsafe {
            (self.device().functions().image_view.destroy_image_view)(
                self.device().handle(),
                self.handle,
                null(),
            )
        };
    }
}
