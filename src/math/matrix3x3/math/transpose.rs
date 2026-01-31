use crate::math::{Matrix3x3, Vector3};
use std::marker::Destruct;

impl<T> Matrix3x3<T> {
    /// Transpose this matrix about the diagonal
    pub const fn transpose(self) -> Matrix3x3<T>
    where
        T: [const] Destruct,
    {
        Matrix3x3::new_rows(
            Vector3::new(self.r0.x, self.r1.x, self.r2.x),
            Vector3::new(self.r0.y, self.r1.y, self.r2.y),
            Vector3::new(self.r0.z, self.r1.z, self.r2.z),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::math::Matrix3x3f;

    macro_rules! tranpose_tests {
        [$(
            $test_name: ident: ($i: expr) -> $o: expr,
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Matrix3x3f = Matrix3x3f::from_row_array($i);
                const OUTPUT: Matrix3x3f = Matrix3x3f::from_row_array($o);

                let output = INPUT.transpose();

                assert!(output.approx_eq(OUTPUT, 1e-6), "transpose failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    tranpose_tests![
        transpose_identity: ([[1.0, 0.0, 0.0],
                              [0.0, 1.0, 0.0],
                              [0.0, 0.0, 1.0]]) -> [[1.0, 0.0, 0.0],
                                                   [0.0, 1.0, 0.0],
                                                   [0.0, 0.0, 1.0]],

        transpose_all_zeros: ([[0.0, 0.0, 0.0],
                               [0.0, 0.0, 0.0],
                               [0.0, 0.0, 0.0]]) -> [[0.0, 0.0, 0.0],
                                                    [0.0, 0.0, 0.0],
                                                    [0.0, 0.0, 0.0]],

        transpose_general_integers: ([[1.0, 2.0, 3.0],
                                      [4.0, 5.0, 6.0],
                                      [7.0, 8.0, 9.0]]) -> [[1.0, 4.0, 7.0],
                                                           [2.0, 5.0, 8.0],
                                                           [3.0, 6.0, 9.0]],

        transpose_with_negatives: ([[-1.0, -2.0, -3.0],
                                    [ 4.0,  5.0,  6.0],
                                    [-7.0,  8.0, -9.0]]) -> [[-1.0,  4.0, -7.0],
                                                             [-2.0,  5.0,  8.0],
                                                             [-3.0,  6.0, -9.0]],

        transpose_symmetric_unchanged: ([[ 2.0, -1.0,  4.0],
                                         [-1.0,  3.0,  0.0],
                                         [ 4.0,  0.0,  5.0]]) -> [[ 2.0, -1.0,  4.0],
                                                                  [-1.0,  3.0,  0.0],
                                                                  [ 4.0,  0.0,  5.0]],

        transpose_upper_triangular: ([[1.0, 2.0, 3.0],
                                      [0.0, 4.0, 5.0],
                                      [0.0, 0.0, 6.0]]) -> [[1.0, 0.0, 0.0],
                                                           [2.0, 4.0, 0.0],
                                                           [3.0, 5.0, 6.0]],

        transpose_lower_triangular: ([[7.0, 0.0, 0.0],
                                      [8.0, 9.0, 0.0],
                                      [1.0, 2.0, 3.0]]) -> [[7.0, 8.0, 1.0],
                                                           [0.0, 9.0, 2.0],
                                                           [0.0, 0.0, 3.0]],

        transpose_dyadic_fractions: ([[ 0.5,  -1.25,  2.75],
                                      [ 4.0,   0.25, -8.5 ],
                                      [-16.0, 32.0,  -0.75]]) -> [[ 0.5,   4.0,  -16.0],
                                                                  [-1.25,  0.25, 32.0 ],
                                                                  [ 2.75, -8.5,  -0.75]],

        transpose_repeated_values: ([[3.0, 3.0, 3.0],
                                     [1.0, 2.0, 1.0],
                                     [0.0, 0.0, 4.0]]) -> [[3.0, 1.0, 0.0],
                                                          [3.0, 2.0, 0.0],
                                                          [3.0, 1.0, 4.0]],

        transpose_large_finite: ([[ 65536.0, -32768.0, 16384.0],
                                  [     8.0,      4.0,     2.0],
                                  [    -1.0,      0.5,   -0.25]]) -> [[ 65536.0,     8.0,  -1.0],
                                                                     [-32768.0,     4.0,   0.5],
                                                                     [ 16384.0,     2.0,  -0.25]],
    ];
}
