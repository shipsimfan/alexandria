use crate::window::{WindowClass, WindowProc, window::windows::class::wnd_proc_trampoline};
use std::{marker::PhantomData, ptr::null, rc::Rc};
use win32::{GetModuleHandle, RegisterClassEx, WNDCLASSEX};

impl<T: WindowProc> WindowClass<T> {
    /// Register a new window class
    pub fn register(class_name: &str, style: u32) -> win32::Result<Rc<WindowClass<T>>> {
        let class_name: Vec<_> = class_name.encode_utf16().chain([0]).collect();

        let wnd_class = WNDCLASSEX {
            style,
            wnd_proc: wnd_proc_trampoline::<T>,
            instance: unsafe { GetModuleHandle(null()) },
            class_name: class_name.as_ptr(),
            ..Default::default()
        };

        let handle = unsafe { RegisterClassEx(&wnd_class) };
        if handle == 0 {
            return Err(win32::Error::get_last_error());
        }

        Ok(Rc::new(WindowClass {
            handle,
            _user_data: PhantomData,
        }))
    }
}
