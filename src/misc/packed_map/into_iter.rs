use crate::PackedMap;

impl<T> PackedMap<T> {
    /// Create an iterator over this slot map yielding the elements
    pub fn into_iter(self) -> std::vec::IntoIter<T> {
        self.values.into_iter()
    }

    /// Create an iterator over this slot map yielding references to the elements
    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, T> {
        self.values.iter()
    }

    /// Create an iterator over this slot map yielding mutable references to the elements
    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, T> {
        self.values.iter_mut()
    }
}

impl<T> IntoIterator for PackedMap<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a PackedMap<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut PackedMap<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
