use crate::{
    DisplayMode, Result, Window, WindowState, WindowWakeHandleInner,
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
        let registry = display
            .clone()
            .get_registry()?
            .add_listener(WaylandGlobals::new())?;

        let wake_handle = WindowWakeHandleInner::new();
        let state = WindowState::new(title, size.unwrap_or(Vector2u::new(0, 0)), display_mode);

        Ok(Box::new(Window {
            kind: WindowKind::Wayland(WaylandWindow { display, registry }),
            wake_handle,
            state,
        }))
    }
}
