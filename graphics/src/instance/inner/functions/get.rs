#[cfg(target_os = "linux")]
use crate::instance::WaylandWindowSurfaceFunctions;
#[cfg(target_os = "windows")]
use crate::instance::Win32WindowSurfaceFunctions;
use crate::instance::{
    GraphicsDebugMessengerFunctions, WindowSurfaceFunctions, inner::GraphicsInstanceFunctions,
};

impl GraphicsInstanceFunctions {
    /// Get the functions for debug messengers
    pub(in crate::instance) fn debug_messenger(&self) -> &GraphicsDebugMessengerFunctions {
        self.debug_messenger.as_ref().unwrap()
    }

    /// Get the functions for surfaces
    pub(in crate::instance) fn surface(&self) -> &WindowSurfaceFunctions {
        self.surface.as_ref().unwrap()
    }

    /// Get the functions for Win32 surfaces
    #[cfg(target_os = "windows")]
    pub(in crate::instance) fn win32_surface(&self) -> &Win32WindowSurfaceFunctions {
        self.win32_surface.as_ref().unwrap()
    }

    /// Get the functions for Wayland surfaces
    #[cfg(target_os = "linux")]
    pub(in crate::instance) fn wayland_surface(&self) -> &WaylandWindowSurfaceFunctions {
        self.wayland_surface.as_ref().unwrap()
    }
}
