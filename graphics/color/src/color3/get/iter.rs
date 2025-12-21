use crate::{Color3, ColorSpace};

impl<T, Space: ColorSpace<T>> Color3<T, Space> {
    /// Get an iterator over the channels of this color
    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, T> {
        self.as_slice().iter()
    }

    /// Get a mutable iterator over the channels of this color
    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, T> {
        self.as_mut_slice().iter_mut()
    }
}

impl<T, Space: ColorSpace<T>> IntoIterator for Color3<T, Space> {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, 3>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_array().into_iter()
    }
}

impl<'a, T, Space: ColorSpace<T>> IntoIterator for &'a Color3<T, Space> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, Space: ColorSpace<T>> IntoIterator for &'a mut Color3<T, Space> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
