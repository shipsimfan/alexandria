use crate::math::Quaternion;

const impl<T: [const] Clone> Clone for Quaternion<T> {
    fn clone(&self) -> Self {
        Quaternion::new(
            self.x.clone(),
            self.y.clone(),
            self.z.clone(),
            self.w.clone(),
        )
    }
}

impl<T: Copy> Copy for Quaternion<T> {}
