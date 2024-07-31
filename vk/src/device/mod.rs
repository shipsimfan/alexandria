use crate::{functions::DeviceFunctions, Instance};
use std::{ptr::null, rc::Rc};
use vulkan::VkDevice;

/// A Vulkan device
pub struct Device {
    /// The handle to the device
    handle: VkDevice,

    /// Device-level functions
    functions: DeviceFunctions,

    /// The instance that created this device
    #[allow(unused)]
    instance: Rc<Instance>,
}

impl Device {
    /// Gets the instance-level functions
    pub(crate) fn f(&self) -> &DeviceFunctions {
        &self.functions
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        self.f().destroy_device(self.handle, null());
    }
}
