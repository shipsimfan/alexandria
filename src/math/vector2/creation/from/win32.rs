use crate::math::Vector2i;
use win32::POINT;

impl From<POINT> for Vector2i {
    fn from(value: POINT) -> Self {
        Vector2i::new(value.x, value.y)
    }
}
