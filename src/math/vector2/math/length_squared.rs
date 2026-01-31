use crate::math::Vector2;
use std::{
    marker::Destruct,
    ops::{Add, Mul},
};

impl<T> Vector2<T> {
    /// Calculate the squared length of this vector
    pub const fn length_squared(self) -> T
    where
        T: [const] Add<Output = T> + [const] Mul<Output = T> + [const] Clone + [const] Destruct,
    {
        self.clone().dot(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{Vector2f, number::ApproxEq};

    macro_rules! length_squared_tests {
        [$(
            $test_name: ident: ($ix: literal, $iy: literal) -> $o: literal,
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Vector2f = Vector2f::new($ix, $iy);
                const OUTPUT: f32 = $o;

                let output = INPUT.length_squared();

                assert!(output.approx_eq(OUTPUT, 1e-6), "length squared failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    length_squared_tests![
        length_squared_zero: (0.0, 0.0)-> 0.0,
        length_squared_unit_x: (1.0, 0.0)-> 1.0,
        length_squared_unit_y: (0.0, 1.0)-> 1.0,
        length_squared_neg_unit_x: (-1.0, 0.0)-> 1.0,
        length_squared_neg_unit_y: (0.0, -1.0)-> 1.0,

        length_squared_two_three: (2.0, 3.0)-> 13.0,
        length_squared_neg_two_three: (-2.0, 3.0)-> 13.0,
        length_squared_two_neg_three: (2.0, -3.0)-> 13.0,
        length_squared_neg_two_neg_three: (-2.0, -3.0)-> 13.0,

        length_squared_three_four: (3.0, 4.0)-> 25.0,
        length_squared_five_twelve: (5.0, 12.0)-> 169.0,
        length_squared_eight_fifteen: (8.0, 15.0)-> 289.0,

        length_squared_half_quarter: (0.5, 0.25)-> 0.3125,
        length_squared_three_quarters_half: (0.75, 0.5)-> 0.8125,
        length_squared_one_and_half_two_and_half: (1.5, 2.5)-> 8.5,
        length_squared_neg_one_and_half_two_and_half: (-1.5, 2.5)-> 8.5,

        length_squared_small_decimals: (0.1, 0.2)-> 0.05,
        length_squared_mixed_decimals: (0.2, 0.3)-> 0.13,
        length_squared_point_six_point_eight: (0.6, 0.8)-> 1.0,

        length_squared_large_safe: (1000.0, 2000.0)-> 5000000.0,
        length_squared_symmetric_large: (12345.0, 12345.0)-> 304798050.0,
        length_squared_neg_large_pair: (-30000.0, 40000.0)-> 2500000000.0,

        length_squared_axis_only_large: (46340.0, 0.0)-> 2147395600.0,
        length_squared_axis_only_medium: (32768.0, 0.0)-> 1073741824.0,

        length_squared_sign_mix: (-7.0, 24.0)-> 625.0,
        length_squared_another_pythag: (9.0, 40.0)-> 1681.0,
    ];
}
