use crate::{
    Error, EventQueue, PackedMap, Result,
    math::{Recti, Vector2},
    window::{
        Win32Window, WindowBuilder, WindowClass, WindowStyle,
        display::DisplayInner,
        window::{WindowInner, windows::StandardWndProc},
    },
};
use std::rc::Rc;

impl<UserEvent: 'static + Send> WindowInner<UserEvent> {
    /// Create a new [`WindowInner`]
    pub fn new(
        class: Rc<WindowClass<StandardWndProc<UserEvent>>>,
        builder: &WindowBuilder<UserEvent>,
        event_queue: &EventQueue<UserEvent>,
        displays: &PackedMap<DisplayInner<UserEvent>>,
    ) -> Result<WindowInner<UserEvent>> {
        // Prepare window style
        let style = if builder.is_fullscreen() {
            WindowStyle::fullscreen()
        } else {
            let mut style = WindowStyle::normal(builder.is_bordered(), builder.is_resizable());

            if builder.is_maximized() {
                style.maximize();
            }
            if builder.is_minimized() {
                style.minimize();
            }
            if !builder.is_hidden() {
                style.show();
            }

            style
        };

        // Prepare for fullscreen
        let (size, position) = if builder.is_fullscreen() {
            // Find the display at the requested position
            let position = builder.get_position().unwrap_or(Vector2::ZERO);
            let mut display = None;
            for test_display in displays {
                if test_display.rect().contains_point(&position) {
                    display = Some(test_display);
                    break;
                }
            }

            match display {
                Some(display) => {
                    // Update the position and size to the display's directly
                    let rect = display.get_rect()?;
                    (Some(rect.size), Some(rect.position))
                }
                None => (None, None),
            }
        } else {
            (
                builder
                    .get_size()
                    .map(|size| Vector2::new(size.x as i32, size.y as i32)),
                builder.get_position(),
            )
        };

        // Create the window
        let mut window = Win32Window::new(
            Some(builder.get_title()),
            position,
            size,
            style,
            class,
            StandardWndProc::new(event_queue.clone()),
        )
        .map_err(|os| Error::new_with("unable to create a window", os))?;

        // Collect actual window size, position, and DPI
        let position = window
            .get_client_position()
            .map_err(|os| Error::new_with("unable to get window position", os))?;
        let size = window
            .get_client_size()
            .map_err(|os| Error::new_with("unable to get window size", os))?;

        window.init(
            Recti::new(position, size),
            builder.get_position(),
            builder.get_size(),
            builder.is_fullscreen(),
        );

        Ok(WindowInner { window })
    }
}
