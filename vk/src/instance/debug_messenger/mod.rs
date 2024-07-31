use crate::{functions::DebugUtilsFunctions, Instance};
use callback::log_callback;
use std::{ptr::null, sync::Arc};
use vulkan::VkDebugUtilsMessengerEXT;

mod callback;
mod create_info;
mod new;

pub(super) use create_info::debug_messenger_create_info;

/// A messenger for debug events
pub struct DebugUtilsMessenger {
    /// The handle to the messenger
    handle: VkDebugUtilsMessengerEXT,

    /// The instance that created this messenger
    instance: Arc<Instance>,
}

impl DebugUtilsMessenger {
    /// Gets the debug utils functions
    fn f(&self) -> Option<&DebugUtilsFunctions> {
        self.instance.f().du()
    }
}

impl Drop for DebugUtilsMessenger {
    fn drop(&mut self) {
        self.f()
            .unwrap()
            .destroy_messenger(self.instance.handle(), self.handle, null());
    }
}

unsafe impl Send for DebugUtilsMessenger {}
