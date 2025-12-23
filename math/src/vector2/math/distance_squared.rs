use crate::Vector2;
use std::{
    marker::Destruct,
    ops::{Add, Mul, Sub},
};

impl<T> Vector2<T> {
    /// Calculate the squared distance between this vector and `rhs`
    pub const fn distance_squared(self, rhs: Vector2<T>) -> T
    where
        T: [const] Add<Output = T>
            + [const] Sub<Output = T>
            + [const] Mul<Output = T>
            + [const] Clone
            + [const] Destruct,
    {
        (self - rhs).length_squared()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Vector2f, number::ApproxEq};

    macro_rules! distance_squared_tests {
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

                let output = INPUT1.distance_squared(INPUT2);

                assert!(output.approx_eq(OUTPUT, 1e-6), "distance squared failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    distance_squared_tests![
        distance_squared_same_point_origin: (0.0, 0.0), (0.0, 0.0) -> 0.0,
        distance_squared_same_point_nonzero: (3.5, -2.25), (3.5, -2.25) -> 0.0,

        distance_squared_axis_x_positive: (0.0, 0.0), (5.0, 0.0) -> 25.0,
        distance_squared_axis_y_positive: (0.0, 0.0), (0.0, 7.0) -> 49.0,
        distance_squared_axis_x_negative: (2.0, -1.0), (-4.0, -1.0) -> 36.0,
        distance_squared_axis_y_negative: (2.0, 3.0), (2.0, -5.0) -> 64.0,

        distance_squared_pythagorean_3_4_5: (0.0, 0.0), (3.0, 4.0) -> 25.0,
        distance_squared_pythagorean_shifted: (1.0, 2.0), (4.0, 6.0) -> 25.0,
        distance_squared_negative_coords: (-1.0, -2.0), (2.0, 2.0) -> 25.0,

        distance_squared_symmetric_order_ab: (1.0, -1.0), (4.0, 3.0) -> 25.0,
        distance_squared_symmetric_order_ba: (4.0, 3.0), (1.0, -1.0) -> 25.0,

        distance_squared_fractional_simple: (0.5, -0.5), (2.5, 1.5) -> 8.0,
        distance_squared_fractional_mixed: (-1.25, 3.75), (2.75, -0.25) -> 32.0,
        distance_squared_fractional_small: (0.25, 0.25), (0.75, -0.75) -> 1.25,

        distance_squared_large_but_safe: (1000.0, -1000.0), (1003.0, -996.0) -> 25.0,
        distance_squared_very_small_deltas: (1.0, 1.0), (1.001, 0.999) -> 0.000002,

        distance_squared_includes_negative_zero: (-0.0, 0.0), (0.0, -0.0) -> 0.0,
        distance_squared_rectangular_6_8_10: (10.0, -3.0), (4.0, 5.0) -> 100.0,
        distance_squared_diag_5_12_13: (-3.0, 4.0), (2.0, -8.0) -> 169.0,
        distance_squared_diff_signs: (-7.0, 9.0), (1.0, -3.0) -> 208.0,
    ];
}
