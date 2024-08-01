use class::WindowClass;
use std::ptr::null_mut;
use win32::{
    DestroyWindow, DispatchMessage, PeekMessage, TranslateMessage, HWND, MSG, PM_REMOVE, WM_QUIT,
};
use wnd_proc::wnd_proc;

mod class;
mod new;
mod wnd_proc;

pub use new::WindowCreationError;

use crate::Surface;

/// A visible window
pub struct Window {
    /// The class the window is a part of
    #[allow(unused)]
    class: WindowClass,

    /// Should this window continue to run?
    should_run: bool,

    /// The surface that allows rendering to this window
    surface: Surface,

    /// The handle to the window
    wnd: HWND,

    /// A buffer to hold the next poll message
    msg: MSG,
}

impl Window {
    /// Polls events for the window, returning if the window should continue to run
    pub fn poll_events(&mut self) -> bool {
        while unsafe { PeekMessage(&mut self.msg, null_mut(), 0, 0, PM_REMOVE) } != 0 {
            if self.msg.message == WM_QUIT {
                self.should_run = false;
            }

            unsafe { TranslateMessage(&self.msg) };
            unsafe { DispatchMessage(&self.msg) };
        }

        self.should_run
    }

    /// Signals the window to exit at the next event loop
    pub fn exit(&mut self) {
        self.should_run = false;
    }

    /// Gets the surface for rendering to this window
    pub fn surface(&self) -> &Surface {
        &self.surface
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe { DestroyWindow(self.wnd) };
    }
}

impl !Send for Window {}
impl !Sync for Window {}
