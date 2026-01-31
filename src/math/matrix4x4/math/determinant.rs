use crate::math::{
    Matrix4x4,
    number::{One, Zero},
};
use std::{
    marker::Destruct,
    ops::{Div, Mul, MulAssign, Neg, SubAssign},
};

impl<T: Zero + One> Matrix4x4<T> {
    /// Calculate the determinant of this matrix
    pub const fn determinant(mut self) -> T
    where
        T: [const] Mul<Output = T>
            + [const] Neg<Output = T>
            + [const] Div<Output = T>
            + [const] SubAssign
            + [const] MulAssign
            + [const] Clone
            + [const] PartialEq
            + [const] Destruct,
    {
        let mut sign = T::ONE;

        let mut i = 0;
        while i < 4 {
            if self[i][i] == T::ZERO {
                let mut found = None;
                let mut k = i;
                while k < 4 {
                    if self[k][i] != T::ZERO {
                        found = Some(k);
                        break;
                    }

                    k += 1;
                }

                match found {
                    Some(k) => {
                        self.swap_rows(i, k);
                        sign = -sign;
                    }
                    None => return T::ZERO,
                }
            }

            let mut j = i + 1;
            while j < 4 {
                let factor = self[j][i].clone() / self[i][i].clone();
                let mut k = i;
                while k < 4 {
                    let t = self[i][k].clone();
                    self[j][k] -= factor.clone() * t;

                    k += 1;
                }

                j += 1;
            }

            i += 1;
        }

        let mut det = self.r0.x;
        det *= self.r1.y;
        det *= self.r2.z;
        det *= self.r3.w;

        sign * det
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{Matrix4x4f, number::ApproxEq};

    macro_rules! determinant_tests {
        [$(
            $test_name: ident: ($i: expr) -> $o: literal,
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Matrix4x4f = Matrix4x4f::from_row_array($i);
                const OUTPUT: f32 = $o;

                let output = INPUT.determinant();

                assert!(output.approx_eq(OUTPUT, 1e-4), "determinant failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    determinant_tests![
        determinant_identity: (
            [[1.0, 0.0, 0.0, 0.0],
             [0.0, 1.0, 0.0, 0.0],
             [0.0, 0.0, 1.0, 0.0],
             [0.0, 0.0, 0.0, 1.0]]
        ) -> 1.0,

        determinant_zero: (
            [[0.0, 0.0, 0.0, 0.0],
             [0.0, 0.0, 0.0, 0.0],
             [0.0, 0.0, 0.0, 0.0],
             [0.0, 0.0, 0.0, 0.0]]
        ) -> 0.0,

        determinant_diagonal_2345: (
            [[2.0, 0.0, 0.0, 0.0],
             [0.0, 3.0, 0.0, 0.0],
             [0.0, 0.0, 4.0, 0.0],
             [0.0, 0.0, 0.0, 5.0]]
        ) -> 120.0,

        determinant_upper_triangular_mixed: (
            [[ 3.0, 1.0, 2.0, 4.0],
             [ 0.0,-2.0, 5.0, 1.0],
             [ 0.0, 0.0, 7.0, 3.0],
             [ 0.0, 0.0, 0.0, 6.0]]
        ) -> -252.0,

        determinant_row_swap_sign_flip: (
            [[0.0, 1.0, 0.0, 0.0],
             [1.0, 0.0, 0.0, 0.0],
             [0.0, 0.0, 1.0, 0.0],
             [0.0, 0.0, 0.0, 1.0]]
        ) -> -1.0,

        determinant_duplicate_rows_singular: (
            [[ 1.0,  2.0,  3.0,  4.0],
             [ 5.0,  6.0,  7.0,  8.0],
             [ 1.0,  2.0,  3.0,  4.0],
             [ 9.0, 10.0, 11.0, 12.0]]
        ) -> 0.0,

        determinant_affine_translation_doesnt_change_det: (
            [[2.0, 0.0, 0.0,  5.0],
             [0.0, 3.0, 0.0, -7.0],
             [0.0, 0.0, 4.0, 11.0],
             [0.0, 0.0, 0.0,  1.0]]
        ) -> 24.0,

        determinant_general_112: (
            [[ 1.0,  1.0, -5.0, -1.0],
             [ 3.0,  2.0,  1.0, -1.0],
             [ 2.0,  0.0,  4.0, -2.0],
             [ 3.0, -3.0, -1.0, -3.0]]
        ) -> 112.0,

        determinant_general_neg_27: (
            [[-4.0,  4.0, -1.0,  3.0],
             [ 4.0, -3.0, -1.0, -4.0],
             [-4.0,  5.0,  0.0,  2.0],
             [ 3.0, -4.0,  0.0,  1.0]]
        ) -> -27.0,

        determinant_general_72: (
            [[1.0, 2.0, 3.0, 4.0],
             [5.0, 6.0, 7.0, 8.0],
             [2.0, 6.0, 4.0, 8.0],
             [3.0, 1.0, 1.0, 2.0]]
        ) -> 72.0,
    ];
}
