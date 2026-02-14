use crate::{PackedMap, PackedMapKeyValueIntoIter};

impl<T> PackedMapKeyValueIntoIter<T> {
    /// Create a new [`PackedMapKeyValueIntoIter`] over `packed_map`
    pub(in crate::misc::packed_map) fn new(
        packed_map: PackedMap<T>,
    ) -> PackedMapKeyValueIntoIter<T> {
        PackedMapKeyValueIntoIter {
            keys: packed_map.ids.into_iter(),
            values: packed_map.values.into_iter(),
        }
    }
}
