use crate::{Matrix4x4, Vector4};
use std::marker::Destruct;

impl<T> Matrix4x4<T> {
    /// Get the basis vector for the x-axis contained in this matrix
    pub const fn basis_x(self) -> Vector4<T>
    where
        T: [const] Destruct,
    {
        self.basis_vectors().0
    }

    /// Get the basis vector for the y-axis contained in this matrix
    pub const fn basis_y(self) -> Vector4<T>
    where
        T: [const] Destruct,
    {
        self.basis_vectors().1
    }

    /// Get the basis vector for the z-axis contained in this matrix
    pub const fn basis_z(self) -> Vector4<T>
    where
        T: [const] Destruct,
    {
        self.basis_vectors().2
    }

    /// Get the basis vector for the w-axis contained in this matrix
    pub const fn basis_w(self) -> Vector4<T>
    where
        T: [const] Destruct,
    {
        self.basis_vectors().3
    }

    /// Get the transformed basis vectors for the x, y, z, and w axes
    pub const fn basis_vectors(self) -> (Vector4<T>, Vector4<T>, Vector4<T>, Vector4<T>)
    where
        T: [const] Destruct,
    {
        self.into_vec4_col_tuple()
    }
}
