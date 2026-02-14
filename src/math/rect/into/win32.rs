use crate::math::Recti;

impl Into<win32::RECT> for Recti {
    fn into(self) -> win32::RECT {
        win32::RECT {
            left: self.left(),
            top: self.top(),
            right: self.right(),
            bottom: self.bottom(),
        }
    }
}
