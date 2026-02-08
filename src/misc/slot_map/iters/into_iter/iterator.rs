use crate::{SlotMapIntoIter, misc::slot_map::Slot};

impl<T> Iterator for SlotMapIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(slot) = self.iter.next() {
            if let Slot::Used { value, .. } = slot {
                return Some(value);
            }
        }
        None
    }
}
