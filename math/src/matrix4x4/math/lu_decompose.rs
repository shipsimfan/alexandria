use std::marker::Destruct;

use crate::{
    Matrix4x4, Vector4,
    number::{Abs, ApproxEq, FromF32, One, Zero},
};

impl<T: Zero + One> Matrix4x4<T> {
    /// Decompose this matrix using LU-decomposition into `LU` combined matrix and `P` permutation vector
    pub const fn lu_decompose(mut self) -> Option<(Matrix4x4<T>, Vector4<T>)>
    where
        T: [const] Abs + [const] ApproxEq + [const] Clone + [const] PartialOrd + [const] Destruct,
        T::Epsilon: [const] FromF32,
    {
        let mut p = Matrix4x4::<T>::IDENTITY;
        let mut l = Matrix4x4::<T>::ZERO;

        let mut k = 0;
        while k < 4 {
            // Pivoting
            let mut pivot_row = k;
            let mut max = self[k][k].clone().abs();
            let mut i = k + 1;
            while i < 4 {
                let abs = self[i][k].clone().abs();
                if abs > max {
                    pivot_row = i;
                    max = abs;
                }

                i += 1;
            }

            // If the pivot row value is close to zero, the matrix is singular
            if max.approx_eq(T::ZERO, T::Epsilon::from_f32(1e-4)) {
                return None;
            }

            // Pivot if needed
            if pivot_row != k {
                self.swap_rows(pivot_row, k);
                p.swap_rows(pivot_row, k);
                let mut j = 0;
                while j < k {
                    let tmp = l[pivot_row][j].clone();
                    l[pivot_row][j] = l[k][j].clone();
                    l[k][j] = tmp;

                    j += 1;
                }
            }

            // Calculate LU

            k += 1;
        }

        todo!()
    }
}
