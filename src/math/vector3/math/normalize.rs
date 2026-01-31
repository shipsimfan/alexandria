use crate::math::{
    Vector3,
    number::{One, Sqrt, Zero},
};
use std::ops::{Add, Div, Mul};

impl<T: Add<Output = T> + Mul<Output = T> + Div<Output = T> + Clone + PartialEq + Zero + One + Sqrt>
    Vector3<T>
{
    /// Normalize the values in the this vector, making its length 1
    pub fn normalize(&mut self) -> &mut Self {
        *self = self.clone().normalized();
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::math::Vector3f;

    macro_rules! normalize_tests {
        [$(
            $test_name: ident:
                ($ix: literal, $iy: literal, $iz: literal)
                -> ($ox: literal, $oy: literal, $oz: literal),
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Vector3f = Vector3f::new($ix, $iy, $iz);
                const OUTPUT: Vector3f = Vector3f::new($ox, $oy, $oz);

                let mut output = INPUT.clone();
                output.normalize();

                assert!(output.approx_eq(OUTPUT, 1e-6), "normalize failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    normalize_tests![
        normalize_axis_x_pos: (1.0, 0.0, 0.0) -> (1.0, 0.0, 0.0),
        normalize_axis_y_scaled: (0.0, 2.0, 0.0) -> (0.0, 1.0, 0.0),
        normalize_axis_z_neg: (0.0, 0.0, -7.0) -> (0.0, 0.0, -1.0),

        normalize_3_4_0: (3.0, 4.0, 0.0) -> (0.6, 0.8, 0.0),
        normalize_neg3_4_0: (-3.0, 4.0, 0.0) -> (-0.6, 0.8, 0.0),

        normalize_0_neg5_12: (0.0, -5.0, 12.0) -> (0.0, -0.38461538, 0.9230769),
        normalize_2_neg3_6: (2.0, -3.0, 6.0) -> (0.2857143, -0.42857143, 0.85714287),

        normalize_1_2_2: (1.0, 2.0, 2.0) -> (0.33333334, 0.6666667, 0.6666667),
        normalize_neg1_neg2_neg2: (-1.0, -2.0, -2.0) -> (-0.33333334, -0.6666667, -0.6666667),

        normalize_half_x: (0.5, 0.0, 0.0) -> (1.0, 0.0, 0.0),
        normalize_small_diag_xy: (0.0001, -0.0001, 0.0) -> (0.70710677, -0.70710677, 0.0),

        normalize_zero: (0.0, 0.0, 0.0) -> (0.0, 0.0, 0.0),
    ];
}
