use super::{wnd_proc, Window, WindowClass};
use crate::{Instance, Surface};
use std::{
    ptr::{null, null_mut},
    sync::Arc,
};
use vulkan::VkWin32SurfaceCreateInfoKHR;
use win32::{
    try_get_last_error, CreateWindowEx, SetWindowLongPtr, CW_USEDEFAULT, GWLP_USERDATA, MSG,
    WS_CAPTION, WS_MINIMIZEBOX, WS_SYSMENU, WS_VISIBLE,
};

mod error;

pub use error::WindowCreationError;

impl Window {
    /// Creates a new [`Window`]
    pub fn new(
        instance: &Arc<Instance>,
        title: &str,
        width: usize,
        height: usize,
    ) -> Result<Box<Self>, WindowCreationError> {
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
        .map_err(WindowCreationError::WindowCreationFailed)?;

        let surface = Surface::new(
            instance.f().ws().unwrap().create_win32_surface(
                instance.handle(),
                &VkWin32SurfaceCreateInfoKHR {
                    hwnd: wnd,
                    ..Default::default()
                },
                null(),
            )?,
            instance.clone(),
        );

        let window = Box::new(Window {
            class,
            should_run: true,
            surface,
            wnd,
            msg: MSG::default(),
        });

        unsafe { SetWindowLongPtr(wnd, GWLP_USERDATA, &*window as *const Self as _) };

        Ok(window)
    }
}
