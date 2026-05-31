use crate::{
    PackedMap, Result,
    math::{Vector2i, Vector2u},
    window::{WaylandWindow, display::DisplayInner},
};
use std::{ffi::CString, str::FromStr};

impl<UserEvent: 'static + Send> WaylandWindow<UserEvent> {
    /// Set the title of the window
    pub(in crate::window::window::linux) fn set_title(&mut self, title: &str) -> Result<()> {
        let title = CString::from_str(title).unwrap();
        self.window.set_title(&title);
        Ok(())
    }

    /// Set the position of the window
    pub(in crate::window::window::linux) fn set_position(
        &mut self,
        _: Vector2i,
        _: &PackedMap<DisplayInner<UserEvent>>,
    ) -> Result<()> {
        Ok(()) // Position cannot be controlled on Wayland, so we just ignore this request
    }

    /// Set the size of the client area of the window
    pub(in crate::window::window::linux) fn set_size(
        &mut self,
        _: Vector2i,
        _: &PackedMap<DisplayInner<UserEvent>>,
    ) -> Result<()> {
        Ok(()) // Size cannot be controlled on Wayland except at initialization, so we just ignore this request
    }

    /// Set the minimum size of the client area of the window
    pub(in crate::window::window::linux) fn set_minimum_size(
        &mut self,
        minimum_size: Option<Vector2u>,
    ) -> Result<()> {
        self.window
            .set_min_size(minimum_size.unwrap_or(Vector2u::ZERO));
        self.minimum_size = minimum_size;
        Ok(())
    }

    /// Set the maximum size of the client area of the window
    pub(in crate::window::window::linux) fn set_maximum_size(
        &mut self,
        maximum_size: Option<Vector2u>,
    ) -> Result<()> {
        self.window
            .set_max_size(maximum_size.unwrap_or(Vector2u::ZERO));
        self.maximum_size = maximum_size;
        Ok(())
    }

    /// Send a close request to the window
    pub(in crate::window::window::linux) fn close(&self) -> Result<()> {
        self.window.close()
    }

    /// Maximize the window
    pub(in crate::window::window::linux) fn maximize(&mut self) -> Result<()> {
        self.window.set_maximized();
        Ok(())
    }

    /// Minimize the window
    pub(in crate::window::window::linux) fn minimize(&mut self) -> Result<()> {
        self.window.set_minimized();
        Ok(())
    }

    /// Hide the window
    pub(in crate::window::window::linux) fn hide(&mut self) -> Result<()> {
        Ok(()) // Hiding cannot be controlled on Wayland, so we just ignore this request
    }

    /// Show the window
    pub(in crate::window::window::linux) fn show(&mut self) -> Result<()> {
        Ok(()) // Showing cannot be controlled on Wayland, so we just ignore this request
    }

    /// Set if the window is borderless
    pub(in crate::window::window::linux) fn set_borderless(
        &mut self,
        borderless: bool,
    ) -> Result<()> {
        self.window.set_decorations(!borderless);
        Ok(())
    }

    /// Set if the window is resizable
    pub(in crate::window::window::linux) fn set_resizable(&mut self, _: bool) -> Result<()> {
        Ok(()) // Resizability cannot be controlled on Wayland, so we just ignore this request
    }

    /// Set if the window is fullscreen
    pub(in crate::window::window::linux) fn set_fullscreen(
        &mut self,
        fullscreen: bool,
        _: &PackedMap<DisplayInner<UserEvent>>,
    ) -> Result<()> {
        if fullscreen {
            self.window.set_fullscreen::<()>(None);
        } else {
            self.window.unset_fullscreen();
        }
        Ok(())
    }
}
