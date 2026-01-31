use crate::math::{
    Vector2,
    number::{One, Sqrt, Zero},
};
use std::ops::{Add, Div, Mul};

impl<T: Add<Output = T> + Mul<Output = T> + Div<Output = T> + Clone + PartialEq + Zero + One + Sqrt>
    Vector2<T>
{
    /// Normalize the values in the this vector, making its length 1
    pub fn normalize(&mut self) -> &mut Self {
        *self = self.clone().normalized();
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::math::Vector2f;

    macro_rules! normalize_tests {
        [$(
            $test_name: ident:
                ($ix: literal, $iy: literal)
                -> ($ox: literal, $oy: literal),
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Vector2f = Vector2f::new($ix, $iy);
                const OUTPUT: Vector2f = Vector2f::new($ox, $oy);

                let mut output = INPUT.clone();
                output.normalize();

                assert!(output.approx_eq(OUTPUT, 1e-6), "normalize failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    normalize_tests![
        normalize_zero: (0.0, 0.0) -> (0.0, 0.0),
        normalize_unit_x: (1.0, 0.0) -> (1.0, 0.0),
        normalize_unit_y: (0.0, 1.0) -> (0.0, 1.0),
        normalize_neg_unit_x: (-1.0, 0.0) -> (-1.0, 0.0),
        normalize_neg_unit_y: (0.0, -1.0) -> (0.0, -1.0),

        normalize_3_4: (3.0, 4.0) -> (0.6, 0.8),
        normalize_6_8: (6.0, 8.0) -> (0.6, 0.8),
        normalize_neg3_4: (-3.0, 4.0) -> (-0.6, 0.8),
        normalize_3_neg4: (3.0, -4.0) -> (0.6, -0.8),
        normalize_neg6_neg8: (-6.0, -8.0) -> (-0.6, -0.8),

        normalize_0_5_12: (0.0, 12.0) -> (0.0, 1.0),
        normalize_5_0: (5.0, 0.0) -> (1.0, 0.0),
    ];
}
