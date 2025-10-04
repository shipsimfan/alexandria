use crate::{
    window::{WindowClass, WindowHandle},
    Result, Window,
};
use std::ptr::null_mut;
use win32::{
    CreateWindowEx, GetModuleHandle, SetWindowLongPtr, CW_USEDEFAULT, GWLP_USERDATA, WS_BORDER,
    WS_CAPTION, WS_MINIMIZEBOX, WS_SYSMENU, WS_VISIBLE,
};

impl WindowHandle {
    /// Creates a new window and returns the handle to it
    pub fn create(
        title: &[u16],
        class: &WindowClass,
        width: u32,
        height: u32,
        window_ptr: *mut Window,
    ) -> Result<Self> {
        assert!(title.last().is_some());
        assert_eq!(*title.last().unwrap(), 0);

        let handle = unsafe {
            CreateWindowEx(
                0,
                **class as _,
                title.as_ptr(),
                WS_BORDER | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                width as _,
                height as _,
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
