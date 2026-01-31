use crate::math::{
    Vector4,
    number::{Atan2, Sqrt},
};
use std::ops::{Add, Mul, Sub};

impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Sqrt + Atan2 + Clone> Vector4<T> {
    /// Calculates the unsigned angle (`0..2π`) between two vectors
    pub fn angle_between(self, other: Self) -> T {
        let dot = self.clone().dot(other.clone());
        let wedge =
            (self.length_squared() * other.length_squared() - (dot.clone() * dot.clone())).sqrt();
        wedge.atan2(dot)
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{Vector4f, number::ApproxEq};

    macro_rules! angle_between_tests {
        [$(
            $test_name: ident:
                ($i1x: literal, $i1y: literal, $i1z: literal, $i1w: literal),
                ($i2x: literal, $i2y: literal, $i2z: literal, $i2w: literal)
                -> $o: literal,
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT1: Vector4f = Vector4f::new($i1x, $i1y, $i1z, $i1w);
                const INPUT2: Vector4f = Vector4f::new($i2x, $i2y, $i2z, $i2w);
                const OUTPUT: f32 = $o;

                let output = INPUT1.angle_between(INPUT2);

                assert!(output.approx_eq(OUTPUT, 1e-6), "angle between failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    angle_between_tests![
        angle_between_same_unit_x: (1.0, 0.0, 0.0, 0.0), (1.0, 0.0, 0.0, 0.0) -> 0.0,
        angle_between_opposite_unit_x: (1.0, 0.0, 0.0, 0.0), (-1.0, 0.0, 0.0, 0.0) -> 3.141592653589793,
        angle_between_orthogonal_xy: (1.0, 0.0, 0.0, 0.0), (0.0, 1.0, 0.0, 0.0) -> 1.5707963267948966,
        angle_between_orthogonal_zw: (0.0, 0.0, 1.0, 0.0), (0.0, 0.0, 0.0, 1.0) -> 1.5707963267948966,
        angle_between_scale_invariant: (2.0, 0.0, 0.0, 0.0), (1.0, 0.0, 0.0, 0.0) -> 0.0,
        angle_between_45deg_in_xy: (1.0, 0.0, 0.0, 0.0), (1.0, 1.0, 0.0, 0.0) -> 0.7853981633974483,
        angle_between_orthogonal_diag_xy: (1.0, 1.0, 0.0, 0.0), (1.0, -1.0, 0.0, 0.0) -> 1.5707963267948966,
        angle_between_acos_0_8: (1.0, 2.0, 0.0, 0.0), (2.0, 1.0, 0.0, 0.0) -> 0.6435011087932844,
        angle_between_acos_2_over_3: (1.0, 2.0, 3.0, 4.0), (4.0, 3.0, 2.0, 1.0) -> 0.8410686705679303,
        angle_between_pi_over_3: (0.5, 0.5, 0.5, 0.5), (1.0, 0.0, 0.0, 0.0) -> 1.0471975511965979,
    ];
}
