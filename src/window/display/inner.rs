use crate::window::{Display, display::DisplayInner};

impl<'a, UserEvent: 'static + Send> Display<'a, UserEvent> {
    /// Get the [`DisplayInner`] this points to
    #[inline]
    pub(in crate::window::display) fn inner(&self) -> &DisplayInner {
        self.r#ref.displays().value_at_index(self.index)
    }
}
