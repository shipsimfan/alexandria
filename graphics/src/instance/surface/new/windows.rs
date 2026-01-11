use crate::{GraphicsError, Result, WindowSurface, instance::inner::GraphicsInstanceInner};
use alexandria_window::Window;
use std::{ptr::null, sync::Arc};
use vulkan::{
    khr_surface::VkSurfaceKhr, khr_win32_surface::VkWin32SurfaceCreateInfoKhr, try_vulkan,
};

impl WindowSurface {
    /// Create a new [`WindowSurface`]
    pub(in crate::instance) fn new(
        instance: Arc<GraphicsInstanceInner>,
        window: &mut Window,
    ) -> Result<WindowSurface> {
        let (hinstance, hwnd) = unsafe { window.surface_creation_handle() };
        let create_info = VkWin32SurfaceCreateInfoKhr {
            hinstance,
            hwnd,
            ..Default::default()
        };
        let mut handle = VkSurfaceKhr::null();
        try_vulkan!((instance.functions.win32_surface().create_surface)(
            instance.handle(),
            &create_info,
            null(),
            &mut handle
        ))
        .map_err(|vk| GraphicsError::new_vk("unable to create window surface", vk))?;

        Ok(WindowSurface { handle, instance })
    }
}
