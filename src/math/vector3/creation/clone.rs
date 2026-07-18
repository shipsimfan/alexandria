use crate::math::Vector3;

const impl<T: [const] Clone> Clone for Vector3<T> {
    fn clone(&self) -> Self {
        Vector3::new(self.x.clone(), self.y.clone(), self.z.clone())
    }
}

impl<T: Copy> Copy for Vector3<T> {}
