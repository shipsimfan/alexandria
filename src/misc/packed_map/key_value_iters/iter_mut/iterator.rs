use crate::{Id, PackedMapKeyValueIterMut};

impl<'a, T> Iterator for PackedMapKeyValueIterMut<'a, T> {
    type Item = (Id<T>, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        self.values
            .next()
            .map(|value| (*self.keys.next().unwrap(), value))
    }
}
