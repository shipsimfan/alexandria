use crate::Vector4;
use std::{
    marker::Destruct,
    ops::{Add, Mul},
};

impl<T> Vector4<T> {
    /// Calculate the dot product between this [`Vector4`] and `rhs`
    pub const fn dot(self, rhs: Self) -> T
    where
        T: [const] Add<Output = T> + [const] Mul<Output = T> + [const] Destruct,
    {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
}

#[cfg(test)]
mod tests {
    use crate::{Vector4f, number::ApproxEq};

    macro_rules! dot_tests {
        [$(
            $test_name: ident:
                ($i1x: literal, $i1y: literal, $i1z: literal, $i1w: literal),
                ($i2x: literal, $i2y: literal, $i2z: literal, $i2w: literal)
                -> $o: literal,
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT1: Vector4f = Vector4f::new($i1x, $i1y, $i1z, $i1w);
                const INPUT2: Vector4f = Vector4f::new($i2x, $i2y, $i2z, $i2w);
                const OUTPUT: f32 = $o;

                let output = INPUT1.dot(INPUT2);

                assert!(output.approx_eq(OUTPUT, 1e-6), "dot failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    dot_tests![
        dot_zero_zero: (0.0, 0.0, 0.0, 0.0), (0.0, 0.0, 0.0, 0.0) -> 0.0,
        dot_zero_any: (0.0, 0.0, 0.0, 0.0), (1.5, -2.0, 3.0, 4.0) -> 0.0,
        dot_basis_x: (1.0, 0.0, 0.0, 0.0), (7.0, 8.0, 9.0, 10.0) -> 7.0,
        dot_basis_w: (0.0, 0.0, 0.0, 1.0), (-1.0, 2.0, -3.0, 4.0) -> 4.0,
        dot_ones_ones: (1.0, 1.0, 1.0, 1.0), (1.0, 1.0, 1.0, 1.0) -> 4.0,
        dot_ones_inc: (1.0, 1.0, 1.0, 1.0), (1.0, 2.0, 3.0, 4.0) -> 10.0,
        dot_neg_mix: (-1.0, 2.0, -3.0, 4.0), (5.0, -6.0, 7.0, -8.0) -> -70.0,
        dot_orthogonal_pairs: (1.0, 2.0, 3.0, 4.0), (4.0, -3.0, 2.0, -1.0) -> 0.0,
        dot_fractions: (0.5, -1.25, 2.5, -3.75), (4.0, 0.25, -1.5, 2.0) -> -9.5625,
        dot_large_small_precision: (10000000000.0, 0.0000000001, -10000000000.0, -0.0000000001), (3.0, 4.0, 5.0, 6.0) -> -19999997952.0,
        dot_underflow_to_zero: (0.00000000000000000000000000000000000001, -0.00000000000000000000000000000000000001, 0.00000000000000000000000000000000000002, -0.00000000000000000000000000000000000002), (10000000000.0, 10000000000.0, 10000000000.0, 10000000000.0) -> 0.0,
        dot_same_vector: (3.5, -2.0, 0.25, 8.0), (3.5, -2.0, 0.25, 8.0) -> 80.3125,
        dot_sign_cancel: (1.0, -1.0, 1.0, -1.0), (100.0, 100.0, 100.0, 100.0) -> 0.0,
        dot_primey: (2.0, 3.0, 5.0, 7.0), (11.0, 13.0, 17.0, 19.0) -> 279.0,
        dot_neg_zero: (-0.0, 0.0, -0.0, 0.0), (1.0, -1.0, 2.0, -2.0) -> 0.0,
    ];
}
