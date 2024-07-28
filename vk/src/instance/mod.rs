use crate::functions::InstanceFunctions;
use std::ptr::null;
use vulkan::VkInstance;

mod debug_messenger;
mod extensions;
mod layers;
mod new;

pub use debug_messenger::DebugUtilsMessenger;
pub use extensions::InstanceExtension;
pub use layers::InstanceLayer;

/// A Vulkan Instance
pub struct Instance {
    /// The handle to the instance
    handle: VkInstance,

    /// Instance-level functions
    functions: InstanceFunctions,
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
    }
}
