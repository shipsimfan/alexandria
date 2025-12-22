use crate::{
    Vector3,
    number::{One, Sqrt, Zero},
};
use std::ops::{Add, Div, Mul};

impl<T: Add<Output = T> + Mul<Output = T> + Div<Output = T> + Clone + PartialEq + Zero + One + Sqrt>
    Vector3<T>
{
    /// Normalize the values in the this vector, making its length 1
    pub fn normalized(self) -> Self {
        let length_squared = self.clone().length_squared();
        if length_squared == T::ONE {
            return self;
        }
        if length_squared == T::ZERO {
            return Vector3::ZERO;
        }

        self / length_squared.sqrt()
    }
}

#[cfg(test)]
mod tests {
    use crate::Vector3f;

    macro_rules! normalized_tests {
        [$(
            $test_name: ident:
                ($ix: literal, $iy: literal, $iz: literal)
                -> ($ox: literal, $oy: literal, $oz: literal),
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Vector3f = Vector3f::new($ix, $iy, $iz);
                const OUTPUT: Vector3f = Vector3f::new($ox, $oy, $oz);

                let output = INPUT.normalized();

                assert!(output.approx_eq(OUTPUT, 1e-6), "normalized failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    normalized_tests![
        normalized_axis_x_pos: (5.0, 0.0, 0.0) -> (1.0, 0.0, 0.0),
        normalized_axis_y_neg: (0.0, -7.0, 0.0) -> (0.0, -1.0, 0.0),
        normalized_axis_z_pos: (0.0, 0.0, 3.0) -> (0.0, 0.0, 1.0),

        normalized_pythag_3_4_0: (3.0, 4.0, 0.0) -> (0.6, 0.8, 0.0),
        normalized_pythag_0_3_4: (0.0, 3.0, 4.0) -> (0.0, 0.6, 0.8),
        normalized_pythag_6_8_0_neg: (-6.0, -8.0, 0.0) -> (-0.6, -0.8, 0.0),

        normalized_diag_xy: (1.0, 1.0, 0.0) -> (0.70710677, 0.70710677, 0.0),
        normalized_diag_xyz: (1.0, 1.0, 1.0) -> (0.57735026, 0.57735026, 0.57735026),

        normalized_mixed_sign: (2.0, -3.0, 6.0) -> (0.2857143, -0.42857143, 0.85714287),

        normalized_small_values: (0.001, 0.0, 0.0) -> (1.0, 0.0, 0.0),
        normalized_zero: (0.0, 0.0, 0.0) -> (0.0, 0.0, 0.0),
    ];
}
