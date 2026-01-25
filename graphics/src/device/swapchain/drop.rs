use crate::Swapchain;
use std::ptr::null;

impl<'surface> Drop for Swapchain<'surface> {
    fn drop(&mut self) {
        (self.device.functions.swapchain().destroy_swapchain)(
            self.device.handle(),
            self.handle,
            null(),
        );
    }
}
