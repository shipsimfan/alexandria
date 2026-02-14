use crate::{PackedMap, PackedMapKeyValueIter};

impl<'a, T> PackedMapKeyValueIter<'a, T> {
    /// Create a new [`PackedMapKeyValueIter`] over `packed_map`
    pub(in crate::misc::packed_map) fn new(
        packed_map: &'a PackedMap<T>,
    ) -> PackedMapKeyValueIter<'a, T> {
        PackedMapKeyValueIter {
            keys: packed_map.ids.iter(),
            values: packed_map.values.iter(),
        }
    }
}
