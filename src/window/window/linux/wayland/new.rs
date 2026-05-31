use crate::{
    EventQueue, Result,
    math::{Vector2i, Vector2u},
    window::{
        WaylandGlobals, WaylandWindow, WindowBuilder,
        display::DisplayInner,
        window::{WindowInner, linux::wayland::WaylandEventHandler},
    },
};
use std::{ffi::CString, str::FromStr};

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
        let xdg_top_level = xdg_surface.get_top_level()?;

        // Get the toplevel decoration for this surface
        let mut window = globals
            .xdg_decoration_manager()
            .unwrap()
            .get_top_level_decoration(xdg_top_level)?;

        // Set the title of the window
        let title = CString::from_str(builder.get_title()).unwrap();
        window.set_title(&title);

        // Setup other properties of the window
        if builder.is_maximized() {
            window.set_maximized();
        }

        if builder.is_minimized() {
            window.set_minimized();
        }

        window.set_decorations(builder.is_bordered());
        window.set_max_size(builder.get_maximum_size().unwrap_or(Vector2u::ZERO));
        window.set_min_size(builder.get_minimum_size().unwrap_or(Vector2u::ZERO));

        // Set fullscreen if needed
        if builder.is_fullscreen() {
            let position = builder.get_position().unwrap_or(Vector2i::ZERO);
            let mut found_display = None;
            for display in globals.displays() {
                let display = match display {
                    DisplayInner::Wayland(display) => display,
                    _ => continue,
                };

                if display.rect().contains_point(&position) {
                    found_display = Some(display);
                    break;
                }
            }

            window.set_fullscreen(found_display.map(|display| display.wl_output()));
        }

        // Commit the initial state of the window to the compositor
        window.commit();

        Ok(WindowInner::Wayland(WaylandWindow {
            window,
            title: builder.get_title().to_string(),
            minimum_size: builder.get_minimum_size(),
            maximum_size: builder.get_maximum_size(),
        }))
    }
}
