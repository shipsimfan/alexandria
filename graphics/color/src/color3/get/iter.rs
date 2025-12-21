use crate::{Color3, ColorSpace};

impl<T, Space: ColorSpace<T>> Color3<T, Space> {
    /// Get an iterator over the channels of this color
    pub fn iter<'a>(&'a self) -> std::array::IntoIter<&'a T, 3> {
        [&self.r, &self.g, &self.b].into_iter()
    }

    /// Get a mutable iterator over the channels of this color
    pub fn iter_mut<'a>(&'a mut self) -> std::array::IntoIter<&'a mut T, 3> {
        [&mut self.r, &mut self.g, &mut self.b].into_iter()
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
    type IntoIter = std::array::IntoIter<&'a T, 3>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, Space: ColorSpace<T>> IntoIterator for &'a mut Color3<T, Space> {
    type Item = &'a mut T;
    type IntoIter = std::array::IntoIter<&'a mut T, 3>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
