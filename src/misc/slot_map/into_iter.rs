use crate::{SlotMap, SlotMapIntoIter, SlotMapIter, SlotMapIterMut};

impl<T> SlotMap<T> {
    /// Create an iterator over this slot map yielding the elements
    pub fn into_iter(self) -> SlotMapIntoIter<T> {
        SlotMapIntoIter::new(self)
    }

    /// Create an iterator over this slot map yielding references to the elements
    pub fn iter<'a>(&'a self) -> SlotMapIter<'a, T> {
        SlotMapIter::new(self)
    }

    /// Create an iterator over this slot map yielding mutable references to the elements
    pub fn iter_mut<'a>(&'a mut self) -> SlotMapIterMut<'a, T> {
        SlotMapIterMut::new(self)
    }
}

impl<T> IntoIterator for SlotMap<T> {
    type Item = T;
    type IntoIter = SlotMapIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a SlotMap<T> {
    type Item = &'a T;
    type IntoIter = SlotMapIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut SlotMap<T> {
    type Item = &'a mut T;
    type IntoIter = SlotMapIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
