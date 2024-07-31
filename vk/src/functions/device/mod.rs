use super::{util::get_device_proc_addr, InstanceFunctions};
use std::ffi::CStr;
use vulkan::{VkAllocationCallbacks, VkDestroyDevice, VkDevice, VK_DESTROY_DEVICE};

/// Device-level function pointers
pub(crate) struct DeviceFunctions {
    destroy_device: VkDestroyDevice,
}

impl DeviceFunctions {
    /// Loads the device-level functions
    pub(crate) fn new(f: &InstanceFunctions, device: VkDevice) -> Result<Self, &'static CStr> {
        let destroy_device: VkDestroyDevice = get_device_proc_addr!(f, device, VK_DESTROY_DEVICE)?;

        Ok(DeviceFunctions { destroy_device })
    }

    /// Destory a Vulkan device
    pub(crate) fn destroy_device(&self, device: VkDevice, allocator: *const VkAllocationCallbacks) {
        (self.destroy_device)(device, allocator)
    }
}
