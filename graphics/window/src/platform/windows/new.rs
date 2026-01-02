use crate::{
    DisplayMode, Result, Window, WindowState,
    platform::windows::{WindowClass, WindowHandle},
};

impl Window {
    /// Create a new [`Window`]
    pub fn new(
        title: &str,
        x: Option<i32>,
        y: Option<i32>,
        width: Option<u32>,
        height: Option<u32>,
        display_mode: DisplayMode,
    ) -> Result<Box<Window>> {
        // Convert the title to UTF-16
        let mut title_utf16: Vec<_> = title.encode_utf16().collect();
        title_utf16.push(0);

        // Create window class
        let class = WindowClass::register(&title_utf16)?;

        // Create window
        let mut window = Box::new_uninit();
        let handle = WindowHandle::new(
            &title_utf16,
            &class,
            x,
            y,
            width,
            height,
            display_mode,
            window.as_mut_ptr(),
        )?;

        // Get position and size
        let (position, size) = handle.get_size_and_position()?;

        // Create state
        let state = WindowState::new(title.to_owned(), position, size, display_mode);

        Ok(Box::write(
            window,
            Window {
                wnd_proc_result: Ok(()),
                handle,
                class,
                state,
            },
        ))
    }
}
