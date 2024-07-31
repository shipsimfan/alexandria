use super::get_instance_proc_addr;
use crate::InstanceExtension;
use std::ffi::CStr;
use vulkan::{
    try_vulkan, VkAllocationCallbacks, VkDestroyInstance, VkDevice, VkEnumeratePhysicalDevices,
    VkGetDeviceProcAddr, VkInstance, VkPhysicalDevice, VkResult, VkVoidFunction,
    VK_DESTROY_INSTANCE, VK_ENUMERATE_PHYSICAL_DEVICES, VK_GET_DEVICE_PROC_ADDR,
};

mod debug_utils;
mod physical_device;
mod surface;
mod win32_surface;

pub(crate) use debug_utils::DebugUtilsFunctions;
pub(crate) use physical_device::PhysicalDeviceFunctions;
pub(crate) use surface::SurfaceFunctions;
pub(crate) use win32_surface::Win32SurfaceFunctions;

/// Instance-level function pointers
pub(crate) struct InstanceFunctions {
    physical_device: PhysicalDeviceFunctions,

    debug_utils: Option<DebugUtilsFunctions>,
    surface: Option<SurfaceFunctions>,
    win32_surface: Option<Win32SurfaceFunctions>,

    destroy_instance: VkDestroyInstance,
    enumerate_physical_devices: VkEnumeratePhysicalDevices,
    get_device_proc_addr: VkGetDeviceProcAddr,
}

impl InstanceFunctions {
    /// Loads the instance-level functions
    pub(crate) fn new(
        instance: VkInstance,
        extensions: &[InstanceExtension],
    ) -> Result<Self, &'static CStr> {
        let physical_device = PhysicalDeviceFunctions::new(instance)?;

        let mut debug_utils = None;
        let mut surface = None;
        let mut win32_surface = None;

        for extension in extensions {
            match extension {
                InstanceExtension::DebugUtils => {
                    debug_utils = Some(DebugUtilsFunctions::new(instance)?)
                }
                InstanceExtension::Surface => surface = Some(SurfaceFunctions::new(instance)?),
                InstanceExtension::Win32Surface => {
                    win32_surface = Some(Win32SurfaceFunctions::new(instance)?)
                }
            }
        }

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
            surface,
            debug_utils,
            win32_surface,
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

    /// Gets the functions for a surface
    pub(crate) fn s(&self) -> Option<&SurfaceFunctions> {
        self.surface.as_ref()
    }

    /// Gets the functions for a win32 surface
    #[cfg_attr(not(target_os = "windows"), allow(unused))]
    pub(crate) fn ws(&self) -> Option<&Win32SurfaceFunctions> {
        self.win32_surface.as_ref()
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
