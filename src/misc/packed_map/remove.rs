use crate::{Id, PackedMap};

impl<T> PackedMap<T> {
    /// Remove the element identified by `id`, returning it if it is contained
    pub fn remove(&mut self, id: Id<T>) -> Option<T> {
        let data_id = self.index_of(id)?;
        let last_data_id = self.values.len() - 1;
        let last_id = self.ids[last_data_id].index();

        self.values.swap(data_id, last_data_id);
        self.ids.swap(data_id, last_data_id);
        self.indices.swap(id.index(), last_id);
        self.values.pop()
    }
}
