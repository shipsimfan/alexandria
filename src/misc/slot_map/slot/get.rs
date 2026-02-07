use crate::{Id, misc::slot_map::Slot};

impl<T> Slot<T> {
    /// Get a reference to the element contained in this slot, if there is one
    pub fn get(&self, id: Id<T>) -> Option<&T> {
        match self {
            Slot::Free { .. } => None,
            Slot::Used {
                generation, value, ..
            } => {
                if id.generation() == *generation {
                    Some(value)
                } else {
                    None
                }
            }
        }
    }

    /// Get a mutable reference the element contained in this slot, if there is one
    pub fn get_mut(&mut self, id: Id<T>) -> Option<&mut T> {
        match self {
            Slot::Free { .. } => None,
            Slot::Used {
                generation, value, ..
            } => {
                if id.generation() == *generation {
                    Some(value)
                } else {
                    None
                }
            }
        }
    }
}
