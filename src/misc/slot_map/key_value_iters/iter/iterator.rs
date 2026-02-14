use crate::{Id, SlotMapKeyValueIter, misc::slot_map::Slot};

impl<'a, T> Iterator for SlotMapKeyValueIter<'a, T> {
    type Item = (Id<T>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((index, slot)) = self.iter.next() {
            match slot {
                Slot::Used { generation, value } => {
                    return Some((Id::new(index, *generation), value));
                }
                Slot::Free { .. } => {}
            }
        }

        None
    }
}
