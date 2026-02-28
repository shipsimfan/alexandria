use crate::{math::Vector2, window::DisplayOrientation};

impl DisplayOrientation {
    /// Rotate a [`Vector2`] by this [`DisplayOrientation`]
    pub fn rotate<T>(&self, v: Vector2<T>) -> Vector2<T> {
        match self {
            DisplayOrientation::Landscape | DisplayOrientation::LandscapeFlipped => v,
            DisplayOrientation::Portrait | DisplayOrientation::PortraitFlipped => {
                Vector2::new(v.y, v.x)
            }
        }
    }
}
