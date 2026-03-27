use crate::{
    Error, Result,
    gpu::{VulkanInstance, VulkanSurface},
    window::Window,
};
use std::ptr::null;
use vulkan::{
    khr_surface::VkSurfaceKhr, khr_win32_surface::VkWin32SurfaceCreateInfoKhr, try_vulkan,
};

impl VulkanSurface {
    /// Create a new [`VulkanSurface`]
    pub(in crate::gpu::instance) fn new<UserEvent: 'static + Send>(
        instance: &VulkanInstance,
        window: &Window<UserEvent>,
    ) -> Result<VulkanSurface> {
        let (hinstance, hwnd) = window
            .surface_creation_handle()
            .ok_or_else(|| Error::new("unable to create surface - window is destroyed"))?;

        let create_info = VkWin32SurfaceCreateInfoKhr {
            hinstance,
            hwnd,
            ..Default::default()
        };

        let mut handle = VkSurfaceKhr::null();
        try_vulkan!((instance.functions().win32_surface().create_win32_surface)(
            instance.handle(),
            &create_info,
            null(),
            &mut handle
        ))
        .map_err(|vk| Error::new_with("unable to create window surface", vk))?;

        Ok(VulkanSurface {
            handle,
            instance: instance.clone(),
        })
    }
}
