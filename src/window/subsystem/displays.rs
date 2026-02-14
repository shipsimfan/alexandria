use crate::window::{DisplayIter, WindowSubsystem};

impl WindowSubsystem {
    /// Get the set of currently active displays
    pub fn displays<'a>(&'a self) -> DisplayIter<'a> {
        DisplayIter::new(self.inner.borrow())
    }
}
