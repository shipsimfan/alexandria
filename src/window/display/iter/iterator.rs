use crate::{
    Id,
    window::{Display, DisplayIter},
};
use std::cell::Ref;

impl<'a, UserEvent: 'static + Send> Iterator for DisplayIter<'a, UserEvent> {
    type Item = (Id<Display<'static, UserEvent>>, Display<'a, UserEvent>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.r#ref.displays().len() {
            return None;
        }

        let id = self.r#ref.displays().key_at_index(self.index);
        let display = Display::new(self.index, Ref::clone(&self.r#ref));

        self.index += 1;

        Some((unsafe { id.cast() }, display))
    }
}
