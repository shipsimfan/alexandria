use crate::{
    Id,
    math::{Recti, Vector2i},
    window::{Display, DisplayIter, Window, WindowSubsystem},
};

impl<UserEvent: 'static + Send> WindowSubsystem<UserEvent> {
    /// Get the current number of displays available
    pub fn num_displays(&self) -> usize {
        self.inner.borrow().displays().len()
    }

    /// Get the set of currently active displays
    pub fn displays<'a>(&'a self) -> DisplayIter<'a, UserEvent> {
        DisplayIter::new(self.inner.borrow())
    }

    /// Get the [`Display`] with `id`
    pub fn display<'a>(
        &'a self,
        id: Id<Display<'static, UserEvent>>,
    ) -> Option<Display<'a, UserEvent>> {
        let inner = self.inner.borrow();
        inner
            .displays()
            .index_of(unsafe { id.cast() })
            .map(|index| Display::new(index, inner))
    }

    /// Find the primary display
    pub fn primary_display<'a>(
        &'a self,
    ) -> Option<(Id<Display<'static, UserEvent>>, Display<'a, UserEvent>)> {
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

    /// Find the display that contains the given `point`
    pub fn display_for_point<'a>(
        &'a self,
        point: Vector2i,
    ) -> Option<(Id<Display<'static, UserEvent>>, Display<'a, UserEvent>)> {
        for (id, display) in self.displays() {
            if display.rect().contains_point(&point) {
                return Some((id, display));
            }
        }
        None
    }

    /// Find the display that most closely intersects with the given `rect`
    pub fn display_for_rect<'a>(
        &'a self,
        rect: Recti,
    ) -> Option<(Id<Display<'static, UserEvent>>, Display<'a, UserEvent>)> {
        // TODO: Improve this to find greatest overlap
        self.display_for_point(rect.top_left())
    }

    /// Find the display that most closely intersects with the given `window`
    pub fn display_for_window<'a>(
        &'a self,
        window: &Window<UserEvent>,
    ) -> Option<(Id<Display<'static, UserEvent>>, Display<'a, UserEvent>)> {
        window
            .with_inner(|inner| self.display_for_rect(inner.rect()))
            .flatten()
    }
}
