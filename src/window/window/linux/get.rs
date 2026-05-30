use crate::{
    gpu::VulkanInstanceExtension,
    math::{Recti, Vector2u},
    window::{WindowSurfaceCreationHandle, window::WindowInner},
};

impl<UserEvent: 'static + Send> WindowInner<UserEvent> {
    /// Get the current title of the window
    pub fn title(&self) -> &str {
        match self {
            WindowInner::Wayland(window) => window.title(),
            WindowInner::X11 => todo!(),
        }
    }

    /// Get the position and size of the window's client area, in screen coordinates
    pub fn rect(&self) -> Recti {
        match self {
            WindowInner::Wayland(window) => window.rect(),
            WindowInner::X11 => todo!(),
        }
    }

    /// Get the minimum size of the window's client area, in pixels
    pub fn minimum_size(&self) -> Option<Vector2u> {
        match self {
            WindowInner::Wayland(window) => window.minimum_size(),
            WindowInner::X11 => todo!(),
        }
    }

    /// Get the maximum size of the window's client area, in pixels
    pub fn maximum_size(&self) -> Option<Vector2u> {
        match self {
            WindowInner::Wayland(window) => window.maximum_size(),
            WindowInner::X11 => todo!(),
        }
    }

    /// Get the current content scale factor of the window
    pub fn content_scale(&self) -> f32 {
        match self {
            WindowInner::Wayland(window) => window.content_scale(),
            WindowInner::X11 => todo!(),
        }
    }

    /// Is the window currently in fullscreen mode?
    pub fn is_fullscreen(&self) -> bool {
        match self {
            WindowInner::Wayland(window) => window.is_fullscreen(),
            WindowInner::X11 => todo!(),
        }
    }

    /// Is the window currently maximized?
    pub fn is_maximized(&self) -> bool {
        match self {
            WindowInner::Wayland(window) => window.is_maximized(),
            WindowInner::X11 => todo!(),
        }
    }

    /// Is the window currently minimized?
    pub fn is_minimized(&self) -> bool {
        match self {
            WindowInner::Wayland(window) => window.is_minimized(),
            WindowInner::X11 => todo!(),
        }
    }

    /// Is the window currently focused?
    pub fn is_focused(&self) -> bool {
        match self {
            WindowInner::Wayland(window) => window.is_focused(),
            WindowInner::X11 => todo!(),
        }
    }

    /// Is the window currently visible?
    pub fn is_visible(&self) -> bool {
        match self {
            WindowInner::Wayland(window) => window.is_visible(),
            WindowInner::X11 => todo!(),
        }
    }

    /// Is the window borderless?
    pub fn is_borderless(&self) -> bool {
        match self {
            WindowInner::Wayland(window) => window.is_borderless(),
            WindowInner::X11 => todo!(),
        }
    }

    /// Is the window resizable?
    pub fn is_resizable(&self) -> bool {
        match self {
            WindowInner::Wayland(window) => window.is_resizable(),
            WindowInner::X11 => todo!(),
        }
    }

    /// Get the required extensions for creating surfaces for this window
    pub fn vulkan_extensions(&self) -> [VulkanInstanceExtension; 2] {
        match self {
            WindowInner::Wayland(window) => window.vulkan_extensions(),
            WindowInner::X11 => todo!(),
        }
    }

    /// Get the handle for creating a surface for this window
    pub fn surface_creation_handle(&self) -> WindowSurfaceCreationHandle {
        match self {
            WindowInner::Wayland(window) => window.surface_creation_handle(),
            WindowInner::X11 => todo!(),
        }
    }
}
