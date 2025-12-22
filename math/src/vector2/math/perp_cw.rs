use crate::Vector2;
use std::{marker::Destruct, ops::Neg};

impl<T> Vector2<T> {
    /// Calculate a clockwise perpendicular vector to this one
    pub const fn perp_cw(self) -> Self
    where
        T: [const] Neg<Output = T> + [const] Destruct,
    {
        Vector2::new(self.y, -self.x)
    }
}

#[cfg(test)]
mod tests {
    use crate::Vector2f;

    macro_rules! perp_cw_tests {
        [$(
            $test_name: ident:
                ($ix: literal, $iy: literal) -> ($ox: literal, $oy: literal),
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Vector2f = Vector2f::new($ix, $iy);
                const OUTPUT: Vector2f = Vector2f::new($ox, $oy);

                let output = INPUT.perp_cw();

                assert!(output.approx_eq(OUTPUT, 1e-6), "perp cw failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    perp_cw_tests![
        perp_cw_zero: (0.0, 0.0) -> (0.0, 0.0),

        perp_cw_unit_x: (1.0, 0.0) -> (0.0, -1.0),
        perp_cw_unit_y: (0.0, 1.0) -> (1.0, 0.0),
        perp_cw_neg_unit_x: (-1.0, 0.0) -> (0.0, 1.0),
        perp_cw_neg_unit_y: (0.0, -1.0) -> (-1.0, 0.0),

        perp_cw_axes_pos_x: (7.0, 0.0) -> (0.0, -7.0),
        perp_cw_axes_neg_y: (0.0, -2.5) -> (-2.5, 0.0),

        perp_cw_int_pos: (2.0, 3.0) -> (3.0, -2.0),
        perp_cw_int_mixed_1: (-2.0, 3.0) -> (3.0, 2.0),
        perp_cw_int_mixed_2: (2.0, -3.0) -> (-3.0, -2.0),
        perp_cw_int_neg: (-2.0, -3.0) -> (-3.0, 2.0),

        perp_cw_frac_mixed_1: (0.5, -1.25) -> (-1.25, -0.5),
        perp_cw_frac_mixed_2: (-0.5, 1.25) -> (1.25, 0.5),

        perp_cw_constants: (3.1415927, 2.7182817) -> (2.7182817, -3.1415927),
        perp_cw_constants_negx: (-3.1415927, 2.7182817) -> (2.7182817, 3.1415927),

        perp_cw_large_small: (10000000000.0, -0.001) -> (-0.001, -10000000000.0),
        perp_cw_large_mixed: (123456.75, -98765.5) -> (-98765.5, -123456.75),

        perp_cw_small_pos: (0.0001, 0.0002) -> (0.0002, -0.0001),
        perp_cw_small_mixed: (-0.0003, 0.0004) -> (0.0004, 0.0003),

        perp_cw_many_decimals: (1.234567, -8.765432) -> (-8.765432, -1.234567),
    ];
}
