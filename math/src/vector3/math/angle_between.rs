use crate::{
    Vector3,
    number::{Atan2, Sqrt},
};
use std::ops::{Add, Mul, Sub};

impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Sqrt + Atan2 + Clone> Vector3<T> {
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
    use crate::{Vector3f, number::ApproxEq};

    macro_rules! angle_between_tests {
        [$(
            $test_name: ident:
                ($i1x: literal, $i1y: literal, $i1z: literal),
                ($i2x: literal, $i2y: literal, $i2z: literal)
                -> $o: literal,
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT1: Vector3f = Vector3f::new($i1x, $i1y, $i1z);
                const INPUT2: Vector3f = Vector3f::new($i2x, $i2y, $i2z);
                const OUTPUT: f32 = $o;

                let output = INPUT1.angle_between(INPUT2);

                assert!(output.approx_eq(OUTPUT, 1e-6), "angle between failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    angle_between_tests![
        angle_between_same_unit_x: (1.0, 0.0, 0.0), (1.0, 0.0, 0.0) -> 0.000000000,
        angle_between_opposite_unit_x: (1.0, 0.0, 0.0), (-1.0, 0.0, 0.0) -> 3.141592654,
        angle_between_orthogonal_xy: (1.0, 0.0, 0.0), (0.0, 1.0, 0.0) -> 1.570796327,

        angle_between_pi_over_4: (1.0, 0.0, 0.0), (1.0, 1.0, 0.0) -> 0.785398163,
        angle_between_pi_over_6: (1.0, 0.0, 0.0), (1.732050808, 1.0, 0.0) -> 0.523598776,
        angle_between_pi_over_3: (1.0, 0.0, 0.0), (1.0, 1.732050808, 0.0) -> 1.047197551,

        angle_between_two_pi_over_3: (1.0, 0.0, 0.0), (-1.0, 1.732050808, 0.0) -> 2.094395102,
        angle_between_three_pi_over_4: (1.0, 0.0, 0.0), (-1.0, -1.0, 0.0) -> 2.356194490,
        angle_between_five_pi_over_6: (1.0, 0.0, 0.0), (-1.732050808, 1.0, 0.0) -> 2.617993878,

        angle_between_scaled_same_direction: (1.0, 2.0, 3.0), (2.0, 4.0, 6.0) -> 0.000000000,
        angle_between_scaled_opposite_direction: (1.0, 2.0, 3.0), (-2.0, -4.0, -6.0) -> 3.141592654,

        angle_between_arbitrary_3d_1: (1.0, 2.0, 3.0), (4.0, 5.0, 6.0) -> 0.225726129,
        angle_between_arbitrary_3d_2: (1.0, -2.0, 3.0), (-4.0, 5.0, -6.0) -> 2.915866525,
        angle_between_arbitrary_3d_orthogonal: (2.0, 0.0, -1.0), (1.0, 2.0, 2.0) -> 1.570796327,
    ];
}
