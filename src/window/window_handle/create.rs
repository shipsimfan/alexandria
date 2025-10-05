use crate::{
    window::{WindowClass, WindowHandle},
    DisplayMode, Result, Window,
};
use std::ptr::null_mut;
use win32::{
    CreateWindowEx, GetModuleHandle, SetWindowLongPtr, CW_USEDEFAULT, GWLP_USERDATA, WS_BORDER,
    WS_CAPTION, WS_EX_APPWINDOW, WS_MINIMIZEBOX, WS_OVERLAPPEDWINDOW, WS_POPUP, WS_SYSMENU,
    WS_VISIBLE,
};

impl WindowHandle {
    /// Creates a new window and returns the handle to it
    pub fn create(
        title: &[u16],
        class: &WindowClass,
        x: Option<i32>,
        y: Option<i32>,
        width: Option<u32>,
        height: Option<u32>,
        display_mode: DisplayMode,
        window_ptr: *mut Window,
    ) -> Result<Self> {
        assert!(title.last().is_some());
        assert_eq!(*title.last().unwrap(), 0);

        let (mut style, ex_style) = match display_mode {
            DisplayMode::Resizable => (WS_OVERLAPPEDWINDOW, 0),
            DisplayMode::Windowed => (WS_BORDER | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX, 0),
            DisplayMode::Borderless => (WS_POPUP, WS_EX_APPWINDOW),
        };
        style |= WS_VISIBLE;

        let handle = unsafe {
            CreateWindowEx(
                ex_style,
                **class as _,
                title.as_ptr(),
                style,
                x.unwrap_or(CW_USEDEFAULT),
                y.unwrap_or(CW_USEDEFAULT),
                width.unwrap_or(CW_USEDEFAULT as _) as _,
                height.unwrap_or(CW_USEDEFAULT as _) as _,
                null_mut(),
                null_mut(),
                GetModuleHandle(null_mut()),
                null_mut(),
            )
        };

        if handle == null_mut() {
            return Err(win32::Error::get_last_error().into());
        }

        unsafe { SetWindowLongPtr(handle, GWLP_USERDATA, window_ptr as _) };

        Ok(WindowHandle { handle })
    }
}
