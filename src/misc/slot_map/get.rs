use crate::{Id, SlotMap};

impl<T> SlotMap<T> {
    /// Get a reference to the element identified by `id`
    pub fn get(&self, id: Id<T>) -> Option<&T> {
        self.slots[id.index()].get(id)
    }

    /// Get a mutable reference to the element identified by `id`
    pub fn get_mut(&mut self, id: Id<T>) -> Option<&mut T> {
        self.slots[id.index()].get_mut(id)
    }
}
