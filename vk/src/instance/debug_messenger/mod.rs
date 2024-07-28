use crate::{functions::DebugUtilsFunctions, Instance};
use callback::log_callback;
use std::{borrow::Cow, ptr::null, rc::Rc};
use util::Severity;
use vulkan::VkDebugUtilsMessengerEXT;

mod callback;
mod create_info;
mod new;

pub(super) use create_info::debug_messenger_create_info;

/// A messenger for debug events
pub struct DebugUtilsMessenger {
    /// The handle to the messenger
    handle: VkDebugUtilsMessengerEXT,

    /// The logging callback itself
    #[allow(unused)]
    callback: *mut dyn Fn(Severity, &str, Vec<Cow<str>>),

    /// The instance that created this messenger
    instance: Rc<Instance>,
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

        drop(unsafe { Box::from_raw(self.callback) });
    }
}

unsafe impl Send for DebugUtilsMessenger {}
