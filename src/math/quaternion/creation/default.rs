use crate::math::Quaternion;

impl<T: [const] Default> const Default for Quaternion<T> {
    fn default() -> Self {
        Quaternion::new(T::default(), T::default(), T::default(), T::default())
    }
}
