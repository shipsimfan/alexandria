use super::get_instance_proc_addr;
use std::ffi::CStr;
use vulkan::{
    try_vulkan, VkAllocationCallbacks, VkDestroyInstance, VkDevice, VkEnumeratePhysicalDevices,
    VkGetDeviceProcAddr, VkInstance, VkPhysicalDevice, VkResult, VkVoidFunction,
    VK_DESTROY_INSTANCE, VK_ENUMERATE_PHYSICAL_DEVICES, VK_GET_DEVICE_PROC_ADDR,
};

mod debug_utils;
mod physical_device;

pub(crate) use debug_utils::DebugUtilsFunctions;
pub(crate) use physical_device::PhysicalDeviceFunctions;

/// Instance-level function pointers
pub(crate) struct InstanceFunctions {
    physical_device: PhysicalDeviceFunctions,
    debug_utils: Option<DebugUtilsFunctions>,

    destroy_instance: VkDestroyInstance,
    enumerate_physical_devices: VkEnumeratePhysicalDevices,
    get_device_proc_addr: VkGetDeviceProcAddr,
}

impl InstanceFunctions {
    /// Loads the instance-level functions
    pub(crate) fn new(instance: VkInstance, debug: bool) -> Result<Self, &'static CStr> {
        let physical_device = PhysicalDeviceFunctions::new(instance)?;
        let debug_utils = if debug {
            Some(DebugUtilsFunctions::new(instance)?)
        } else {
            None
        };

        let destroy_instance: VkDestroyInstance =
            get_instance_proc_addr!(instance, VK_DESTROY_INSTANCE)?;
        let enumerate_physical_devices: VkEnumeratePhysicalDevices =
            get_instance_proc_addr!(instance, VK_ENUMERATE_PHYSICAL_DEVICES)?;
        let get_device_proc_addr: VkGetDeviceProcAddr =
            get_instance_proc_addr!(instance, VK_GET_DEVICE_PROC_ADDR)?;

        Ok(InstanceFunctions {
            destroy_instance,
            enumerate_physical_devices,
            get_device_proc_addr,
            physical_device,
            debug_utils,
        })
    }

    /// Gets the functions for a physical device
    pub(crate) fn pd(&self) -> &PhysicalDeviceFunctions {
        &self.physical_device
    }

    /// Gets the functions for debug utilities
    #[cfg_attr(not(debug_assertions), allow(unused))]
    pub(crate) fn du(&self) -> Option<&DebugUtilsFunctions> {
        self.debug_utils.as_ref()
    }

    /// Destroy an instance of Vulkan
    pub(crate) fn destroy_instance(
        &self,
        instance: VkInstance,
        allocator: *const VkAllocationCallbacks,
    ) {
        (self.destroy_instance)(instance, allocator)
    }

    /// Enumerates the physical devices accessible to a Vulkan instance
    pub(crate) fn enumerate_physical_devices(
        &self,
        instance: VkInstance,
        physical_device_count: &mut u32,
        physical_devices: *mut VkPhysicalDevice,
    ) -> Result<(), VkResult> {
        try_vulkan!((self.enumerate_physical_devices)(
            instance,
            physical_device_count,
            physical_devices
        ))
        .map(|_| ())
    }

    /// Return a function pointer for a command
    pub(crate) fn get_device_proc_addr(
        &self,
        device: VkDevice,
        name: &CStr,
    ) -> Option<VkVoidFunction> {
        (self.get_device_proc_addr)(device, name.as_ptr())
    }
}
