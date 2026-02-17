use crate::{Id, PackedMap};

impl<T> PackedMap<T> {
    /// Get the index of the element with `id`
    pub fn index_of(&self, id: Id<T>) -> Option<usize> {
        if id.index() >= self.indices.len() {
            return None;
        }

        let target_index = self.indices[id.index()];
        if target_index >= self.values.len() {
            return None;
        }

        if self.ids[target_index].generation() != id.generation() {
            return None;
        }

        Some(target_index)
    }
}
