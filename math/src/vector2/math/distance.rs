use crate::{Vector2, number::Sqrt};
use std::ops::{Add, Mul, Sub};

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Sqrt + Clone> Vector2<T> {
    /// Calculate the distance between this vector and `rhs`
    pub fn distance(self, rhs: Self) -> T {
        self.distance_squared(rhs).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Vector2f, number::ApproxEq};

    macro_rules! distance_tests {
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

                let output = INPUT1.distance(INPUT2);

                assert!(output.approx_eq(OUTPUT, 1e-6), "distance failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    distance_tests![
        distance_zero: (0.0, 0.0), (0.0, 0.0) -> 0.0,
        distance_same_point_nonzero_coords: (123.0, -456.0), (123.0, -456.0) -> 0.0,

        distance_axis_x_positive: (0.0, 0.0), (7.0, 0.0) -> 7.0,
        distance_axis_y_positive: (2.0, -3.0), (2.0, 5.0) -> 8.0,

        distance_pythag_3_4_5: (1.0, 2.0), (4.0, 6.0) -> 5.0,
        distance_pythag_3_4_5_negative_coords: (-1.0, -1.0), (2.0, 3.0) -> 5.0,
        distance_symmetry_swapped_inputs: (4.0, 6.0), (1.0, 2.0) -> 5.0,

        distance_pythag_6_8_10: (10.0, 10.0), (16.0, 18.0) -> 10.0,
        distance_pythag_5_12_13: (-5.0, 0.0), (0.0, 12.0) -> 13.0,
        distance_pythag_8_15_17: (3.0, -4.0), (11.0, 11.0) -> 17.0,

        distance_scaled_quarter_3_4_5: (0.0, 0.0), (0.75, 1.0) -> 1.25,
        distance_scaled_half_3_4_5: (-1.5, -2.0), (0.0, 0.0) -> 2.5,

        distance_exact_halves_3_4_5: (0.5, 0.5), (4.5, 3.5) -> 5.0,
        distance_exact_fractions_5_12_13: (1.25, -0.5), (6.25, 11.5) -> 13.0,

        distance_tiny_scaled_1_over_1024: (0.0, 0.0), (0.0029296875, 0.00390625) -> 0.0048828125,

        distance_large_3k_4k_5k: (-1000.0, -2000.0), (2000.0, 2000.0) -> 5000.0,
        distance_large_offset_small_delta: (100000.0, 100000.0), (100003.0, 100004.0) -> 5.0,
        distance_negative_to_origin_6_8_10: (-8.0, 6.0), (0.0, 0.0) -> 10.0,
    ];
}
