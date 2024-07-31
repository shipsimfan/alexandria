use crate::{functions::DeviceFunctions, Instance};
use std::{ptr::null, sync::Arc};
use vulkan::VkDevice;

mod new;
mod queue;

pub use queue::{Queue, QueueCreateInfo};

/// A Vulkan device
pub struct Device {
    /// The queues this device was created with
    queues: Vec<Queue>,

    /// The handle to the device
    handle: VkDevice,

    /// Device-level functions
    functions: DeviceFunctions,

    /// The instance that created this device
    #[allow(unused)]
    instance: Arc<Instance>,
}

impl Device {
    /// Gets the queues that were created with this device
    pub fn queues(&self) -> &[Queue] {
        &self.queues
    }

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

unsafe impl Send for Device {}
unsafe impl Sync for Device {}
