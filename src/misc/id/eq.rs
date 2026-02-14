use crate::Id;

impl<T> const PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.generation == other.generation
    }
}

impl<T> const Eq for Id<T> {}
