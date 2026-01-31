use crate::math::{Vector3, number::Atan2};
use std::ops::{Add, Div, Mul, Neg, Sub};

impl<
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Neg<Output = T>
        + Atan2
        + Clone,
> Vector3<T>
{
    /// Calculates the left-handed signed angle (`-π..π`) between two vectors around `normal`
    pub fn signed_angle(self, other: Self, normal: Self) -> T {
        let a = self.clone() - normal.clone() * self.dot(normal.clone());
        let b = other.clone() - normal.clone() * other.dot(normal.clone());
        -normal.dot(a.clone().cross(b.clone())).atan2(a.dot(b))
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{Vector3f, number::ApproxEq};

    macro_rules! signed_angle_tests {
        [$(
            $test_name: ident:
                ($i1x: literal, $i1y: literal, $i1z: literal),
                ($i2x: literal, $i2y: literal, $i2z: literal),
                ($nx: literal, $ny: literal, $nz: literal)
                -> $o: literal,
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT1: Vector3f = Vector3f::new($i1x, $i1y, $i1z);
                const INPUT2: Vector3f = Vector3f::new($i2x, $i2y, $i2z);
                const NORMAL: Vector3f = Vector3f::new($nx, $ny, $nz);
                const OUTPUT: f32 = $o;

                let output = INPUT1.signed_angle(INPUT2, NORMAL);

                assert!(output.approx_eq(OUTPUT, 1e-6), "signed angle failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    signed_angle_tests![
        signed_angle_x_to_y_about_pos_z_is_neg_pi_over_2: (1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0) -> -1.5707964,
        signed_angle_y_to_x_about_pos_z_is_pos_pi_over_2: (0.0, 1.0, 0.0), (1.0, 0.0, 0.0), (0.0, 0.0, 1.0) -> 1.5707964,

        signed_angle_x_to_y_about_neg_z_is_pos_pi_over_2: (1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, -1.0) -> 1.5707964,
        signed_angle_y_to_x_about_neg_z_is_neg_pi_over_2: (0.0, 1.0, 0.0), (1.0, 0.0, 0.0), (0.0, 0.0, -1.0) -> -1.5707964,

        signed_angle_x_to_diag_xy_about_pos_z_is_neg_pi_over_4: (1.0, 0.0, 0.0), (1.0, 1.0, 0.0), (0.0, 0.0, 1.0) -> -0.7853982,
        signed_angle_diag_xy_to_x_about_pos_z_is_pos_pi_over_4: (1.0, 1.0, 0.0), (1.0, 0.0, 0.0), (0.0, 0.0, 1.0) -> 0.7853982,

        signed_angle_x_to_135deg_about_pos_z_is_neg_3pi_over_4: (1.0, 0.0, 0.0), (-1.0, 1.0, 0.0), (0.0, 0.0, 1.0) -> -2.3561945,
        signed_angle_135deg_to_x_about_pos_z_is_pos_3pi_over_4: (-1.0, 1.0, 0.0), (1.0, 0.0, 0.0), (0.0, 0.0, 1.0) -> 2.3561945,

        signed_angle_y_to_z_about_pos_x_is_neg_pi_over_2: (0.0, 1.0, 0.0), (0.0, 0.0, 1.0), (1.0, 0.0, 0.0) -> -1.5707964,
        signed_angle_z_to_y_about_pos_x_is_pos_pi_over_2: (0.0, 0.0, 1.0), (0.0, 1.0, 0.0), (1.0, 0.0, 0.0) -> 1.5707964,

        signed_angle_parallel_same_direction_is_zero: (2.0, 0.0, 0.0), (5.0, 0.0, 0.0), (0.0, 0.0, 1.0) -> 0.0,
        signed_angle_parallel_same_direction_scaled_normal_is_zero: (2.0, 0.0, 0.0), (5.0, 0.0, 0.0), (0.0, 0.0, 2.0) -> 0.0,

        signed_angle_with_components_along_normal_projects_to_neg_pi_over_2: (1.0, 0.0, 1.0), (0.0, 1.0, 1.0), (0.0, 0.0, 1.0) -> -1.5707964,

        signed_angle_x_to_1deg_about_pos_z_is_neg_1deg: (1.0, 0.0, 0.0), (0.9998477, 0.017452406, 0.0), (0.0, 0.0, 1.0) -> -0.017453292,
        signed_angle_1deg_to_x_about_pos_z_is_pos_1deg: (0.9998477, 0.017452406, 0.0), (1.0, 0.0, 0.0), (0.0, 0.0, 1.0) -> 0.017453292,
    ];
}
