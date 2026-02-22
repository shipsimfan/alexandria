use crate::{
    DisplayMode, Result, Window, WindowError, WindowEvents, WindowWakeHandleInner,
    platform::linux::{
        WaylandWindow,
        wayland::{WaylandEventHandler, WaylandGlobals, WlDisplay},
    },
};
use alexandria_math::Vector2u;
use std::{borrow::Cow, ffi::CString, rc::Rc, str::FromStr};

impl<Callbacks: WindowEvents> WaylandWindow<Callbacks> {
    /// Create a new [`WaylandWindow`]
    pub(in crate::platform::linux) fn new(
        title: Cow<'static, str>,
        size: Option<Vector2u>,
        display_mode: DisplayMode,
        display: Rc<WlDisplay>,
        callbacks: Callbacks,
    ) -> Result<Box<Window<Callbacks>>> {
        // Create surface
        let wl_surface = registry
            .data_mut()
            .compositor_mut()
            .unwrap()
            .create_surface()?;

        let c_title = CString::from_str(&title).unwrap();
        let xdg_surface = registry
            .data()
            .xdg_wm_base()
            .unwrap()
            .clone()
            .get_xdg_surface(wl_surface)?
            .add_listener(WaylandEventHandler::new(
                title,
                size.unwrap_or(Vector2u::new(0, 0)),
                display_mode,
                callbacks,
            ))?;

        let mut toplevel_surface = xdg_surface.get_toplevel()?;
        toplevel_surface.set_title(&c_title);
        toplevel_surface.set_app_id(&c_title);

        toplevel_surface.wl_surface_mut().commit();

        display.roundtrip()?;

        // Create runtime state
        let wake_handle = WindowWakeHandleInner::new()?;

        Ok(Box::new(Window::Wayland(WaylandWindow {
            wake_handle,
            toplevel_surface,
            display,
            registry,
        })))
    }
}
