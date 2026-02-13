use crate::math::Rect;

impl<T: [const] Clone> const Clone for Rect<T> {
    fn clone(&self) -> Self {
        Rect {
            position: self.position.clone(),
            size: self.size.clone(),
        }
    }
}

impl<T: Clone + Copy> Copy for Rect<T> {}
