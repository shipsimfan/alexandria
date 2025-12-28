use crate::{
    Matrix3x3, Vector3,
    number::{Abs, ApproxEq, FromF32, One, Zero},
};
use std::{
    marker::Destruct,
    ops::{DivAssign, Mul, SubAssign},
};

impl<T: Zero + One> Matrix3x3<T> {
    /// Decompose this matrix using LU-decomposition into `LU` combined matrix and `P` permutation vector
    pub const fn lu_decompose(mut self) -> Option<(Matrix3x3<T>, Vector3<usize>)>
    where
        T: [const] Mul<Output = T>
            + [const] SubAssign
            + [const] DivAssign
            + [const] Abs
            + [const] ApproxEq
            + [const] FromF32
            + [const] Clone
            + [const] PartialOrd
            + [const] Destruct,
        T::Epsilon: [const] FromF32,
    {
        let mut p = Vector3::new(0, 1, 2);

        let mut k = 0;
        while k < 3 {
            // Choose pivot row
            let mut pivot_row = k;
            let mut pivot_value = self[k][k].clone().abs();
            let mut i = k + 1;
            while i < 3 {
                let value = self[i][k].clone().abs();
                if value > pivot_value {
                    pivot_value = value;
                    pivot_row = i;
                }

                i += 1;
            }

            // Check pivot point for singularity
            if pivot_value.approx_eq(T::ZERO, T::Epsilon::from_f32(1e-12)) {
                return None;
            }

            // Swap rows if needed
            if pivot_row != k {
                self.swap_rows(k, pivot_row);
                p.swap(k, pivot_row);
            }

            // Eliminate and store multipliers
            let mut i = k + 1;
            while i < 3 {
                let v = self[k][k].clone();
                self[i][k] /= v;

                let mut j = k + 1;
                while j < 3 {
                    let v1 = self[i][k].clone();
                    let v2 = self[k][j].clone();
                    self[i][j] -= v1 * v2;

                    j += 1;
                }

                i += 1;
            }

            k += 1;
        }

        Some((self, p))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Matrix3x3f, Vector3};

    macro_rules! lu_decompose_tests {
        [$(
            $test_name: ident: ($i: expr) -> ($om: expr, [$ovx: literal, $ovy: literal, $ovz: literal]),
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Matrix3x3f = Matrix3x3f::from_row_array($i);
                const OUTPUT: Matrix3x3f = Matrix3x3f::from_row_array($om);
                const PERMUTATIONS: Vector3<usize> = Vector3::new($ovx, $ovy, $ovz);

                let (output, permutations) = INPUT.lu_decompose().unwrap();

                assert!(
                    output.approx_eq(OUTPUT, 1e-6) && permutations == PERMUTATIONS,
                    "lu decompose failed: ({}, {}) vs. ({}, {})",
                    output,
                    permutations,
                    OUTPUT,
                    PERMUTATIONS
                );
            }
        )*};
    }

    lu_decompose_tests![
        lu_decompose_identity: (
          [[1.0, 0.0, 0.0],
           [0.0, 1.0, 0.0],
           [0.0, 0.0, 1.0]]
        ) -> (
          [[1.0, 0.0, 0.0],
           [0.0, 1.0, 0.0],
           [0.0, 0.0, 1.0]],
          [0, 1, 2]
        ),

        lu_decompose_upper_triangular_no_pivot: (
          [[2.0, -1.0,  3.0],
           [0.0,  4.0,  5.0],
           [0.0,  0.0, -6.0]]
        ) -> (
          [[2.0, -1.0,  3.0],
           [0.0,  4.0,  5.0],
           [0.0,  0.0, -6.0]],
          [0, 1, 2]
        ),

        lu_decompose_spd_tridiagonal_no_pivot: (
          [[4.0, 2.0, 0.0],
           [2.0, 4.0, 2.0],
           [0.0, 2.0, 4.0]]
        ) -> (
          [[4.0,       2.0,       0.0],
           [0.5,       3.0,       2.0],
           [0.0,       0.6666667, 2.6666667]],
          [0, 1, 2]
        ),

        lu_decompose_pivot_first_step_classic: (
          [[ 2.0,  1.0, 1.0],
           [ 4.0, -6.0, 0.0],
           [-2.0,  7.0, 2.0]]
        ) -> (
          [[ 4.0, -6.0, 0.0],
           [ 0.5,  4.0, 1.0],
           [-0.5,  1.0, 1.0]],
          [1, 0, 2]
        ),

        lu_decompose_pivot_second_step_only: (
          [[1.0, 5.0, 5.0],
           [1.0, 5.0, 3.0],
           [0.0, 3.0, 2.0]]
        ) -> (
          [[1.0, 5.0,  5.0],
           [0.0, 3.0,  2.0],
           [1.0, 0.0, -2.0]],
          [0, 2, 1]
        ),
    ];
}
