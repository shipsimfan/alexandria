use crate::math::{Matrix3x3, Vector3};
use std::marker::Destruct;

impl<T> Matrix3x3<T> {
    /// Get the basis vector for the x-axis contained in this matrix
    pub const fn basis_x(self) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        self.basis_vectors().0
    }

    /// Get the basis vector for the y-axis contained in this matrix
    pub const fn basis_y(self) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        self.basis_vectors().1
    }

    /// Get the basis vector for the z-axis contained in this matrix
    pub const fn basis_z(self) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        self.basis_vectors().2
    }

    /// Get the transformed basis vectors for the x, y, and z axes
    pub const fn basis_vectors(self) -> (Vector3<T>, Vector3<T>, Vector3<T>)
    where
        T: [const] Destruct,
    {
        self.into_vec3_col_tuple()
    }
}
