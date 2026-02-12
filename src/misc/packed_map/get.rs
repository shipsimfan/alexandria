use crate::{Id, PackedMap};

impl<T> PackedMap<T> {
    /// Get a reference to the element identified by `id`
    pub fn get(&self, id: Id<T>) -> Option<&T> {
        self.convert_index(id).map(|index| &self.values[index])
    }

    /// Get a mutable reference to the element identified by `id`
    pub fn get_mut(&mut self, id: Id<T>) -> Option<&mut T> {
        self.convert_index(id).map(|index| &mut self.values[index])
    }
}
