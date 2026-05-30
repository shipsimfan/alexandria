use std::{ffi::CString, str::FromStr};

use crate::{
    EventQueue, Result,
    math::Vector2u,
    window::{
        WaylandGlobals, WaylandWindow, WindowBuilder,
        window::{WindowInner, linux::wayland::WaylandEventHandler},
    },
};

/// The default size of a window, in pixels
///
/// TODO: Change this to be based on the size of the display
const DEFAULT_SIZE: Vector2u = Vector2u::new(1280, 720);

impl<UserEvent: 'static + Send> WaylandWindow<UserEvent> {
    /// Create a new [`WindowInner`]
    pub fn new(
        builder: &WindowBuilder<UserEvent>,
        event_queue: &EventQueue<UserEvent>,
        globals: &mut WaylandGlobals<UserEvent>,
    ) -> Result<WindowInner<UserEvent>> {
        // Create surface
        let wl_surface = globals.compositor_mut().unwrap().create_surface()?;

        // Get the XDG surface for this surface
        let xdg_surface = globals
            .xdg_wm_base()
            .unwrap()
            .clone()
            .get_xdg_surface(wl_surface)?
            .add_listener(WaylandEventHandler::new(
                event_queue.clone(),
                builder.get_size().unwrap_or(DEFAULT_SIZE),
            ))?;

        // Get the toplevel for this surface
        let window = xdg_surface.get_top_level()?;

        // Set the title of the window
        let title = CString::from_str(builder.get_title()).unwrap();
        window.set_title(&title);

        // Commit the initial state of the window to the compositor
        window.commit();

        Ok(WindowInner::Wayland(WaylandWindow {
            window,
            title: builder.get_title().to_string(),
        }))
    }
}
