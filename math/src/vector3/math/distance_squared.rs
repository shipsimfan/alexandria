use crate::Vector3;
use std::{
    marker::Destruct,
    ops::{Add, Mul, Sub},
};

impl<T> Vector3<T> {
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
    use crate::{Vector3f, number::ApproxEq};

    macro_rules! distance_squared_tests {
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

                let output = INPUT1.distance_squared(INPUT2);

                assert!(output.approx_eq(OUTPUT, 1e-6), "distance squared failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    distance_squared_tests![
        distance_squared_same_point: (0.0, 0.0, 0.0), (0.0, 0.0, 0.0) -> 0.0,
        distance_squared_unit_x: (0.0, 0.0, 0.0), (1.0, 0.0, 0.0) -> 1.0,
        distance_squared_3_4_0: (1.0, 2.0, 3.0), (4.0, 6.0, 3.0) -> 25.0,
        distance_squared_mixed_negatives: (-1.0, -2.0, -3.0), (2.0, 2.0, 1.0) -> 41.0,
        distance_squared_fractional_ends_in_quarter: (0.5, -1.5, 2.0), (-1.0, 0.5, -2.0) -> 22.25,
        distance_squared_large_symmetric: (1000.0, 2000.0, 3000.0), (-1000.0, 2000.0, 1000.0) -> 8000000.0,
        distance_squared_axis_z_5: (0.0, 0.0, 0.0), (0.0, 0.0, 5.0) -> 25.0,
        distance_squared_sum_to_200: (7.0, -8.0, 9.0), (-1.0, 2.0, 3.0) -> 200.0,
        distance_squared_binary_fractions: (0.25, 0.5, 0.75), (1.25, -0.5, 0.25) -> 2.25,
        distance_squared_offset_large_coords: (123456.0, -654321.0, 111111.0), (123400.0, -654000.0, 110000.0) -> 1340498.0,
        distance_squared_opposite_corners: (1.0, 1.0, 1.0), (-1.0, -1.0, -1.0) -> 12.0,
        distance_squared_random_small_ints: (-10.0, 5.0, -2.0), (4.0, -1.0, 3.0) -> 257.0,
    ];
}
