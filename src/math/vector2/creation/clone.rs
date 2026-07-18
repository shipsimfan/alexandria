use crate::math::Vector2;

const impl<T: [const] Clone> Clone for Vector2<T> {
    fn clone(&self) -> Self {
        Vector2::new(self.x.clone(), self.y.clone())
    }
}

impl<T: Copy> Copy for Vector2<T> {}
