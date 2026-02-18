use crate::{
    Id,
    window::{Display, DisplayIter, WindowSubsystem},
};

impl WindowSubsystem {
    /// Get the current number of displays available
    pub fn num_displays(&self) -> usize {
        self.inner.borrow().displays().len()
    }

    /// Get the set of currently active displays
    pub fn displays<'a>(&'a self) -> DisplayIter<'a> {
        DisplayIter::new(self.inner.borrow())
    }

    /// Get the [`Display`] with `id`
    pub fn display<'a>(&'a self, id: Id<Display<'static>>) -> Option<Display<'a>> {
        let inner = self.inner.borrow();
        inner
            .displays()
            .index_of(unsafe { id.cast() })
            .map(|index| Display::new(index, inner))
    }

    /// Find the primary display
    pub fn primary_display<'a>(&'a self) -> Option<(Id<Display<'static>>, Display<'a>)> {
        let inner = self.inner.borrow();
        for i in 0..inner.displays().len() {
            let (id, display) = inner.displays().at_index(i);
            if display.is_primary() {
                return Some((unsafe { id.cast() }, Display::new(i, inner)));
            }
        }

        if inner.displays().len() > 0 {
            Some((
                unsafe { inner.displays().key_at_index(0).cast() },
                Display::new(0, inner),
            ))
        } else {
            None
        }
    }
}
