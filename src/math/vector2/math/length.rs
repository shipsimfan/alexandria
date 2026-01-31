use crate::math::{Vector2, number::Sqrt};
use std::ops::{Add, Mul};

impl<T: Add<Output = T> + Mul<Output = T> + Sqrt + Clone> Vector2<T> {
    /// Calculate the length of this vector
    pub fn length(self) -> T {
        self.length_squared().sqrt()
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{Vector2f, number::ApproxEq};

    macro_rules! length_tests {
        [$(
            $test_name: ident: ($ix: literal, $iy: literal) -> $o: literal,
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Vector2f = Vector2f::new($ix, $iy);
                const OUTPUT: f32 = $o;

                let output = INPUT.length();

                assert!(output.approx_eq(OUTPUT, 1e-6), "length failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    length_tests![
        length_zero: (0.0, 0.0)-> 0.0,
        length_neg_zero: (0.0, -0.0)-> 0.0,
        length_x_axis_pos: (2.0, 0.0)-> 2.0,
        length_y_axis_neg: (0.0, -7.0)-> 7.0,
        length_neg_components_3_4_5: (-3.0, -4.0)-> 5.0,
        length_triple_5_12_13: (5.0, 12.0)-> 13.0,
        length_triple_8_15_17_mixed_sign: (8.0, -15.0)-> 17.0,
        length_scaled_3_4_5_by_1024: (4096.0, 3072.0)-> 5120.0,
        length_large_scaled_3_4_5_by_10000: (30000.0, 40000.0)-> 50000.0,
        length_fractional_exact_3_4_5_over_8: (0.375, 0.5)-> 0.625,
        length_fractional_exact_3_4_5_over_2: (1.5, 2.0)-> 2.5,
        length_negative_x_axis: (-1024.0, 0.0)-> 1024.0,
    ];
}
