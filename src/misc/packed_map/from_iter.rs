use crate::PackedMap;

impl<T> FromIterator<T> for PackedMap<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut packed_map = PackedMap::new();
        for value in iter {
            packed_map.insert(value);
        }
        packed_map
    }
}
