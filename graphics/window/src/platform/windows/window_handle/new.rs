use crate::{
    DisplayMode, Result, WindowError,
    platform::windows::{Window, WindowClass, WindowHandle},
};
use alexandria_math::{Vector2i, Vector2u};
use std::ptr::null_mut;
use win32::{CW_USEDEFAULT, CreateWindowEx, GWLP_USERDATA, GetModuleHandle, SetWindowLongPtr};

impl WindowHandle {
    /// Creates a new window and returns the handle to it
    pub fn new(
        title: &[u16],
        class: &WindowClass,
        size: Option<Vector2u>,
        display_mode: DisplayMode,
        window_ptr: *mut Window,
    ) -> Result<Self> {
        assert!(title.last().is_some());
        assert_eq!(*title.last().unwrap(), 0);

        let (style, ex_style) = display_mode.style();

        let size = match size {
            Some(size) => {
                let size = display_mode.client_to_window(size)?;
                Vector2i::new(size.x as _, size.y as _)
            }
            None => Vector2i::new(CW_USEDEFAULT, CW_USEDEFAULT),
        };

        let handle = unsafe {
            CreateWindowEx(
                ex_style,
                **class as _,
                title.as_ptr(),
                style,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                size.x,
                size.y,
                null_mut(),
                null_mut(),
                GetModuleHandle(null_mut()),
                null_mut(),
            )
        };

        if handle == null_mut() {
            return Err(WindowError::new_os(
                "unable to create window",
                win32::Error::get_last_error(),
            ));
        }

        unsafe { SetWindowLongPtr(handle, GWLP_USERDATA, window_ptr as _) };

        Ok(WindowHandle { handle })
    }
}
