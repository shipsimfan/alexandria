use crate::{
    Error, Result,
    gpu::{VulkanInstance, VulkanSurface},
    window::{Window, WindowSurfaceCreationHandle},
};
use std::ptr::null;
use vulkan::{
    khr_surface::VkSurfaceKhr, khr_wayland_surface::VkWaylandSurfaceCreateInfoKhr, try_vulkan,
};
use wayland::{wl_display, wl_surface};

impl VulkanSurface {
    /// Create a new [`VulkanSurface`]
    pub(in crate::gpu::instance) fn new<UserEvent: 'static + Send>(
        instance: &VulkanInstance,
        window: &Window<UserEvent>,
    ) -> Result<VulkanSurface> {
        match window
            .surface_creation_handle()
            .ok_or_else(|| Error::new("unable to create surface - window is destroyed"))?
        {
            WindowSurfaceCreationHandle::Wayland { display, surface } => {
                VulkanSurface::new_wayland(instance, display, surface)
            }
            WindowSurfaceCreationHandle::X11 => todo!(),
        }
    }

    /// Create a new [`VulkanSurface`] for a Wayland window
    fn new_wayland(
        instance: &VulkanInstance,
        display: *mut wl_display,
        surface: *mut wl_surface,
    ) -> Result<VulkanSurface> {
        let create_info = VkWaylandSurfaceCreateInfoKhr {
            display,
            surface,
            ..Default::default()
        };

        let mut handle = VkSurfaceKhr::null();
        try_vulkan!((instance
            .functions()
            .wayland_surface()
            .create_wayland_surface)(
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
