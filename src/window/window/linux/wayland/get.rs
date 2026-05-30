use crate::{
    gpu::VulkanInstanceExtension,
    math::{Recti, Vector2u},
    window::{WaylandWindow, WindowSurfaceCreationHandle},
};

impl<UserEvent: 'static + Send> WaylandWindow<UserEvent> {
    /// Get the current title of the window
    pub(in crate::window::window::linux) fn title(&self) -> &str {
        &self.title
    }

    /// Get the position and size of the window's client area, in screen coordinates
    pub fn rect(&self) -> Recti {
        self.window.rect()
    }

    /// Get the minimum size of the window's client area, in pixels
    pub fn minimum_size(&self) -> Option<Vector2u> {
        todo!()
    }

    /// Get the maximum size of the window's client area, in pixels
    pub fn maximum_size(&self) -> Option<Vector2u> {
        todo!()
    }

    /// Get the current content scale factor of the window
    pub fn content_scale(&self) -> f32 {
        todo!()
    }

    /// Is the window currently in fullscreen mode?
    pub fn is_fullscreen(&self) -> bool {
        self.window.is_fullscreen()
    }

    /// Is the window currently maximized?
    pub fn is_maximized(&self) -> bool {
        self.window.is_maximized()
    }

    /// Is the window currently minimized?
    pub fn is_minimized(&self) -> bool {
        self.window.is_minimized()
    }

    /// Is the window currently focused?
    pub fn is_focused(&self) -> bool {
        todo!()
    }

    /// Is the window currently visible?
    pub fn is_visible(&self) -> bool {
        todo!()
    }

    /// Is the window borderless?
    pub fn is_borderless(&self) -> bool {
        todo!()
    }

    /// Is the window resizable?
    pub fn is_resizable(&self) -> bool {
        todo!()
    }

    /// Get the required extensions for creating surfaces for this window
    pub(in crate::window::window::linux) fn vulkan_extensions(
        &self,
    ) -> [VulkanInstanceExtension; 2] {
        [
            VulkanInstanceExtension::Surface,
            VulkanInstanceExtension::WaylandSurface,
        ]
    }

    /// Get the handle for creating a surface for this window
    pub fn surface_creation_handle(&self) -> WindowSurfaceCreationHandle {
        let (surface, display) = unsafe { self.window.surface_and_display() };

        WindowSurfaceCreationHandle::Wayland { display, surface }
    }
}
