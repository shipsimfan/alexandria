use crate::math::Matrix4x4;

impl<T> Matrix4x4<T> {
    /// Swap columns `a` and `b` in this matrix
    pub fn swap_cols(&mut self, a: usize, b: usize) {
        if a == b {
            return;
        }

        // SAFETY: The `if` statement above prevents borrow the same element twice
        let a = match unsafe { &mut *(self as *mut Self) }.col_ref_mut(a) {
            Some(a) => a,
            None => panic!("index out of bounds: the len is 4 but the index is {}", a),
        };

        let b = match self.col_ref_mut(b) {
            Some(b) => b,
            None => panic!("index out of bounds: the len is 4 but the index is {}", b),
        };

        for (a, b) in a.into_iter().zip(b) {
            std::mem::swap(a, b);
        }
    }
}
