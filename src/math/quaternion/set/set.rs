use crate::math::Quaternion;

impl<T> Quaternion<T> {
    /// Sets the value at `i` to `v`
    pub fn set(&mut self, i: usize, v: T) {
        match self.get_ref_mut(i) {
            Some(value) => *value = v,
            None => panic!("index out of bounds: the len is 4 but the index is {}", i),
        }
    }
}
