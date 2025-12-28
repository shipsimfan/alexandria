use crate::{
    Matrix3x3,
    number::{ApproxEq, One, Zero},
};
use std::{
    marker::Destruct,
    ops::{Add, Mul},
};

impl<T: Zero + One> Matrix3x3<T> {
    /// Is this matrix orthonormal?
    pub const fn is_orthonormal(self, epsilon: T::Epsilon) -> bool
    where
        T: [const] Add<Output = T>
            + [const] Mul<Output = T>
            + [const] ApproxEq
            + [const] Clone
            + [const] Destruct,
        T::Epsilon: [const] Clone + [const] Destruct,
    {
        let (right, up, forward) = self.basis_vectors();
        right
            .clone()
            .length_squared()
            .approx_eq(T::ONE, epsilon.clone())
            && up
                .clone()
                .length_squared()
                .approx_eq(T::ONE, epsilon.clone())
            && forward
                .clone()
                .length_squared()
                .approx_eq(T::ONE, epsilon.clone())
            && right
                .clone()
                .dot(up.clone())
                .approx_eq(T::ZERO, epsilon.clone())
            && right
                .dot(forward.clone())
                .approx_eq(T::ZERO, epsilon.clone())
            && up.dot(forward).approx_eq(T::ZERO, epsilon)
    }
}
