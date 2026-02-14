use crate::{Id, SlotMapKeyValueIntoIter, misc::slot_map::Slot};

impl<T> Iterator for SlotMapKeyValueIntoIter<T> {
    type Item = (Id<T>, T);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((index, slot)) = self.iter.next() {
            match slot {
                Slot::Used { generation, value } => {
                    return Some((Id::new(index, generation), value));
                }
                Slot::Free { .. } => {}
            }
        }

        None
    }
}
