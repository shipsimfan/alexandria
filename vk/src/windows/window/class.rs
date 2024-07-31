use crate::WindowCreationError;
use std::ptr::null_mut;
use win32::{
    try_get_last_error, RegisterClassEx, UnregisterClass, ATOM, CS_HREDRAW, CS_OWNDC, CS_VREDRAW,
    WNDCLASSEXW, WNDPROC,
};

/// A description of a class of windows
pub(super) struct WindowClass(ATOM);

impl WindowClass {
    /// Creates a new [`WindowClass`]
    pub(super) fn new(wnd_proc: WNDPROC, class_name: &[u16]) -> Result<Self, WindowCreationError> {
        debug_assert_eq!(class_name.last(), Some(&0));

        let wnd_class = WNDCLASSEXW {
            style: CS_HREDRAW | CS_VREDRAW | CS_OWNDC,
            wnd_proc,
            class_name: class_name.as_ptr(),
            ..Default::default()
        };

        try_get_last_error!(RegisterClassEx(&wnd_class))
            .map(|atom| WindowClass(atom))
            .map_err(WindowCreationError::ClassCreationFailed)
    }

    /// Gets the underlying [`ATOM`]
    pub(super) fn atom(&self) -> ATOM {
        self.0
    }
}

impl Drop for WindowClass {
    fn drop(&mut self) {
        assert_ne!(unsafe { UnregisterClass(self.0 as _, null_mut()) }, 0);
    }
}
