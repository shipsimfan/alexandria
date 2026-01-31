use crate::math::Vector2;
use std::{marker::Destruct, ops::Neg};

impl<T> Vector2<T> {
    /// Calculate a counter-clockwise perpendicular vector to this one
    pub const fn perp_ccw(self) -> Self
    where
        T: [const] Neg<Output = T> + [const] Destruct,
    {
        Vector2::new(-self.y, self.x)
    }
}

#[cfg(test)]
mod tests {
    use crate::math::Vector2f;

    macro_rules! perp_ccw_tests {
        [$(
            $test_name: ident:
                ($ix: literal, $iy: literal) -> ($ox: literal, $oy: literal),
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Vector2f = Vector2f::new($ix, $iy);
                const OUTPUT: Vector2f = Vector2f::new($ox, $oy);

                let output = INPUT.perp_ccw();

                assert!(output.approx_eq(OUTPUT, 1e-6), "perp ccw failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    perp_ccw_tests![
        perp_ccw_zero: (0.0, 0.0) -> (-0.0, 0.0),
        perp_ccw_unit_x: (1.0, 0.0) -> (-0.0, 1.0),
        perp_ccw_unit_y: (0.0, 1.0) -> (-1.0, 0.0),
        perp_ccw_neg_unit_x: (-1.0, 0.0) -> (-0.0, -1.0),
        perp_ccw_neg_unit_y: (0.0, -1.0) -> (1.0, 0.0),

        perp_ccw_pos_pos_int: (3.0, 4.0) -> (-4.0, 3.0),
        perp_ccw_pos_neg_int: (3.0, -4.0) -> (4.0, 3.0),
        perp_ccw_neg_pos_int: (-3.0, 4.0) -> (-4.0, -3.0),
        perp_ccw_neg_neg_int: (-3.0, -4.0) -> (4.0, -3.0),

        perp_ccw_axis_x_large: (123456.0, 0.0) -> (-0.0, 123456.0),
        perp_ccw_axis_y_large: (0.0, -123456.0) -> (123456.0, 0.0),

        perp_ccw_powers_of_two: (8.0, 32.0) -> (-32.0, 8.0),
        perp_ccw_mixed_scale: (1024.0, -2.0) -> (2.0, 1024.0),
        perp_ccw_small_powers: (0.125, 0.5) -> (-0.5, 0.125),
        perp_ccw_small_neg: (-0.25, 0.125) -> (-0.125, -0.25),

        perp_ccw_halves: (0.5, -0.5) -> (0.5, 0.5),
        perp_ccw_quarters: (1.25, 2.75) -> (-2.75, 1.25),
        perp_ccw_eighths: (-7.5, 0.25) -> (-0.25, -7.5),

        perp_ccw_many_zeros_x: (0.0, 6.5) -> (-6.5, 0.0),
        perp_ccw_many_zeros_y: (6.5, 0.0) -> (-0.0, 6.5),
    ];
}
