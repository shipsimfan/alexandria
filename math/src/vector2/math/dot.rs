use crate::Vector2;
use std::{
    marker::Destruct,
    ops::{Add, Mul},
};

impl<T> Vector2<T> {
    /// Calculate the dot product between this [`Vector2`] and `rhs`
    pub const fn dot(self, rhs: Self) -> T
    where
        T: [const] Add<Output = T> + [const] Mul<Output = T> + [const] Destruct,
    {
        self.x * rhs.x + self.y * rhs.y
    }
}

#[cfg(test)]
mod tests {
    use crate::{Vector2f, number::ApproxEq};

    macro_rules! dot_tests {
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

                let output = INPUT1.dot(INPUT2);

                assert!(output.approx_eq(OUTPUT, 1e-6), "dot failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    dot_tests![
        dot_zero_zero: (0.0, 0.0), (0.0, 0.0) -> 0.0,
        dot_unit_x_unit_x: (1.0, 0.0), (1.0, 0.0) -> 1.0,
        dot_unit_x_unit_y: (1.0, 0.0), (0.0, 1.0) -> 0.0,
        dot_simple_integers: (1.0, 2.0), (3.0, 4.0) -> 11.0,
        dot_mixed_signs: (-1.0, 2.0), (3.0, -4.0) -> -11.0,
        dot_both_negative_mixed: (-2.0, -5.0), (-3.0, 4.0) -> -14.0,
        dot_fractions_cancel: (0.5, 0.25), (2.0, -4.0) -> 0.0,
        dot_fractional_result: (1.5, -0.5), (-2.0, 3.0) -> -4.5,
        dot_small_fractions: (0.75, 0.5), (0.25, 1.5) -> 0.9375,
        dot_large_scale: (1000.0, -2000.0), (3.0, 0.5) -> 2000.0,
        dot_opposites: (7.0, -9.0), (-7.0, 9.0) -> -130.0,
        dot_axis_projection: (2.0, 2.0), (2.0, 0.0) -> 4.0,
    ];
}
