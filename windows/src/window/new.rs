use super::{wnd_proc, Window, WindowClass, WindowCreationError};
use std::ptr::null_mut;
use win32::{
    try_get_last_error, CreateWindowEx, SetWindowLongPtr, CW_USEDEFAULT, GWLP_USERDATA, MSG,
    WS_CAPTION, WS_MINIMIZEBOX, WS_SYSMENU, WS_VISIBLE,
};

impl Window {
    /// Creates a new [`Window`]
    pub fn new(title: &str, width: usize, height: usize) -> Result<Box<Self>, WindowCreationError> {
        let mut title: Vec<_> = title.encode_utf16().collect();
        title.push(0);

        let class = WindowClass::new(wnd_proc, &title)?;

        let wnd = try_get_last_error!(CreateWindowEx(
            0,
            class.atom() as _,
            title.as_ptr(),
            WS_CAPTION | WS_MINIMIZEBOX | WS_VISIBLE | WS_SYSMENU,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            width as i32,
            height as i32,
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
        ))
        .map_err(WindowCreationError::window_creation)?;

        let window = Box::new(Window {
            class,
            should_run: true,
            wnd,
            msg: MSG::default(),
        });

        unsafe { SetWindowLongPtr(wnd, GWLP_USERDATA, &*window as *const Self as _) };

        Ok(window)
    }
}
