use crate::math::{Vector3, number::Sqrt};
use std::ops::{Add, Mul, Sub};

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Sqrt + Clone> Vector3<T> {
    /// Calculate the distance between this vector and `rhs`
    pub fn distance(self, rhs: Self) -> T {
        self.distance_squared(rhs).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{Vector3f, number::ApproxEq};

    macro_rules! distance_tests {
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

                let output = INPUT1.distance(INPUT2);

                assert!(output.approx_eq(OUTPUT, 1e-6), "distance failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    distance_tests![
        distance_zero: (0.0, 0.0, 0.0), (0.0, 0.0, 0.0) -> 0.0,
        distance_unit_x: (0.0, 0.0, 0.0), (1.0, 0.0, 0.0) -> 1.0,
        distance_unit_y: (0.0, 0.0, 0.0), (0.0, 1.0, 0.0) -> 1.0,
        distance_unit_z: (0.0, 0.0, 0.0), (0.0, 0.0, 1.0) -> 1.0,

        distance_axis_3_4_0: (0.0, 0.0, 0.0), (3.0, 4.0, 0.0) -> 5.0,
        distance_axis_5_12_0: (0.0, 0.0, 0.0), (5.0, 12.0, 0.0) -> 13.0,

        distance_diag_1_1_1: (0.0, 0.0, 0.0), (1.0, 1.0, 1.0) -> 1.732050808,
        distance_diag_2_3_6: (0.0, 0.0, 0.0), (2.0, 3.0, 6.0) -> 7.0,

        distance_neg_to_pos: (-1.0, -2.0, -3.0), (4.0, 0.0, 3.0) -> 8.062257748,
        distance_swap_symmetry: (4.0, 0.0, 3.0), (-1.0, -2.0, -3.0) -> 8.062257748,

        distance_same_point_nonzero: (7.5, -2.25, 100.0), (7.5, -2.25, 100.0) -> 0.0,
        distance_small_offsets: (0.001, -0.002, 0.003), (0.0, 0.0, 0.0) -> 0.003741657,

        distance_fractional: (-1.5, 2.25, -3.75), (4.125, -0.5, 1.25) -> 8.012685255,

        distance_large_opposite: (10000.0, 20000.0, -30000.0), (-10000.0, -20000.0, 30000.0) -> 74833.147735479,

        distance_mixed_signs: (-8.0, 15.0, 0.0), (0.0, 0.0, 20.0) -> 26.248809497,
        distance_two_axes: (9.0, -12.0, 5.0), (9.0, 3.0, 5.0) -> 15.0,
        distance_z_only: (-2.0, 7.0, 11.0), (-2.0, 7.0, -13.0) -> 24.0,

        distance_nontrivial_int: (1.0, 2.0, 3.0), (4.0, 6.0, 8.0) -> 7.071067812,
        distance_power_of_two: (16.0, 32.0, 64.0), (8.0, 16.0, 32.0) -> 36.660605560,
        distance_colinear: (-10.0, -10.0, -10.0), (10.0, 10.0, 10.0) -> 34.641016151,
    ];
}
