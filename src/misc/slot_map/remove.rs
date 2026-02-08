use crate::{Id, SlotMap};

impl<T> SlotMap<T> {
    /// Remove the element identified by `id`, returning it if it is contained
    pub fn remove(&mut self, id: Id<T>) -> Option<T> {
        self.slots[id.index()]
            .free(id, self.first_free)
            .map(|value| {
                self.len -= 1;
                self.first_free = Some(id.index());
                value
            })
    }
}
