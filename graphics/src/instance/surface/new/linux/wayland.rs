use crate::{GraphicsError, Result, WindowSurface, instance::inner::GraphicsInstanceInner};
use alexandria_window::{WaylandWindow, WindowEvents};
use std::{ptr::null, sync::Arc};
use vulkan::{khr_surface::VkSurfaceKhr, try_vulkan};

impl WindowSurface {
    /// Create a new Wayland [`WindowSurface`]
    pub(in crate::instance) fn new_wayland<Callbacks: WindowEvents>(
        instance: Arc<GraphicsInstanceInner>,
        window: &mut WaylandWindow<Callbacks>,
    ) -> Result<WindowSurface> {
        let (display, surface) = unsafe { window.surface_creation_handle() };
        let create_info = VkWaylandSurfaceCreateInfoKhr {
            display,
            surface,
            ..Default::default()
        };
        let mut handle = VkSurfaceKhr::null();
        try_vulkan!((instance.functions.wayland_surface().create_surface)(
            instance.handle(),
            &create_info,
            null(),
            &mut handle
        ))
        .map_err(|vk| GraphicsError::new_vk("unable to create window surface", vk))?;

        Ok(WindowSurface { handle, instance })
    }
}
