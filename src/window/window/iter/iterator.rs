use crate::{
    Id,
    window::{Window, WindowIter},
};

impl<'a, UserEvent: 'static + Send> Iterator for WindowIter<'a, UserEvent> {
    type Item = Id<Window<UserEvent>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.r#ref.windows().len() {
            return None;
        }

        let id = self.r#ref.windows().key_at_index(self.index);

        self.index += 1;

        Some(unsafe { id.cast() })
    }
}
