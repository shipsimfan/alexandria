use crate::graphics::color::{Color4, ColorSpace};

impl<T, Space: ColorSpace<T>> Color4<T, Space> {
    /// Get an iterator over the channels of this color
    pub fn iter<'a>(&'a self) -> std::array::IntoIter<&'a T, 4> {
        [&self.r, &self.g, &self.b, &self.a].into_iter()
    }

    /// Get a mutable iterator over the channels of this color
    pub fn iter_mut<'a>(&'a mut self) -> std::array::IntoIter<&'a mut T, 4> {
        [&mut self.r, &mut self.g, &mut self.b, &mut self.a].into_iter()
    }
}

impl<T, Space: ColorSpace<T>> IntoIterator for Color4<T, Space> {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, 4>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_array().into_iter()
    }
}

impl<'a, T, Space: ColorSpace<T>> IntoIterator for &'a Color4<T, Space> {
    type Item = &'a T;
    type IntoIter = std::array::IntoIter<&'a T, 4>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, Space: ColorSpace<T>> IntoIterator for &'a mut Color4<T, Space> {
    type Item = &'a mut T;
    type IntoIter = std::array::IntoIter<&'a mut T, 4>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
