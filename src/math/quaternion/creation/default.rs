use crate::math::Quaternion;

const impl<T: [const] Default> Default for Quaternion<T> {
    fn default() -> Self {
        Quaternion::new(T::default(), T::default(), T::default(), T::default())
    }
}
