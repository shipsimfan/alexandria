#[cfg(target_os = "windows")]
use crate::gpu::instance::VulkanWin32SurfaceFunctions;
use crate::gpu::{
    VulkanInstanceFunctions,
    instance::{VulkanDebugMessengerFunctions, VulkanSurfaceFunctions},
};

impl VulkanInstanceFunctions {
    /// Get the functions for debug messengers
    pub(in crate::gpu::instance) fn debug_messenger(&self) -> &VulkanDebugMessengerFunctions {
        self.debug_messenger.as_ref().unwrap()
    }

    /// Get the functions for surfaces
    pub(in crate::gpu::instance) fn surface(&self) -> &VulkanSurfaceFunctions {
        self.surface.as_ref().unwrap()
    }

    /// Get the functions for Win32 surfaces
    #[cfg(target_os = "windows")]
    pub(in crate::gpu::instance) fn win32_surface(&self) -> &VulkanWin32SurfaceFunctions {
        self.win32_surface.as_ref().unwrap()
    }
}
