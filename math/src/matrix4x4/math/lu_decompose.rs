use crate::{
    Matrix4x4, Vector4,
    number::{Abs, ApproxEq, FromF32, One, Zero},
};
use std::{
    marker::Destruct,
    ops::{DivAssign, Mul, SubAssign},
};

impl<T: Zero + One> Matrix4x4<T> {
    /// Decompose this matrix using LU-decomposition into `LU` combined matrix and `P` permutation vector
    pub const fn lu_decompose(mut self) -> Option<(Matrix4x4<T>, Vector4<usize>)>
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
        let mut p = Vector4::new(0, 1, 2, 3);

        let mut k = 0;
        while k < 4 {
            // Choose pivot row
            let mut pivot_row = k;
            let mut pivot_value = self[k][k].clone().abs();
            let mut i = k + 1;
            while i < 4 {
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
            while i < 4 {
                let v = self[k][k].clone();
                self[i][k] /= v;

                let mut j = k + 1;
                while j < 4 {
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
    use crate::{Matrix4x4f, Vector4};

    macro_rules! lu_decompose_tests {
        [$(
            $test_name: ident: ($i: expr) -> ($om: expr, [$ovx: literal, $ovy: literal, $ovz: literal, $ovw: literal]),
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Matrix4x4f = Matrix4x4f::from_row_array($i);
                const OUTPUT: Matrix4x4f = Matrix4x4f::from_row_array($om);
                const PERMUTATIONS: Vector4<usize> = Vector4::new($ovx, $ovy, $ovz, $ovw);

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

    #[test]
    fn lu_decompose_singular_rank_deficient() {
        const INPUT: Matrix4x4f = Matrix4x4f::from_row_array([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 6.0, 8.0],
            [1.0, 0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0, 1.0],
        ]);

        assert!(INPUT.lu_decompose().is_none())
    }

    lu_decompose_tests![
        lu_decompose_identity: (
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        ) -> (
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            [0, 1, 2, 3]
        ),

        lu_decompose_no_pivot_symmetric: (
            [
                [4.0, 3.0, 2.0, 1.0],
                [3.0, 4.0, 3.0, 2.0],
                [2.0, 3.0, 4.0, 3.0],
                [1.0, 2.0, 3.0, 4.0],
            ]
        ) -> (
            [
                [4.0, 3.0, 2.0, 1.0],
                [0.75, 1.75, 1.5, 1.25],
                [0.5, 0.857142857, 1.71428571, 1.42857143],
                [0.25, 0.714285714, 0.833333333, 1.66666667],
            ],
            [0, 1, 2, 3]
        ),

        lu_decompose_pivot_first_column: (
            [
                [0.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [1.0, 0.0, 4.0, 2.0],
                [2.0, 1.0, 3.0, 0.0],
            ]
        ) -> (
            [
                [5.0, 6.0, 7.0, 8.0],
                [0.0, 2.0, 3.0, 4.0],
                [0.2, -0.6, 4.4, 2.8],
                [0.4, -0.7, 0.522727273, -1.86363636],
            ],
            [1, 0, 2, 3]
        ),

        lu_decompose_multi_pivot_negatives: (
            [
                [-2.0, 1.0, 0.0, 3.0],
                [4.0, -1.0, 2.0, 1.0],
                [0.0, 5.0, -3.0, 2.0],
                [1.0, 2.0, 1.0, -1.0],
            ]
        ) -> (
            [
                [4.0, -1.0, 2.0, 1.0],
                [0.0, 5.0, -3.0, 2.0],
                [0.25, 0.45, 1.85, -2.15],
                [-0.5, 0.1, 0.702702703, 4.81081081],
            ],
            [1, 2, 3, 0]
        ),
    ];
}
