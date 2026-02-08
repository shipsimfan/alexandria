use crate::{Id, SlotMap, misc::slot_map::Slot};

impl<T> SlotMap<T> {
    /// Insert a new element into this map
    pub fn insert(&mut self, value: T) -> Id<T> {
        self.len += 1;
        match self.first_free {
            Some(index) => {
                let (generation, next_free) = self.slots[index].set(value);
                self.first_free = next_free;
                Id::new(index, generation)
            }
            None => {
                self.slots.push(Slot::Used {
                    generation: 0,
                    value,
                });
                Id::new(self.slots.len() - 1, 0)
            }
        }
    }
}
