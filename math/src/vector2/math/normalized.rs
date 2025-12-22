use crate::{
    Vector2,
    number::{One, Sqrt, Zero},
};
use std::ops::{Add, Div, Mul};

impl<T: Add<Output = T> + Mul<Output = T> + Div<Output = T> + Clone + PartialEq + Zero + One + Sqrt>
    Vector2<T>
{
    /// Normalize the values in the this vector, making its length 1
    pub fn normalized(self) -> Self {
        let length_squared = self.clone().length_squared();
        if length_squared == T::ONE {
            return self;
        }
        if length_squared == T::ZERO {
            return Vector2::ZERO;
        }

        self / length_squared.sqrt()
    }
}

#[cfg(test)]
mod tests {
    use crate::Vector2f;

    macro_rules! normalized_tests {
        [$(
            $test_name: ident:
                ($ix: literal, $iy: literal)
                -> ($ox: literal, $oy: literal),
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Vector2f = Vector2f::new($ix, $iy);
                const OUTPUT: Vector2f = Vector2f::new($ox, $oy);

                let output = INPUT.normalized();

                assert!(output.approx_eq(OUTPUT, 1e-6), "normalized failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    normalized_tests![
        normalized_zero: (0.0, 0.0) -> (0.0, 0.0),
        normalized_unit_x: (1.0, 0.0) -> (1.0, 0.0),
        normalized_unit_y: (0.0, 1.0) -> (0.0, 1.0),
        normalized_neg_unit_x: (-1.0, 0.0) -> (-1.0, 0.0),
        normalized_neg_unit_y: (0.0, -1.0) -> (0.0, -1.0),

        normalized_axis_x_scaled: (2.0, 0.0) -> (1.0, 0.0),
        normalized_axis_y_scaled: (0.0, 5.0) -> (0.0, 1.0),

        normalized_three_four: (3.0, 4.0) -> (0.6, 0.8),
        normalized_neg_three_four: (-3.0, 4.0) -> (-0.6, 0.8),

        normalized_diag: (1.0, 1.0) -> (0.70710677, 0.70710677),
        normalized_left_hand_clockwise45: (1.0, -1.0) -> (0.70710677, -0.70710677),

        normalized_small: (0.001, -0.002) -> (0.44721356, -0.8944271),
        normalized_mixed: (123.0, -456.0) -> (0.26042902, -0.96549296),

        normalized_pythag_5_12_13: (5.0, 12.0) -> (0.3846154, 0.9230769),
    ];
}
