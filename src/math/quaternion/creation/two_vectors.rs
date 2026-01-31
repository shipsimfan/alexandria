use crate::math::{
    Quaternion, Vector3,
    number::{Abs, ApproxEq, FromF32, One, Sqrt, Zero},
};
use std::ops::{Add, Div, Mul, Neg, Sub};

impl<
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Neg<Output = T>
        + Div<Output = T>
        + ApproxEq
        + Abs
        + Sqrt
        + Clone
        + PartialOrd
        + Zero
        + One,
> Quaternion<T>
{
    /// Create a new [`Quaternion`] representing the shortest arc rotation between `u` and `v`
    pub fn from_two_vectors(u: Vector3<T>, v: Vector3<T>) -> Quaternion<T>
    where
        T::Epsilon: FromF32 + Clone,
    {
        let dot = u.clone().dot(v.clone());
        let epsilon = T::Epsilon::from_f32(1e-6);

        if dot.clone().approx_eq(T::ONE, epsilon.clone()) {
            return Quaternion::IDENTITY;
        }

        if dot.clone().approx_eq(-T::ONE, epsilon) {
            let u_abs = u.clone().abs();

            let t = if u_abs.x < u_abs.y && u_abs.x < u_abs.z {
                Vector3::<T>::X
            } else if u_abs.y < u_abs.z {
                Vector3::<T>::Y
            } else {
                Vector3::<T>::Z
            };

            let cross = u.cross(t);
            let k = cross.clone() / cross.length();

            return Quaternion::new(k.x, k.y, k.z, T::ZERO);
        }

        let cross = u.cross(v);

        Quaternion::new(cross.x, cross.y, cross.z, T::ONE + dot)
    }
}
