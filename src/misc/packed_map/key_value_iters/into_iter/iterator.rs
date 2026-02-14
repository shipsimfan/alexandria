use crate::{Id, PackedMapKeyValueIntoIter};

impl<T> Iterator for PackedMapKeyValueIntoIter<T> {
    type Item = (Id<T>, T);

    fn next(&mut self) -> Option<Self::Item> {
        self.values
            .next()
            .map(|value| (self.keys.next().unwrap(), value))
    }
}
