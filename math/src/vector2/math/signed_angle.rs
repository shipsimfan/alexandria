use crate::{Vector2, number::Atan2};
use std::ops::{Add, Mul, Neg, Sub};

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Neg<Output = T> + Atan2 + Clone>
    Vector2<T>
{
    /// Calculates the left-handed signed angle (`-π..π`) between two vectors around `normal`
    pub fn signed_angle(self, other: Self) -> T {
        -self.clone().cross(other.clone()).atan2(self.dot(other))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Vector2f, number::ApproxEq};

    macro_rules! signed_angle_tests {
        [$(
            $test_name: ident:
                ($i1x: literal, $i1y: literal),
                ($i2x: literal, $i2y: literal)
                -> $o: literal,
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT1: Vector2f = Vector2f::new($i1x, $i1y);
                const INPUT2: Vector2f = Vector2f::new($i2x, $i2y);
                const OUTPUT: f32 = $o;

                let output = INPUT1.signed_angle(INPUT2);

                assert!(output.approx_eq(OUTPUT, 1e-6), "signed angle failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    signed_angle_tests![
        signed_angle_same_dir_x: (1.0, 0.0), (1.0, 0.0) -> 0.0,
        signed_angle_ccw_90: (1.0, 0.0), (0.0, 1.0) -> -1.5707963705062866,
        signed_angle_cw_90: (1.0, 0.0), (0.0, -1.0) -> 1.5707963705062866,
        signed_angle_opposite_x: (1.0, 0.0), (-1.0, 0.0) -> -3.1415927410125732,

        signed_angle_45_ccw: (1.0, 0.0), (1.0, 1.0) -> -0.7853981852531433,
        signed_angle_45_cw: (1.0, 0.0), (1.0, -1.0) -> 0.7853981852531433,

        signed_angle_y_to_x: (0.0, 1.0), (1.0, 0.0) -> 1.5707963705062866,
        signed_angle_y_to_negx: (0.0, 1.0), (-1.0, 0.0) -> -1.5707963705062866,

        signed_angle_diag_to_y: (1.0, 1.0), (0.0, 1.0) -> -0.7853981852531433,
        signed_angle_diag_to_leftup: (1.0, 1.0), (-1.0, 1.0) -> -1.5707963705062866,

        signed_angle_scale_invariant: (2.0, 0.0), (0.0, 4.0) -> -1.5707963705062866,

        signed_angle_atan3_4_ccw: (1.0, 0.0), (4.0, 3.0) -> -0.6435011029243469,
        signed_angle_atan3_4_cw: (1.0, 0.0), (4.0, -3.0) -> 0.6435011029243469,

        signed_angle_arbitrary_1: (2.0, 3.0), (-4.0, 5.0) -> -1.262743592262268,
        signed_angle_arbitrary_2: (-2.0, 1.0), (3.0, -6.0) -> -2.498091459274292,

        signed_angle_near_pi_ccw: (1.0, 0.0), (-1.0, 0.001) -> -3.140592575073242,
    ];
}
