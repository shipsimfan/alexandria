use crate::{
    PackedMap, Result,
    math::{Vector2i, Vector2u},
    window::{display::DisplayInner, window::WindowInner},
};

impl<UserEvent: 'static + Send> WindowInner<UserEvent> {
    /// Set the title of the window
    pub fn set_title(&mut self, title: &str) -> Result<()> {
        match self {
            WindowInner::Wayland(window) => window.set_title(title),
            WindowInner::X11 => todo!(),
        }
    }

    /// Set the position of the window
    pub fn set_position(
        &mut self,
        position: Vector2i,
        displays: &PackedMap<DisplayInner<UserEvent>>,
    ) -> Result<()> {
        match self {
            WindowInner::Wayland(window) => window.set_position(position, displays),
            WindowInner::X11 => todo!(),
        }
    }

    /// Set the size of the client area of the window
    pub fn set_size(
        &mut self,
        size: Vector2i,
        displays: &PackedMap<DisplayInner<UserEvent>>,
    ) -> Result<()> {
        match self {
            WindowInner::Wayland(window) => window.set_size(size, displays),
            WindowInner::X11 => todo!(),
        }
    }

    /// Set the minimum size of the client area of the window
    pub fn set_minimum_size(&mut self, minimum_size: Option<Vector2u>) -> Result<()> {
        match self {
            WindowInner::Wayland(window) => window.set_minimum_size(minimum_size),
            WindowInner::X11 => todo!(),
        }
    }

    /// Set the maximum size of the client area of the window
    pub fn set_maximum_size(&mut self, maximum_size: Option<Vector2u>) -> Result<()> {
        match self {
            WindowInner::Wayland(window) => window.set_maximum_size(maximum_size),
            WindowInner::X11 => todo!(),
        }
    }

    /// Send a close request to the window
    pub fn close(&self) -> Result<()> {
        match self {
            WindowInner::Wayland(window) => window.close(),
            WindowInner::X11 => todo!(),
        }
    }

    /// Maximize the window
    pub fn maximize(&mut self) -> Result<()> {
        match self {
            WindowInner::Wayland(window) => window.maximize(),
            WindowInner::X11 => todo!(),
        }
    }

    /// Minimize the window
    pub fn minimize(&mut self) -> Result<()> {
        match self {
            WindowInner::Wayland(window) => window.minimize(),
            WindowInner::X11 => todo!(),
        }
    }

    /// Hide the window
    pub fn hide(&mut self) -> Result<()> {
        match self {
            WindowInner::Wayland(window) => window.hide(),
            WindowInner::X11 => todo!(),
        }
    }

    /// Show the window
    pub fn show(&mut self) -> Result<()> {
        match self {
            WindowInner::Wayland(window) => window.show(),
            WindowInner::X11 => todo!(),
        }
    }

    /// Set if the window is borderless
    pub fn set_borderless(&mut self, borderless: bool) -> Result<()> {
        match self {
            WindowInner::Wayland(window) => window.set_borderless(borderless),
            WindowInner::X11 => todo!(),
        }
    }

    /// Set if the window is resizable
    pub fn set_resizable(&mut self, resizable: bool) -> Result<()> {
        match self {
            WindowInner::Wayland(window) => window.set_resizable(resizable),
            WindowInner::X11 => todo!(),
        }
    }

    /// Set if the window is fullscreen
    pub fn set_fullscreen(
        &mut self,
        fullscreen: bool,
        displays: &PackedMap<DisplayInner<UserEvent>>,
    ) -> Result<()> {
        match self {
            WindowInner::Wayland(window) => window.set_fullscreen(fullscreen, displays),
            WindowInner::X11 => todo!(),
        }
    }
}
