use crate::math::Vector3;
use std::{
    marker::Destruct,
    ops::{Add, Mul},
};

impl<T> Vector3<T> {
    /// Calculate the dot product between this [`Vector3`] and `rhs`
    pub const fn dot(self, rhs: Self) -> T
    where
        T: [const] Add<Output = T> + [const] Mul<Output = T> + [const] Destruct,
    {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{Vector3f, number::ApproxEq};

    macro_rules! dot_tests {
        [$(
            $test_name: ident:
                ($i1x: literal, $i1y: literal, $i1z: literal),
                ($i2x: literal, $i2y: literal, $i2z: literal)
                -> $o: literal,
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT1: Vector3f = Vector3f::new($i1x, $i1y, $i1z);
                const INPUT2: Vector3f = Vector3f::new($i2x, $i2y, $i2z);
                const OUTPUT: f32 = $o;

                let output = INPUT1.dot(INPUT2);

                assert!(output.approx_eq(OUTPUT, 1e-6), "dot failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    dot_tests![
        dot_zero_zero: (0.0, 0.0, 0.0), (0.0, 0.0, 0.0) -> 0.0,
        dot_zero_nonzero: (0.0, 0.0, 0.0), (3.0, -2.0, 5.0) -> 0.0,

        dot_unit_x_x: (1.0, 0.0, 0.0), (1.0, 0.0, 0.0) -> 1.0,
        dot_unit_x_y: (1.0, 0.0, 0.0), (0.0, 1.0, 0.0) -> 0.0,
        dot_unit_y_z: (0.0, 1.0, 0.0), (0.0, 0.0, 1.0) -> 0.0,
        dot_unit_z_negz: (0.0, 0.0, 1.0), (0.0, 0.0, -1.0) -> -1.0,

        dot_parallel_positive: (2.0, 4.0, 6.0), (1.0, 2.0, 3.0) -> 28.0,
        dot_parallel_negative: (2.0, 4.0, 6.0), (-1.0, -2.0, -3.0) -> -28.0,

        dot_mixed_signs_1: (3.0, -2.0, 5.0), (-7.0, 4.0, -1.0) -> -34.0,
        dot_mixed_signs_2: (-1.0, -2.0, -3.0), (4.0, 5.0, 6.0) -> -32.0,

        dot_fraction_halves: (0.5, -1.5, 2.0), (4.0, 0.5, -2.0) -> -2.75,
        dot_fraction_quarters: (0.25, 0.5, 0.75), (2.0, -4.0, 8.0) -> 4.5,
        dot_fraction_eighths: (0.125, -0.375, 0.5), (8.0, 4.0, -2.0) -> -1.5,

        dot_large_safe: (100000.0, -200000.0, 300000.0), (4000.0, 5000.0, -6000.0) -> -2400000000.0,
        dot_large_cancellation: (100000000.0, 100000000.0, 0.0), (1.0, -1.0, 0.0) -> 0.0,

        dot_commutativity_a: (9.0, -8.0, 7.0), (6.0, 5.0, -4.0) -> -14.0,
        dot_commutativity_b: (6.0, 5.0, -4.0), (9.0, -8.0, 7.0) -> -14.0,

        dot_self_positivity_1: (3.0, 4.0, 12.0), (3.0, 4.0, 12.0) -> 169.0,
        dot_self_positivity_2: (-5.0, 2.0, -1.0), (-5.0, 2.0, -1.0) -> 30.0,

        dot_axis_weighted: (10.0, 0.0, -10.0), (0.0, 3.0, 0.0) -> 0.0,
        dot_two_axes: (1.0, 2.0, 0.0), (3.0, 4.0, 0.0) -> 11.0,
        dot_three_axes: (1.0, 2.0, 3.0), (4.0, 5.0, 6.0) -> 32.0,
    ];
}
