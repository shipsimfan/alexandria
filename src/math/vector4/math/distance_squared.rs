use crate::math::Vector4;
use std::{
    marker::Destruct,
    ops::{Add, Mul, Sub},
};

impl<T> Vector4<T> {
    /// Calculate the squared distance between this vector and `rhs`
    pub const fn distance_squared(self, rhs: Self) -> T
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
    use crate::math::{Vector4f, number::ApproxEq};

    macro_rules! distance_squared_tests {
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

                let output = INPUT1.distance_squared(INPUT2);

                assert!(output.approx_eq(OUTPUT, 1e-6), "distance squared failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    distance_squared_tests![
        distance_squared_zero: (0.0, 0.0, 0.0, 0.0), (0.0, 0.0, 0.0, 0.0) -> 0.0,
        distance_squared_unit_x: (0.0, 0.0, 0.0, 0.0), (1.0, 0.0, 0.0, 0.0) -> 1.0,
        distance_squared_unit_y: (0.0, 0.0, 0.0, 0.0), (0.0, 1.0, 0.0, 0.0) -> 1.0,
        distance_squared_unit_z: (0.0, 0.0, 0.0, 0.0), (0.0, 0.0, 1.0, 0.0) -> 1.0,
        distance_squared_unit_w: (0.0, 0.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0) -> 1.0,

        distance_squared_all_ones: (0.0, 0.0, 0.0, 0.0), (1.0, 1.0, 1.0, 1.0) -> 4.0,
        distance_squared_mixed_signs_small: (1.0, -2.0, 3.0, -4.0), (-1.0, 2.0, -3.0, 4.0) -> 120.0,
        distance_squared_translation_invariant: (5.0, 6.0, 7.0, 8.0), (6.0, 8.0, 10.0, 12.0) -> 30.0,

        distance_squared_symmetry_a_to_b: (2.0, 4.0, 6.0, 8.0), (1.0, 3.0, 5.0, 7.0) -> 4.0,
        distance_squared_symmetry_b_to_a: (1.0, 3.0, 5.0, 7.0), (2.0, 4.0, 6.0, 8.0) -> 4.0,

        distance_squared_one_axis_large_gap: (1000.0, 0.0, 0.0, 0.0), (-1000.0, 0.0, 0.0, 0.0) -> 4000000.0,
        distance_squared_two_axes_pythagorean: (0.0, 0.0, 0.0, 0.0), (3.0, 4.0, 0.0, 0.0) -> 25.0,
        distance_squared_three_axes_pythagorean: (0.0, 0.0, 0.0, 0.0), (2.0, 3.0, 6.0, 0.0) -> 49.0,
        distance_squared_four_axes_pythagorean: (0.0, 0.0, 0.0, 0.0), (1.0, 2.0, 2.0, 3.0) -> 18.0,

        distance_squared_nonzero_origin: (10.0, 10.0, 10.0, 10.0), (13.0, 14.0, 16.0, 19.0) -> 142.0,
        distance_squared_canceling_components: (8.0, -8.0, 8.0, -8.0), (-8.0, 8.0, -8.0, 8.0) -> 1024.0,

        distance_squared_fraction_exact_halves: (0.5, 1.5, -2.0, -3.5), (-0.5, 0.5, -1.0, -1.5) -> 7.0,
        distance_squared_fraction_exact_quarters: (0.25, -0.75, 1.25, -1.75), (1.25, 0.25, -0.75, 0.25) -> 10.0,
        distance_squared_nearby_points: (1.0, 2.0, 3.0, 4.0), (1.0, 2.0, 3.0, 5.0) -> 1.0,
        distance_squared_spread_axes: (-3.0, 7.0, -11.0, 13.0), (5.0, -1.0, 9.0, -7.0) -> 928.0,
    ];
}
