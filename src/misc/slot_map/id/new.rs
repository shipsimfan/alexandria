use crate::Id;
use std::marker::PhantomData;

impl<T> Id<T> {
    /// Create a new [`Id`]
    pub(in crate::misc::slot_map) const fn new(index: usize, generation: u32) -> Id<T> {
        Id {
            index: index as u32,
            generation,
            _type: PhantomData,
        }
    }
}
