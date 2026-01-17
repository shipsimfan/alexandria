#[cfg(target_os = "linux")]
use crate::instance::WaylandWindowSurfaceFunctions;
#[cfg(target_os = "windows")]
use crate::instance::Win32WindowSurfaceFunctions;
use crate::instance::{
    GraphicsDebugMessengerFunctions, WindowSurfaceFunctions,
    inner::functions::GraphicsInstanceFunctions,
};

impl GraphicsInstanceFunctions {
    /// Get the functions for debug messengers
    pub fn debug_messenger(&self) -> &GraphicsDebugMessengerFunctions {
        self.debug_messenger.as_ref().unwrap()
    }

    /// Get the functions for surfaces
    pub fn surface(&self) -> &WindowSurfaceFunctions {
        self.surface.as_ref().unwrap()
    }

    /// Get the functions for Win32 surfaces
    #[cfg(target_os = "windows")]
    pub fn win32_surface(&self) -> &Win32WindowSurfaceFunctions {
        self.win32_surface.as_ref().unwrap()
    }

    /// Get the functions for Wayland surfaces
    #[cfg(target_os = "linux")]
    pub fn wayland_surface(&self) -> &WaylandWindowSurfaceFunctions {
        self.wayland_surface.as_ref().unwrap()
    }
}
