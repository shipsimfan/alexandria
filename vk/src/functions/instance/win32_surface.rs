use crate::functions::get_instance_proc_addr;
use std::{ffi::CStr, ptr::null_mut};
use vulkan::{
    try_vulkan, VkAllocationCallbacks, VkCreateWin32SurfaceKHR, VkInstance, VkResult, VkSurfaceKHR,
    VkWin32SurfaceCreateInfoKHR, VK_CREATE_WIN32_SURFACE_KHR,
};

/// Functions for win32 surfaces
pub(crate) struct Win32SurfaceFunctions {
    create_win32_surface: VkCreateWin32SurfaceKHR,
}

impl Win32SurfaceFunctions {
    /// Loads functions for win32 surfaces
    pub(super) fn new(instance: VkInstance) -> Result<Self, &'static CStr> {
        let create_win32_surface: VkCreateWin32SurfaceKHR =
            get_instance_proc_addr!(instance, VK_CREATE_WIN32_SURFACE_KHR)?;

        Ok(Win32SurfaceFunctions {
            create_win32_surface,
        })
    }

    /// Creates a [`VkSurfaceKHR`] for a win32 window
    pub(crate) fn create_win32_surface(
        &self,
        instance: VkInstance,
        create_info: &VkWin32SurfaceCreateInfoKHR,
        allocator: *const VkAllocationCallbacks,
    ) -> Result<VkSurfaceKHR, VkResult> {
        let mut surface = null_mut();
        try_vulkan!((self.create_win32_surface)(
            instance,
            create_info,
            allocator,
            &mut surface
        ))
        .map(|_| surface)
    }
}
