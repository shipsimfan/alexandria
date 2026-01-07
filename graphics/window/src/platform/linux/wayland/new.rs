use crate::{
    DisplayMode, Result, Window, WindowError, WindowState, WindowWakeHandleInner,
    platform::linux::{
        WaylandWindow, WindowKind,
        wayland::{WaylandGlobals, WlDisplay},
    },
};
use alexandria_math::Vector2u;
use std::{borrow::Cow, rc::Rc};

impl WaylandWindow {
    /// Create a new [`WaylandWindow`]
    pub fn new(
        title: Cow<'static, str>,
        size: Option<Vector2u>,
        display_mode: DisplayMode,
        display: Rc<WlDisplay>,
    ) -> Result<Box<Window>> {
        // Get the registered globals
        let mut registry = display
            .clone()
            .get_registry()?
            .add_listener(WaylandGlobals::new())?;
        display.roundtrip()?;

        // Make sure all required global were bound
        if let Err(error) = registry.data_mut().result() {
            return Err(error);
        }

        if registry.data().compositor().is_none() {
            return Err(WindowError::new("no Wayland compositor available"));
        }
        if registry.data().xdg_wm_base().is_none() {
            return Err(WindowError::new("no XDG window manager available"));
        }

        // Create surface
        let wl_surface = registry
            .data_mut()
            .compositor_mut()
            .unwrap()
            .create_surface()?;

        // Create runtime state
        let wake_handle = WindowWakeHandleInner::new();
        let state = WindowState::new(title, size.unwrap_or(Vector2u::new(0, 0)), display_mode);

        Ok(Box::new(Window {
            kind: WindowKind::Wayland(WaylandWindow {
                display,
                registry,
                wl_surface,
            }),
            wake_handle,
            state,
        }))
    }
}
