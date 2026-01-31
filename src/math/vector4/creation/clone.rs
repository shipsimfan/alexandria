use crate::math::Vector4;

impl<T: [const] Clone> const Clone for Vector4<T> {
    fn clone(&self) -> Self {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.z.clone(),
            self.w.clone(),
        )
    }
}

impl<T: Copy> Copy for Vector4<T> {}
