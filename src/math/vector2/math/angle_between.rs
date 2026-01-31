use crate::math::{
    Vector2,
    number::{Atan2, FromF32},
};
use std::ops::{Add, Mul, Neg, Rem, Sub};

impl<
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Neg<Output = T>
        + Rem<Output = T>
        + Atan2
        + Clone
        + FromF32,
> Vector2<T>
{
    /// Calculates the left-handed unsigned angle (`0..2π`) between two vectors
    pub fn angle_between(self, other: Vector2<T>) -> T {
        let pi2 = T::from_f32(6.2831853071);
        ((-self.clone().cross(other.clone())).atan2(self.dot(other)) + pi2.clone()) % pi2
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{Vector2f, number::ApproxEq};

    macro_rules! angle_between_tests {
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

                let output = INPUT1.angle_between(INPUT2);

                assert!(output.approx_eq(OUTPUT, 1e-6), "angle between failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    angle_between_tests![
        angle_between_same_direction_x: (1.0, 0.0), (2.0, 0.0) -> 0.0,
        angle_between_opposite_direction_x: (1.0, 0.0), (-3.0, 0.0) -> 3.141592653589793,

        angle_between_perp_clockwise_x_to_neg_y: (1.0, 0.0), (0.0, -1.0) -> 1.5707963267948966,
        angle_between_perp_counterclockwise_x_to_pos_y: (1.0, 0.0), (0.0, 1.0) -> 4.71238898038469,

        angle_between_perp_clockwise_y_to_x: (0.0, 1.0), (1.0, 0.0) -> 1.5707963267948966,
        angle_between_perp_counterclockwise_y_to_neg_x: (0.0, 1.0), (-1.0, 0.0) -> 4.71238898038469,

        angle_between_45deg_clockwise_x_to_down_right: (1.0, 0.0), (1.0, -1.0) -> 0.7853981633974483,
        angle_between_45deg_counterclockwise_x_to_up_right: (1.0, 0.0), (1.0, 1.0) -> 5.497787143782138,

        angle_between_30deg_clockwise_x_to_minus30: (1.0, 0.0), (0.8660254, -0.5) -> 0.5235987755982988,
        angle_between_30deg_counterclockwise_x_to_plus30: (1.0, 0.0), (0.8660254, 0.5) -> 5.759586531581287,

        angle_between_90deg_clockwise_diag45_to_diag_minus45: (1.0, 1.0), (1.0, -1.0) -> 1.5707963267948966,
        angle_between_90deg_counterclockwise_diag45_to_diag135: (1.0, 1.0), (-1.0, 1.0) -> 4.71238898038469,

        angle_between_180deg_diag45_to_diag225: (1.0, 1.0), (-1.0, -1.0) -> 3.141592653589793,

        angle_between_non_normalized_90deg_clockwise: (2.0, 0.0), (0.0, -5.0) -> 1.5707963267948966,
        angle_between_non_normalized_90deg_counterclockwise: (0.0, -4.0), (3.0, 0.0) -> 4.71238898038469,
    ];
}
