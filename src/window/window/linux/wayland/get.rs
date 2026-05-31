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
    pub(in crate::window::window::linux) fn rect(&self) -> Recti {
        self.window.rect()
    }

    /// Get the minimum size of the window's client area, in pixels
    pub(in crate::window::window::linux) fn minimum_size(&self) -> Option<Vector2u> {
        self.minimum_size
    }

    /// Get the maximum size of the window's client area, in pixels
    pub(in crate::window::window::linux) fn maximum_size(&self) -> Option<Vector2u> {
        self.maximum_size
    }

    /// Get the current content scale factor of the window
    pub(in crate::window::window::linux) fn content_scale(&self) -> f32 {
        todo!()
    }

    /// Is the window currently in fullscreen mode?
    pub(in crate::window::window::linux) fn is_fullscreen(&self) -> bool {
        self.window.is_fullscreen()
    }

    /// Is the window currently maximized?
    pub(in crate::window::window::linux) fn is_maximized(&self) -> bool {
        self.window.is_maximized()
    }

    /// Is the window currently minimized?
    pub(in crate::window::window::linux) fn is_minimized(&self) -> bool {
        false // Minimized state cannot be detected on Wayland, so we just return false
    }

    /// Is the window currently focused?
    pub(in crate::window::window::linux) fn is_focused(&self) -> bool {
        self.window.is_focused()
    }

    /// Is the window currently visible?
    pub(in crate::window::window::linux) fn is_visible(&self) -> bool {
        true // Visibility cannot be controlled on Wayland, so we just return true
    }

    /// Is the window borderless?
    pub(in crate::window::window::linux) fn is_borderless(&self) -> bool {
        self.window.is_borderless()
    }

    /// Is the window resizable?
    pub(in crate::window::window::linux) fn is_resizable(&self) -> bool {
        true // Resizability cannot be controlled on Wayland, so we just return true
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
    pub(in crate::window::window::linux) fn surface_creation_handle(
        &self,
    ) -> WindowSurfaceCreationHandle {
        let (surface, display) = unsafe { self.window.surface_and_display() };

        WindowSurfaceCreationHandle::Wayland { display, surface }
    }
}
