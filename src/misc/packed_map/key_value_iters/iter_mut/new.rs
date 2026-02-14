use crate::{PackedMap, PackedMapKeyValueIterMut};

impl<'a, T> PackedMapKeyValueIterMut<'a, T> {
    /// Create a new [`PackedMapKeyValueIterMut`] over `packed_map`
    pub(in crate::misc::packed_map) fn new(
        packed_map: &'a mut PackedMap<T>,
    ) -> PackedMapKeyValueIterMut<'a, T> {
        PackedMapKeyValueIterMut {
            keys: packed_map.ids.iter(),
            values: packed_map.values.iter_mut(),
        }
    }
}
