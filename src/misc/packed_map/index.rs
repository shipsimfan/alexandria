use crate::{Id, PackedMap};
use std::ops::{Index, IndexMut};

impl<T> Index<Id<T>> for PackedMap<T> {
    type Output = T;

    fn index(&self, index: Id<T>) -> &Self::Output {
        match self.get(index) {
            Some(value) => value,
            None => panic!("index out of bounds: packed map id {} is not valid", index),
        }
    }
}

impl<T> IndexMut<Id<T>> for PackedMap<T> {
    fn index_mut(&mut self, index: Id<T>) -> &mut Self::Output {
        match self.get_mut(index) {
            Some(value) => value,
            None => panic!("index out of bounds: packed map id {} is not valid", index),
        }
    }
}
