use crate::WindowWakeHandle;

impl WindowWakeHandle {
    /// Wake the window thread if it is blocking
    pub fn wake(&self) {
        self.inner.wake();
    }
}
