use crate::functions::InstanceFunctions;
use std::ptr::{null, null_mut};
use vulkan::VkInstance;

mod debug_messenger;
mod event_callback;
mod extensions;
mod layers;
mod new;
mod physical_device;
mod physical_devices;

pub use debug_messenger::DebugUtilsMessenger;
pub use event_callback::EventCallback;
pub use extensions::InstanceExtension;
pub use layers::InstanceLayer;
pub use physical_device::{PhysicalDevice, PhysicalDeviceProperties};

/// A Vulkan Instance
pub struct Instance {
    /// The handle to the instance
    handle: VkInstance,

    /// Instance-level functions
    functions: InstanceFunctions,

    /// The callback for events that Vulkan emits
    event_callback: *mut Box<dyn EventCallback>,
}

impl Instance {
    /// Gets the handle to this instance
    pub(crate) fn handle(&self) -> VkInstance {
        self.handle
    }

    /// Gets the instance-level functions
    pub(crate) fn f(&self) -> &InstanceFunctions {
        &self.functions
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        self.f().destroy_instance(self.handle, null());

        if self.event_callback != null_mut() {
            drop(unsafe { Box::from_raw(self.event_callback) });
        }
    }
}

unsafe impl Send for Instance {}
unsafe impl Sync for Instance {}
