use crate::{Id, PackedMapKeyValueIter};

impl<'a, T> Iterator for PackedMapKeyValueIter<'a, T> {
    type Item = (Id<T>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.values
            .next()
            .map(|value| (*self.keys.next().unwrap(), value))
    }
}
