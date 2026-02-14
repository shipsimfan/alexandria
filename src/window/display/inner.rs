use crate::window::{Display, display::DisplayInner};

impl<'a> Display<'a> {
    /// Get the [`DisplayInner`] this points to
    #[inline]
    pub(in crate::window::display) fn inner(&self) -> &DisplayInner {
        self.r#ref.displays().value_at_index(self.index)
    }
}
