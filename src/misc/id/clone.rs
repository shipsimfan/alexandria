use crate::Id;
use std::marker::PhantomData;

impl<T> const Clone for Id<T> {
    fn clone(&self) -> Self {
        Id {
            index: self.index,
            generation: self.generation,
            _type: PhantomData,
        }
    }
}

impl<T> Copy for Id<T> {}
