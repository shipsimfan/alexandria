use crate::{Id, PackedMap};

impl<T> PackedMap<T> {
    /// Converts `id` to the actual index of the value in `self.values`
    pub(in crate::misc::packed_map) fn convert_index(&self, id: Id<T>) -> Option<usize> {
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
