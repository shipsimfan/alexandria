use crate::window::{Window, window::WindowInner};

impl<UserEvent: 'static + Send> Window<UserEvent> {
    /// Get the [`WindowInner`] this points to
    #[inline]
    pub(in crate::window) fn with_inner<R, F: FnOnce(&WindowInner<UserEvent>) -> R>(
        &self,
        f: F,
    ) -> Option<R> {
        self.context.with_window_inner(unsafe { self.id.cast() }, f)
    }
}
