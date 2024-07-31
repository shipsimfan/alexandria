use super::{get_device_proc_addr, InstanceFunctions};
use std::{ffi::CStr, ptr::null_mut};
use vulkan::{
    VkAllocationCallbacks, VkDestroyDevice, VkDevice, VkGetDeviceQueue, VkQueue, VkResult,
    VK_DESTROY_DEVICE, VK_GET_DEVICE_QUEUE,
};

/// Device-level function pointers
pub(crate) struct DeviceFunctions {
    destroy_device: VkDestroyDevice,
    get_device_queue: VkGetDeviceQueue,
}

impl DeviceFunctions {
    /// Loads the device-level functions
    pub(crate) fn new(f: &InstanceFunctions, device: VkDevice) -> Result<Self, &'static CStr> {
        let destroy_device: VkDestroyDevice = get_device_proc_addr!(f, device, VK_DESTROY_DEVICE)?;
        let get_device_queue: VkGetDeviceQueue =
            get_device_proc_addr!(f, device, VK_GET_DEVICE_QUEUE)?;

        Ok(DeviceFunctions {
            destroy_device,
            get_device_queue,
        })
    }

    /// Destory a Vulkan device
    pub(crate) fn destroy_device(&self, device: VkDevice, allocator: *const VkAllocationCallbacks) {
        (self.destroy_device)(device, allocator)
    }

    /// Gets a [`VkQueue`] that was created with a [`VkDevice`]
    pub(crate) fn get_device_queue(
        &self,
        device: VkDevice,
        queue_family_index: u32,
        queue_index: u32,
    ) -> VkQueue {
        let mut queue = null_mut();
        (self.get_device_queue)(device, queue_family_index, queue_index, &mut queue);
        queue
    }
}
