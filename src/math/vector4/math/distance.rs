use crate::math::{Vector4, number::Sqrt};
use std::ops::{Add, Mul, Sub};

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Sqrt + Clone> Vector4<T> {
    /// Calculate the distance between this vector and `rhs`
    pub fn distance(self, rhs: Self) -> T {
        self.distance_squared(rhs).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{Vector4f, number::ApproxEq};

    macro_rules! distance_tests {
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

                let output = INPUT1.distance(INPUT2);

                assert!(output.approx_eq(OUTPUT, 1e-6), "distance failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    distance_tests![
        distance_zero: (0.0, 0.0, 0.0, 0.0), (0.0, 0.0, 0.0, 0.0) -> 0.0,
        distance_unit_x: (0.0, 0.0, 0.0, 0.0), (1.0, 0.0, 0.0, 0.0) -> 1.0,
        distance_unit_w: (0.0, 0.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0) -> 1.0,
        distance_diag_4d: (0.0, 0.0, 0.0, 0.0), (1.0, 1.0, 1.0, 1.0) -> 2.0,
        distance_one_component_change: (2.0, 2.0, 2.0, 2.0), (2.0, 2.0, 2.0, -2.0) -> 4.0,
        distance_pythagorean: (1.0, 2.0, 3.0, 4.0), (5.0, 6.0, 7.0, 8.0) -> 8.0,
        distance_fractional_small: (0.1, 0.2, 0.3, 0.4), (0.4, 0.3, 0.2, 0.1) -> 0.4472136,
        distance_swap_sym: (3.5, -2.25, 0.75, -1.5), (-1.0, 4.25, -3.25, 2.5) -> 9.721111,
        distance_neg_pos_mix: (-1.0, 2.0, -3.0, 4.0), (5.0, -6.0, 7.0, -8.0) -> 18.547237,
        distance_large_coords: (1000.0, 2000.0, -3000.0, 4000.0), (-1000.0, 1500.0, 2500.0, -3500.0) -> 9526.279442,
    ];
}
