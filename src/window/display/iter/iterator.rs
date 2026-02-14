use std::cell::Ref;

use crate::{
    Id,
    window::{Display, DisplayIter},
};

impl<'a> Iterator for DisplayIter<'a> {
    type Item = (Id<Display<'static>>, Display<'a>);

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
