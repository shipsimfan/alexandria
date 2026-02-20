use crate::{
    Id,
    window::{Window, WindowIter, WindowSubsystem, window::WindowInner},
};

impl<UserEvent: 'static + Send> WindowSubsystem<UserEvent> {
    /// Get the current number of windows created
    pub fn num_windows(&self) -> usize {
        self.inner.borrow().windows().len()
    }

    /// Get the set of currently active windows
    pub fn windows<'a>(&'a self) -> WindowIter<'a, UserEvent> {
        WindowIter::new(self.inner.borrow())
    }

    /// Get the [`Window`] with `id`
    pub fn window<'a>(&'a self, id: Id<Window<UserEvent>>) -> Option<Window<UserEvent>> {
        if self
            .inner
            .borrow()
            .windows()
            .get(unsafe { id.cast() })
            .is_some()
        {
            Some(Window::new(id, self.clone()))
        } else {
            None
        }
    }

    /// Get the [`WindowInner`] with `id`
    pub(in crate::window) fn with_window_inner<R, F: FnOnce(&WindowInner<UserEvent>) -> R>(
        &self,
        id: Id<WindowInner<UserEvent>>,
        f: F,
    ) -> Option<R> {
        self.inner.borrow().windows().get(id).map(f)
    }
}
