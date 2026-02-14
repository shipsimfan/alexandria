use crate::Id;
use std::cmp::Ordering;

impl<T> const PartialOrd for Id<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> const Ord for Id<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.index.cmp(&other.index) {
            Ordering::Equal => self.generation.cmp(&other.generation),
            ordering => ordering,
        }
    }
}
