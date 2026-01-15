use crate::{
    DisplayMode, Result, Window, WindowEvents, WindowState, WindowWakeHandleInner,
    platform::windows::{WindowClass, WindowHandle},
};
use alexandria_math::Vector2u;
use std::borrow::Cow;

impl<Callbacks: WindowEvents> Window<Callbacks> {
    /// Create a new [`Window`]
    pub(crate) fn new(
        title: Cow<'static, str>,
        size: Option<Vector2u>,
        display_mode: DisplayMode,
        mut callbacks: Callbacks,
    ) -> Result<Box<Window<Callbacks>>> {
        // Convert the title to UTF-16
        let title_utf16: Vec<_> = title.encode_utf16().chain([0]).collect();

        // Create window class
        let class = WindowClass::register::<Callbacks>(&title_utf16)?;

        // Create window
        let mut window = Box::new_uninit();
        let handle = WindowHandle::new(
            &title_utf16,
            &class,
            size,
            display_mode,
            window.as_mut_ptr(),
        )?;

        // Get position
        let size = handle.get_size()?;
        callbacks.on_resize(size);

        // Create state
        let state = WindowState::new(title.to_owned(), size, display_mode);

        // Create wake handle
        let wake_handle = WindowWakeHandleInner::new();

        Ok(Box::write(
            window,
            Window {
                wnd_proc_result: Ok(()),
                handle,
                wake_handle,
                class,
                state,
                callbacks,
                is_resizing: None,
            },
        ))
    }
}
