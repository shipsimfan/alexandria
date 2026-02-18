use crate::gpu::{VulkanDebugMessenger, VulkanDebugMessengerCallback};
use std::ptr::null;

impl<C: VulkanDebugMessengerCallback> Drop for VulkanDebugMessenger<C> {
    fn drop(&mut self) {
        unsafe {
            (self
                .instance
                .functions()
                .debug_messenger()
                .destroy_debug_messenger)(self.instance.handle(), self.handle, null())
        };
    }
}
