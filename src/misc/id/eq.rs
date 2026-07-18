use crate::Id;

const impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.generation == other.generation
    }
}

const impl<T> Eq for Id<T> {}
