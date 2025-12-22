use crate::Vector2;
use std::{
    marker::Destruct,
    ops::{Mul, Sub},
};

impl<T> Vector2<T> {
    /// Get the cross product of two [`Vector2`]s
    pub const fn cross(self, other: Self) -> T
    where
        T: [const] Sub<Output = T> + [const] Mul<Output = T> + [const] Destruct,
    {
        self.x * other.y - self.y * other.x
    }
}

#[cfg(test)]
mod tests {
    use crate::{Vector2f, number::ApproxEq};

    macro_rules! cross_tests {
        [$(
            $test_name: ident:
                ($i1x: literal, $i1y: literal),
                ($i2x: literal, $i2y: literal)
                -> $o: literal,
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT1: Vector2f = Vector2f::new($i1x, $i1y);
                const INPUT2: Vector2f = Vector2f::new($i2x, $i2y);
                const OUTPUT: f32 = $o;

                let output = INPUT1.cross(INPUT2);

                assert!(output.approx_eq(OUTPUT, 1e-6), "cross failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    cross_tests![
        cross_basis_xy_pos: (1.0, 0.0), (0.0, 1.0) -> 1.0,
        cross_basis_yx_neg: (0.0, 1.0), (1.0, 0.0) -> -1.0,

        cross_simple_int_neg2: (1.0, 2.0), (3.0, 4.0) -> -2.0,
        cross_simple_int_pos2: (3.0, 4.0), (1.0, 2.0) -> 2.0,

        cross_mixed_sign_pos31: (5.0, -7.0), (-2.0, 9.0) -> 31.0,
        cross_mixed_sign_neg31: (-2.0, 9.0), (5.0, -7.0) -> -31.0,

        cross_parallel_zero1: (2.0, 2.0), (4.0, 4.0) -> 0.0,
        cross_parallel_zero2: (2.0, 2.0), (-4.0, -4.0) -> 0.0,

        cross_axis_neg12: (-3.0, 0.0), (0.0, 4.0) -> -12.0,
        cross_axis_pos12: (0.0, 4.0), (-3.0, 0.0) -> 12.0,

        cross_decimals_pos675: (1.5, -2.0), (3.0, 0.5) -> 6.75,
        cross_decimals_neg675: (3.0, 0.5), (1.5, -2.0) -> -6.75,

        cross_decimals_neg5625: (-1.25, 2.5), (4.0, -3.5) -> -5.625,

        cross_large_small_neg3000000p000002: (1000.0, 0.001), (0.002, -3000.0) -> -3000000.000002,
        cross_tiny_pos0p00000038: (0.0003, -0.0004), (0.0005, 0.0006) -> 0.00000038,

        cross_realistic_pos13410p6204: (123.45, 67.89), (-98.76, 54.32) -> 13410.6204,
        cross_realistic_neg13410p6204: (-98.76, 54.32), (123.45, 67.89) -> -13410.6204,

        cross_swap_sign_pos5375: (-2.5, 3.0), (-0.75, -1.25) -> 5.375,
        cross_swap_sign_neg5375: (-0.75, -1.25), (-2.5, 3.0) -> -5.375,
    ];
}
