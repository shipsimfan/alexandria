use crate::{
    math::{Recti, Vector2i},
    window::{Win32Window, WindowClass, WindowProc, WindowStyle},
};
use std::{
    ptr::{null, null_mut},
    rc::Rc,
};
use win32::{
    CW_USEDEFAULT, CreateWindowEx, GWLP_USERDATA, GetLastError, GetModuleHandle, MAKEINTATOM,
    SetLastError, SetWindowLongPtr,
};

const VEC_CW_USEDEFAULT: Vector2i = Vector2i::new(CW_USEDEFAULT, CW_USEDEFAULT);

impl<T: WindowProc> Win32Window<T> {
    /// Creates a new window and returns the handle to it
    pub fn new(
        title: Option<&str>,
        position: Option<Vector2i>,
        size: Option<Vector2i>,
        style: WindowStyle,
        class: Rc<WindowClass<T>>,
        user_data: T,
    ) -> win32::Result<Win32Window<T>> {
        let title: Option<Vec<_>> = title.map(|title| title.encode_utf16().chain([0]).collect());

        let window_rect = match (position, size) {
            (Some(position), Some(size)) => style.client_to_window(Recti::new(position, size))?,
            (Some(position), None) => {
                let mut rect = style.client_to_window(Recti::new(position, Vector2i::ONE))?;
                rect.size = VEC_CW_USEDEFAULT;
                rect
            }
            (None, Some(size)) => {
                let mut rect = style.client_to_window(Recti::new(Vector2i::ZERO, size))?;
                rect.position = VEC_CW_USEDEFAULT;
                rect
            }
            (None, None) => Recti::new(VEC_CW_USEDEFAULT, VEC_CW_USEDEFAULT),
        };

        let handle = unsafe {
            CreateWindowEx(
                style.ex_style,
                MAKEINTATOM!(class.handle()),
                title.as_ref().map(|title| title.as_ptr()).unwrap_or(null()),
                style.style,
                window_rect.position.x,
                window_rect.position.y,
                window_rect.size.x,
                window_rect.size.y,
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
