use crate::math::{
    Matrix3x3,
    number::{One, Zero},
};
use std::{
    marker::Destruct,
    ops::{Div, Mul, MulAssign, Neg, SubAssign},
};

impl<T: Zero + One> Matrix3x3<T> {
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
        while i < 3 {
            if self[i][i] == T::ZERO {
                let mut found = None;
                let mut k = i;
                while k < 3 {
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
            while j < 3 {
                let factor = self[j][i].clone() / self[i][i].clone();
                let mut k = i;
                while k < 3 {
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

        sign * det
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{Matrix3x3f, number::ApproxEq};

    macro_rules! determinant_tests {
        [$(
            $test_name: ident: ($i: expr) -> $o: literal,
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Matrix3x3f = Matrix3x3f::from_row_array($i);
                const OUTPUT: f32 = $o;

                let output = INPUT.determinant();

                assert!(output.approx_eq(OUTPUT, 1e-4), "determinant failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    determinant_tests![
        determinant_identity: ([[1.0, 0.0, 0.0],
                                [0.0, 1.0, 0.0],
                                [0.0, 0.0, 1.0]]) -> 1.0,

        determinant_zero: ([[0.0, 0.0, 0.0],
                            [0.0, 0.0, 0.0],
                            [0.0, 0.0, 0.0]]) -> 0.0,

        determinant_diag_neg: ([[-2.0, 0.0, 0.0],
                                [ 0.0, 3.0, 0.0],
                                [ 0.0, 0.0, 0.5]]) -> -3.0,

        determinant_upper_triangular: ([[ 2.0, 1.0, -3.0],
                                        [ 0.0,-4.0,  5.0],
                                        [ 0.0, 0.0,  6.0]]) -> -48.0,

        determinant_permutation_swap_0_1: ([[0.0, 1.0, 0.0],
                                           [1.0, 0.0, 0.0],
                                           [0.0, 0.0, 1.0]]) -> -1.0,

        determinant_singular_row_multiple: ([[1.0, 2.0, 3.0],
                                             [2.0, 4.0, 6.0],
                                             [7.0, 8.0, 9.0]]) -> 0.0,

        determinant_singular_equal_rows: ([[ 1.0,-1.0, 2.0],
                                           [ 1.0,-1.0, 2.0],
                                           [ 3.0, 0.0, 4.0]]) -> 0.0,

        determinant_general_1: ([[1.0, 2.0, 3.0],
                                 [0.0, 4.0, 5.0],
                                 [1.0, 0.0, 6.0]]) -> 22.0,

        determinant_general_2: ([[ 3.0,-1.0, 2.0],
                                 [ 2.0, 0.0,-2.0],
                                 [ 1.0, 4.0, 5.0]]) -> 52.0,

        determinant_general_fractional: ([[ 0.5, -1.5,  2.0],
                                          [ 3.0,  0.25, -4.0],
                                          [ 1.0,  2.0,  0.0]]) -> 21.5,

        determinant_unimodular_shear: ([[1.0, 2.0, 0.0],
                                        [0.0, 1.0, 0.0],
                                        [0.0, 0.0, 1.0]]) -> 1.0,

        determinant_diag_fraction: ([[ 0.25, 0.0, 0.0],
                                     [ 0.0,  2.0, 0.0],
                                     [ 0.0,  0.0,-4.0]]) -> -2.0,
    ];
}
