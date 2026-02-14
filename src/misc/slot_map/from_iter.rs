use crate::SlotMap;

impl<T> FromIterator<T> for SlotMap<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut slot_map = SlotMap::new();
        for value in iter {
            slot_map.insert(value);
        }
        slot_map
    }
}
