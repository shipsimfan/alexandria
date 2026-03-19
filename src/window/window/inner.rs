use crate::{
    PackedMap,
    window::{Window, display::DisplayInner, window::WindowInner},
};

impl<UserEvent: 'static + Send> Window<UserEvent> {
    /// Get the [`WindowInner`] this points to
    #[inline]
    pub(in crate::window) fn with_inner<R, F: FnOnce(&WindowInner<UserEvent>) -> R>(
        &self,
        f: F,
    ) -> Option<R> {
        self.context.with_window_inner(unsafe { self.id.cast() }, f)
    }

    /// Get the [`WindowInner`] this points to mutably
    #[inline]
    pub(in crate::window) fn with_inner_mut<
        F: FnOnce(&mut WindowInner<UserEvent>, &PackedMap<DisplayInner<UserEvent>>) -> R,
        R,
    >(
        &mut self,
        f: F,
    ) -> Option<R> {
        self.context
            .with_window_inner_mut(unsafe { self.id.cast() }, f)
    }
}
