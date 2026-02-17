use crate::{
    math::Vector2i,
    window::{Win32Window, WindowClass, WindowProc},
};
use std::{
    ptr::{null, null_mut},
    rc::Rc,
};
use win32::{
    CW_USEDEFAULT, CreateWindowEx, DWORD, GWLP_USERDATA, GetLastError, GetModuleHandle,
    HWND_MESSAGE, MAKEINTATOM, SetLastError, SetWindowLongPtr,
};

impl<T: WindowProc> Win32Window<T> {
    /// Creates a new window and returns the handle to it
    pub fn new(
        title: Option<&str>,
        position: Option<Vector2i>,
        size: Option<Vector2i>,
        style: DWORD,
        ex_style: DWORD,
        class: Rc<WindowClass<T>>,
        user_data: T,
    ) -> win32::Result<Win32Window<T>> {
        let title: Option<Vec<_>> = title.map(|title| title.encode_utf16().chain([0]).collect());

        let position = position.unwrap_or(Vector2i::new(CW_USEDEFAULT, CW_USEDEFAULT));

        let size = match size {
            Some(size) => {
                let size = todo!("Convert size from client to window based on style"); // display_mode.client_to_window(size)?;
                //Vector2i::new(size.x as _, size.y as _)
            }
            None => Vector2i::new(CW_USEDEFAULT, CW_USEDEFAULT),
        };

        let handle = unsafe {
            CreateWindowEx(
                ex_style,
                MAKEINTATOM!(class.handle()),
                title.as_ref().map(|title| title.as_ptr()).unwrap_or(null()),
                style,
                position.x,
                position.y,
                size.x,
                size.y,
                null_mut(),
                null_mut(),
                GetModuleHandle(null_mut()),
                null_mut(),
            )
        };
        if handle == null_mut() {
            return Err(win32::Error::get_last_error());
        }

        let user_data = Box::new(user_data);
        let window = Win32Window {
            handle,
            user_data,
            class,
        };

        unsafe { SetLastError(0) };
        if unsafe {
            SetWindowLongPtr(
                window.handle,
                GWLP_USERDATA,
                Box::as_ptr(&window.user_data) as _,
            )
        } == 0
        {
            let last_error = unsafe { GetLastError() };
            if last_error != 0 {
                return Err(win32::Error::new_win32(last_error));
            }
        }

        Ok(window)
    }
}
