use crate::Vector3;

impl<T: [const] Default> const Default for Vector3<T> {
    fn default() -> Self {
        Vector3 {
            x: T::default(),
            y: T::default(),
            z: T::default(),
        }
    }
}
