use crate::math::{Vector3, number::Sqrt};
use std::ops::{Add, Mul};

impl<T: Add<Output = T> + Mul<Output = T> + Sqrt + Clone> Vector3<T> {
    /// Calculate the length of this vector
    pub fn length(self) -> T {
        self.length_squared().sqrt()
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{Vector3f, number::ApproxEq};

    macro_rules! length_tests {
        [$(
            $test_name: ident: ($ix: literal, $iy: literal, $iz: literal) -> $o: literal,
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Vector3f = Vector3f::new($ix, $iy, $iz);
                const OUTPUT: f32 = $o;

                let output = INPUT.length();

                assert!(output.approx_eq(OUTPUT, 1e-6), "length failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    length_tests![
        length_zero: (0.0, 0.0, 0.0) -> 0.0,
        length_axis_x_pos: (7.0, 0.0, 0.0) -> 7.0,
        length_axis_y_neg: (0.0, -9.0, 0.0) -> 9.0,
        length_axis_z_mix: (0.0, 0.0, -13.0) -> 13.0,

        length_xy_3_4_0: (3.0, 4.0, 0.0) -> 5.0,
        length_xy_5_12_0: (-5.0, 12.0, 0.0) -> 13.0,
        length_yz_0_5_12: (0.0, 5.0, -12.0) -> 13.0,
        length_xz_8_15_0: (8.0, 0.0, 15.0) -> 17.0,

        length_xyz_1_2_2: (1.0, 2.0, 2.0) -> 3.0,
        length_xyz_2_2_1: (-2.0, 2.0, 1.0) -> 3.0,
        length_xyz_2_3_6: (2.0, -3.0, 6.0) -> 7.0,
        length_xyz_1_4_8: (-1.0, 4.0, -8.0) -> 9.0,
        length_xyz_4_4_7: (4.0, -4.0, 7.0) -> 9.0,
        length_xyz_3_4_12: (-3.0, 4.0, 12.0) -> 13.0,
        length_xyz_9_12_20: (9.0, -12.0, 20.0) -> 25.0,

        length_large_3000_4000_0: (3000.0, -4000.0, 0.0) -> 5000.0,
        length_large_11_60_0: (-11.0, 60.0, 0.0) -> 61.0,
    ];
}
