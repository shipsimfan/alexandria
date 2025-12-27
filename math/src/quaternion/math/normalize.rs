use crate::{
    Quaternion,
    number::{One, Sqrt, Zero},
};
use std::ops::{Add, Div, Mul};

impl<T: Add<Output = T> + Mul<Output = T> + Div<Output = T> + Clone + PartialEq + Zero + One + Sqrt>
    Quaternion<T>
{
    /// Normalize the values in the this quaternion, making its length 1
    pub fn normalize(&mut self) -> &mut Self {
        *self = self.clone().normalized();
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::Quaternionf;

    macro_rules! normalize_tests {
        [$(
            $test_name: ident:
                ($ix: literal, $iy: literal, $iz: literal, $iw: literal)
                -> ($ox: literal, $oy: literal, $oz: literal, $ow: literal),
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Quaternionf = Quaternionf::new($ix, $iy, $iz, $iw);
                const OUTPUT: Quaternionf = Quaternionf::new($ox, $oy, $oz, $ow);

                let mut output = INPUT.clone();
                output.normalize();

                assert!(output.approx_eq(OUTPUT, 1e-6), "normalize failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    normalize_tests![
        normalize_zero: (0.0, 0.0, 0.0, 0.0) -> (0.0, 0.0, 0.0, 0.0),
        normalize_x_axis_scaled: (2.0, 0.0, 0.0, 0.0) -> (1.0, 0.0, 0.0, 0.0),
        normalize_w_axis_negative: (0.0, 0.0, 0.0, -5.0) -> (0.0, 0.0, 0.0, -1.0),
        normalize_all_ones: (1.0, 1.0, 1.0, 1.0) -> (0.5, 0.5, 0.5, 0.5),
        normalize_half_ones_unit: (0.5, 0.5, 0.5, 0.5) -> (0.5, 0.5, 0.5, 0.5),
        normalize_three_four: (3.0, 4.0, 0.0, 0.0) -> (0.600000024, 0.800000012, 0.0, 0.0),
        normalize_neg_three_four: (0.0, -3.0, -4.0, 0.0) -> (0.0, -0.600000024, -0.800000012, 0.0),
        normalize_xy_opposite: (1.0, -1.0, 0.0, 0.0) -> (0.707106769, -0.707106769, 0.0, 0.0),
        normalize_xw_diagonal: (1.0, 0.0, 0.0, 1.0) -> (0.707106769, 0.0, 0.0, 0.707106769),
        normalize_yzw_three: (0.0, 1.0, 2.0, 2.0) -> (0.0, 0.333333343, 0.666666687, 0.666666687),
        normalize_xyw_three: (1.0, 2.0, 0.0, 2.0) -> (0.333333343, 0.666666687, 0.0, 0.666666687),
        normalize_alternating_twos: (2.0, -2.0, 2.0, -2.0) -> (0.5, -0.5, 0.5, -0.5),
        normalize_scaled_1_2_2_1: (0.1, 0.2, 0.2, 0.1) -> (0.316227764, 0.632455528, 0.632455528, 0.316227764),
        normalize_xyz_unit: (1.0, 1.0, 1.0, 0.0) -> (0.577350259, 0.577350259, 0.577350259, 0.0),
    ];
}
